use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;

#[derive(Debug)]
pub struct ScanCounters {
    pub files_scanned: AtomicU64,
    pub files_skipped: AtomicU64,
    pub matches: AtomicU64,
    pub cancelled: AtomicBool,
}

impl Default for ScanCounters {
    fn default() -> Self {
        Self {
            files_scanned: AtomicU64::new(0),
            files_skipped: AtomicU64::new(0),
            matches: AtomicU64::new(0),
            cancelled: AtomicBool::new(false),
        }
    }
}
