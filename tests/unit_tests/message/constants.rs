use sealed_udp::message::*;

#[test]
fn message_types_are_distinct() {
    assert_ne!(REQUEST_TYPE, RESPONSE_TYPE);
}

#[test]
fn message_types_do_not_collide_with_packet_types() {
    use sealed_udp::protocol::{CLIENT_HELLO, DATA_PACKET, SERVER_HELLO};
    for t in [REQUEST_TYPE, RESPONSE_TYPE] {
        assert_ne!(t, CLIENT_HELLO);
        assert_ne!(t, SERVER_HELLO);
        assert_ne!(t, DATA_PACKET);
    }
}

#[test]
fn request_offsets_are_contiguous() {
    assert_eq!(MESSAGE_TYPE_OFFSET, 0);
    assert_eq!(REQUEST_ID_OFFSET, MESSAGE_TYPE_OFFSET + 1);
    assert_eq!(OFFSET_REQUEST_OFFSET, REQUEST_ID_OFFSET + REQUEST_ID_SIZE);
    assert_eq!(LENGTH_OFFSET, OFFSET_REQUEST_OFFSET + OFFSET_SIZE);
    assert_eq!(FILENAME_OFFSET, LENGTH_OFFSET + LENGTH_SIZE);
    assert_eq!(REQUEST_HEADER_SIZE, FILENAME_OFFSET);
}

#[test]
fn response_offsets_are_contiguous() {
    assert_eq!(OFFSET_RESPONSE_OFFSET, REQUEST_ID_OFFSET + REQUEST_ID_SIZE);
    assert_eq!(DATA_OFFSET, OFFSET_RESPONSE_OFFSET + OFFSET_SIZE);
    assert_eq!(RESPONSE_HEADER_SIZE, DATA_OFFSET);
}

#[test]
fn field_sizes_match_types() {
    assert_eq!(OFFSET_SIZE, std::mem::size_of::<u64>());
    assert_eq!(LENGTH_SIZE, std::mem::size_of::<u32>());
}

#[test]
fn max_response_is_header_plus_chunk() {
    assert_eq!(MAX_RESPONSE_DATA, RESPONSE_HEADER_SIZE + MAX_CHUNK_SIZE);
}
