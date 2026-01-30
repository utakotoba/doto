use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RunOptions {
    pub roots: usize,
    pub depth: usize,
    pub dirs_per_level: usize,
    pub files_per_dir: usize,
    pub min_lines: usize,
    pub max_lines: usize,
    pub mark_ratio: f64,
    pub seed: u64,
    pub out_dir: Option<PathBuf>,
    pub scan: bool,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            roots: 1,
            depth: 2,
            dirs_per_level: 4,
            files_per_dir: 20,
            min_lines: 10,
            max_lines: 120,
            mark_ratio: 0.02,
            seed: 42,
            out_dir: None,
            scan: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MatrixOptions {
    pub roots: Vec<usize>,
    pub depth: Vec<usize>,
    pub dirs_per_level: Vec<usize>,
    pub files_per_dir: Vec<usize>,
    pub min_lines: usize,
    pub max_lines: usize,
    pub mark_ratio: Vec<f64>,
    pub seed: u64,
    pub out_dir: Option<PathBuf>,
}

impl Default for MatrixOptions {
    fn default() -> Self {
        Self {
            roots: vec![1],
            depth: vec![2],
            dirs_per_level: vec![4],
            files_per_dir: vec![20],
            min_lines: 10,
            max_lines: 120,
            mark_ratio: vec![0.02],
            seed: 42,
            out_dir: None,
        }
    }
}
