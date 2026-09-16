//! Port range parsing

use crate::error::{Result, WebScanError};
use std::collections::HashSet;

/// Parse port specification
pub fn parse_ports(spec: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();
    let mut seen = HashSet::new();
    
    for part in spec.split(',') {
        let part = part.trim();
        
        if part.contains('-') {
            // Range like "25565-25600"
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return Err(WebScanError::InvalidPort(part.to_string()));
            }
            
            let start = parts[0].trim().parse::<u16>()
                .map_err(|_| WebScanError::InvalidPort(part.to_string()))?;
            let end = parts[1].trim().parse::<u16>()
                .map_err(|_| WebScanError::InvalidPort(part.to_string()))?;
            
            if start > end {
                return Err(WebScanError::InvalidPort(format!("Invalid range: {} > {}", start, end)));
            }
            
            for port in start..=end {
                if seen.insert(port) {
                    ports.push(port);
                }
            }
        } else {
            // Single port
            let port = part.parse::<u16>()
                .map_err(|_| WebScanError::InvalidPort(part.to_string()))?;
            
            if seen.insert(port) {
                ports.push(port);
            }
        }
    }
    
    Ok(ports)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_single_port() {
        let ports = parse_ports("25565").unwrap();
        assert_eq!(ports, vec![25565]);
    }
    
    #[test]
    fn test_multiple_ports() {
        let ports = parse_ports("25565,25566,25567").unwrap();
        assert_eq!(ports, vec![25565, 25566, 25567]);
    }
    
    #[test]
    fn test_port_range() {
        let ports = parse_ports("25565-25567").unwrap();
        assert_eq!(ports, vec![25565, 25566, 25567]);
    }
    
    #[test]
    fn test_invalid_range() {
        assert!(parse_ports("25600-25565").is_err());
    }
}
