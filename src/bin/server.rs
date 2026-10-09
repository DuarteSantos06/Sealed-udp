use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::net::{SocketAddr, UdpSocket};
use std::path::Path;

use sealed_udp::crypto::{compute_shared, generate_keypair, random_nonce};
use sealed_udp::message::{
    MAX_CHUNK_SIZE, Message, Request, Response, parse_message, serialize_message
};
use sealed_udp::protocol::{
    parse_packet, serialize, DataPacket, Handshake, Packet, CONNECTION_ID_SIZE,
    VERSION,
};

const LISTEN_ADDR: &str = "127.0.0.1:8080";
const FILES_DIR: &str = "./files";
const RECV_BUF_SIZE: usize = 1500;


type ConnectionId = [u8; CONNECTION_ID_SIZE];

fn main() {
    let socket = match UdpSocket::bind(LISTEN_ADDR) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("bind failed: {}", e);
            return;
        }
    };
    println!("listening on {}", LISTEN_ADDR);

    let mut buf = [0u8; RECV_BUF_SIZE];
    loop {
        let (n, client_addr) = match socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("recv_from failed: {}", e);
                continue;
            }
        };

        match parse_packet(&buf[..n]) {
            Some(Packet::ClientHello(hello)) => handle_hello(&socket, &client_addr, &hello),
            Some(Packet::Data(packet)) => handle_data(&socket, &client_addr, &packet),
            _ => {} // garbage or a ServerHello: drop it, never crash
        }
    }
}

/// ClientHello -> ServerHello, then compute the shared secret.
fn handle_hello(socket: &UdpSocket, client_addr: &SocketAddr, hello: &Handshake) {
    let (secret, public) = generate_keypair();

    let reply = Packet::ServerHello(Handshake {
        version: VERSION,
        connection_id: hello.connection_id, // echo the client's ID
        nonce: random_nonce(),
        public_key: public.to_bytes(),
    });
    match socket.send_to(&serialize(&reply), client_addr) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("sending ServerHello failed: {}", e);
            return;
        }
    }

    let _shared = compute_shared(secret, hello.public_key);
    // Next step: store this in a session map keyed by connection_id.
}

/// Open the DataPacket's payload. The server only answers Requests.
fn handle_data(socket: &UdpSocket, client_addr: &SocketAddr, packet: &DataPacket) {
    match parse_message(&packet.data) {
        Some(Message::Request(request)) => {
            handle_request(socket, client_addr, packet.connection_id, &request);
        }
        _ => {}
}
}

/// Read the requested chunk and send it back as a Response.
fn handle_request(
    socket: &UdpSocket,
    client_addr: &SocketAddr,
    connection_id: ConnectionId,
    request: &Request,
) {
    let Some(data) = read_chunk(request) else { return };

    let response = Message::Response(Response {
        request_id: request.request_id,
        offset: request.offset,
        data,
    });
    let packet = Packet::Data(DataPacket {
        connection_id,
        data: serialize_message(&response),
    });

    if let Err(e) = socket.send_to(&serialize(&packet), client_addr) {
        eprintln!("sending Response failed: {}", e);
    }
}

/// Read up to `request.length` bytes at `request.offset` from a file in FILES_DIR.
fn read_chunk(request: &Request) -> Option<Vec<u8>> {
    // Path traversal guard: only plain file names inside FILES_DIR.
    if request.filename.contains("..") || request.filename.contains('/') {
        return None;
    }
    let path = Path::new(FILES_DIR).join(&request.filename);

    let mut file = File::open(path).ok()?;
    file.seek(SeekFrom::Start(request.offset)).ok()?;

    let len = (request.length as usize).min(MAX_CHUNK_SIZE);
    let mut data = Vec::with_capacity(len);
    file.take(len as u64).read_to_end(&mut data).ok()?;
    return Some(data)
}