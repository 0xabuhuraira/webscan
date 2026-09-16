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

/// Reverse resolve IP to hostname (stub - requires DNS library)
pub async fn reverse_resolve(_ip: IpAddr, _timeout_dur: Duration) -> Result<Option<String>> {
    // Tokio doesn't have built-in reverse DNS, so we return None
    // In production, you'd use a DNS library like trust-dns
    Ok(None)
}
