pub const CLIENT_HELLO: u8 = 0x01;
pub const SERVER_HELLO: u8 = 0x02;
pub const DATA_PACKET: u8 = 0x3;

pub const VERSION: u8 = 0x01;

// offset on handshakes
pub const MSG_TYPE_OFFSET: usize = 0;
pub const VERSION_OFFSET: usize = 1;
pub const CONNECTION_ID_OFFSET: usize = 2;
pub const NONCE_OFFSET: usize = 10;
pub const PUBLIC_KEY_OFFSET: usize = 42;
pub const HANDSHAKE_SIZE: usize = 1 + 1 + CONNECTION_ID_SIZE + NONCE_SIZE + PUBLIC_KEY_SIZE;

// offset in data packets
pub const CONNECTION_ID_OFFSET_IN_DATA: usize = 1;
pub const DATA_HEADER_SIZE: usize = CONNECTION_ID_SIZE + 1;

pub const CONNECTION_ID_SIZE: usize = 8;
pub const NONCE_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 32;
pub const MAX_CHUNK_SIZE: usize = 1200;

pub enum Packet {
    ClientHello(Handshake),
    ServerHello(Handshake),
    Data(DataPacket),
}

pub struct Connection {
    pub connection_id: [u8; CONNECTION_ID_SIZE],
}

pub struct DataPacket {
    pub connection_id: [u8; CONNECTION_ID_SIZE],
    pub data: Vec<u8>,
}

pub struct Handshake {
    pub version: u8,
    pub connection_id: [u8; CONNECTION_ID_SIZE],
    pub nonce: [u8; NONCE_SIZE],
    pub public_key: [u8; PUBLIC_KEY_SIZE],
}

pub fn parse_packet(buf: &[u8]) -> Option<Packet> {
    if buf.is_empty() {
        panic!("wrong");
    }
    return get_struct_packet(buf);
}

fn get_struct_packet(buf: &[u8]) -> Option<Packet> {
    let msg_type: u8 = buf[MSG_TYPE_OFFSET];
    if msg_type == CLIENT_HELLO || msg_type == SERVER_HELLO {
        if buf.len() != HANDSHAKE_SIZE {
            panic!("something wrong happened");
        }
        let version: u8 = buf[VERSION_OFFSET];
        if version != VERSION {
            return None;
        }
        let connection_id: [u8; CONNECTION_ID_SIZE] =
            buf[CONNECTION_ID_OFFSET..NONCE_OFFSET].try_into().unwrap();
        let nonce: [u8; NONCE_SIZE] = buf[NONCE_OFFSET..PUBLIC_KEY_OFFSET].try_into().unwrap();
        let key_bytes: [u8; PUBLIC_KEY_SIZE] =
            buf[PUBLIC_KEY_OFFSET..HANDSHAKE_SIZE].try_into().unwrap();

        match msg_type {
            CLIENT_HELLO => {
                return Some(Packet::ClientHello(Handshake {
                    version,
                    connection_id,
                    nonce,
                    public_key: key_bytes,
                }));
            }
            SERVER_HELLO => {
                return Some(Packet::ServerHello(Handshake {
                    version,
                    connection_id,
                    nonce,
                    public_key: key_bytes,
                }));
            }
            _ => return None,
        }
    } else if msg_type == DATA_PACKET {
        if buf.len() < DATA_HEADER_SIZE {
            panic!("something wrong happened");
        }
        if buf.len() - DATA_HEADER_SIZE > MAX_CHUNK_SIZE {
            panic!("something is broken");
        }
        let connection_id: [u8; CONNECTION_ID_SIZE] = buf
            [CONNECTION_ID_OFFSET_IN_DATA..DATA_HEADER_SIZE]
            .try_into()
            .unwrap();
        let data: Vec<u8> = buf[DATA_HEADER_SIZE..].to_vec();
        return Some(Packet::Data(DataPacket {
            connection_id,
            data,
        }));
    }
    return None;
}

pub fn serialize(packet: &Packet) -> Vec<u8> {
    match packet {
        Packet::ClientHello(h) => return construct_buffer(h, CLIENT_HELLO),
        Packet::ServerHello(h) => return construct_buffer(h, SERVER_HELLO),
        Packet::Data(d) => return construct_buffer_to_data(d),
    }
}

fn construct_buffer(handshake: &Handshake, msg_type: u8) -> Vec<u8> {
    let mut buf = vec![0u8; HANDSHAKE_SIZE];
    buf[MSG_TYPE_OFFSET] = msg_type;
    buf[VERSION_OFFSET] = handshake.version;
    buf[CONNECTION_ID_OFFSET..NONCE_OFFSET].copy_from_slice(&handshake.connection_id);
    buf[NONCE_OFFSET..PUBLIC_KEY_OFFSET].copy_from_slice(&handshake.nonce);
    buf[PUBLIC_KEY_OFFSET..HANDSHAKE_SIZE].copy_from_slice(&handshake.public_key);
    return buf;
}

fn construct_buffer_to_data(data_packet: &DataPacket) -> Vec<u8> {
    if data_packet.data.len() > MAX_CHUNK_SIZE {
        panic!("this should not be happening");
    }
    let total = DATA_HEADER_SIZE + data_packet.data.len();
    let mut buf = vec![0u8; total];
    buf[MSG_TYPE_OFFSET] = DATA_PACKET;
    buf[CONNECTION_ID_OFFSET_IN_DATA..DATA_HEADER_SIZE].copy_from_slice(&data_packet.connection_id);
    buf[DATA_HEADER_SIZE..].copy_from_slice(&data_packet.data);

    return buf;
}
