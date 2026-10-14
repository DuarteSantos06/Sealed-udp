use sealed_udp::protocol::{
    CLIENT_HELLO, DATA_PACKET, HANDSHAKE_SIZE, MAX_CHUNK_SIZE, NONCE_SIZE, PUBLIC_KEY_SIZE, Packet,
    SERVER_HELLO, VERSION, parse_packet,
};

use super::{TEST_CONNECTION_ID, raw_client_hello, raw_data_packet, raw_handshake};

// ---- handshakes ----

#[test]
fn parses_client_hello() {
    let buf = raw_client_hello();
    match parse_packet(&buf) {
        Some(Packet::ClientHello(h)) => {
            assert_eq!(h.version, VERSION);
            assert_eq!(h.connection_id, TEST_CONNECTION_ID);
            assert_eq!(h.nonce, [0xAA; NONCE_SIZE]);
            assert_eq!(h.public_key, [0xBB; PUBLIC_KEY_SIZE]);
        }
        _ => panic!("expected ClientHello"),
    }
}

#[test]
fn parses_server_hello() {
    let buf = raw_handshake(SERVER_HELLO, VERSION);
    match parse_packet(&buf) {
        Some(Packet::ServerHello(h)) => {
            assert_eq!(h.version, VERSION);
            assert_eq!(h.connection_id, TEST_CONNECTION_ID);
            assert_eq!(h.nonce, [0xAA; NONCE_SIZE]);
            assert_eq!(h.public_key, [0xBB; PUBLIC_KEY_SIZE]);
        }
        _ => panic!("expected ServerHello"),
    }
}

#[test]
fn rejects_handshake_with_wrong_version() {
    let buf = raw_handshake(CLIENT_HELLO, VERSION.wrapping_add(1));
    assert!(parse_packet(&buf).is_none());
}

#[test]
fn rejects_server_hello_with_version_zero() {
    let buf = raw_handshake(SERVER_HELLO, 0);
    assert!(parse_packet(&buf).is_none());
}

#[test]
#[should_panic]
fn panics_on_truncated_handshake() {
    let buf = raw_client_hello();
    parse_packet(&buf[..HANDSHAKE_SIZE - 1]);
}

#[test]
#[should_panic]
fn panics_on_oversized_handshake() {
    let mut buf = raw_client_hello();
    buf.push(0);
    parse_packet(&buf);
}

#[test]
#[should_panic]
fn panics_on_handshake_with_only_type_byte() {
    parse_packet(&[CLIENT_HELLO]);
}

// ---- data packets ----

#[test]
fn parses_data_packet() {
    let payload = b"hello world";
    let buf = raw_data_packet(payload);
    match parse_packet(&buf) {
        Some(Packet::Data(d)) => {
            assert_eq!(d.connection_id, TEST_CONNECTION_ID);
            assert_eq!(d.data, payload.to_vec());
        }
        _ => panic!("expected Data"),
    }
}

#[test]
fn parses_data_packet_with_empty_payload() {
    let buf = raw_data_packet(&[]);
    match parse_packet(&buf) {
        Some(Packet::Data(d)) => {
            assert_eq!(d.connection_id, TEST_CONNECTION_ID);
            assert!(d.data.is_empty());
        }
        _ => panic!("expected Data"),
    }
}

#[test]
fn parses_data_packet_with_max_payload() {
    let payload = vec![0x5A; MAX_CHUNK_SIZE];
    let buf = raw_data_packet(&payload);
    match parse_packet(&buf) {
        Some(Packet::Data(d)) => assert_eq!(d.data.len(), MAX_CHUNK_SIZE),
        _ => panic!("expected Data"),
    }
}

#[test]
#[should_panic]
fn panics_on_data_payload_over_max() {
    let payload = vec![0; MAX_CHUNK_SIZE + 1];
    parse_packet(&raw_data_packet(&payload));
}

#[test]
#[should_panic]
fn panics_on_data_packet_shorter_than_header() {
    parse_packet(&[DATA_PACKET, 1, 2, 3]);
}

// ---- generic input ----

#[test]
#[should_panic]
fn panics_on_empty_buffer() {
    parse_packet(&[]);
}

#[test]
fn returns_none_for_unknown_type() {
    assert!(parse_packet(&[0xFF]).is_none());
    assert!(parse_packet(&[0x00; HANDSHAKE_SIZE]).is_none());
}

#[test]
fn message_types_are_not_packet_types() {
    assert!(parse_packet(&[sealed_udp::message::REQUEST_TYPE, 0, 0]).is_none());
    assert!(parse_packet(&[sealed_udp::message::RESPONSE_TYPE, 0, 0]).is_none());
}
