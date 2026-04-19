use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dnm")]
#[command(about = "🗑️  dnm (Delete Node Modules) - Recursively delete node_modules directories")]
#[command(version)]
pub struct CliArgs {
    /// Target directory path
    #[arg(index = 1)]
    pub target_path: Option<PathBuf>,

    /// Run in interactive mode
    #[arg(short = 'i', long)]
    pub interactive: bool,

    /// Show size of deleted directories
    #[arg(short = 's', long)]
    pub size: bool,

    /// Disable progress bar
    #[arg(long)]
    pub no_progress: bool,

    /// Set log level (debug, info, warn, error)
    #[arg(short = 'l', long, default_value = "info")]
    pub log_level: String,

    /// Write logs to file
    #[arg(short = 'f', long)]
    pub log_file: Option<PathBuf>,

    /// Suppress console output
    #[arg(long)]
    pub silent: bool,

    /// Set language (zh-CN, en-US)
    #[arg(long, default_value = "zh-CN")]
    pub lang: String,

    /// Ignore specified directories (can be used multiple times)
    #[arg(long)]
    pub ignore: Vec<String>,

    /// Disable safe mode (delete without confirmation)
    #[arg(long)]
    pub no_safe: bool,

    /// Run with administrator/elevated privileges
    #[arg(long)]
    pub elevate: bool,
}

impl CliArgs {
    pub fn resolve_target_path(&self) -> PathBuf {
        self.target_path.clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }
}
