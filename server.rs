use std::net::UdpSocket;

static IP_ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

fn main()
{
    let listener = match UdpSocket::bind(format!("{}:{}", IP_ADDRESS, PORT)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("bind failed: {}", e);
            return;
        }
    };

    let mut buf= [0; 100];
    let (number_of_bytes, _src_addr) = listener.recv_from(&mut buf).expect("recv_from should succeed");
    println!("{}", number_of_bytes);
    
}