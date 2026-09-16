//! Configuration management

use crate::cli::CliArgs;
use crate::error::{Result, WebScanError};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub targets: Option<String>,
    pub ports: Option<String>,
    pub excludefile: Option<String>,
    pub concurrency: u32,
    pub rate: u64,
    pub connect_timeout: u64,
    pub minecraft_timeout: u64,
    pub proxy_timeout: u64,
    pub retries: u32,
    pub proxy: Option<String>,
    pub proxy_list: Option<String>,
    pub per_proxy_concurrency: Option<u32>,
    pub protocol_version: u32,
    pub host_header: Option<String>,
    pub minecraft_ping: bool,
    pub show_open: bool,
    pub output_format: String,
    pub output_file: Option<String>,
    pub no_progress: bool,
    pub randomize_targets: bool,
    pub seed: Option<u64>,
    pub resolve: bool,
}

impl Config {
    pub fn from_cli(args: &CliArgs) -> Result<Self> {
        let concurrency = parse_concurrency(&args.concurrency)?;
        let protocol_version = args.protocol_version.parse::<u32>()
            .map_err(|_| WebScanError::Config("Invalid protocol version".to_string()))?;

        Ok(Config {
            targets: args.targets.clone(),
            ports: args.ports.clone(),
            excludefile: args.excludefile.as_ref().map(|p| p.to_string_lossy().to_string()),
            concurrency,
            rate: args.rate,
            connect_timeout: args.connect_timeout,
            minecraft_timeout: args.minecraft_timeout,
            proxy_timeout: args.proxy_timeout,
            retries: args.retries,
            proxy: args.proxy.clone(),
            proxy_list: args.proxy_list.as_ref().map(|p| p.to_string_lossy().to_string()),
            per_proxy_concurrency: args.per_proxy_concurrency,
            protocol_version,
            host_header: args.host_header.clone(),
            minecraft_ping: args.minecraft_ping,
            show_open: args.show_open,
            output_format: args.output.clone(),
            output_file: args.output_file.as_ref().map(|p| p.to_string_lossy().to_string()),
            no_progress: args.no_progress,
            randomize_targets: args.randomize_targets,
            seed: args.seed,
            resolve: args.resolve,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| WebScanError::Config(format!("Failed to read config: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| WebScanError::Config(format!("Failed to parse config: {}", e)))
    }

    pub fn merge(&mut self, other: Config) {
        if other.targets.is_some() {
            self.targets = other.targets;
        }
        if other.ports.is_some() {
            self.ports = other.ports;
        }
        if other.excludefile.is_some() {
            self.excludefile = other.excludefile;
        }
        if other.concurrency != 0 {
            self.concurrency = other.concurrency;
        }
        if other.rate != 0 {
            self.rate = other.rate;
        }
        if other.connect_timeout != 0 {
            self.connect_timeout = other.connect_timeout;
        }
        if other.minecraft_timeout != 0 {
            self.minecraft_timeout = other.minecraft_timeout;
        }
        if other.proxy_timeout != 0 {
            self.proxy_timeout = other.proxy_timeout;
        }
        if other.retries != 0 {
            self.retries = other.retries;
        }
        if other.proxy.is_some() {
            self.proxy = other.proxy;
        }
        if other.proxy_list.is_some() {
            self.proxy_list = other.proxy_list;
        }
        if other.per_proxy_concurrency.is_some() {
            self.per_proxy_concurrency = other.per_proxy_concurrency;
        }
        if other.protocol_version != 0 {
            self.protocol_version = other.protocol_version;
        }
        if other.host_header.is_some() {
            self.host_header = other.host_header;
        }
        if other.minecraft_ping {
            self.minecraft_ping = other.minecraft_ping;
        }
        if other.show_open {
            self.show_open = other.show_open;
        }
        if !other.output_format.is_empty() && other.output_format != "text" {
            self.output_format = other.output_format;
        }
        if other.output_file.is_some() {
            self.output_file = other.output_file;
        }
        if other.no_progress {
            self.no_progress = other.no_progress;
        }
        if other.randomize_targets {
            self.randomize_targets = other.randomize_targets;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        if other.resolve {
            self.resolve = other.resolve;
        }
    }
}

fn parse_concurrency(s: &str) -> Result<u32> {
    if s == "auto" {
        Ok(num_cpus::get() as u32 * 100)
    } else {
        s.parse::<u32>()
            .map_err(|_| WebScanError::Config("Invalid concurrency value".to_string()))
    }
}
