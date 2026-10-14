mod constants;
mod parse;
mod roundtrip;
mod serialize;

use sealed_udp::message::{REQUEST_ID_SIZE, REQUEST_TYPE, RESPONSE_TYPE, Request, Response};

pub const TEST_REQUEST_ID: [u8; REQUEST_ID_SIZE] = [9, 8, 7, 6, 5, 4, 3, 2];

pub fn sample_request() -> Request {
    return Request {
        request_id: TEST_REQUEST_ID,
        offset: 0x0102_0304_0506_0708,
        length: 0x0A0B_0C0D,
        filename: String::from("file.txt"),
    };
}

pub fn sample_response() -> Response {
    return Response {
        request_id: TEST_REQUEST_ID,
        offset: 4096,
        data: b"chunk of file".to_vec(),
    };
}

pub fn raw_request(offset: u64, length: u32, filename: &[u8]) -> Vec<u8> {
    let mut buf = vec![REQUEST_TYPE];
    buf.extend_from_slice(&TEST_REQUEST_ID);
    buf.extend_from_slice(&offset.to_be_bytes());
    buf.extend_from_slice(&length.to_be_bytes());
    buf.extend_from_slice(filename);
    return buf;
}

pub fn raw_response(offset: u64, data: &[u8]) -> Vec<u8> {
    let mut buf = vec![RESPONSE_TYPE];
    buf.extend_from_slice(&TEST_REQUEST_ID);
    buf.extend_from_slice(&offset.to_be_bytes());
    buf.extend_from_slice(data);
    return buf;
}
