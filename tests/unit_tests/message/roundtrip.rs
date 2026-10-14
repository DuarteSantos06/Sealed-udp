use sealed_udp::message::{
    MAX_CHUNK_SIZE, Message, Request, Response, parse_message, serialize_message,
};

use super::{TEST_REQUEST_ID, sample_request, sample_response};

#[test]
fn request_roundtrip() {
    let original = sample_request();
    let buf = serialize_message(&Message::Request(sample_request()));
    match parse_message(&buf) {
        Some(Message::Request(r)) => {
            assert_eq!(r.request_id, original.request_id);
            assert_eq!(r.offset, original.offset);
            assert_eq!(r.length, original.length);
            assert_eq!(r.filename, original.filename);
        }
        _ => panic!("expected Request"),
    }
}

#[test]
fn request_roundtrip_with_long_filename() {
    let filename = "a/".repeat(200) + "file.bin";
    let buf = serialize_message(&Message::Request(Request {
        request_id: TEST_REQUEST_ID,
        offset: 42,
        length: 1024,
        filename: filename.clone(),
    }));
    match parse_message(&buf) {
        Some(Message::Request(r)) => assert_eq!(r.filename, filename),
        _ => panic!("expected Request"),
    }
}

#[test]
fn response_roundtrip() {
    let original = sample_response();
    let buf = serialize_message(&Message::Response(sample_response()));
    match parse_message(&buf) {
        Some(Message::Response(r)) => {
            assert_eq!(r.request_id, original.request_id);
            assert_eq!(r.offset, original.offset);
            assert_eq!(r.data, original.data);
        }
        _ => panic!("expected Response"),
    }
}

#[test]
fn response_roundtrip_various_sizes() {
    for size in [0, 1, 100, MAX_CHUNK_SIZE] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let buf = serialize_message(&Message::Response(Response {
            request_id: TEST_REQUEST_ID,
            offset: size as u64,
            data: data.clone(),
        }));
        match parse_message(&buf) {
            Some(Message::Response(r)) => {
                assert_eq!(r.offset, size as u64, "size {size}");
                assert_eq!(r.data, data, "size {size}");
            }
            _ => panic!("expected Response for size {size}"),
        }
    }
}

#[test]
fn reserialize_parsed_message_is_identical() {
    let buf = serialize_message(&Message::Request(sample_request()));
    let parsed = parse_message(&buf).expect("valid request");
    assert_eq!(serialize_message(&parsed), buf);
}

#[test]
fn max_response_fits_in_data_packet() {
    use sealed_udp::protocol::{DataPacket, Packet, parse_packet, serialize};

    let msg = serialize_message(&Message::Response(Response {
        request_id: TEST_REQUEST_ID,
        offset: 0,
        data: vec![7; MAX_CHUNK_SIZE],
    }));
    let packet = serialize(&Packet::Data(DataPacket {
        connection_id: [0; 8],
        data: msg.clone(),
    }));
    match parse_packet(&packet) {
        Some(Packet::Data(d)) => assert_eq!(d.data, msg),
        _ => panic!("expected Data"),
    }
}
