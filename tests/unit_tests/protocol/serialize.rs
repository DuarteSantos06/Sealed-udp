use sealed_udp::protocol::{
    CLIENT_HELLO, CONNECTION_ID_OFFSET, CONNECTION_ID_OFFSET_IN_DATA, DATA_HEADER_SIZE, DATA_PACKET,
    DataPacket, HANDSHAKE_SIZE, MAX_CHUNK_SIZE, MSG_TYPE_OFFSET, NONCE_OFFSET, PUBLIC_KEY_OFFSET,
    Packet, SERVER_HELLO, VERSION, VERSION_OFFSET, serialize,
};

use super::{TEST_CONNECTION_ID, raw_client_hello, raw_data_packet, sample_handshake};

#[test]
fn client_hello_has_handshake_size() {
    let buf = serialize(&Packet::ClientHello(sample_handshake()));
    assert_eq!(buf.len(), HANDSHAKE_SIZE);
}

#[test]
fn client_hello_layout() {
    let h = sample_handshake();
    let buf = serialize(&Packet::ClientHello(sample_handshake()));
    assert_eq!(buf[MSG_TYPE_OFFSET], CLIENT_HELLO);
    assert_eq!(buf[VERSION_OFFSET], VERSION);
    assert_eq!(&buf[CONNECTION_ID_OFFSET..NONCE_OFFSET], &h.connection_id);
    assert_eq!(&buf[NONCE_OFFSET..PUBLIC_KEY_OFFSET], &h.nonce);
    assert_eq!(&buf[PUBLIC_KEY_OFFSET..HANDSHAKE_SIZE], &h.public_key);
}

#[test]
fn client_hello_matches_raw_bytes() {
    let buf = serialize(&Packet::ClientHello(sample_handshake()));
    assert_eq!(buf, raw_client_hello());
}

#[test]
fn server_hello_uses_server_type_byte() {
    let buf = serialize(&Packet::ServerHello(sample_handshake()));
    assert_eq!(buf.len(), HANDSHAKE_SIZE);
    assert_eq!(buf[MSG_TYPE_OFFSET], SERVER_HELLO);
}

#[test]
fn hello_serializes_version_field_verbatim() {
    let mut h = sample_handshake();
    h.version = 0x7F;
    let buf = serialize(&Packet::ClientHello(h));
    assert_eq!(buf[VERSION_OFFSET], 0x7F);
}

#[test]
fn data_packet_layout() {
    let payload = b"abc".to_vec();
    let buf = serialize(&Packet::Data(DataPacket {
        connection_id: TEST_CONNECTION_ID,
        data: payload.clone(),
    }));
    assert_eq!(buf.len(), DATA_HEADER_SIZE + payload.len());
    assert_eq!(buf[MSG_TYPE_OFFSET], DATA_PACKET);
    assert_eq!(
        &buf[CONNECTION_ID_OFFSET_IN_DATA..DATA_HEADER_SIZE],
        &TEST_CONNECTION_ID
    );
    assert_eq!(&buf[DATA_HEADER_SIZE..], payload.as_slice());
    assert_eq!(buf, raw_data_packet(&payload));
}

#[test]
fn data_packet_with_empty_payload_is_header_only() {
    let buf = serialize(&Packet::Data(DataPacket {
        connection_id: TEST_CONNECTION_ID,
        data: Vec::new(),
    }));
    assert_eq!(buf.len(), DATA_HEADER_SIZE);
}

#[test]
fn data_packet_with_max_payload() {
    let buf = serialize(&Packet::Data(DataPacket {
        connection_id: TEST_CONNECTION_ID,
        data: vec![1; MAX_CHUNK_SIZE],
    }));
    assert_eq!(buf.len(), DATA_HEADER_SIZE + MAX_CHUNK_SIZE);
}

#[test]
#[should_panic]
fn data_packet_over_max_panics() {
    serialize(&Packet::Data(DataPacket {
        connection_id: TEST_CONNECTION_ID,
        data: vec![1; MAX_CHUNK_SIZE + 1],
    }));
}
