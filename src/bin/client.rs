use std::net::UdpSocket;
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};



fn main()
{
    let server_addr = "127.0.0.1:8080";

    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
    socket.send_to(public.as_bytes(),server_addr).expect("send to should succeed");
}