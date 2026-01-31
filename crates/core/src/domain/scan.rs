use crate::domain::{GroupTree, Mark};

#[derive(Clone, Debug, Default)]
pub struct ScanStats {
    pub files_scanned: u64,
    pub files_skipped: u64,
    pub matches: u64,
    pub cancelled: bool,
}

#[derive(Clone, Debug)]
pub struct ScanWarning {
    pub path: Option<std::path::PathBuf>,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct ScanResult {
    pub marks: Vec<Mark>,
    pub stats: ScanStats,
    pub warnings: Vec<ScanWarning>,
}

#[derive(Clone, Debug)]
pub struct GroupedScanResult {
    pub tree: GroupTree,
    pub stats: ScanStats,
    pub warnings: Vec<ScanWarning>,
}
