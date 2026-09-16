//! Asynchronous task scheduler

use crate::error::Result;
use crate::minecraft::probe_minecraft;
use crate::minecraft_status::MinecraftStatus;
use crate::output::OutputWriter;
use crate::progress::ProgressStats;
use crate::proxy::ProxyPool;
use crate::socks5::connect_socks5;
use crate::http_connect::connect_http_connect;
use crate::proxy::ProxyConfig;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::Duration;
use crossbeam_channel::{bounded, Sender, Receiver};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub addr: SocketAddr,
    pub status: Option<MinecraftStatus>,
    pub is_open: bool,
}

pub struct Scheduler {
    concurrency: u32,
    connect_timeout: Duration,
    minecraft_timeout: Duration,
    protocol_version: u32,
    show_open: bool,
    proxy_pool: Option<Arc<ProxyPool>>,
}

impl Scheduler {
    pub fn new(
        concurrency: u32,
        connect_timeout: u64,
        minecraft_timeout: u64,
        protocol_version: u32,
        show_open: bool,
        proxy_pool: Option<Arc<ProxyPool>>,
    ) -> Self {
        Scheduler {
            concurrency,
            connect_timeout: Duration::from_secs(connect_timeout),
            minecraft_timeout: Duration::from_secs(minecraft_timeout),
            protocol_version,
            show_open,
            proxy_pool,
        }
    }

    pub async fn run(
        &self,
        targets: impl Iterator<Item = SocketAddr>,
        stats: ProgressStats,
    ) -> Result<Receiver<ScanResult>> {
        let (tx, rx) = bounded(self.concurrency as usize * 2);
        let semaphore = Arc::new(Semaphore::new(self.concurrency as usize));

        let concurrency = self.concurrency;
        let connect_timeout = self.connect_timeout;
        let minecraft_timeout = self.minecraft_timeout;
        let protocol_version = self.protocol_version;
        let show_open = self.show_open;
        let proxy_pool = self.proxy_pool.clone();
        let stats_clone = stats.clone();

        tokio::spawn(async move {
            let mut handles = Vec::new();

            for target in targets {
                let permit = semaphore.acquire().await.unwrap();
                let tx = tx.clone();
                let stats = stats_clone.clone();
                let proxy_pool = proxy_pool.clone();

                let handle = tokio::spawn(async move {
                    let _permit = permit;
                    stats.increment_attempts();

                    let result = Self::scan_target(
                        target,
                        connect_timeout,
                        minecraft_timeout,
                        protocol_version,
                        show_open,
                        proxy_pool,
                        &stats,
                    )
                    .await;

                    let _ = tx.send(result);
                });

                handles.push(handle);
                stats.set_active(handles.len() as u64);
            }

            for handle in handles {
                let _ = handle.await;
            }
        });

        Ok(rx)
    }

    async fn scan_target(
        addr: SocketAddr,
        connect_timeout: Duration,
        minecraft_timeout: Duration,
        protocol_version: u32,
        show_open: bool,
        proxy_pool: Option<Arc<ProxyPool>>,
        stats: &ProgressStats,
    ) -> ScanResult {
        // Try to connect
        let connect_result = if let Some(pool) = proxy_pool {
            if let Some(proxy) = pool.get_next() {
                Self::connect_via_proxy(addr, &proxy, connect_timeout).await
            } else {
                Self::connect_direct(addr, connect_timeout).await
            }
        } else {
            Self::connect_direct(addr, connect_timeout).await
        };

        match connect_result {
            Ok(_stream) => {
                stats.increment_tcp_open();

                // Try Minecraft probe
                match probe_minecraft(addr, protocol_version, minecraft_timeout).await {
                    Ok(Some(status)) => {
                        stats.increment_minecraft();
                        ScanResult {
                            addr,
                            status: Some(status),
                            is_open: true,
                        }
                    }
                    Ok(None) => {
                        stats.increment_invalid();
                        if show_open {
                            ScanResult {
                                addr,
                                status: None,
                                is_open: true,
                            }
                        } else {
                            ScanResult {
                                addr,
                                status: None,
                                is_open: false,
                            }
                        }
                    }
                    Err(_) => {
                        stats.increment_errors();
                        ScanResult {
                            addr,
                            status: None,
                            is_open: false,
                        }
                    }
                }
            }
            Err(_) => {
                stats.increment_errors();
                ScanResult {
                    addr,
                    status: None,
                    is_open: false,
                }
            }
        }
    }

    async fn connect_direct(addr: SocketAddr, timeout: Duration) -> Result<TcpStream> {
        match tokio::time::timeout(timeout, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => Ok(stream),
            Ok(Err(e)) => Err(crate::error::WebScanError::Network(e.to_string())),
            Err(_) => Err(crate::error::WebScanError::Timeout),
        }
    }

    async fn connect_via_proxy(
        addr: SocketAddr,
        proxy: &ProxyConfig,
        timeout: Duration,
    ) -> Result<TcpStream> {
        match proxy {
            ProxyConfig::Socks5(config) => connect_socks5(config, addr, timeout).await,
            ProxyConfig::HttpConnect(config) => connect_http_connect(config, addr, timeout).await,
        }
    }
}
