use bytes::{Buf, BufMut, BytesMut};
use std::io::{Error, ErrorKind};
use crate::packets::{Packet, PacketError};
use crate::utility::{read_varint, write_varint};

#[derive(Debug, Default)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl Packet for HandshakePacket {
    fn get_id(&self) -> i32 {
        0x00
    }

    fn read(buf: &mut BytesMut) -> Result<Self, PacketError> {
        let (protocol_version, _) = read_varint(buf)?;

        let (server_address_len, _) = read_varint(buf)?;
        if server_address_len < 0 {
            return Err(PacketError::Custom("Server address length cannot be negative".to_string()));
        }
        let server_address_len = server_address_len as usize;

        if buf.len() < server_address_len {
            return Err(PacketError::Io(Error::new(ErrorKind::UnexpectedEof, "Not enough bytes for server address")));
        }
        let server_address_bytes = buf.copy_to_bytes(server_address_len);
        let server_address = String::from_utf8(server_address_bytes.to_vec())
            .map_err(|e| PacketError::Custom(format!("Invalid UTF-8 for server address: {}", e)))?;

        let server_port = buf.get_u16();
        let (next_state, _) = read_varint(buf)?;

        Ok(HandshakePacket {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }

    fn write(&self, buf: &mut BytesMut) -> Result<(), PacketError> {
        write_varint(buf, self.protocol_version);

        write_varint(buf, self.server_address.len() as i32);
        buf.put_slice(self.server_address.as_bytes());

        buf.put_u16(self.server_port);
        write_varint(buf, self.next_state);
        Ok(())
    }
}