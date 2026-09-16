//! Command-line interface argument parsing

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "WebScan")]
#[command(about = "High-performance Minecraft Java Edition discovery scanner", long_about = None)]
pub struct CliArgs {
    /// Target specification (CIDR, IP, or file with targets)
    #[arg(value_name = "TARGET")]
    pub targets: Option<String>,

    /// Ports to scan (e.g., 25565 or 25565,25566 or 25565-25600)
    #[arg(short, long, value_name = "PORTS")]
    pub ports: Option<String>,

    /// Exclusion file with CIDRs or IPs to exclude
    #[arg(long, value_name = "FILE")]
    pub excludefile: Option<PathBuf>,

    /// Maximum concurrent connections
    #[arg(long, value_name = "NUM", default_value = "10000")]
    pub concurrency: String,

    /// Rate limiting (0 = best effort)
    #[arg(long, value_name = "NUM", default_value = "0")]
    pub rate: u64,

    /// Connection timeout in seconds
    #[arg(long, value_name = "SECS", default_value = "2")]
    pub connect_timeout: u64,

    /// Minecraft protocol timeout in seconds
    #[arg(long, value_name = "SECS", default_value = "3")]
    pub minecraft_timeout: u64,

    /// Proxy timeout in seconds
    #[arg(long, value_name = "SECS", default_value = "3")]
    pub proxy_timeout: u64,

    /// Number of retries on failure
    #[arg(long, value_name = "NUM", default_value = "1")]
    pub retries: u32,

    /// SOCKS5 or HTTP proxy
    #[arg(long, value_name = "PROXY")]
    pub proxy: Option<String>,

    /// Proxy list file
    #[arg(long, value_name = "FILE")]
    pub proxy_list: Option<PathBuf>,

    /// Per-proxy concurrency limit
    #[arg(long, value_name = "NUM")]
    pub per_proxy_concurrency: Option<u32>,

    /// Minecraft protocol version
    #[arg(long, value_name = "VERSION", default_value = "770")]
    pub protocol_version: String,

    /// Host header for Minecraft handshake
    #[arg(long, value_name = "HOST")]
    pub host_header: Option<String>,

    /// Enable Minecraft ping for latency measurement
    #[arg(long)]
    pub minecraft_ping: bool,

    /// Show all TCP open ports (not just Minecraft)
    #[arg(long)]
    pub show_open: bool,

    /// Output format (text, json, csv, ndjson)
    #[arg(long, value_name = "FORMAT", default_value = "text")]
    pub output: String,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    pub output_file: Option<PathBuf>,

    /// Disable progress display
    #[arg(long)]
    pub no_progress: bool,

    /// Randomize target order
    #[arg(long)]
    pub randomize_targets: bool,

    /// Seed for randomization
    #[arg(long, value_name = "SEED")]
    pub seed: Option<u64>,

    /// Checkpoint file path
    #[arg(long, value_name = "FILE")]
    pub checkpoint: Option<PathBuf>,

    /// Resume from checkpoint
    #[arg(long)]
    pub resume: bool,

    /// Configuration file path
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable DNS resolution of results
    #[arg(long)]
    pub resolve: bool,

    /// Run benchmark
    #[arg(long)]
    pub benchmark: bool,

    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
