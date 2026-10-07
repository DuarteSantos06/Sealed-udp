use std::net::UdpSocket;
use sealed_udp::protocol::{CONNECTION_ID_SIZE, Handshake,  VERSION, serialize,Packet,parse_handshake};
use sealed_udp::crypto::{generate_keypair, random_nonce, compute_shared};

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

        let ( secret, public)= generate_keypair();
        let ( number_of_bytes, client_addr) = listener.recv_from(&mut buf).expect("recv_from should succeed");
        let handshake = match parse_handshake(&buf[..number_of_bytes]){
            Some(Packet::ClientHello(h)) =>h,
            _ => continue
        };
        let mut conn_id = [0u8;CONNECTION_ID_SIZE];
        conn_id.copy_from_slice(&handshake.connection_id);
        let nonce = random_nonce();

        let packet = Packet::ServerHello(Handshake {
            version: VERSION,
            connection_id: conn_id,
            nonce,
            public_key: public.to_bytes(),
        });

        let buf_out = serialize(&packet);
        let _number_bytes_sent = listener.send_to(&buf_out, client_addr).expect("send to should work");

        let shared= compute_shared(secret,handshake.public_key);
        println!("{:?}", shared);

        }
}
    

