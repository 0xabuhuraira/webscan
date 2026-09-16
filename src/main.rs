//! WebScan - Production-quality Minecraft Java Edition discovery scanner
//! High-performance unprivileged TCP-based scanning with streaming architecture

mod cli;
mod config;
mod scanner;
mod scheduler;
mod targets;
mod cidr;
mod ports;
mod exclude;
mod transport;
mod proxy;
mod socks5;
mod http_connect;
mod minecraft;
mod minecraft_varint;
mod minecraft_status;
mod dns;
mod output;
mod progress;
mod checkpoint;
mod benchmark;
mod error;

use anyhow::Result;
use clap::Parser;
use log::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    let args = cli::CliArgs::parse();

    // Handle benchmark mode
    if args.benchmark {
        return benchmark::run_benchmark().await;
    }

    // Load configuration
    let mut config = config::Config::from_cli(&args)?;

    // Load from config file if specified
    if let Some(config_path) = &args.config {
        let file_config = config::Config::from_file(config_path)?;
        config.merge(file_config);
    }

    info!("WebScan v1.0.0 - Minecraft Java Edition Scanner");
    info!("Config: {:?}", config);

    // Load or create checkpoint if resume is enabled
    let checkpoint = if args.resume {
        if let Some(cp_path) = &args.checkpoint {
            checkpoint::Checkpoint::load(cp_path).ok()
        } else {
            None
        }
    } else {
        None
    };

    // Run the scanner
    scanner::run_scanner(config, checkpoint).await?;

    Ok(())
}
