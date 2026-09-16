//! HTTP CONNECT proxy support

use crate::error::{Result, WebScanError};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
pub struct HttpConnectConfig {
    pub proxy_addr: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl HttpConnectConfig {
    pub fn parse(url: &str) -> Result<Self> {
        // Parse http://[user:pass@]host:port
        if !url.starts_with("http://") {
            return Err(WebScanError::Proxy("Invalid HTTP proxy URL".to_string()));
        }
        
        let rest = &url[7..];
        let (auth, host_port) = if let Some(at_pos) = rest.rfind('@') {
            let auth = &rest[..at_pos];
            let host_port = &rest[at_pos + 1..];
            
            let (user, pass) = if let Some(colon) = auth.find(':') {
                (auth[..colon].to_string(), Some(auth[colon + 1..].to_string()))
            } else {
                (auth.to_string(), None)
            };
            
            (Some((user, pass)), host_port)
        } else {
            (None, rest)
        };
        
        let proxy_addr = host_port.parse::<SocketAddr>()
            .map_err(|_| WebScanError::Proxy("Invalid proxy address".to_string()))?;
        
        let (username, password) = auth.map(|(u, p)| (Some(u), p)).unwrap_or((None, None));
        
        Ok(HttpConnectConfig {
            proxy_addr,
            username,
            password,
        })
    }
}

/// Connect through HTTP CONNECT proxy
pub async fn connect_http_connect(
    proxy_config: &HttpConnectConfig,
    target_addr: SocketAddr,
    timeout_dur: Duration,
) -> Result<TcpStream> {
    // Connect to proxy
    let mut stream = match timeout(timeout_dur, TcpStream::connect(proxy_config.proxy_addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return Err(WebScanError::Proxy(e.to_string())),
        Err(_) => return Err(WebScanError::Timeout),
    };

    // Build CONNECT request
    let mut request = format!("CONNECT {} HTTP/1.1\r\n", target_addr);
    request.push_str(&format!("Host: {}\r\n", target_addr));
    request.push_str("Connection: close\r\n");

    // Add auth if present
    if let (Some(username), Some(password)) = (&proxy_config.username, &proxy_config.password) {
        let credentials = format!("{}:{}", username, password);
        let encoded = base64_encode(credentials.as_bytes());
        request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded));
    }

    request.push_str("\r\n");

    if stream.write_all(request.as_bytes()).await.is_err() {
        return Err(WebScanError::Proxy("Failed to send CONNECT request".to_string()));
    }

    // Read response
    let mut buf = [0u8; 1024];
    let n = match timeout(timeout_dur, stream.read(&mut buf)).await {
        Ok(Ok(n)) => n,
        Ok(Err(e)) => return Err(WebScanError::Proxy(e.to_string())),
        Err(_) => return Err(WebScanError::Timeout),
    };

    let response = String::from_utf8_lossy(&buf[..n]);
    
    if response.contains("200") {
        Ok(stream)
    } else {
        Err(WebScanError::Proxy(format!("HTTP CONNECT failed: {}", response)))
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        
        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
        
        result.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARS[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_connect_parse() {
        let config = HttpConnectConfig::parse("http://127.0.0.1:8080").unwrap();
        assert_eq!(config.proxy_addr.port(), 8080);
        assert_eq!(config.username, None);
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b"user:pass"), "dXNlcjpwYXNz");
    }
}
