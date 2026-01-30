use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use doto_core::{CancellationToken, ProgressReporter, ScanConfig, SkipReason, scan};
use tempfile::TempDir;

#[derive(Default)]
struct TestProgress {
    files_scanned: AtomicUsize,
    files_skipped: AtomicUsize,
    matches: AtomicUsize,
    warnings: AtomicUsize,
    cancelled: AtomicUsize,
    cancel_after_match: Option<CancellationToken>,
}

impl ProgressReporter for TestProgress {
    fn on_file_scanned(&self, _path: &Path) {
        self.files_scanned.fetch_add(1, Ordering::Relaxed);
    }

    fn on_file_skipped(&self, _path: &Path, _reason: SkipReason) {
        self.files_skipped.fetch_add(1, Ordering::Relaxed);
    }

    fn on_match(&self, _mark: &doto_core::Mark) {
        self.matches.fetch_add(1, Ordering::Relaxed);
        if let Some(token) = &self.cancel_after_match {
            token.cancel();
        }
    }

    fn on_warning(&self, _warning: &doto_core::ScanWarning) {
        self.warnings.fetch_add(1, Ordering::Relaxed);
    }

    fn on_cancelled(&self) {
        self.cancelled.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn scan_reports_progress_events() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("one.rs");
    fs::write(&file_path, "// TODO: one\n")?;

    let reporter = Arc::new(TestProgress::default());
    let reporter_dyn: Arc<dyn ProgressReporter> = reporter.clone();
    let config = ScanConfig::builder()
        .root(temp.path())
        .progress_reporter_arc(reporter_dyn)
        .build();

    let result = scan(config)?;

    assert_eq!(result.stats.files_scanned, 1);
    assert_eq!(reporter.files_scanned.load(Ordering::Relaxed), 1);
    assert_eq!(reporter.matches.load(Ordering::Relaxed), 1);
    assert_eq!(reporter.cancelled.load(Ordering::Relaxed), 0);
    Ok(())
}

#[test]
fn scan_can_cancel_via_token() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let first = temp.path().join("first.rs");
    let second = temp.path().join("second.rs");
    fs::write(&first, "// TODO: first\n")?;
    fs::write(&second, "// TODO: second\n")?;

    let token = CancellationToken::new();
    let reporter = Arc::new(TestProgress {
        cancel_after_match: Some(token.clone()),
        ..TestProgress::default()
    });
    let reporter_dyn: Arc<dyn ProgressReporter> = reporter.clone();

    let config = ScanConfig::builder()
        .root(temp.path())
        .threads(Some(1))
        .progress_reporter_arc(reporter_dyn)
        .cancellation_token(token)
        .build();

    let result = scan(config)?;

    assert!(result.stats.cancelled);
    assert_eq!(reporter.cancelled.load(Ordering::Relaxed), 1);
    assert_eq!(reporter.matches.load(Ordering::Relaxed), 1);
    Ok(())
}
