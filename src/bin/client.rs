use std::{ net::UdpSocket};
use rand_core::{OsRng, RngCore};
use sealed_udp::protocol::{CONNECTION_ID_SIZE, Handshake, NONCE_SIZE, VERSION, serialize,Packet};
use x25519_dalek::{EphemeralSecret, PublicKey};



fn main()
{
    let server_addr = "127.0.0.1:8080";

    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
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

    let number_bytes_sent = socket.send_to(&buf,server_addr).expect("send_to shoud succeed");

}