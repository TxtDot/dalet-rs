use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dalet", bin_name = "dalet")]
#[command(about = "dalet cli")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum Commands {
    /// Format file
    Format { path: PathBuf },
}
