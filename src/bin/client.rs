use std::{ net::UdpSocket};
use sealed_udp::protocol::{ Handshake, Packet, VERSION, parse_handshake, serialize};
use sealed_udp::crypto::{generate_keypair, random_connection_id, random_nonce, compute_shared};



fn main()
{
    let server_addr = "127.0.0.1:8080";

    let( secret,public)= generate_keypair();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
    socket.connect(&server_addr).expect("connect should succeed"); // this connect is different from tcp, it only sets the dest of that socket as server_addr
    let conn_id = random_connection_id();
    let nonce = random_nonce();

    let packet = Packet::ClientHello(Handshake {
        version: VERSION,
        connection_id: conn_id,
        nonce,
        public_key: public.to_bytes(),
    });
    let buf = serialize(&packet);

    let number_bytes_sent = socket.send(&buf).expect("send_to should succeed");
    if number_bytes_sent != buf.len() {
        panic!("send_to sent {} bytes, expected {}", number_bytes_sent, buf.len());
    }

    let mut buf_in =  [0u8; 1500];
    let number_bytes_received= socket.recv(&mut buf_in).expect("recvfrom should succeed");
    let handshake = match parse_handshake(&buf_in[..number_bytes_received]){
        Some(Packet::ServerHello(h)) =>h,
        _ => {
                eprintln!("expected a ServerHello, got something else");
                return;
            }
    };

    if handshake.connection_id != conn_id 
    {
        panic!("handshake connection id {:?} different than conn_id {:?}",handshake.connection_id,conn_id);
    }

    let shared= compute_shared(secret, handshake.public_key);
    println!("{:?}", shared);

    

}