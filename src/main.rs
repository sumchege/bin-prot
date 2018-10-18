#[macro_use]
extern crate nom;

use nom::*;

#[derive(Debug)]
enum OpCode 
{
    Continue,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Reserved,
}

impl From<u8> for OpCode 
{
    fn from(opcode: u8) -> OpCode
    {
        match opcode 
        {
            0 => OpCode::Continue,
            1 => OpCode::Text,
            2 => OpCode::Binary,
            8 => OpCode::Close,
            9 => OpCode::Ping,
            10 => OpCode::Pong,
            _ => OpCode::Reserved,
        }
    }

}
#[derive(Debug)]
struct WebSocketFrame 
{
    fin : bool,
    opcode : OpCode,
    mask : bool,
    length : u64,
    masking_key : u16,
    payload : Vec<u8>,
}

named!(parse_frame<&[u8], WebSocketFrame>,
    do_parse!(
        first_byte : bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 3), take_bits!(u8, 4))) >>
        mask_and_length : bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 7))) >>
        extendend_length : be_u64 >>
        length : value!(match mask_and_length.1 {
            127u8 => extendend_length,
            126u8 => extendend_length & 0xFFFF_0000_0000_0000u64 >> 24,
            _ => mask_and_length.1 as u64 
        }) >>
        masking_key: be_u16 >>
        payload: take!(length) >>

        (WebSocketFrame 
        {
            fin: first_byte.0 == 1,
            opcode: OpCode::from(first_byte.2),
            mask: mask_and_length.0 == 1,
            length: length,
            masking_key: masking_key,
            payload: payload.into(),
        })     

    )
);


fn main() {
    let frame = vec![0b10000001, 0b10000011, 0b00000000, 0b00000000,
                 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                 0b00000000, 0b00000000, 0b00010010, 0b10111001,
                 0b00000001, 0b00000010, 0b00000011];
    println!("Raw frame: {:?}", &frame);
    println!("Parsed frame: {:?}", parse_frame(&frame[..]).unwrap());
}
