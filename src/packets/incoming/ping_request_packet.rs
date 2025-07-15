use bytes::{Buf, BufMut, BytesMut};
use crate::packets::{Packet, PacketError};

#[derive(Debug, Default)]
pub struct PingRequestPacket {
    pub payload: i64,
}

impl Packet for PingRequestPacket {
    fn get_id(&self) -> i32 {
        0x01
    }

    fn read(buf: &mut BytesMut) -> Result<Self, PacketError> {
        if buf.len() < 8 {
            return Err(PacketError::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough bytes for PingRequestPacket payload")));
        }
        let payload = buf.get_i64();
        Ok(PingRequestPacket { payload })
    }

    fn write(&self, buf: &mut BytesMut) -> Result<(), PacketError> {
        buf.put_i64(self.payload);
        Ok(())
    }
}