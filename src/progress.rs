//! Progress display

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct ProgressStats {
    pub total_targets: Arc<AtomicU64>,
    pub excluded: Arc<AtomicU64>,
    pub attempts: Arc<AtomicU64>,
    pub tcp_open: Arc<AtomicU64>,
    pub minecraft_found: Arc<AtomicU64>,
    pub invalid_minecraft: Arc<AtomicU64>,
    pub errors: Arc<AtomicU64>,
    pub active_connections: Arc<AtomicU64>,
}

impl ProgressStats {
    pub fn new() -> Self {
        ProgressStats {
            total_targets: Arc::new(AtomicU64::new(0)),
            excluded: Arc::new(AtomicU64::new(0)),
            attempts: Arc::new(AtomicU64::new(0)),
            tcp_open: Arc::new(AtomicU64::new(0)),
            minecraft_found: Arc::new(AtomicU64::new(0)),
            invalid_minecraft: Arc::new(AtomicU64::new(0)),
            errors: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn increment_attempts(&self) {
        self.attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_excluded(&self) {
        self.excluded.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_tcp_open(&self) {
        self.tcp_open.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_minecraft(&self) {
        self.minecraft_found.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_invalid(&self) {
        self.invalid_minecraft.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_errors(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_active(&self, count: u64) {
        self.active_connections.store(count, Ordering::Relaxed);
    }

    pub fn get_rate(&self, start: Instant) -> f64 {
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.attempts.load(Ordering::Relaxed) as f64 / elapsed
        } else {
            0.0
        }
    }
}

impl Default for ProgressStats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProgressDisplay {
    stats: ProgressStats,
    enabled: bool,
    start: Instant,
}

impl ProgressDisplay {
    pub fn new(stats: ProgressStats, enabled: bool) -> Self {
        ProgressDisplay {
            stats,
            enabled,
            start: Instant::now(),
        }
    }

    pub async fn run(&self) {
        if !self.enabled {
            return;
        }

        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            self.display();
        }
    }

    fn display(&self) {
        let attempts = self.stats.attempts.load(Ordering::Relaxed);
        let tcp_open = self.stats.tcp_open.load(Ordering::Relaxed);
        let minecraft = self.stats.minecraft_found.load(Ordering::Relaxed);
        let invalid = self.stats.invalid_minecraft.load(Ordering::Relaxed);
        let active = self.stats.active_connections.load(Ordering::Relaxed);
        let rate = self.stats.get_rate(self.start);
        let elapsed = self.start.elapsed();

        // Clear line and print progress
        eprint!("\r");
        eprint!(
            "Attempts: {:>10} | TCP Open: {:>8} | Minecraft: {:>6} | Invalid: {:>6} | Active: {:>6} | Rate: {:>8.0}/s | Elapsed: {:>6}s  ",
            attempts,
            tcp_open,
            minecraft,
            invalid,
            active,
            rate,
            elapsed.as_secs()
        );
    }
}
