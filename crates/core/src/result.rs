use crate::model::{ScanStats, ScanWarning};
use crate::sort::GroupTree;

#[derive(Clone, Debug)]
pub struct GroupedScanResult {
    pub tree: GroupTree,
    pub stats: ScanStats,
    pub warnings: Vec<ScanWarning>,
}
