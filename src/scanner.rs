//! Main scanner orchestration

use crate::config::Config;
use crate::checkpoint::Checkpoint;
use crate::exclude::ExclusionList;
use crate::ports::parse_ports;
use crate::cidr::IpRange;
use crate::targets::TargetGenerator;
use crate::scheduler::Scheduler;
use crate::output::{OutputWriter, OutputFormat};
use crate::progress::{ProgressStats, ProgressDisplay};
use crate::proxy::ProxyPool;
use crate::error::Result;
use log::info;
use std::sync::Arc;
use std::time::Instant;

pub async fn run_scanner(config: Config, _checkpoint: Option<Checkpoint>) -> Result<()> {
    info!("Starting WebScan scanner");

    // Parse targets
    let targets = config.targets.as_ref().ok_or(
        crate::error::WebScanError::Config("No targets specified".to_string())
    )?;

    let mut ranges = Vec::new();
    for target in targets.split(',') {
        let target = target.trim();
        ranges.push(IpRange::parse(target)?);
    }

    // Parse ports
    let ports_spec = config.ports.as_ref().ok_or(
        crate::error::WebScanError::Config("No ports specified".to_string())
    )?;
    let ports = parse_ports(ports_spec)?;

    info!("Scanning {} ranges on {} ports", ranges.len(), ports.len());

    // Load exclusions if provided
    let mut exclusions = ExclusionList::new();
    if let Some(exclude_path) = &config.excludefile {
        if let Ok(exclude_list) = ExclusionList::from_file(std::path::Path::new(exclude_path)) {
            info!("Loaded {} exclusions", exclude_list.count());
            exclusions = exclude_list;
        }
    }

    // Load proxy pool if provided
    let proxy_pool = if let Some(proxy_list_path) = &config.proxy_list {
        if let Ok(pool) = ProxyPool::from_file(std::path::Path::new(proxy_list_path)) {
            info!("Loaded {} proxies", pool.count());
            Some(Arc::new(pool))
        } else {
            None
        }
    } else {
        None
    };

    // Create target generator
    let target_gen = TargetGenerator::new(ranges, ports, exclusions);
    info!("Total targets: {}", target_gen.total_targets());

    // Create output writer
    let output_format = OutputFormat::from_str(&config.output_format);
    let output_path = config.output_file.as_ref().map(|p| std::path::Path::new(p));
    let mut output = OutputWriter::new(output_format, output_path)?;
    output.write_header()?;

    // Create progress stats
    let stats = ProgressStats::new();
    let stats_display = stats.clone();

    // Start progress display
    let progress_enabled = !config.no_progress;
    let progress = ProgressDisplay::new(stats_display, progress_enabled);
    let progress_handle = tokio::spawn(async move {
        progress.run().await;
    });

    // Create scheduler
    let scheduler = Scheduler::new(
        config.concurrency,
        config.connect_timeout,
        config.minecraft_timeout,
        config.protocol_version,
        config.show_open,
        proxy_pool,
    );

    // Run scanner
    let start = Instant::now();
    let results = scheduler.run(target_gen, stats.clone()).await?;

    // Process results
    while let Ok(result) = results.recv() {
        if let Some(status) = result.status {
            output.write_result(result.addr, &status)?;
        } else if result.is_open && config.show_open {
            output.write_open_port(result.addr)?;
        }
    }

    output.write_footer()?;

    let elapsed = start.elapsed();
    let rate = stats.attempts.load(std::sync::atomic::Ordering::Relaxed) as f64 / elapsed.as_secs_f64();

    info!("\n\nScan completed in {:.1}s", elapsed.as_secs_f64());
    info!("Attempts: {}", stats.attempts.load(std::sync::atomic::Ordering::Relaxed));
    info!("TCP Open: {}", stats.tcp_open.load(std::sync::atomic::Ordering::Relaxed));
    info!("Minecraft: {}", stats.minecraft_found.load(std::sync::atomic::Ordering::Relaxed));
    info!("Rate: {:.0}/s", rate);

    progress_handle.abort();
    Ok(())
}
