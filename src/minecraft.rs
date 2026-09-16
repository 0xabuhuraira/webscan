//! Minecraft Java Edition server detection

use crate::error::Result;
use crate::minecraft_status::{MinecraftStatus, generate_handshake, generate_status_request, read_framed_packet, parse_status_response};
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::Duration;
use bytes::BytesMut;

/// Probe a target for Minecraft Java server
pub async fn probe_minecraft(
    addr: SocketAddr,
    protocol_version: u32,
    timeout_duration: Duration,
) -> Result<Option<MinecraftStatus>> {
    let start = Instant::now();
    
    // Connect to the target
    let mut stream = match tokio::time::timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(_)) => return Ok(None),
        Err(_) => return Ok(None),
    };
    
    // Send Handshake
    let handshake = generate_handshake(protocol_version, addr, 1);
    if stream.try_write_all(&handshake).is_err() {
        return Ok(None);
    }
    
    // Send Status Request
    let status_req = generate_status_request();
    if stream.try_write_all(&status_req).is_err() {
        return Ok(None);
    }
    
    // Read Status Response
    let mut buf = BytesMut::with_capacity(16384);
    let packet = match read_framed_packet(&mut stream, &mut buf, timeout_duration).await? {
        Some(p) => p,
        None => return Ok(None),
    };
    
    // Parse Status Response
    match parse_status_response(&packet) {
        Ok(json) => {
            let latency_ms = start.elapsed().as_millis();
            match MinecraftStatus::from_json(&json, latency_ms) {
                Ok(status) => Ok(Some(status)),
                Err(_) => Ok(None),
            }
        }
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[tokio::test]
    async fn test_probe_invalid_target() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);
        let result = probe_minecraft(addr, 770, Duration::from_secs(2)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
