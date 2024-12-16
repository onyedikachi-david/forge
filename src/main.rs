mod core;
mod error;
mod ui;

use clap::{Parser, ValueEnum};
use core::Engine;
use error::Result;
use tracing::{info, Level};
use tracing_subscriber::filter::LevelFilter;

#[derive(Default, Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// API Key to be used
    #[arg(short, long)]
    key: String,

    /// Model to be used
    #[arg(short, long)]
    model: Option<String>,

    /// Base URL to be used
    #[arg(short, long)]
    base_url: Option<String>,

    /// Log level to use
    #[arg(long)]
    log_level: Option<LogLevel>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging with level from CLI
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level.unwrap_or_default())
        .init();

    // Initialize chat engine
    let engine = Engine::open_router(cli.key, cli.model.clone(), cli.base_url.clone());

    // Testing if the connection is successful
    engine.test().await?;

    Ok(())
}
