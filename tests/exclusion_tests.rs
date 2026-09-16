#[cfg(test)]
mod tests {
    use webscan::exclude::ExclusionList;
    use webscan::cidr::IpRange;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_exclusion_single_ip() {
        let mut list = ExclusionList::new();
        list.add(IpRange::parse("192.168.1.1").unwrap());
        
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert!(!list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))));
    }

    #[test]
    fn test_exclusion_cidr() {
        let mut list = ExclusionList::new();
        list.add(IpRange::parse("192.168.1.0/24").unwrap());
        
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255))));
        assert!(!list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 1))));
    }

    #[test]
    fn test_exclusion_multiple() {
        let mut list = ExclusionList::new();
        list.add(IpRange::parse("10.0.0.0/8").unwrap());
        list.add(IpRange::parse("172.16.0.0/12").unwrap());
        list.add(IpRange::parse("192.168.0.0/16").unwrap());
        
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1))));
        assert!(list.is_excluded(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))));
        assert!(!list.is_excluded(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
    }

    #[test]
    fn test_exclusion_count() {
        let mut list = ExclusionList::new();
        list.add(IpRange::parse("10.0.0.0/8").unwrap());
        list.add(IpRange::parse("172.16.0.0/12").unwrap());
        
        assert_eq!(list.count(), 2);
    }
}
