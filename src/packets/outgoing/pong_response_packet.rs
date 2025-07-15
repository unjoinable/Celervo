use bytes::{BufMut, BytesMut};
use crate::packets::{Packet, PacketError};

#[derive(Debug, Default)]
pub struct PongResponsePacket {
    pub payload: i64,
}

impl Packet for PongResponsePacket {
    fn get_id(&self) -> i32 {
        0x01
    }

    fn read(_buf: &mut BytesMut) -> Result<Self, PacketError> {
        Err(PacketError::Custom("PongResponsePacket cannot be read from client".to_string()))
    }

    fn write(&self, buf: &mut BytesMut) -> Result<(), PacketError> {
        buf.put_i64(self.payload);
        Ok(())
    }
}