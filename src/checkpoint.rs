//! Checkpoint and resume functionality

use crate::error::{Result, WebScanError};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub total_scanned: u64,
    pub minecraft_found: u64,
    pub tcp_open: u64,
    pub errors: u64,
    pub timestamp: i64,
}

impl Checkpoint {
    pub fn new() -> Self {
        Checkpoint {
            total_scanned: 0,
            minecraft_found: 0,
            tcp_open: 0,
            errors: 0,
            timestamp: chrono::Local::now().timestamp(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| WebScanError::Checkpoint(e.to_string()))?;
        
        std::fs::write(path, json)
            .map_err(|e| WebScanError::Io(e))?;
        
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| WebScanError::Io(e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| WebScanError::Checkpoint(e.to_string()))
    }
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self::new()
    }
}
