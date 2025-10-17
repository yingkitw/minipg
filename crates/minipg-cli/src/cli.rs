//! CLI argument definitions.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "minipg")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate parser from grammar file
    Generate {
        /// Grammar file to process
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Target language
        #[arg(short = 'l', long, default_value = "rust")]
        target_language: String,

        /// Package name for generated code
        #[arg(short, long)]
        package: Option<String>,

        /// Generate visitor pattern
        #[arg(long)]
        visitor: bool,

        /// Generate listener pattern
        #[arg(long, default_value = "true")]
        listener: bool,
    },

    /// Validate grammar file
    Validate {
        /// Grammar file to validate
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Show grammar information
    Info {
        /// Grammar file to analyze
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
}
