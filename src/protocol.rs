

pub const CLIENT_HELLO: u8 = 0x01;
pub const SERVER_HELLO: u8 = 0x02;
pub const VERSION : u8 = 0x01;

pub const MSG_TYPE_OFFSET: usize = 0;
pub const VERSION_OFFSET: usize = 1;
pub const CONNECTION_ID_OFFSET: usize = 2;
pub const NONCE_OFFSET: usize = 10;
pub const PUBLIC_KEY_OFFSET: usize = 42;
pub const HANDSHAKE_SIZE: usize = 74;


pub const CONNECTION_ID_SIZE: usize = 8;
pub const NONCE_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE : usize = 32;
pub enum Packet {
    ClientHello(Handshake),
    ServerHello(Handshake),
    //Data(DataPacket),
}

pub struct Handshake {           
    pub version: u8,
    pub connection_id: [u8; CONNECTION_ID_SIZE],
    pub nonce: [u8; NONCE_SIZE],
    pub public_key: [u8; PUBLIC_KEY_SIZE],
}

pub fn parse_handshake(buf: &[u8]) -> Option<Packet>
{
    if buf.len() != HANDSHAKE_SIZE 
    {
        return None;
    }
    let msg_type : u8 = buf[MSG_TYPE_OFFSET];
    let version : u8 = buf [VERSION_OFFSET];
    let connection_id : [u8; CONNECTION_ID_SIZE ]= buf [CONNECTION_ID_OFFSET..NONCE_OFFSET].try_into().unwrap();
    let nonce : [u8; NONCE_SIZE] = buf [NONCE_OFFSET.. PUBLIC_KEY_OFFSET].try_into().unwrap();
    let key_bytes: [u8; PUBLIC_KEY_SIZE] = buf[PUBLIC_KEY_OFFSET..HANDSHAKE_SIZE].try_into().unwrap();

    match msg_type {
        CLIENT_HELLO => return Some (Packet::ClientHello(Handshake {
            version, 
            connection_id, 
            nonce, 
            public_key: key_bytes 
        })),
        SERVER_HELLO => return Some (Packet::ServerHello(Handshake { 
            version, 
            connection_id, 
            nonce, 
            public_key: key_bytes 
        })),

        _ => return None 

    }
}

pub fn serialize (packet: &Packet) -> Vec<u8>
{
    match packet{
        Packet::ClientHello(h)=> return construct_buffer(h,CLIENT_HELLO),
        Packet::ServerHello(h)=> return construct_buffer(h,SERVER_HELLO),
    }
} 

fn construct_buffer(handshake: &Handshake,msg_type: u8) -> Vec<u8>
{
    let mut buf = vec![0u8; HANDSHAKE_SIZE];
    buf[MSG_TYPE_OFFSET]=msg_type;
    buf[VERSION_OFFSET]=handshake.version;
    buf[CONNECTION_ID_OFFSET..NONCE_OFFSET].copy_from_slice(&handshake.connection_id);
    buf[NONCE_OFFSET.. PUBLIC_KEY_OFFSET].copy_from_slice(&handshake.nonce);
    buf[PUBLIC_KEY_OFFSET..HANDSHAKE_SIZE].copy_from_slice(&handshake.public_key);
    return buf;
}


