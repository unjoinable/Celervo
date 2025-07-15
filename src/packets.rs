
pub mod incoming;
pub mod outgoing;

use bytes::BytesMut;
use std::fmt;
use std::error::Error as StdError;

pub use incoming::handshake_packet::HandshakePacket;
pub use incoming::status_request_packet::StatusRequestPacket;
pub use incoming::ping_request_packet::PingRequestPacket;
pub use outgoing::status_response_packet::{StatusResponsePacket, StatusResponseJson, VersionInfo, PlayersInfo};
pub use outgoing::pong_response_packet::PongResponsePacket;

pub trait Packet: Sized + Send + Sync {
    fn get_id(&self) -> i32;
    fn read(buf: &mut BytesMut) -> Result<Self, PacketError>;
    fn write(&self, buf: &mut BytesMut) -> Result<(), PacketError>;
}

#[derive(Debug)]
pub enum PacketError {
    Io(std::io::Error),
    InvalidPacketId(i32),
    InvalidVarInt,
    Custom(String),
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketError::Io(err) => write!(f, "IO error: {}", err),
            PacketError::InvalidPacketId(id) => write!(f, "Invalid packet ID: {}", id),
            PacketError::InvalidVarInt => write!(f, "Invalid VarInt"),
            PacketError::Custom(msg) => write!(f, "Custom packet error: {}", msg),
        }
    }
}

impl StdError for PacketError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            PacketError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PacketError {
    fn from(err: std::io::Error) -> Self {
        PacketError::Io(err)
    }
}


#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Clone)]
pub struct PacketRegistry;

#[allow(dead_code)]
pub enum DecodedPacket {
    Handshake(HandshakePacket),
    StatusRequest(StatusRequestPacket),
    PingRequest(PingRequestPacket),
    PongResponse(PongResponsePacket),
    StatusResponse(StatusResponsePacket),
}

impl PacketRegistry {
    pub fn new() -> Self {
        PacketRegistry
    }

    pub fn decode_packet(state: ConnectionState, packet_id: i32, buf: &mut BytesMut) -> Result<DecodedPacket, PacketError> {
        match state {
            ConnectionState::Handshake => {
                match packet_id {
                    0x00 => Ok(DecodedPacket::Handshake(HandshakePacket::read(buf)?)),
                    _ => Err(PacketError::InvalidPacketId(packet_id)),
                }
            },
            ConnectionState::Status => {
                match packet_id {
                    0x00 => Ok(DecodedPacket::StatusRequest(StatusRequestPacket::read(buf)?)),
                    0x01 => Ok(DecodedPacket::PingRequest(PingRequestPacket::read(buf)?)),
                    _ => Err(PacketError::InvalidPacketId(packet_id)),
                }
            },
            ConnectionState::Login => {
                match packet_id {
                    _ => Err(PacketError::InvalidPacketId(packet_id)),
                }
            },
            ConnectionState::Configuration => {
                match packet_id {
                    _ => Err(PacketError::InvalidPacketId(packet_id)),
                }
            },
            ConnectionState::Play => {
                match packet_id {
                    _ => Err(PacketError::InvalidPacketId(packet_id)),
                }
            },
        }
    }
}