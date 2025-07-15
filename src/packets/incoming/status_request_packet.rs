use bytes::BytesMut;
use crate::packets::{Packet, PacketError};

#[derive(Debug, Default)]
pub struct StatusRequestPacket {}

impl Packet for StatusRequestPacket {
    fn get_id(&self) -> i32 {
        0x00
    }

    fn read(_buf: &mut BytesMut) -> Result<Self, PacketError> {
        Ok(StatusRequestPacket {})
    }

    fn write(&self, _buf: &mut BytesMut) -> Result<(), PacketError> {
        Ok(())
    }
}