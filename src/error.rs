//! Error types for WebScan

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebScanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid CIDR: {0}")]
    InvalidCidr(String),

    #[error("Invalid port specification: {0}")]
    InvalidPort(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Minecraft protocol error: {0}")]
    MinecraftProtocol(String),

    #[error("Proxy error: {0}")]
    Proxy(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout")]
    Timeout,

    #[error("Checkpoint error: {0}")]
    Checkpoint(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Address parse error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),

    #[error("Resource error: {0}")]
    Resource(String),
}

pub type Result<T> = std::result::Result<T, WebScanError>;
