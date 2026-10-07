use std::{ net::UdpSocket};
use rand_core::{OsRng, RngCore};
use sealed_udp::protocol::{CONNECTION_ID_SIZE, Handshake, NONCE_SIZE, Packet, VERSION, parse_handshake, serialize};
use x25519_dalek::{EphemeralSecret, PublicKey};



fn main()
{
    let server_addr = "127.0.0.1:8080";

    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
    socket.connect(&server_addr).expect("connect should succeed"); // this connect is different from tcp, it only sets the dest of that socket as server_addr
    let mut conn_id = [0u8;CONNECTION_ID_SIZE];
    OsRng.fill_bytes(&mut conn_id);
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);

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

    let server_public = PublicKey::from(handshake.public_key); // this wraps up again to library type
    let shared = secret.diffie_hellman(&server_public);
    println!("{:?}", shared.as_bytes());

    

}