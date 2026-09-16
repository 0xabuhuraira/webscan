//! DNS resolution support

use crate::error::Result;
use std::net::IpAddr;
use tokio::net::lookup_host;
use tokio::time::{timeout, Duration};

/// Resolve hostname to IP address
pub async fn resolve_hostname(hostname: &str, timeout_dur: Duration) -> Result<Vec<IpAddr>> {
    match timeout(timeout_dur, lookup_host(format!("{}:0", hostname))).await {
        Ok(Ok(addrs)) => {
            let ips: Vec<IpAddr> = addrs.map(|addr| addr.ip()).collect();
            Ok(ips)
        }
        Ok(Err(e)) => Err(crate::error::WebScanError::Network(e.to_string())),
        Err(_) => Err(crate::error::WebScanError::Timeout),
    }
}

/// Reverse resolve IP to hostname
pub async fn reverse_resolve(ip: IpAddr, timeout_dur: Duration) -> Result<Option<String>> {
    use std::net::SocketAddr;
    
    let socket_addr = SocketAddr::new(ip, 0);
    
    match timeout(timeout_dur, async {
        // Tokio doesn't have built-in reverse DNS, so we skip this for now
        // In production, you'd use a DNS library like trust-dns
        Result::<Option<String>>::Ok(None)
    }).await {
        Ok(result) => result,
        Err(_) => Err(crate::error::WebScanError::Timeout),
    }
}
