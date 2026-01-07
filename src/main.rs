//! Main entry point for the skry CLI

use anyhow::Result;
use clap::{Parser, Subcommand};
use skry::{assembly, pipeline, standby, Config};
use std::path::PathBuf;
use tracing::info;

/// skry - A deterministic, LSP-powered context assembler for LLMs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the project root (defaults to current directory)
    #[arg(short, long)]
    root: Option<PathBuf>,

    /// Path to configuration file (defaults to .skry.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate context in one-shot mode
    Generate {
        /// File path for symbol resolution
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Symbol name to resolve
        #[arg(short, long)]
        symbol: Option<String>,

        /// Natural language search query
        #[arg(long)]
        search: Option<String>,

        /// Maximum traversal depth
        #[arg(short, long)]
        depth: Option<usize>,

        /// User intent/instruction
        #[arg(short, long)]
        intent: Option<String>,
    },
    /// Start the standby daemon
    Standby,
    /// Query the running standby daemon
    Query {
        /// Symbol to query
        #[arg(short, long)]
        symbol: Option<String>,

        /// Search query
        #[arg(long)]
        search: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_path) = cli.config {
        Config::load(config_path)?
    } else {
        Config::load_or_default()
    };

    // Determine project root
    let root = cli
        .root
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    info!("Using project root: {:?}", root);

    // Execute command
    match cli.command {
        Commands::Generate {
            file,
            symbol,
            search,
            depth,
            intent,
        } => {
            generate(config, root, file, symbol, search, depth, intent).await?;
        }
        Commands::Standby => {
            run_standby().await?;
        }
        Commands::Query { symbol, search } => {
            query_daemon(symbol, search).await?;
        }
    }

    Ok(())
}

/// Execute one-shot generation mode
async fn generate(
    config: Config,
    _root: PathBuf,
    file: Option<PathBuf>,
    symbol: Option<String>,
    search: Option<String>,
    depth: Option<usize>,
    intent: Option<String>,
) -> Result<()> {
    info!("Running in one-shot mode");

    // Parse intent
    let parsed_intent = pipeline::Pipeline::parse_intent(file, symbol, search, depth)?;

    // Create pipeline
    let max_depth = match &parsed_intent {
        pipeline::Intent::Search { depth, .. } => *depth,
        _ => depth.unwrap_or(3),
    };
    let pipeline = pipeline::Pipeline::new(max_depth);

    // Resolve entry point
    let entry = pipeline.resolve_entry_point(&parsed_intent).await?;
    info!("Entry point: {:?}", entry);

    // Traverse dependencies
    let graph = pipeline.traverse_dependencies(&entry).await?;
    info!("Traversed {} dependencies", graph.nodes.len());

    // Assemble context
    let assembler =
        assembly::Assembler::new(config.general.token_budget, config.general.pruning_strategy);

    let intent_str = intent.unwrap_or_else(|| "Context retrieval".to_string());
    let output = assembler.assemble(&graph, &intent_str)?;

    // Print to stdout
    println!("{}", output);

    Ok(())
}

/// Run the standby daemon
async fn run_standby() -> Result<()> {
    info!("Starting standby daemon");

    let daemon = standby::StandbyDaemon::new();
    daemon.start().await?;

    // Keep the daemon running
    // In a full implementation, this would listen for signals or commands
    info!("Press Ctrl+C to stop the daemon");
    tokio::signal::ctrl_c().await?;

    info!("Received shutdown signal");
    daemon.stop().await?;

    Ok(())
}

/// Query the running daemon
async fn query_daemon(symbol: Option<String>, search: Option<String>) -> Result<()> {
    // TODO: Implement IPC to connect to the actual running daemon process
    // For now, this creates a new instance which won't have access to the running daemon
    let daemon = standby::StandbyDaemon::new();

    let query = if let Some(sym) = symbol {
        format!("symbol:{}", sym)
    } else if let Some(s) = search {
        format!("search:{}", s)
    } else {
        anyhow::bail!("Must provide either --symbol or --search")
    };

    let result = daemon.query(&query).await?;
    println!("{}", result);

    Ok(())
}
