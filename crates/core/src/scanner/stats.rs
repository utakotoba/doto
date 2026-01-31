use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::control::SkipReason;

#[derive(Debug)]
pub struct ScanCounters {
    pub files_scanned: AtomicU64,
    pub files_skipped: AtomicU64,
    pub matches: AtomicU64,
    pub cancelled: AtomicBool,
    pub skipped_expected: AtomicU64,
    pub skipped_issues: AtomicU64,
    pub skip_max_file_size: AtomicU64,
    pub skip_metadata: AtomicU64,
    pub skip_io: AtomicU64,
    pub skip_unsupported_syntax: AtomicU64,
    pub skip_binary: AtomicU64,
    pub warn_walk: AtomicU64,
    pub warn_metadata: AtomicU64,
    pub warn_io: AtomicU64,
}

impl Default for ScanCounters {
    fn default() -> Self {
        Self {
            files_scanned: AtomicU64::new(0),
            files_skipped: AtomicU64::new(0),
            matches: AtomicU64::new(0),
            cancelled: AtomicBool::new(false),
            skipped_expected: AtomicU64::new(0),
            skipped_issues: AtomicU64::new(0),
            skip_max_file_size: AtomicU64::new(0),
            skip_metadata: AtomicU64::new(0),
            skip_io: AtomicU64::new(0),
            skip_unsupported_syntax: AtomicU64::new(0),
            skip_binary: AtomicU64::new(0),
            warn_walk: AtomicU64::new(0),
            warn_metadata: AtomicU64::new(0),
            warn_io: AtomicU64::new(0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WarningKind {
    Walk,
    Metadata,
    Io,
}

impl ScanCounters {
    pub fn record_skip(&self, reason: SkipReason) {
        match reason {
            SkipReason::MaxFileSize => {
                self.skip_max_file_size.fetch_add(1, Ordering::Relaxed);
                self.skipped_expected.fetch_add(1, Ordering::Relaxed);
            }
            SkipReason::Metadata => {
                self.skip_metadata.fetch_add(1, Ordering::Relaxed);
                self.skipped_issues.fetch_add(1, Ordering::Relaxed);
            }
            SkipReason::Io => {
                self.skip_io.fetch_add(1, Ordering::Relaxed);
                self.skipped_issues.fetch_add(1, Ordering::Relaxed);
            }
            SkipReason::UnsupportedSyntax => {
                self.skip_unsupported_syntax
                    .fetch_add(1, Ordering::Relaxed);
                self.skipped_expected.fetch_add(1, Ordering::Relaxed);
            }
            SkipReason::Binary => {
                self.skip_binary.fetch_add(1, Ordering::Relaxed);
                self.skipped_expected.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn record_issue(&self, kind: WarningKind) {
        match kind {
            WarningKind::Walk => {
                self.warn_walk.fetch_add(1, Ordering::Relaxed);
            }
            WarningKind::Metadata => {
                self.warn_metadata.fetch_add(1, Ordering::Relaxed);
            }
            WarningKind::Io => {
                self.warn_io.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}
