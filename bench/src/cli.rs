use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "doto-bench",
    version,
    about = "Synthetic benchmark generator for doto"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate a corpus and optionally scan it
    Run(RunArgs),
    /// Run a matrix of benchmark configurations
    Matrix(MatrixArgs),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    /// Number of root directories to generate
    #[arg(long, default_value_t = 1)]
    pub roots: usize,

    /// Directory depth
    #[arg(long, default_value_t = 2)]
    pub depth: usize,

    /// Directories per level
    #[arg(long = "dirs", default_value_t = 4)]
    pub dirs_per_level: usize,

    /// Files per directory
    #[arg(long = "files", default_value_t = 20)]
    pub files_per_dir: usize,

    /// Minimum lines per file
    #[arg(long, default_value_t = 10)]
    pub min_lines: usize,

    /// Maximum lines per file
    #[arg(long, default_value_t = 120)]
    pub max_lines: usize,

    /// Ratio of lines that contain marks
    #[arg(long, default_value_t = 0.02)]
    pub mark_ratio: f64,

    /// Seed for deterministic generation
    #[arg(long, default_value_t = 42)]
    pub seed: u64,

    /// Output directory to reuse
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Skip scanning (generate only)
    #[arg(long)]
    pub no_scan: bool,
}

#[derive(Debug, Parser)]
pub struct MatrixArgs {
    /// Roots list (comma-separated)
    #[arg(long, value_delimiter = ',', default_value = "1")]
    pub roots: Vec<usize>,

    /// Depth list (comma-separated)
    #[arg(long, value_delimiter = ',', default_value = "2")]
    pub depth: Vec<usize>,

    /// Dirs per level list (comma-separated)
    #[arg(long = "dirs", value_delimiter = ',', default_value = "4")]
    pub dirs_per_level: Vec<usize>,

    /// Files per dir list (comma-separated)
    #[arg(long = "files", value_delimiter = ',', default_value = "20")]
    pub files_per_dir: Vec<usize>,

    /// Minimum lines per file
    #[arg(long, default_value_t = 10)]
    pub min_lines: usize,

    /// Maximum lines per file
    #[arg(long, default_value_t = 120)]
    pub max_lines: usize,

    /// Mark ratio list (comma-separated)
    #[arg(long, value_delimiter = ',', default_value = "0.02")]
    pub mark_ratio: Vec<f64>,

    /// Seed for deterministic generation
    #[arg(long, default_value_t = 42)]
    pub seed: u64,

    /// Output directory base to store generated corpora
    #[arg(long)]
    pub out: Option<PathBuf>,
}
