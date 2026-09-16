//! SOCKS5 proxy support

use crate::error::{Result, WebScanError};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
pub struct Socks5Config {
    pub proxy_addr: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Socks5Config {
    pub fn parse(url: &str) -> Result<Self> {
        // Parse socks5://[user:pass@]host:port
        if !url.starts_with("socks5://") {
            return Err(WebScanError::Proxy("Invalid SOCKS5 URL".to_string()));
        }
        
        let rest = &url[9..];
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
        
        Ok(Socks5Config {
            proxy_addr,
            username,
            password,
        })
    }
}

/// Connect through SOCKS5 proxy
pub async fn connect_socks5(
    proxy_config: &Socks5Config,
    target_addr: SocketAddr,
    timeout_dur: Duration,
) -> Result<TcpStream> {
    // Connect to proxy
    let mut stream = match timeout(timeout_dur, TcpStream::connect(proxy_config.proxy_addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return Err(WebScanError::Proxy(e.to_string())),
        Err(_) => return Err(WebScanError::Timeout),
    };

    // SOCKS5 greeting
    let mut greeting = vec![0x05]; // SOCKS version 5
    
    if proxy_config.username.is_some() {
        greeting.push(0x02); // Number of methods: 1 (username/password)
        greeting.push(0x00); // No authentication
        greeting.push(0x02); // Username/password authentication
    } else {
        greeting.push(0x01); // Number of methods: 1
        greeting.push(0x00); // No authentication
    }

    if stream.write_all(&greeting).await.is_err() {
        return Err(WebScanError::Proxy("Failed to send greeting".to_string()));
    }

    // Read server response
    let mut buf = [0u8; 2];
    if timeout(timeout_dur, stream.read_exact(&mut buf)).await.is_err() {
        return Err(WebScanError::Timeout);
    }

    if buf[0] != 0x05 {
        return Err(WebScanError::Proxy("Invalid SOCKS5 response".to_string()));
    }

    let method = buf[1];

    // Handle authentication if needed
    if method == 0x02 {
        // Username/password authentication
        if let (Some(username), Some(password)) = (&proxy_config.username, &proxy_config.password) {
            let mut auth = vec![0x01]; // Subnegotiation version
            auth.push(username.len() as u8);
            auth.extend_from_slice(username.as_bytes());
            auth.push(password.len() as u8);
            auth.extend_from_slice(password.as_bytes());

            if stream.write_all(&auth).await.is_err() {
                return Err(WebScanError::Proxy("Failed to send auth".to_string()));
            }

            let mut auth_response = [0u8; 2];
            if timeout(timeout_dur, stream.read_exact(&mut auth_response)).await.is_err() {
                return Err(WebScanError::Timeout);
            }

            if auth_response[1] != 0x00 {
                return Err(WebScanError::Proxy("Authentication failed".to_string()));
            }
        } else {
            return Err(WebScanError::Proxy("Authentication required but no credentials provided".to_string()));
        }
    } else if method != 0x00 {
        return Err(WebScanError::Proxy("Unsupported authentication method".to_string()));
    }

    // Send connect request
    let mut connect_req = vec![0x05, 0x01, 0x00]; // SOCKS5, CONNECT, reserved

    match target_addr.ip() {
        std::net::IpAddr::V4(ip) => {
            connect_req.push(0x01); // IPv4
            connect_req.extend_from_slice(&ip.octets());
        }
        std::net::IpAddr::V6(ip) => {
            connect_req.push(0x04); // IPv6
            connect_req.extend_from_slice(&ip.octets());
        }
    }

    connect_req.extend_from_slice(&target_addr.port().to_be_bytes());

    if stream.write_all(&connect_req).await.is_err() {
        return Err(WebScanError::Proxy("Failed to send connect request".to_string()));
    }

    // Read connect response
    let mut connect_response = [0u8; 10];
    let n = match timeout(timeout_dur, stream.read(&mut connect_response)).await {
        Ok(Ok(n)) => n,
        Ok(Err(e)) => return Err(WebScanError::Proxy(e.to_string())),
        Err(_) => return Err(WebScanError::Timeout),
    };

    if n < 2 || connect_response[1] != 0x00 {
        return Err(WebScanError::Proxy("Connection request failed".to_string()));
    }

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socks5_parse() {
        let config = Socks5Config::parse("socks5://127.0.0.1:1080").unwrap();
        assert_eq!(config.proxy_addr.port(), 1080);
        assert_eq!(config.username, None);
    }

    #[test]
    fn test_socks5_parse_with_auth() {
        let config = Socks5Config::parse("socks5://user:pass@127.0.0.1:1080").unwrap();
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
    }
}
