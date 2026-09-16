#[cfg(test)]
mod tests {
    use webscan::socks5::Socks5Config;
    use webscan::http_connect::HttpConnectConfig;

    #[test]
    fn test_socks5_parse_simple() {
        let config = Socks5Config::parse("socks5://127.0.0.1:1080").unwrap();
        assert_eq!(config.proxy_addr.port(), 1080);
        assert_eq!(config.username, None);
        assert_eq!(config.password, None);
    }

    #[test]
    fn test_socks5_parse_with_auth() {
        let config = Socks5Config::parse("socks5://user:pass@127.0.0.1:1080").unwrap();
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.proxy_addr.port(), 1080);
    }

    #[test]
    fn test_socks5_parse_invalid() {
        assert!(Socks5Config::parse("http://127.0.0.1:1080").is_err());
        assert!(Socks5Config::parse("socks5://invalid").is_err());
    }

    #[test]
    fn test_http_connect_parse_simple() {
        let config = HttpConnectConfig::parse("http://127.0.0.1:8080").unwrap();
        assert_eq!(config.proxy_addr.port(), 8080);
        assert_eq!(config.username, None);
        assert_eq!(config.password, None);
    }

    #[test]
    fn test_http_connect_parse_with_auth() {
        let config = HttpConnectConfig::parse("http://user:pass@127.0.0.1:8080").unwrap();
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
    }

    #[test]
    fn test_http_connect_parse_invalid() {
        assert!(HttpConnectConfig::parse("socks5://127.0.0.1:8080").is_err());
    }
}
