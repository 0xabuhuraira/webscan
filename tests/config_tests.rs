#[cfg(test)]
mod tests {
    use webscan::config::Config;

    #[test]
    fn test_config_from_cli() {
        use webscan::cli::CliArgs;
        use clap::Parser;
        
        let args = vec!["webscan", "192.168.1.0/24", "-p", "25565"];
        let cli: CliArgs = CliArgs::try_parse_from(args).unwrap();
        let config = Config::from_cli(&cli).unwrap();
        
        assert_eq!(config.targets, Some("192.168.1.0/24".to_string()));
        assert_eq!(config.ports, Some("25565".to_string()));
        assert_eq!(config.protocol_version, 770);
    }
}
