use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "koda",
    version,
    about = "Track TODO/FIXME marks in a workspace"
)]
pub struct Cli {
    /// Optional config file path (toml/json/yaml)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Config format override
    #[arg(long, value_enum, global = true)]
    pub config_format: Option<ConfigFormat>,

    /// Optional .env path (defaults to searching for .env)
    #[arg(long, global = true)]
    pub dotenv: Option<PathBuf>,

    /// Skip loading .env entirely
    #[arg(long, global = true)]
    pub no_dotenv: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Scan and list comment marks
    List(ListArgs),
    /// Start interactive TUI (not implemented yet)
    Tui(TuiArgs),
}

#[derive(Debug, Parser)]
pub struct ListArgs {
    /// Root paths to scan (repeatable)
    #[arg(short = 'r', long = "root")]
    pub roots: Vec<PathBuf>,

    /// Override regex pattern
    #[arg(long)]
    pub regex: Option<String>,

    /// Include glob patterns (repeatable)
    #[arg(long = "include")]
    pub include: Vec<String>,

    /// Exclude glob patterns (repeatable)
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,

    /// Whether to follow .gitignore (true/false)
    #[arg(long)]
    pub gitignore: Option<bool>,

    /// Whether to include hidden files (true/false)
    #[arg(long)]
    pub hidden: Option<bool>,

    /// Skip files larger than this size in bytes
    #[arg(long)]
    pub max_file_size: Option<u64>,

    /// Number of traversal threads
    #[arg(long)]
    pub threads: Option<usize>,

    /// Read buffer size in bytes
    #[arg(long)]
    pub read_buffer_size: Option<usize>,
}

#[derive(Debug, Parser)]
pub struct TuiArgs {}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ConfigFormat {
    Toml,
    Json,
    Yaml,
}

impl ConfigFormat {
    pub fn to_file_format(self) -> config::FileFormat {
        match self {
            ConfigFormat::Toml => config::FileFormat::Toml,
            ConfigFormat::Json => config::FileFormat::Json,
            ConfigFormat::Yaml => config::FileFormat::Yaml,
        }
    }
}
