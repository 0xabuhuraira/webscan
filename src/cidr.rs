//! CIDR and IP address range parsing

use crate::error::{Result, WebScanError};
use ipnet::IpNet;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum IpRange {
    Ipv4Net(ipnet::Ipv4Net),
    Ipv6Net(ipnet::Ipv6Net),
}

impl IpRange {
    /// Parse a CIDR or single IP address
    pub fn parse(s: &str) -> Result<Self> {
        // Try parsing as IpNet (CIDR)
        if let Ok(net) = IpNet::from_str(s) {
            return match net {
                IpNet::V4(v4) => Ok(IpRange::Ipv4Net(v4)),
                IpNet::V6(v6) => Ok(IpRange::Ipv6Net(v6)),
            };
        }

        // Try parsing as single IP address
        if let Ok(addr) = IpAddr::from_str(s) {
            return match addr {
                IpAddr::V4(v4) => Ok(IpRange::Ipv4Net(ipnet::Ipv4Net::new(v4, 32)?)),
                IpAddr::V6(v6) => Ok(IpRange::Ipv6Net(ipnet::Ipv6Net::new(v6, 128)?)),
            };
        }

        Err(WebScanError::Parsing(format!("Invalid CIDR or IP: {}", s)))
    }

    /// Get the length of the range
    pub fn len(&self) -> u128 {
        match self {
            IpRange::Ipv4Net(net) => {
                let broadcast = net.broadcast();
                let network = net.network();
                u32::from(broadcast) as u128 - u32::from(network) as u128 + 1
            }
            IpRange::Ipv6Net(net) => {
                let broadcast = net.broadcast();
                let network = net.network();
                u128::from(broadcast) - u128::from(network) + 1
            }
        }
    }

    /// Check if range is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all addresses in the range
    pub fn addresses(&self) -> Box<dyn Iterator<Item = IpAddr> + '_> {
        match self {
            IpRange::Ipv4Net(net) => {
                Box::new(net.hosts().map(IpAddr::V4))
            }
            IpRange::Ipv6Net(net) => {
                // For IPv6, we can't iterate over all addresses in a reasonable time,
                // so we'll return a limited iterator or just the network/broadcast
                let network = net.network();
                let broadcast = net.broadcast();
                Box::new(std::iter::once(IpAddr::V6(network)).chain(std::iter::once(IpAddr::V6(broadcast))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_ipv4() {
        let range = IpRange::parse("192.168.1.1").unwrap();
        assert_eq!(range.len(), 1);
    }

    #[test]
    fn test_parse_ipv4_cidr() {
        let range = IpRange::parse("192.168.1.0/24").unwrap();
        assert!(range.len() > 250);
    }
}
