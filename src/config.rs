//! Configuration parsing and management
//!
//! Handles loading and parsing `.skry.toml` configuration files.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Main configuration structure for skry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub lsp: LspConfig,
}

/// General configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Maximum number of tokens to include in the context
    #[serde(default = "default_token_budget")]
    pub token_budget: usize,
    
    /// Pruning strategy for dependencies
    #[serde(default = "default_pruning_strategy")]
    pub pruning_strategy: PruningStrategy,
}

/// Strategy for pruning dependency trees
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PruningStrategy {
    /// Aggressively prune deep dependencies
    Aggressive,
    /// Only include function signatures, not bodies
    Bodies,
    /// Include full implementations
    Full,
}

/// LSP configuration mapping file extensions to language server executables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// Map of file extensions to LSP server commands
    #[serde(flatten)]
    pub servers: HashMap<String, String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            token_budget: default_token_budget(),
            pruning_strategy: default_pruning_strategy(),
        }
    }
}

impl Default for LspConfig {
    fn default() -> Self {
        let mut servers = HashMap::new();
        servers.insert("rs".to_string(), "rust-analyzer".to_string());
        servers.insert("cpp".to_string(), "clangd".to_string());
        servers.insert(
            "ts".to_string(),
            "typescript-language-server --stdio".to_string(),
        );
        Self { servers }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            lsp: LspConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse TOML configuration")?;
        
        Ok(config)
    }
    
    /// Try to load config from default locations, or return default config
    pub fn load_or_default() -> Self {
        Self::try_load_default().unwrap_or_default()
    }
    
    /// Try to load config from default location (.skry.toml in current directory)
    fn try_load_default() -> Option<Self> {
        let config_path = Path::new(".skry.toml");
        if config_path.exists() {
            Self::load(config_path).ok()
        } else {
            None
        }
    }
    
    /// Get the LSP server command for a given file extension
    pub fn get_lsp_command(&self, extension: &str) -> Option<&str> {
        self.lsp.servers.get(extension).map(|s| s.as_str())
    }
}

fn default_token_budget() -> usize {
    4096
}

fn default_pruning_strategy() -> PruningStrategy {
    PruningStrategy::Aggressive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.token_budget, 4096);
        assert!(matches!(
            config.general.pruning_strategy,
            PruningStrategy::Aggressive
        ));
    }

    #[test]
    fn test_lsp_command_lookup() {
        let config = Config::default();
        assert_eq!(config.get_lsp_command("rs"), Some("rust-analyzer"));
        assert_eq!(config.get_lsp_command("cpp"), Some("clangd"));
    }
}
