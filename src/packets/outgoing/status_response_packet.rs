use bytes::{BufMut, BytesMut};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::packets::{Packet, PacketError};
use crate::utility::write_varint;

#[derive(Debug, Default)]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl Packet for StatusResponsePacket {
    fn get_id(&self) -> i32 {
        0x00
    }

    fn read(_buf: &mut BytesMut) -> Result<Self, PacketError> {
        Err(PacketError::Custom("StatusResponsePacket cannot be read from client".to_string()))
    }

    fn write(&self, buf: &mut BytesMut) -> Result<(), PacketError> {
        write_varint(buf, self.json_response.len() as i32);
        buf.put_slice(self.json_response.as_bytes());
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponseJson {
    pub version: VersionInfo,
    pub players: PlayersInfo,
    pub description: Value, // Changed String to Value
    #[serde(rename = "favicon")]
    pub favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionInfo {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayersInfo {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<PlayerSample>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}
