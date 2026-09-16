#[cfg(test)]
mod tests {
    use webscan::output::{OutputFormat, OutputWriter};
    use webscan::minecraft_status::MinecraftStatus;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tempfile::NamedTempFile;

    fn create_test_status() -> MinecraftStatus {
        MinecraftStatus {
            version: Some("Paper 1.21.5".to_string()),
            protocol_version: Some(770),
            players_online: Some(12),
            players_max: Some(100),
            description: Some("Test Server".to_string()),
            latency_ms: 18,
        }
    }

    #[test]
    fn test_output_text_format() {
        let mut output = OutputWriter::new(OutputFormat::Text, None).unwrap();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let status = create_test_status();
        
        // Should not panic
        output.write_result(addr, &status).unwrap();
    }

    #[test]
    fn test_output_json_format() {
        let mut output = OutputWriter::new(OutputFormat::Json, None).unwrap();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let status = create_test_status();
        
        output.write_header().unwrap();
        output.write_result(addr, &status).unwrap();
        output.write_footer().unwrap();
    }

    #[test]
    fn test_output_csv_format() {
        let mut output = OutputWriter::new(OutputFormat::Csv, None).unwrap();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let status = create_test_status();
        
        output.write_header().unwrap();
        output.write_result(addr, &status).unwrap();
    }

    #[test]
    fn test_output_ndjson_format() {
        let mut output = OutputWriter::new(OutputFormat::Ndjson, None).unwrap();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let status = create_test_status();
        
        output.write_result(addr, &status).unwrap();
    }

    #[test]
    fn test_output_to_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        
        let mut output = OutputWriter::new(OutputFormat::Text, Some(path)).unwrap();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
        let status = create_test_status();
        
        output.write_result(addr, &status).unwrap();
        
        // Verify file was written
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("127.0.0.1"));
        assert!(content.contains("25565"));
    }
}
