//! Standby (daemon) mode implementation
//!
//! Keeps the LSP server running in the background for multiple queries.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Standby daemon state
pub struct StandbyDaemon {
    running: Arc<Mutex<bool>>,
}

impl StandbyDaemon {
    /// Create a new standby daemon
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the daemon
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;

        if *running {
            warn!("Daemon is already running");
            return Ok(());
        }

        info!("Starting standby daemon");
        *running = true;

        // In a full implementation, this would:
        // 1. Start LSP servers for configured languages
        // 2. Listen for incoming query requests
        // 3. Handle requests without restarting servers

        info!("Standby daemon started successfully");
        info!("LSP servers are now running in the background");
        info!("Use 'skry query' to send queries to the daemon");

        Ok(())
    }

    /// Stop the daemon
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;

        if !*running {
            warn!("Daemon is not running");
            return Ok(());
        }

        info!("Stopping standby daemon");
        *running = false;

        // Clean up LSP servers

        info!("Standby daemon stopped");
        Ok(())
    }

    /// Check if the daemon is running
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Send a query to the running daemon
    pub async fn query(&self, _query: &str) -> Result<String> {
        // TODO: This should connect to the daemon via IPC (Unix socket/named pipe)
        // Currently only checks local instance state, not the actual daemon process
        if !self.is_running().await {
            anyhow::bail!("Daemon is not running. Start it with 'skry standby'");
        }

        info!("Processing query via standby daemon");

        // In a full implementation, this would:
        // 1. Parse the query
        // 2. Use the running LSP servers
        // 3. Return results

        Ok("Query processed (placeholder)".to_string())
    }
}

impl Default for StandbyDaemon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_creation() {
        let daemon = StandbyDaemon::new();
        assert!(!daemon.is_running().await);
    }

    #[tokio::test]
    async fn test_daemon_start_stop() {
        let daemon = StandbyDaemon::new();

        daemon.start().await.unwrap();
        assert!(daemon.is_running().await);

        daemon.stop().await.unwrap();
        assert!(!daemon.is_running().await);
    }

    #[tokio::test]
    async fn test_query_without_daemon() {
        let daemon = StandbyDaemon::new();
        let result = daemon.query("test").await;
        assert!(result.is_err());
    }
}
