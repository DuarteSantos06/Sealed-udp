



pub const REQUEST_TYPE:u8= 0x4;
pub const RESPONSE_TYPE: u8 = 0x5;


pub const REQUEST_ID_SIZE : usize = 8;
pub const OFFSET_SIZE: usize = 8;
pub const LENGTH_SIZE : usize = 4;




pub const MESSAGE_TYPE_OFFSET: usize = 0;
pub const REQUEST_ID_OFFSET : usize = 1;
pub const OFFSET_REQUEST_OFFSET : usize = 9;
pub const LENGTH_OFFSET : usize = 17;
pub const FILENAME_OFFSET : usize = 21;

pub const DATA_OFFSET : usize =17;
pub const OFFSET_RESPONSE_OFFSET : usize = 9 ;

pub const MAX_CHUNK_SIZE : usize =1024;
pub const RESPONSE_HEADER_SIZE : usize = 17;
pub const REQUEST_HEADER_SIZE : usize = 21;

pub const MAX_RESPONSE_DATA : usize = MAX_CHUNK_SIZE + RESPONSE_HEADER_SIZE;


pub enum Message {
    Request(Request),
    Response(Response),
}

pub struct Request {
    pub request_id: [u8;REQUEST_ID_SIZE],
    pub offset: u64,
    pub length: u32,
    pub filename: String,
}

pub struct Response {
    pub request_id: [u8; REQUEST_ID_SIZE],
    pub offset: u64, 
    pub data: Vec<u8>,
}


pub fn parse_message(data: &Vec<u8>) -> Option<Message>
{
    let msg_type = data[MESSAGE_TYPE_OFFSET];
    match msg_type {
        REQUEST_TYPE => parse_request(&data),
        RESPONSE_TYPE => parse_response(&data),
        _ => return None
    }
}

fn parse_request(data: &Vec<u8>) -> Option<Message>
{
    let request_id: [u8; REQUEST_ID_SIZE] = data [REQUEST_ID_OFFSET.. OFFSET_REQUEST_OFFSET].try_into().unwrap();
    let offset = u64::from_be_bytes(data[OFFSET_REQUEST_OFFSET..LENGTH_OFFSET].try_into().unwrap());
    let length = u32::from_be_bytes(data[LENGTH_OFFSET.. FILENAME_OFFSET].try_into().unwrap()); 
    let filename = String::from_utf8(data[FILENAME_OFFSET..].to_vec()).ok()?;
    return Some(Message::Request(Request{
        request_id,
        offset,
        length,
        filename
    }))
}

fn parse_response(data: &Vec<u8>) -> Option<Message>
{
    if data.len() < DATA_OFFSET
    {
        panic!("something wrong");
    }
    let request_id: [u8; REQUEST_ID_SIZE] = data [REQUEST_ID_OFFSET.. OFFSET_RESPONSE_OFFSET].try_into().unwrap();
    let offset = u64::from_be_bytes(data[OFFSET_RESPONSE_OFFSET..DATA_OFFSET].try_into().unwrap()); 
    let data : Vec<u8>=data[DATA_OFFSET..].to_vec();
    return Some(Message::Response(Response{
        request_id,
        offset,
        data,
    }))
}

pub fn serialize_message (message : &Message) -> Vec<u8>
{
    match message {
        Message::Request(r) => serialize_request(&r),
        Message::Response(r) => serialize_response(&r),
    }
}

fn serialize_response(r: &Response) -> Vec<u8>
{
    let mut buf = vec![0u8; RESPONSE_HEADER_SIZE + r.data.len()];
    buf[MESSAGE_TYPE_OFFSET]=RESPONSE_TYPE;
    buf[REQUEST_ID_OFFSET..OFFSET_RESPONSE_OFFSET].copy_from_slice(&r.request_id);
    buf[OFFSET_RESPONSE_OFFSET..RESPONSE_HEADER_SIZE].copy_from_slice(&r.offset.to_be_bytes());
    buf[RESPONSE_HEADER_SIZE..].copy_from_slice(&r.data);

    return buf;
}

fn serialize_request(r: &Request) -> Vec<u8>
{
    let mut buf = vec![0u8 ; REQUEST_HEADER_SIZE+ r.filename.len()];
    buf[MESSAGE_TYPE_OFFSET]= REQUEST_TYPE;
    buf[REQUEST_ID_OFFSET..OFFSET_RESPONSE_OFFSET].copy_from_slice(&r.request_id);
    buf[OFFSET_REQUEST_OFFSET..LENGTH_OFFSET].copy_from_slice(&r.offset.to_be_bytes());
    buf[LENGTH_OFFSET..FILENAME_OFFSET].copy_from_slice(&r.length.to_be_bytes());
    buf[FILENAME_OFFSET..].copy_from_slice(r.filename.as_bytes());

    return buf;
}