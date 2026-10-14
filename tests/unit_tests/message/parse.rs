use sealed_udp::message::{
    MAX_CHUNK_SIZE, Message, REQUEST_HEADER_SIZE, REQUEST_TYPE, RESPONSE_HEADER_SIZE,
    RESPONSE_TYPE, parse_message,
};

use super::{TEST_REQUEST_ID, raw_request, raw_response};

// ---- requests ----

#[test]
fn parses_request() {
    let buf = raw_request(1234, 1024, b"docs/report.pdf");
    match parse_message(&buf) {
        Some(Message::Request(r)) => {
            assert_eq!(r.request_id, TEST_REQUEST_ID);
            assert_eq!(r.offset, 1234);
            assert_eq!(r.length, 1024);
            assert_eq!(r.filename, "docs/report.pdf");
        }
        _ => panic!("expected Request"),
    }
}

#[test]
fn request_integers_are_big_endian() {
    let buf = raw_request(0x0102_0304_0506_0708, 0x0A0B_0C0D, b"f");
    assert_eq!(&buf[9..17], &[1u8, 2, 3, 4, 5, 6, 7, 8]);
    match parse_message(&buf) {
        Some(Message::Request(r)) => {
            assert_eq!(r.offset, 0x0102_0304_0506_0708);
            assert_eq!(r.length, 0x0A0B_0C0D);
        }
        _ => panic!("expected Request"),
    }
}

#[test]
fn parses_request_with_max_integer_values() {
    let buf = raw_request(u64::MAX, u32::MAX, b"x");
    match parse_message(&buf) {
        Some(Message::Request(r)) => {
            assert_eq!(r.offset, u64::MAX);
            assert_eq!(r.length, u32::MAX);
        }
        _ => panic!("expected Request"),
    }
}

#[test]
fn parses_request_with_empty_filename() {
    let buf = raw_request(0, 0, b"");
    assert_eq!(buf.len(), REQUEST_HEADER_SIZE);
    match parse_message(&buf) {
        Some(Message::Request(r)) => assert!(r.filename.is_empty()),
        _ => panic!("expected Request"),
    }
}

#[test]
fn parses_request_with_utf8_filename() {
    let name = "relatório_ção.txt";
    let buf = raw_request(0, 10, name.as_bytes());
    match parse_message(&buf) {
        Some(Message::Request(r)) => assert_eq!(r.filename, name),
        _ => panic!("expected Request"),
    }
}

#[test]
fn rejects_request_with_invalid_utf8_filename() {
    let buf = raw_request(0, 10, &[0xFF, 0xFE, 0xFD]);
    assert!(parse_message(&buf).is_none());
}

#[test]
#[should_panic]
fn panics_on_truncated_request() {
    let mut buf = raw_request(0, 0, b"");
    buf.pop();
    parse_message(&buf);
}

#[test]
#[should_panic]
fn panics_on_request_with_only_type_byte() {
    parse_message(&vec![REQUEST_TYPE]);
}

// ---- responses ----

#[test]
fn parses_response() {
    let buf = raw_response(4096, b"some bytes");
    match parse_message(&buf) {
        Some(Message::Response(r)) => {
            assert_eq!(r.request_id, TEST_REQUEST_ID);
            assert_eq!(r.offset, 4096);
            assert_eq!(r.data, b"some bytes".to_vec());
        }
        _ => panic!("expected Response"),
    }
}

#[test]
fn parses_response_with_empty_data() {
    let buf = raw_response(0, &[]);
    assert_eq!(buf.len(), RESPONSE_HEADER_SIZE);
    match parse_message(&buf) {
        Some(Message::Response(r)) => assert!(r.data.is_empty()),
        _ => panic!("expected Response"),
    }
}

#[test]
fn parses_response_with_max_chunk() {
    let data = vec![0x42; MAX_CHUNK_SIZE];
    let buf = raw_response(0, &data);
    match parse_message(&buf) {
        Some(Message::Response(r)) => assert_eq!(r.data, data),
        _ => panic!("expected Response"),
    }
}

#[test]
fn response_data_may_contain_non_utf8_bytes() {
    let data = vec![0x00, 0xFF, 0x80, 0xC3];
    let buf = raw_response(7, &data);
    match parse_message(&buf) {
        Some(Message::Response(r)) => assert_eq!(r.data, data),
        _ => panic!("expected Response"),
    }
}

#[test]
#[should_panic]
fn panics_on_truncated_response() {
    let mut buf = raw_response(0, &[]);
    buf.pop();
    parse_message(&buf);
}

#[test]
#[should_panic]
fn panics_on_response_with_only_type_byte() {
    parse_message(&vec![RESPONSE_TYPE]);
}

// ---- generic input ----

#[test]
#[should_panic]
fn panics_on_empty_message() {
    parse_message(&Vec::new());
}

#[test]
fn returns_none_for_unknown_type() {
    assert!(parse_message(&vec![0x00]).is_none());
    assert!(parse_message(&vec![0xFF; REQUEST_HEADER_SIZE]).is_none());
}

#[test]
fn packet_types_are_not_message_types() {
    use sealed_udp::protocol::{CLIENT_HELLO, DATA_PACKET, SERVER_HELLO};
    for t in [CLIENT_HELLO, SERVER_HELLO, DATA_PACKET] {
        let mut buf = vec![0u8; REQUEST_HEADER_SIZE];
        buf[0] = t;
        assert!(parse_message(&buf).is_none(), "type {t:#x}");
    }
}
