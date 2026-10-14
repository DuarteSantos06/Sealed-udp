use sealed_udp::protocol::*;

#[test]
fn message_types_are_distinct() {
    assert_ne!(CLIENT_HELLO, SERVER_HELLO);
    assert_ne!(CLIENT_HELLO, DATA_PACKET);
    assert_ne!(SERVER_HELLO, DATA_PACKET);
}

#[test]
fn handshake_offsets_are_contiguous() {
    assert_eq!(MSG_TYPE_OFFSET, 0);
    assert_eq!(VERSION_OFFSET, MSG_TYPE_OFFSET + 1);
    assert_eq!(CONNECTION_ID_OFFSET, VERSION_OFFSET + 1);
    assert_eq!(NONCE_OFFSET, CONNECTION_ID_OFFSET + CONNECTION_ID_SIZE);
    assert_eq!(PUBLIC_KEY_OFFSET, NONCE_OFFSET + NONCE_SIZE);
    assert_eq!(HANDSHAKE_SIZE, PUBLIC_KEY_OFFSET + PUBLIC_KEY_SIZE);
}

#[test]
fn handshake_size_is_74_bytes() {
    assert_eq!(HANDSHAKE_SIZE, 74);
}

#[test]
fn data_header_layout() {
    assert_eq!(CONNECTION_ID_OFFSET_IN_DATA, MSG_TYPE_OFFSET + 1);
    assert_eq!(DATA_HEADER_SIZE, CONNECTION_ID_OFFSET_IN_DATA + CONNECTION_ID_SIZE);
    assert_eq!(DATA_HEADER_SIZE, 9);
}

#[test]
fn max_data_packet_fits_in_common_mtu() {
    assert!(DATA_HEADER_SIZE + MAX_CHUNK_SIZE <= 1280);
}

#[test]
fn message_fits_inside_data_packet() {
    assert!(sealed_udp::message::MAX_RESPONSE_DATA <= MAX_CHUNK_SIZE);
}
