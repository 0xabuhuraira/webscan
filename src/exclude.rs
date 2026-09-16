//! Target exclusion matching

use crate::error::{Result, WebScanError};
use crate::cidr::IpRange;
use std::net::IpAddr;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ExclusionList {
    ranges: Vec<IpRange>,
}

impl ExclusionList {
    pub fn new() -> Self {
        ExclusionList {
            ranges: Vec::new(),
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| WebScanError::Io(e))?;
        
        let mut ranges = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            match IpRange::parse(line) {
                Ok(range) => ranges.push(range),
                Err(_) => {
                    log::warn!("Invalid exclusion entry: {}", line);
                }
            }
        }
        
        Ok(ExclusionList { ranges })
    }

    pub fn add(&mut self, range: IpRange) {
        self.ranges.push(range);
    }

    /// Check if an IP is excluded
    pub fn is_excluded(&self, ip: IpAddr) -> bool {
        for range in &self.ranges {
            if self.is_in_range(ip, range) {
                return true;
            }
        }
        false
    }

    fn is_in_range(&self, ip: IpAddr, range: &IpRange) -> bool {
        match (ip, range) {
            (IpAddr::V4(addr), IpRange::Ipv4Net(net)) => net.contains(&addr),
            (IpAddr::V6(addr), IpRange::Ipv6Net(net)) => net.contains(&addr),
            _ => false,
        }
    }

    pub fn count(&self) -> usize {
        self.ranges.len()
    }
}

impl Default for ExclusionList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_exclusion_ipv4() {
        let mut list = ExclusionList::new();
        list.add(IpRange::parse("192.168.1.0/24").unwrap());
        
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255))));
        assert!(!list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 1))));
    }
}
