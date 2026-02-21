#[allow(clippy::module_inception)]
pub mod cli;
pub mod commands;

pub use cli::{Cli, Commands, run_cli};
pub use commands::execute;
