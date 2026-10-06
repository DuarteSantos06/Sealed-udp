use std::net::UdpSocket;
use sealed_udp::protocol::{ parse_handshake, Packet};
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

static IP_ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

fn main()
{
    
    loop
    {
        let listener = match UdpSocket::bind(format!("{}:{}", IP_ADDRESS, PORT)) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("bind failed: {}", e);
                return;
            }
        };


        let mut buf = [0u8; 1500];

        let secret = EphemeralSecret::random_from_rng(OsRng);
        let _public = PublicKey::from(&secret);
        let (_number_of_bytes, _src_addr) = listener.recv_from(&mut buf).expect("recv_from should succeed");
        let handshake = match parse_handshake(&buf){
            Some(Packet::ClientHello(h)) =>h,
            Some(Packet::ServerHello(_h)) =>return,
            None => return
        };
        println!("{:?}",handshake.public_key);
    }
    
}
