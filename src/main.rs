//! Command-line interface for minipg parser generator.

mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();
    commands::execute(cli)
}
