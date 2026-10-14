mod constants;
mod parse;
mod roundtrip;
mod serialize;

use sealed_udp::protocol::{
    CLIENT_HELLO, CONNECTION_ID_SIZE, DATA_PACKET, Handshake, NONCE_SIZE, PUBLIC_KEY_SIZE, VERSION,
};

pub const TEST_CONNECTION_ID: [u8; CONNECTION_ID_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8];

pub fn sample_handshake() -> Handshake {
    return Handshake {
        version: VERSION,
        connection_id: TEST_CONNECTION_ID,
        nonce: [0xAA; NONCE_SIZE],
        public_key: [0xBB; PUBLIC_KEY_SIZE],
    };
}

pub fn raw_handshake(msg_type: u8, version: u8) -> Vec<u8> {
    let mut buf = vec![msg_type, version];
    buf.extend_from_slice(&TEST_CONNECTION_ID);
    buf.extend_from_slice(&[0xAA; NONCE_SIZE]);
    buf.extend_from_slice(&[0xBB; PUBLIC_KEY_SIZE]);
    return buf;
}

pub fn raw_client_hello() -> Vec<u8> {
    return raw_handshake(CLIENT_HELLO, VERSION);
}

pub fn raw_data_packet(payload: &[u8]) -> Vec<u8> {
    let mut buf = vec![DATA_PACKET];
    buf.extend_from_slice(&TEST_CONNECTION_ID);
    buf.extend_from_slice(payload);
    return buf;
}
