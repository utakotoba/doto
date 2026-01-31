use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::control::{CancellationToken, ProgressReporter, SkipReason};
use crate::scanner::stats::{ScanCounters, WarningKind};

pub fn record_issue(
    counters: &ScanCounters,
    kind: WarningKind,
) {
    counters.record_issue(kind);
}

pub fn report_file_scanned(progress: &Option<Arc<dyn ProgressReporter>>, path: &Path) {
    if let Some(progress) = progress.as_deref() {
        progress.on_file_scanned(path);
    }
}

pub fn report_file_skipped(
    progress: &Option<Arc<dyn ProgressReporter>>,
    path: &Path,
    reason: SkipReason,
) {
    if let Some(progress) = progress.as_deref() {
        progress.on_file_skipped(path, reason);
    }
}

pub fn is_cancelled(cancel: &Option<CancellationToken>) -> bool {
    cancel.as_ref().is_some_and(CancellationToken::is_cancelled)
}

pub fn mark_cancelled(flag: &AtomicBool, progress: &Option<Arc<dyn ProgressReporter>>) {
    if !flag.swap(true, Ordering::Relaxed) {
        if let Some(progress) = progress.as_deref() {
            progress.on_cancelled();
        }
    }
}
