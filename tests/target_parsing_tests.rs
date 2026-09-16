#[cfg(test)]
mod tests {
    use webscan::cidr::IpRange;
    use webscan::ports::parse_ports;
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_single_ipv4() {
        let range = IpRange::parse("192.168.1.1").unwrap();
        let addrs: Vec<_> = range.addresses().collect();
        assert_eq!(addrs.len(), 1);
    }

    #[test]
    fn test_parse_ipv4_slash32() {
        let range = IpRange::parse("192.168.1.1/32").unwrap();
        let addrs: Vec<_> = range.addresses().collect();
        assert_eq!(addrs.len(), 1);
    }

    #[test]
    fn test_parse_ipv4_slash24() {
        let range = IpRange::parse("192.168.1.0/24").unwrap();
        let addrs: Vec<_> = range.addresses().collect();
        // /24 gives 256 - 2 = 254 usable addresses (excluding network and broadcast)
        assert!(addrs.len() > 250);
    }

    #[test]
    fn test_parse_invalid_cidr() {
        assert!(IpRange::parse("invalid").is_err());
        assert!(IpRange::parse("999.999.999.999").is_err());
        assert!(IpRange::parse("192.168.1.0/33").is_err());
    }

    #[test]
    fn test_parse_single_port() {
        let ports = parse_ports("25565").unwrap();
        assert_eq!(ports, vec![25565]);
    }

    #[test]
    fn test_parse_multiple_ports() {
        let ports = parse_ports("25565,25566,25567").unwrap();
        assert_eq!(ports, vec![25565, 25566, 25567]);
    }

    #[test]
    fn test_parse_port_range() {
        let ports = parse_ports("25565-25567").unwrap();
        assert_eq!(ports, vec![25565, 25566, 25567]);
    }

    #[test]
    fn test_parse_mixed_ports() {
        let ports = parse_ports("80,443,8000-8002").unwrap();
        assert_eq!(ports.len(), 5);
    }

    #[test]
    fn test_parse_duplicate_ports() {
        let ports = parse_ports("25565,25565,25565").unwrap();
        assert_eq!(ports, vec![25565]);
    }

    #[test]
    fn test_parse_invalid_range() {
        assert!(parse_ports("25600-25565").is_err());
    }

    #[test]
    fn test_parse_invalid_port() {
        assert!(parse_ports("invalid").is_err());
        assert!(parse_ports("70000").is_err());
    }
}
