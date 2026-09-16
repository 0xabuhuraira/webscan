//! Transport layer abstractions

use crate::error::{Result, WebScanError};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::Duration;

/// Transport abstraction for connecting to targets
pub struct Transport {
    timeout: Duration,
}

impl Transport {
    pub fn new(timeout: Duration) -> Self {
        Transport { timeout }
    }

    /// Connect to a target
    pub async fn connect(&self, addr: SocketAddr) -> Result<TcpStream> {
        match tokio::time::timeout(self.timeout, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => Ok(stream),
            Ok(Err(e)) => Err(WebScanError::Network(e.to_string())),
            Err(_) => Err(WebScanError::Timeout),
        }
    }
}
