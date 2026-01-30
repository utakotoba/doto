use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::control::{CancellationToken, ProgressReporter, SkipReason};
use crate::model::{Mark, ScanWarning};

pub fn push_marks(store: &Arc<Mutex<Vec<Mark>>>, mut marks: Vec<Mark>) {
    if marks.is_empty() {
        return;
    }
    let mut guard = store.lock().unwrap_or_else(|err| err.into_inner());
    guard.append(&mut marks);
}

pub fn push_warning(
    store: &Arc<Mutex<Vec<ScanWarning>>>,
    progress: &Option<Arc<dyn ProgressReporter>>,
    path: Option<std::path::PathBuf>,
    message: String,
) {
    let warning = ScanWarning { path, message };
    if let Some(progress) = progress.as_deref() {
        progress.on_warning(&warning);
    }
    let mut guard = store.lock().unwrap_or_else(|err| err.into_inner());
    guard.push(warning);
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
