use bytes::{Buf, BufMut, BytesMut};
use std::io::{Error, ErrorKind};
use crate::packets::PacketError;

pub fn read_varint(buf: &mut BytesMut) -> Result<(i32, usize), PacketError> {
    let mut num_read = 0;
    let mut result = 0;
    let original_len = buf.len();
    loop {
        if num_read >= 5 {
            return Err(PacketError::InvalidVarInt);
        }
        if buf.is_empty() {
            return Err(PacketError::Io(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF while reading VarInt")));
        }
        let byte = buf.get_u8();
        let value = (byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;
        if (byte & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok((result, original_len - buf.len()))
}

pub fn write_varint(buf: &mut BytesMut, mut value: i32) {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value = (value as u32 >> 7) as i32;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        buf.put_u8(temp);
        if value == 0 {
            break;
        }
    }
}