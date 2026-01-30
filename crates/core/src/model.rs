use std::path::PathBuf;
use std::sync::Arc;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Mark {
    pub path: Arc<PathBuf>,
    pub line: u32,
    pub column: u32,
    pub mark: String,
}

#[derive(Clone, Debug, Default)]
pub struct ScanStats {
    pub files_scanned: u64,
    pub files_skipped: u64,
    pub matches: u64,
}

#[derive(Clone, Debug)]
pub struct ScanWarning {
    pub path: Option<PathBuf>,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct ScanResult {
    pub marks: Vec<Mark>,
    pub stats: ScanStats,
    pub warnings: Vec<ScanWarning>,
}
