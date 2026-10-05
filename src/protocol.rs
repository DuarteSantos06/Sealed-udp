const HANDSHAKE_SIZE:usize =32;




pub struct Handshake {
    pub public_key: [u8; 32],
}

pub fn parse_handshake(buf: &[u8]) -> Option<Handshake>
{
    let key_bytes: [u8; HANDSHAKE_SIZE] = buf[0..HANDSHAKE_SIZE].try_into().unwrap();
    return Some (Handshake{
        public_key : key_bytes,
    })
}

