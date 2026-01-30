use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "doto",
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

    /// Root paths to scan (positional, repeatable)
    #[arg(value_name = "PATH")]
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

    /// Sort pipeline stages (comma separated). Example: mark,language,folder
    #[arg(long, value_name = "STAGES")]
    pub sort: Option<String>,

    /// Mark priority overrides (comma separated). Example: FIXME=0,TODO=1
    #[arg(long, value_name = "PAIR")]
    pub sort_mark_priority: Option<String>,

    /// Language group ordering (count|name)
    #[arg(long, value_enum)]
    pub sort_lang_order: Option<SortLangOrderArg>,

    /// Path group order (asc|desc)
    #[arg(long, value_enum)]
    pub sort_path_order: Option<SortOrderArg>,


    /// Folder group depth (relative to scan root)
    #[arg(long)]
    pub sort_folder_depth: Option<usize>,

    /// Folder group order (asc|desc)
    #[arg(long, value_enum)]
    pub sort_folder_order: Option<SortOrderArg>,

    /// Disable file headers in output
    #[arg(long)]
    pub no_file_header: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ConfigFormat {
    Toml,
    Json,
    Yaml,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortOrderArg {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortLangOrderArg {
    Count,
    Name,
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
