use std::net::UdpSocket;
use std::env;


fn main()
{
    let _args : Vec<String> = env::args().collect();

    let message = "Hello from UDP client!";
    let server_addr = "127.0.0.1:8080";

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind should succeed");
    socket.send_to(message.as_bytes(),server_addr).expect("send to should succeed");
}