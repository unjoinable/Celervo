use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{BytesMut, Buf, BufMut};
use std::net::SocketAddr;
use serde_json::json;
use crate::packets::{PacketRegistry, HandshakePacket, ConnectionState, PacketError, DecodedPacket, StatusResponsePacket, StatusResponseJson, VersionInfo, PlayersInfo, Packet, PongResponsePacket};
use crate::utility::{read_varint, write_varint};

const PROTOCOL_VERSION: i32 = 772;
const VERSION: &str = "1.21.7";

pub struct Server {
    packet_registry: PacketRegistry,
}

impl Server {
    pub fn new() -> Self {
        Server {
            packet_registry: PacketRegistry::new(),
        }
    }

    pub async fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on port 25565");

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("New connection from: {}", addr);

            let packet_registry_clone = self.packet_registry.clone(); // Clone for the spawned task
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(socket, addr, packet_registry_clone).await {
                    eprintln!("Error handling connection for {}: {}", addr, e);
                }
            });
        }
    }

    async fn handle_connection(
        mut socket: TcpStream,
        addr: SocketAddr,
        _packet_registry: PacketRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = BytesMut::new();
        let mut current_state = ConnectionState::Handshake;

        loop {
            let bytes_read = match socket.read_buf(&mut buffer).await {
                Ok(0) => {
                    println!("Client {} disconnected", addr);
                    return Ok(());
                },
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket for {}: {}", addr, e);
                    return Err(e.into());
                }
            };

            if bytes_read == 0 {
                continue;
            }

            while buffer.len() > 0 {
                let mut temp_buffer = buffer.clone();

                let (packet_length, len_of_packet_length_varint) = match read_varint(&mut temp_buffer) {
                    Ok((len, bytes_read)) => (len, bytes_read),
                    Err(PacketError::Io(_)) => {
                        break;
                    },
                    Err(e) => {
                        eprintln!("Error reading packet length for {}: {:?}", addr, e);
                        return Err(e.into());
                    }
                };

                if packet_length < 0 {
                    eprintln!("Invalid packet length for {}: {}", addr, packet_length);
                    return Err("Invalid packet length".into());
                }

                let total_bytes_needed = len_of_packet_length_varint + packet_length as usize;

                if buffer.len() < total_bytes_needed {
                    break;
                }

                buffer.advance(len_of_packet_length_varint);

                let (packet_id, len_of_packet_id_varint) = match read_varint(&mut buffer) {
                    Ok((id, bytes_read)) => (id, bytes_read),
                    Err(e) => {
                        eprintln!("Error reading packet ID for {}: {:?}", addr, e);
                        return Err(e.into());
                    }
                };

                let packet_data_len = packet_length as usize - len_of_packet_id_varint;
                if buffer.len() < packet_data_len {
                    eprintln!("Not enough data for packet content for {}. Expected: {}, Actual: {}", addr, packet_data_len, buffer.len());
                    return Err("Not enough data for packet content".into());
                }
                let mut packet_buffer = buffer.split_to(packet_data_len);

                match PacketRegistry::decode_packet(current_state, packet_id, &mut packet_buffer) {
                    Ok(decoded_packet) => {
                        if let Err(e) = Self::handle_decoded_packet(addr, &mut socket, &mut current_state, decoded_packet).await {
                            eprintln!("Error handling decoded packet for {}: {}", addr, e);
                            return Err(e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error decoding packet for {}: {:?}", addr, e);
                        return Err(e.into());
                    }
                }
            }
        }
    }

    async fn handle_decoded_packet(
        addr: SocketAddr,
        stream: &mut TcpStream,
        current_state: &mut ConnectionState,
        decoded_packet: DecodedPacket,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match decoded_packet {
            DecodedPacket::Handshake(handshake_packet) => {
                Self::handle_handshake_packet(addr, stream, current_state, handshake_packet).await
            },
            DecodedPacket::StatusRequest(status_request_packet) => {
                Self::handle_status_request_packet(addr, stream, current_state, status_request_packet).await
            },
            DecodedPacket::PingRequest(ping_request_packet) => {
                Self::handle_ping_request_packet(addr, stream, current_state, ping_request_packet).await
            },
            DecodedPacket::PongResponse(pong_response_packet) => {
                println!("Client {}: Decoded Pong Response Packet: {:?}", addr, pong_response_packet);
                Ok(())
            },
            DecodedPacket::StatusResponse(status_response_packet) => {
                println!("Client {}: Decoded Status Response Packet: {:?}", addr, status_response_packet);
                Ok(())
            }
        }
    }

    async fn handle_handshake_packet(
        addr: SocketAddr,
        _stream: &mut TcpStream,
        current_state: &mut ConnectionState,
        packet: HandshakePacket,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Client {}: Decoded Handshake Packet: {:?}", addr, packet);
        *current_state = ConnectionState::Status;
        Ok(())
    }

    async fn handle_status_request_packet(
        addr: SocketAddr,
        stream: &mut TcpStream,
        _current_state: &mut ConnectionState,
        _packet: crate::packets::StatusRequestPacket,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Client {}: Decoded Status Request Packet", addr);

        let status_json = StatusResponseJson {
            version: VersionInfo {
                name: VERSION.to_string(),
                protocol: PROTOCOL_VERSION,
            },
            players: PlayersInfo {
                max: 500,
                online: 0,
                sample: vec![],
            },
            description: json!({
                "text": "Hello world from Celervo! (a Minecraft server in Rust)",
                "color": "gold"
            }),
            favicon: None,
            enforces_secure_chat: Some(false),
        };

        let json_string = serde_json::to_string(&status_json)?;

        let status_response_packet = StatusResponsePacket {
            json_response: json_string,
        };

        let mut response_buf = BytesMut::new();
        status_response_packet.write(&mut response_buf)?;

        let mut final_response = BytesMut::new();
        write_varint(&mut final_response, response_buf.len() as i32 + 1);
        write_varint(&mut final_response, status_response_packet.get_id());
        final_response.put_slice(&response_buf);

        stream.write_all(&final_response).await?;
        println!("Client {}: Sent Status Response", addr);
        Ok(())
    }

    async fn handle_ping_request_packet(
        addr: SocketAddr,
        stream: &mut TcpStream,
        _current_state: &mut ConnectionState,
        packet: crate::packets::PingRequestPacket,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Client {}: Decoded Ping Request Packet: {:?}", addr, packet);

        let pong_response_packet = PongResponsePacket {
            payload: packet.payload,
        };

        let mut response_buf = BytesMut::new();
        pong_response_packet.write(&mut response_buf)?;

        let mut final_response = BytesMut::new();
        write_varint(&mut final_response, response_buf.len() as i32 + 1); // +1 for packet ID
        write_varint(&mut final_response, pong_response_packet.get_id());
        final_response.put_slice(&response_buf);

        stream.write_all(&final_response).await?;
        println!("Client {}: Sent Pong Response", addr);
        Ok(())
    }
}