//! CIDR range parsing and IP generation

use crate::error::{Result, WebScanError};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone)]
pub enum IpRange {
    Ipv4Net(Ipv4Net),
    Ipv6Net(Ipv6Net),
}

impl IpRange {
    /// Parse a CIDR or single IP address
    pub fn parse(s: &str) -> Result<Self> {
        // Try as CIDR first
        if let Ok(net) = s.parse::<IpNet>() {
            return match net {
                IpNet::V4(n) => Ok(IpRange::Ipv4Net(n)),
                IpNet::V6(n) => Ok(IpRange::Ipv6Net(n)),
            };
        }
        
        // Try as single IP
        match s.parse::<IpAddr>() {
            Ok(IpAddr::V4(ip)) => {
                let net = Ipv4Net::new(ip, 32)
                    .map_err(|_| WebScanError::InvalidCidr(s.to_string()))?;
                Ok(IpRange::Ipv4Net(net))
            }
            Ok(IpAddr::V6(ip)) => {
                let net = Ipv6Net::new(ip, 128)
                    .map_err(|_| WebScanError::InvalidCidr(s.to_string()))?;
                Ok(IpRange::Ipv6Net(net))
            }
            Err(_) => Err(WebScanError::InvalidCidr(s.to_string())),
        }
    }
    
    /// Get total number of addresses in the range
    pub fn len(&self) -> u128 {
        match self {
            IpRange::Ipv4Net(net) => {
                if net.prefix_len() == 0 {
                    u32::MAX as u128 + 1
                } else {
                    1u128 << (32 - net.prefix_len())
                }
            }
            IpRange::Ipv6Net(net) => {
                if net.prefix_len() == 0 {
                    u128::MAX
                } else {
                    1u128 << (128 - net.prefix_len())
                }
            }
        }
    }
    
    /// Iterate over addresses in range (streaming)
    pub fn addresses(&self) -> Box<dyn Iterator<Item = IpAddr> + '_> {
        match self {
            IpRange::Ipv4Net(net) => Box::new(
                net.hosts().map(IpAddr::V4)
            ),
            IpRange::Ipv6Net(net) => Box::new(
                net.hosts().map(IpAddr::V6)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ipv4_cidr() {
        let range = IpRange::parse("192.168.1.0/24").unwrap();
        assert!(matches!(range, IpRange::Ipv4Net(_)));
    }
    
    #[test]
    fn test_parse_single_ip() {
        let range = IpRange::parse("127.0.0.1").unwrap();
        assert!(matches!(range, IpRange::Ipv4Net(n) if n.prefix_len() == 32));
    }
    
    #[test]
    fn test_invalid_cidr() {
        assert!(IpRange::parse("invalid").is_err());
    }
}
