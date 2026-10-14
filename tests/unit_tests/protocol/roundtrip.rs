use sealed_udp::protocol::{DataPacket, MAX_CHUNK_SIZE, Packet, parse_packet, serialize};

use super::{TEST_CONNECTION_ID, sample_handshake};

#[test]
fn client_hello_roundtrip() {
    let original = sample_handshake();
    let buf = serialize(&Packet::ClientHello(sample_handshake()));
    match parse_packet(&buf) {
        Some(Packet::ClientHello(h)) => {
            assert_eq!(h.version, original.version);
            assert_eq!(h.connection_id, original.connection_id);
            assert_eq!(h.nonce, original.nonce);
            assert_eq!(h.public_key, original.public_key);
        }
        _ => panic!("expected ClientHello"),
    }
}

#[test]
fn server_hello_roundtrip() {
    let original = sample_handshake();
    let buf = serialize(&Packet::ServerHello(sample_handshake()));
    match parse_packet(&buf) {
        Some(Packet::ServerHello(h)) => {
            assert_eq!(h.connection_id, original.connection_id);
            assert_eq!(h.nonce, original.nonce);
            assert_eq!(h.public_key, original.public_key);
        }
        _ => panic!("expected ServerHello"),
    }
}

#[test]
fn data_roundtrip_various_sizes() {
    for size in [0, 1, 17, 512, MAX_CHUNK_SIZE - 1, MAX_CHUNK_SIZE] {
        let payload: Vec<u8> = (0..size).map(|i| (i % 251) as u8).collect();
        let buf = serialize(&Packet::Data(DataPacket {
            connection_id: TEST_CONNECTION_ID,
            data: payload.clone(),
        }));
        match parse_packet(&buf) {
            Some(Packet::Data(d)) => {
                assert_eq!(d.connection_id, TEST_CONNECTION_ID, "size {size}");
                assert_eq!(d.data, payload, "size {size}");
            }
            _ => panic!("expected Data for size {size}"),
        }
    }
}

#[test]
fn reserialize_parsed_packet_is_identical() {
    let buf = serialize(&Packet::ClientHello(sample_handshake()));
    let parsed = parse_packet(&buf).expect("valid hello");
    assert_eq!(serialize(&parsed), buf);
}
