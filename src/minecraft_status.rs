//! Minecraft Server List Ping protocol implementation

use crate::error::{Result, WebScanError};
use crate::minecraft_varint::{decode_varint, encode_varint, encode_string, decode_string};
use bytes::{BytesMut, BufMut, Buf};
use serde_json::{json, Value};
use std::io::Cursor;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
pub struct MinecraftStatus {
    pub version: Option<String>,
    pub protocol_version: Option<u32>,
    pub players_online: Option<u32>,
    pub players_max: Option<u32>,
    pub description: Option<String>,
    pub latency_ms: u128,
}

impl MinecraftStatus {
    pub fn from_json(json: &Value, latency_ms: u128) -> Result<Self> {
        let version = json
            .get("version")
            .and_then(|v| v.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string());

        let protocol_version = json
            .get("version")
            .and_then(|v| v.get("protocol"))
            .and_then(|p| p.as_u64())
            .map(|p| p as u32);

        let players_online = json
            .get("players")
            .and_then(|p| p.get("online"))
            .and_then(|o| o.as_u64())
            .map(|o| o as u32);

        let players_max = json
            .get("players")
            .and_then(|p| p.get("max"))
            .and_then(|m| m.as_u64())
            .map(|m| m as u32);

        let description = normalize_description(json.get("description"));

        Ok(MinecraftStatus {
            version,
            protocol_version,
            players_online,
            players_max,
            description,
            latency_ms,
        })
    }
}

/// Normalize description from various formats
fn normalize_description(desc: Option<&Value>) -> Option<String> {
    match desc {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Object(obj)) => {
            // Handle chat component format
            if let Some(Value::String(text)) = obj.get("text") {
                Some(text.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Generate Minecraft Handshake packet
pub fn generate_handshake(protocol_version: u32, addr: SocketAddr, next_state: u8) -> Vec<u8> {
    let mut packet = Vec::new();
    
    // Packet ID for Handshake is 0x00
    packet.extend_from_slice(&encode_varint(0x00));
    
    // Protocol Version
    packet.extend_from_slice(&encode_varint(protocol_version));
    
    // Server Address
    packet.extend_from_slice(&encode_string(&addr.ip().to_string()));
    
    // Server Port
    packet.extend_from_slice(&(addr.port() as u16).to_be_bytes());
    
    // Next State (1 = Status)
    packet.extend_from_slice(&encode_varint(next_state as u32));
    
    // Frame packet with length prefix
    let mut framed = encode_varint(packet.len() as u32);
    framed.extend_from_slice(&packet);
    framed
}

/// Generate Status Request packet (Packet ID 0x00 in Status state)
pub fn generate_status_request() -> Vec<u8> {
    // Empty packet with just the ID
    let packet = vec![0x00];
    
    // Frame with length prefix
    let mut framed = encode_varint(packet.len() as u32);
    framed.extend_from_slice(&packet);
    framed
}

/// Read a framed packet from the stream
pub async fn read_framed_packet(
    stream: &mut TcpStream,
    buf: &mut BytesMut,
    timeout_dur: Duration,
) -> Result<Option<Vec<u8>>> {
    loop {
        // Try to decode packet length
        let mut cursor = Cursor::new(&buf[..]);
        let mut length_buf = BytesMut::new();
        let mut pos = 0;
        
        // Read bytes to get VarInt length
        loop {
            if pos >= buf.len() {
                break;
            }
            let byte = buf[pos];
            pos += 1;
            length_buf.put_u8(byte);
            if byte & 0x80 == 0 {
                break;
            }
            if length_buf.len() > 4 {
                return Err(WebScanError::MinecraftProtocol("Packet length VarInt too large".to_string()));
            }
        }
        
        if length_buf.is_empty() {
            // Need more data
            let mut temp = BytesMut::with_capacity(4096);
            match timeout(timeout_dur, stream.read_buf(&mut temp)).await {
                Ok(Ok(0)) => return Ok(None), // Connection closed
                Ok(Ok(_)) => buf.extend_from_slice(&temp),
                Ok(Err(e)) => return Err(WebScanError::Network(e.to_string())),
                Err(_) => return Err(WebScanError::Timeout),
            }
            continue;
        }
        
        // Decode length
        let mut length_copy = length_buf.clone();
        match crate::minecraft_varint::decode_varint(&mut length_copy)? {
            Some(packet_len) => {
                let packet_len = packet_len as usize;
                if packet_len > 262144 {
                    return Err(WebScanError::MinecraftProtocol("Packet too large".to_string()));
                }
                
                let total_needed = pos + packet_len;
                if buf.len() >= total_needed {
                    // We have the full packet
                    buf.advance(pos);
                    let packet = buf.split_to(packet_len).to_vec();
                    return Ok(Some(packet));
                } else {
                    // Need more data
                    let mut temp = BytesMut::with_capacity(4096);
                    match timeout(timeout_dur, stream.read_buf(&mut temp)).await {
                        Ok(Ok(0)) => return Ok(None),
                        Ok(Ok(_)) => buf.extend_from_slice(&temp),
                        Ok(Err(e)) => return Err(WebScanError::Network(e.to_string())),
                        Err(_) => return Err(WebScanError::Timeout),
                    }
                }
            }
            None => {
                // Need more data
                let mut temp = BytesMut::with_capacity(4096);
                match timeout(timeout_dur, stream.read_buf(&mut temp)).await {
                    Ok(Ok(0)) => return Ok(None),
                    Ok(Ok(_)) => buf.extend_from_slice(&temp),
                    Ok(Err(e)) => return Err(WebScanError::Network(e.to_string())),
                    Err(_) => return Err(WebScanError::Timeout),
                }
            }
        }
    }
}

/// Parse Status Response packet
pub fn parse_status_response(packet: &[u8]) -> Result<Value> {
    if packet.is_empty() {
        return Err(WebScanError::MinecraftProtocol("Empty packet".to_string()));
    }
    
    // First byte is packet ID (0x00 for Status Response)
    if packet[0] != 0x00 {
        return Err(WebScanError::MinecraftProtocol("Invalid packet ID".to_string()));
    }
    
    let mut buf = BytesMut::from(&packet[1..]);
    
    // Read JSON string
    match crate::minecraft_varint::decode_string(&mut buf)? {
        Some(json_str) => serde_json::from_str(&json_str)
            .map_err(|e| WebScanError::MinecraftProtocol(format!("Invalid JSON: {}", e))),
        None => Err(WebScanError::MinecraftProtocol("No JSON string found".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[test]
    fn test_handshake_generation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let handshake = generate_handshake(770, addr, 1);
        assert!(!handshake.is_empty());
    }
    
    #[test]
    fn test_status_request_generation() {
        let req = generate_status_request();
        assert!(!req.is_empty());
    }
    
    #[test]
    fn test_normalize_description() {
        let json = json!({
            "text": "Test Server"
        });
        assert_eq!(normalize_description(json.get("text")), Some("Test Server".to_string()));
    }
}
