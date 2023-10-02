use std::path::PathBuf;

use clap::Parser;

/// Program launch argument
#[derive(Debug, Default, Parser)]
#[command(
    author,
    about,
    name = "ng_stat",
    disable_version_flag = true,
    version,
    // disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    // #[arg(long)]
    // pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    /// Don't print anything during load: no progress bar or file list
    // #[arg(long)]
    // pub silent_load: bool,

    /// The log file or folder to analyze
    pub files: Vec<PathBuf>,
}
