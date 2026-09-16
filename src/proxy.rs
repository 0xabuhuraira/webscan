//! Proxy management and pooling

use crate::error::{Result, WebScanError};
use crate::socks5::Socks5Config;
use crate::http_connect::HttpConnectConfig;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub enum ProxyConfig {
    Socks5(Socks5Config),
    HttpConnect(HttpConnectConfig),
}

impl ProxyConfig {
    pub fn parse(url: &str) -> Result<Self> {
        if url.starts_with("socks5://") {
            Socks5Config::parse(url).map(ProxyConfig::Socks5)
        } else if url.starts_with("http://") {
            HttpConnectConfig::parse(url).map(ProxyConfig::HttpConnect)
        } else {
            Err(WebScanError::Proxy("Unsupported proxy scheme".to_string()))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProxyStats {
    pub healthy: u32,
    pub failed: u32,
    pub quarantined: u32,
    pub errors: u32,
    pub timeouts: u32,
    pub auth_errors: u32,
}

#[derive(Debug)]
pub struct ProxyPool {
    proxies: Vec<ProxyConfig>,
    stats: Arc<RwLock<HashMap<usize, ProxyStats>>>,
    quarantined: Arc<RwLock<Vec<usize>>>,
    next_idx: Arc<RwLock<usize>>,
}

impl ProxyPool {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        let mut stats = HashMap::new();
        for i in 0..proxies.len() {
            stats.insert(i, ProxyStats::default());
        }
        
        ProxyPool {
            proxies,
            stats: Arc::new(RwLock::new(stats)),
            quarantined: Arc::new(RwLock::new(Vec::new())),
            next_idx: Arc::new(RwLock::new(0)),
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| WebScanError::Io(e))?;
        
        let mut proxies = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            match ProxyConfig::parse(line) {
                Ok(config) => proxies.push(config),
                Err(_) => {
                    log::warn!("Invalid proxy: {}", line);
                }
            }
        }
        
        Ok(ProxyPool::new(proxies))
    }

    pub fn get_next(&self) -> Option<ProxyConfig> {
        if self.proxies.is_empty() {
            return None;
        }
        
        let mut idx = self.next_idx.write();
        let quarantined = self.quarantined.read();
        
        let mut attempts = 0;
        loop {
            let current_idx = *idx % self.proxies.len();
            *idx = (*idx + 1) % self.proxies.len();
            
            if !quarantined.contains(&current_idx) {
                return Some(self.proxies[current_idx].clone());
            }
            
            attempts += 1;
            if attempts >= self.proxies.len() {
                return None; // All proxies are quarantined
            }
        }
    }

    pub fn mark_success(&self, proxy_idx: usize) {
        if let Some(stats) = self.stats.write().get_mut(&proxy_idx) {
            stats.healthy += 1;
        }
    }

    pub fn mark_failure(&self, proxy_idx: usize, is_timeout: bool, is_auth_error: bool) {
        if let Some(stats) = self.stats.write().get_mut(&proxy_idx) {
            stats.failed += 1;
            if is_timeout {
                stats.timeouts += 1;
            }
            if is_auth_error {
                stats.auth_errors += 1;
            } else {
                stats.errors += 1;
            }
            
            // Quarantine after 3 failures
            if stats.failed >= 3 {
                self.quarantined.write().push(proxy_idx);
                stats.quarantined += 1;
            }
        }
    }

    pub fn get_stats(&self) -> ProxyStats {
        let stats = self.stats.read();
        let mut combined = ProxyStats::default();
        
        for stat in stats.values() {
            combined.healthy += stat.healthy;
            combined.failed += stat.failed;
            combined.quarantined += stat.quarantined;
            combined.errors += stat.errors;
            combined.timeouts += stat.timeouts;
            combined.auth_errors += stat.auth_errors;
        }
        
        combined
    }

    pub fn count(&self) -> usize {
        self.proxies.len()
    }
}
