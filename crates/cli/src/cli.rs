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

    /// Skip loading .env entirely
    #[arg(long, global = true)]
    pub no_dotenv: bool,

    /// Root paths to scan (positional, repeatable)
    #[arg(value_name = "PATH")]
    pub roots: Vec<PathBuf>,

    /// Show verbose scan summary
    #[arg(long)]
    pub verbose: bool,

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

    /// Allow list for marks (repeatable)
    #[arg(long = "filter-mark", value_name = "MARK")]
    pub filter_mark: Vec<String>,

    /// Deny list for marks (repeatable)
    #[arg(long = "filter-mark-deny", value_name = "MARK")]
    pub filter_mark_deny: Vec<String>,

    /// Allow list for languages (repeatable)
    #[arg(long = "filter-language", value_name = "LANG")]
    pub filter_language: Vec<String>,

    /// Deny list for languages (repeatable)
    #[arg(long = "filter-language-deny", value_name = "LANG")]
    pub filter_language_deny: Vec<String>,

    /// Allow list for paths (repeatable)
    #[arg(long = "filter-path", value_name = "PATH")]
    pub filter_path: Vec<PathBuf>,

    /// Deny list for paths (repeatable)
    #[arg(long = "filter-path-deny", value_name = "PATH")]
    pub filter_path_deny: Vec<PathBuf>,

    /// Allow list for folders (repeatable)
    #[arg(long = "filter-folder", value_name = "PATH")]
    pub filter_folder: Vec<PathBuf>,

    /// Deny list for folders (repeatable)
    #[arg(long = "filter-folder-deny", value_name = "PATH")]
    pub filter_folder_deny: Vec<PathBuf>,

    /// Disable file headers in output
    #[arg(long)]
    pub no_file_header: bool,
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
