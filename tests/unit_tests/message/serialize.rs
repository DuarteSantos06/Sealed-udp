use sealed_udp::message::{
    FILENAME_OFFSET, LENGTH_OFFSET, MESSAGE_TYPE_OFFSET, Message, OFFSET_REQUEST_OFFSET,
    OFFSET_RESPONSE_OFFSET, REQUEST_HEADER_SIZE, REQUEST_ID_OFFSET, REQUEST_TYPE,
    RESPONSE_HEADER_SIZE, RESPONSE_TYPE, Request, Response, serialize_message,
};

use super::{TEST_REQUEST_ID, raw_request, raw_response, sample_request, sample_response};

// ---- requests ----

#[test]
fn request_length_is_header_plus_filename() {
    let r = sample_request();
    let expected = REQUEST_HEADER_SIZE + r.filename.len();
    let buf = serialize_message(&Message::Request(r));
    assert_eq!(buf.len(), expected);
}

#[test]
fn request_layout() {
    let r = sample_request();
    let buf = serialize_message(&Message::Request(sample_request()));
    assert_eq!(buf[MESSAGE_TYPE_OFFSET], REQUEST_TYPE);
    assert_eq!(&buf[REQUEST_ID_OFFSET..OFFSET_REQUEST_OFFSET], &r.request_id);
    assert_eq!(
        &buf[OFFSET_REQUEST_OFFSET..LENGTH_OFFSET],
        &r.offset.to_be_bytes()
    );
    assert_eq!(&buf[LENGTH_OFFSET..FILENAME_OFFSET], &r.length.to_be_bytes());
    assert_eq!(&buf[FILENAME_OFFSET..], r.filename.as_bytes());
}

#[test]
fn request_matches_raw_bytes() {
    let r = sample_request();
    let expected = raw_request(r.offset, r.length, r.filename.as_bytes());
    assert_eq!(serialize_message(&Message::Request(r)), expected);
}

#[test]
fn request_with_empty_filename_is_header_only() {
    let buf = serialize_message(&Message::Request(Request {
        request_id: TEST_REQUEST_ID,
        offset: 0,
        length: 0,
        filename: String::new(),
    }));
    assert_eq!(buf.len(), REQUEST_HEADER_SIZE);
}

#[test]
fn request_filename_uses_utf8_byte_length() {
    let name = "ção";
    let buf = serialize_message(&Message::Request(Request {
        request_id: TEST_REQUEST_ID,
        offset: 0,
        length: 0,
        filename: String::from(name),
    }));
    assert_eq!(buf.len(), REQUEST_HEADER_SIZE + name.len());
    assert!(name.len() > name.chars().count());
}

// ---- responses ----

#[test]
fn response_length_is_header_plus_data() {
    let r = sample_response();
    let expected = RESPONSE_HEADER_SIZE + r.data.len();
    let buf = serialize_message(&Message::Response(r));
    assert_eq!(buf.len(), expected);
}

#[test]
fn response_layout() {
    let r = sample_response();
    let buf = serialize_message(&Message::Response(sample_response()));
    assert_eq!(buf[MESSAGE_TYPE_OFFSET], RESPONSE_TYPE);
    assert_eq!(&buf[REQUEST_ID_OFFSET..OFFSET_RESPONSE_OFFSET], &r.request_id);
    assert_eq!(
        &buf[OFFSET_RESPONSE_OFFSET..RESPONSE_HEADER_SIZE],
        &r.offset.to_be_bytes()
    );
    assert_eq!(&buf[RESPONSE_HEADER_SIZE..], r.data.as_slice());
}

#[test]
fn response_matches_raw_bytes() {
    let r = sample_response();
    let expected = raw_response(r.offset, &r.data);
    assert_eq!(serialize_message(&Message::Response(r)), expected);
}

#[test]
fn response_with_empty_data_is_header_only() {
    let buf = serialize_message(&Message::Response(Response {
        request_id: TEST_REQUEST_ID,
        offset: 0,
        data: Vec::new(),
    }));
    assert_eq!(buf.len(), RESPONSE_HEADER_SIZE);
}
