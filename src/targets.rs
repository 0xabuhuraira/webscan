//! Target streaming generation

use crate::cidr::IpRange;
use crate::error::Result;
use crate::exclude::ExclusionList;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone)]
pub struct TargetGenerator {
    ranges: Vec<IpRange>,
    ports: Vec<u16>,
    exclusions: ExclusionList,
    current_range_idx: usize,
    current_addr: Option<IpAddr>,
    current_port_idx: usize,
}

impl TargetGenerator {
    pub fn new(ranges: Vec<IpRange>, ports: Vec<u16>, exclusions: ExclusionList) -> Self {
        TargetGenerator {
            ranges,
            ports,
            exclusions,
            current_range_idx: 0,
            current_addr: None,
            current_port_idx: 0,
        }
    }

    /// Get total targets count (approximate for large ranges)
    pub fn total_targets(&self) -> u128 {
        self.ranges.iter()
            .map(|r| r.len() * self.ports.len() as u128)
            .sum()
    }

    /// Get next target or None if exhausted
    pub fn next(&mut self) -> Option<SocketAddr> {
        loop {
            // If we don't have a current range, try to get one
            if self.current_addr.is_none() {
                if self.current_range_idx >= self.ranges.len() {
                    return None; // No more ranges
                }
                
                // Get first address from current range
                let range = &self.ranges[self.current_range_idx];
                self.current_addr = range.addresses().next();
                self.current_port_idx = 0;
            }

            // Try current address with next port
            if let Some(addr) = self.current_addr {
                // Skip excluded addresses
                if self.exclusions.is_excluded(addr) {
                    self.current_addr = None;
                    continue;
                }

                if self.current_port_idx < self.ports.len() {
                    let port = self.ports[self.current_port_idx];
                    self.current_port_idx += 1;
                    return Some(SocketAddr::new(addr, port));
                } else {
                    // Move to next address
                    self.current_addr = None;
                }
            }
        }
    }
}

impl Iterator for TargetGenerator {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        TargetGenerator::next(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_generation() {
        let range = IpRange::parse("127.0.0.1/32").unwrap();
        let ports = vec![25565];
        let exclusions = ExclusionList::new();
        
        let mut gen = TargetGenerator::new(vec![range], ports, exclusions);
        let target = gen.next();
        
        assert!(target.is_some());
        assert_eq!(target.unwrap().port(), 25565);
    }
}
