use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use doto_core::{Mark, ProgressReporter, SkipReason};

pub struct DeferredProgress {
    active: AtomicBool,
    finished: AtomicBool,
    bar: Mutex<Option<ProgressBar>>,
    files_scanned: AtomicU64,
    files_skipped: AtomicU64,
    matches: AtomicU64,
}

impl DeferredProgress {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            active: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            bar: Mutex::new(None),
            files_scanned: AtomicU64::new(0),
            files_skipped: AtomicU64::new(0),
            matches: AtomicU64::new(0),
        })
    }

    pub fn start_if_slow(self: Arc<Self>, delay: Duration) {
        thread::spawn(move || {
            thread::sleep(delay);
            if self.finished.load(Ordering::Relaxed) || !io::stderr().is_terminal() {
                return;
            }
            self.activate();
        });
    }

    pub fn finish(&self) {
        self.finished.store(true, Ordering::Relaxed);
        if let Some(bar) = self
            .bar
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().cloned())
        {
            bar.finish_and_clear();
        }
    }

    fn activate(&self) {
        if self.active.swap(true, Ordering::Relaxed) {
            return;
        }
        let bar = ProgressBar::new_spinner();
        let style = ProgressStyle::with_template("{msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner());
        bar.set_style(style);
        self.update_message(&bar);
        if let Ok(mut guard) = self.bar.lock() {
            *guard = Some(bar);
        }
    }

    fn update_message(&self, bar: &ProgressBar) {
        let scanned = self.files_scanned.load(Ordering::Relaxed);
        let skipped = self.files_skipped.load(Ordering::Relaxed);
        let matches = self.matches.load(Ordering::Relaxed);
        bar.set_message(format!(
            "scanning (files {scanned}, skipped {skipped}, matches {matches})"
        ));
    }

    fn refresh(&self, tick_matches: bool) {
        if !self.active.load(Ordering::Relaxed) {
            return;
        }
        if let Ok(guard) = self.bar.lock() {
            if let Some(bar) = guard.as_ref() {
                if tick_matches {
                    self.update_message(bar);
                }
            }
        }
    }
}

impl ProgressReporter for DeferredProgress {
    fn on_file_scanned(&self, _path: &std::path::Path) {
        self.files_scanned.fetch_add(1, Ordering::Relaxed);
        self.refresh(true);
    }

    fn on_file_skipped(&self, _path: &std::path::Path, _reason: SkipReason) {
        self.files_skipped.fetch_add(1, Ordering::Relaxed);
        self.refresh(true);
    }

    fn on_match(&self, _mark: &Mark) {
        let next = self.matches.fetch_add(1, Ordering::Relaxed) + 1;
        if next % 100 == 0 {
            self.refresh(true);
        }
    }

    fn on_cancelled(&self) {
        self.finish();
    }
}
