use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Mark {
    pub path: Arc<PathBuf>,
    pub line: u32,
    pub column: u32,
    pub mark: &'static str,
    pub language: &'static str,
}

#[derive(Clone, Debug, Default)]
pub struct ScanStats {
    pub files_scanned: u64,
    pub files_skipped: u64,
    pub matches: u64,
    pub cancelled: bool,
    pub skipped_expected: u64,
    pub skipped_issues: u64,
    pub skips: ScanSkipCounts,
    pub issues: ScanIssueCounts,
}

#[derive(Clone, Debug, Default)]
pub struct ScanSkipCounts {
    pub max_file_size: u64,
    pub metadata: u64,
    pub io: u64,
    pub unsupported_syntax: u64,
    pub binary: u64,
}

#[derive(Clone, Debug, Default)]
pub struct ScanIssueCounts {
    pub walk_errors: u64,
    pub metadata_errors: u64,
    pub io_errors: u64,
}

#[derive(Clone, Debug)]
pub struct ScanResult {
    pub marks: Vec<Mark>,
    pub stats: ScanStats,
}

#[derive(Clone, Debug)]
pub struct GroupedScanResult {
    pub tree: GroupTree,
    pub stats: ScanStats,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Mark,
    Language,
    Path,
    Folder,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DimensionValue {
    Mark(Cow<'static, str>),
    Language(Cow<'static, str>),
    Path(PathBuf),
    Folder(PathBuf),
}

#[derive(Clone, Debug)]
pub struct GroupTree {
    pub groups: Vec<GroupNode>,
    pub items: Vec<Mark>,
}

impl GroupTree {
    pub fn total(&self) -> usize {
        if self.groups.is_empty() {
            return self.items.len();
        }
        self.groups.iter().map(|group| group.count).sum()
    }
}

#[derive(Clone, Debug)]
pub struct GroupNode {
    pub key: DimensionValue,
    pub count: usize,
    pub groups: Vec<GroupNode>,
    pub items: Vec<Mark>,
}
