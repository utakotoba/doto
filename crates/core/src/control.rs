use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::model::{Mark, ScanWarning};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkipReason {
    MaxFileSize,
    Metadata,
    Io,
    UnsupportedSyntax,
}

pub trait ProgressReporter: Send + Sync {
    fn on_file_scanned(&self, _path: &Path) {}
    fn on_file_skipped(&self, _path: &Path, _reason: SkipReason) {}
    fn on_match(&self, _mark: &Mark) {}
    fn on_warning(&self, _warning: &ScanWarning) {}
    fn on_cancelled(&self) {}
}

#[derive(Clone)]
pub struct ProgressConfig {
    reporter: Arc<dyn ProgressReporter>,
}

impl ProgressConfig {
    pub fn new(reporter: Arc<dyn ProgressReporter>) -> Self {
        Self { reporter }
    }

    pub fn reporter(&self) -> &Arc<dyn ProgressReporter> {
        &self.reporter
    }
}

impl std::fmt::Debug for ProgressConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressConfig").finish()
    }
}

#[derive(Clone, Debug)]
pub struct CancellationToken {
    flag: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.flag.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}
