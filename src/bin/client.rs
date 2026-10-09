use std::fs::File;
use std::io::Write;
use std::net::UdpSocket;

use sealed_udp::crypto::{compute_shared, generate_keypair, random_connection_id, random_nonce};
use sealed_udp::message::{
    parse_message, serialize_message, Message, Request, Response, MAX_CHUNK_SIZE, REQUEST_ID_SIZE,
};
use sealed_udp::protocol::{
    parse_packet, serialize, DataPacket, Handshake, Packet, CONNECTION_ID_SIZE, VERSION,
};

const SERVER_ADDR: &str = "127.0.0.1:8080";
const RECV_BUF_SIZE: usize = 1500;

type ConnectionId = [u8; CONNECTION_ID_SIZE];
type RequestId = [u8; REQUEST_ID_SIZE];

fn main() {
    let socket = connect(SERVER_ADDR);

    let Some((conn_id, _shared)) = handshake(&socket) else {
        eprintln!("handshake failed");
        return;
    };

    let Some(request_id) = send_request(&socket, conn_id, "test.txt", 0, MAX_CHUNK_SIZE as u32)
    else {
        eprintln!("failed to send request");
        return;
    };

    let Some(response) = receive_response(&socket, request_id) else {
        eprintln!("no valid response");
        return;
    };

    save_to_file("downloaded.txt", &response.data);
}

/// Bind a local socket and set the server as its default peer.
/// For UDP, `connect` sends nothing: it only fixes the destination
/// and makes the OS drop packets from other addresses.
fn connect(server_addr: &str) -> UdpSocket {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
    socket.connect(server_addr).expect("connect should succeed");
    socket
}

/// ClientHello -> ServerHello. Returns the connection ID and the shared secret.
fn handshake(socket: &UdpSocket) -> Option<(ConnectionId, [u8; 32])> {
    let (secret, public) = generate_keypair();
    let conn_id = random_connection_id();

    let hello = Packet::ClientHello(Handshake {
        version: VERSION,
        connection_id: conn_id,
        nonce: random_nonce(),
        public_key: public.to_bytes(),
    });
    socket.send(&serialize(&hello)).ok()?;

    let mut buf = [0u8; RECV_BUF_SIZE];
    let n = socket.recv(&mut buf).ok()?;

    let Some(Packet::ServerHello(reply)) = parse_packet(&buf[..n]) else {
        return None;
    };
    if reply.connection_id != conn_id {
        return None;
    }

    return Some((conn_id, compute_shared(secret, reply.public_key)))
}

/// Wrap a Request in a DataPacket and send it. Returns the request ID to match the reply.
fn send_request(
    socket: &UdpSocket,
    conn_id: ConnectionId,
    filename: &str,
    offset: u64,
    length: u32,
) -> Option<RequestId> {
    let request_id = random_connection_id(); // same size today; see note below

    let request = Message::Request(Request {
        request_id,
        offset,
        length,
        filename: filename.to_string(),
    });
    let packet = Packet::Data(DataPacket {
        connection_id: conn_id,
        data: serialize_message(&request),
    });

    socket.send(&serialize(&packet)).ok()?;
    return Some(request_id)
}

/// Receive one packet and accept it only if it's the Response to `request_id`.
fn receive_response(socket: &UdpSocket, request_id: RequestId) -> Option<Response> {
    let mut buf = [0u8; RECV_BUF_SIZE];
    let n = socket.recv(&mut buf).ok()?;

    let Some(Packet::Data(packet)) = parse_packet(&buf[..n]) else {
        return None;
    };
    let Some(Message::Response(response)) = parse_message(&packet.data) else {
        return None;
    };
    if response.request_id != request_id {
        return None;
    }

    return Some(response)
}

fn save_to_file(path: &str, data: &[u8]) {
    let mut file = File::create(path).expect("could not create output file");
    file.write_all(data).expect("could not write output file");
}