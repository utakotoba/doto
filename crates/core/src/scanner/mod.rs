mod file;
mod report;
mod stats;
mod walk;

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use ignore::WalkState;
use crate::config::ScanConfig;
use crate::control::SkipReason;
use crate::error::ScanError;
use crate::model::{GroupedScanResult, Mark, ScanIssueCounts, ScanResult, ScanSkipCounts, ScanStats};
use crate::scanner::file::{ScanOutcome, scan_file};
use crate::scanner::report::{
    is_cancelled, mark_cancelled, record_issue, report_file_scanned, report_file_skipped,
};
use crate::scanner::stats::{ScanCounters, WarningKind};
use crate::scanner::walk::build_walk_builder;
use crate::sort::{apply_sort_pipeline, build_group_tree};

pub struct Scanner {
    config: ScanConfig,
}

impl Scanner {
    pub fn new(config: ScanConfig) -> Result<Self, ScanError> {
        if config.roots().is_empty() {
            return Err(ScanError::EmptyRoots);
        }

        Ok(Self { config })
    }

    pub fn scan(&self) -> Result<ScanResult, ScanError> {
        let output = self.scan_raw()?;
        let filtered = self
            .config
            .filter_config()
            .apply(output.marks, self.config.roots());
        let sorted_marks =
            apply_sort_pipeline(filtered, self.config.sort_config(), self.config.roots());

        Ok(ScanResult {
            marks: sorted_marks,
            stats: output.stats,
        })
    }

    pub fn scan_grouped(&self) -> Result<GroupedScanResult, ScanError> {
        let output = self.scan_raw()?;
        let filtered = self
            .config
            .filter_config()
            .apply(output.marks, self.config.roots());
        let tree = build_group_tree(filtered, self.config.sort_config(), self.config.roots());

        Ok(GroupedScanResult {
            tree,
            stats: output.stats,
        })
    }

    fn scan_raw(&self) -> Result<RawScanOutput, ScanError> {
        let counters = Arc::new(ScanCounters::default());
        let output = Arc::new(Mutex::new(SharedOutput::default()));

        let config = self.config.clone();
        let progress = self
            .config
            .progress()
            .map(|progress| Arc::clone(progress.reporter()));
        let cancellation = self.config.cancellation_token().cloned();

        for root in self.config.roots() {
            if is_cancelled(&cancellation) {
                mark_cancelled(&counters.cancelled, &progress);
                break;
            }

            let builder = build_walk_builder(&self.config, root)?;
            let walker = builder.build_parallel();

            let counters_ref = Arc::clone(&counters);
            let output_ref = Arc::clone(&output);
            let config = config.clone();
            let progress = progress.clone();
            let cancellation = cancellation.clone();

            walker.run(move || {
                let config = config.clone();
                let progress = progress.clone();
                let cancellation = cancellation.clone();
                let counters = Arc::clone(&counters_ref);
                let output = Arc::clone(&output_ref);

                let mut local = LocalOutput::new(output);

                Box::new(move |entry| {
                    if is_cancelled(&cancellation) {
                        mark_cancelled(&counters.cancelled, &progress);
                        return WalkState::Quit;
                    }

                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(_) => {
                            record_issue(&counters, WarningKind::Walk);
                            counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                            counters.record_skip(SkipReason::Io);
                            return WalkState::Continue;
                        }
                    };

                    let file_type = match entry.file_type() {
                        Some(file_type) => file_type,
                        None => return WalkState::Continue,
                    };
                    if !file_type.is_file() {
                        return WalkState::Continue;
                    }

                    let path = entry.path();
                    if let Some(max_file_size) = config.max_file_size() {
                        match entry.metadata() {
                            Ok(metadata) if metadata.len() > max_file_size => {
                                report_file_skipped(&progress, path, SkipReason::MaxFileSize);
                                counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                                counters.record_skip(SkipReason::MaxFileSize);
                                return WalkState::Continue;
                            }
                            Err(_) => {
                                record_issue(&counters, WarningKind::Metadata);
                                report_file_skipped(&progress, path, SkipReason::Metadata);
                                counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                                counters.record_skip(SkipReason::Metadata);
                                return WalkState::Continue;
                            }
                            _ => {}
                        }
                    }

                    let before = local.marks.len();
                    match scan_file(
                        path,
                        &config,
                        &progress,
                        &cancellation,
                        &mut local.marks,
                    ) {
                        Ok(ScanOutcome::Completed) => {
                            counters.files_scanned.fetch_add(1, Ordering::Relaxed);
                            report_file_scanned(&progress, path);
                            let added = local.marks.len().saturating_sub(before);
                            if added > 0 {
                                counters.matches.fetch_add(added as u64, Ordering::Relaxed);
                            }
                        }
                        Ok(ScanOutcome::Skipped(reason)) => {
                            report_file_skipped(&progress, path, reason);
                            counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                            counters.record_skip(reason);
                        }
                        Ok(ScanOutcome::Cancelled) => {
                            let added = local.marks.len().saturating_sub(before);
                            if added > 0 {
                                counters.matches.fetch_add(added as u64, Ordering::Relaxed);
                            }
                            mark_cancelled(&counters.cancelled, &progress);
                            return WalkState::Quit;
                        }
                        Err(_) => {
                            record_issue(&counters, WarningKind::Io);
                            report_file_skipped(&progress, path, SkipReason::Io);
                            counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                            counters.record_skip(SkipReason::Io);
                        }
                    }

                    WalkState::Continue
                })
            });
        }

        let output = Arc::try_unwrap(output)
            .map(|inner| inner.into_inner().unwrap_or_else(|err| err.into_inner()))
            .unwrap_or_else(|arc| arc.lock().unwrap_or_else(|err| err.into_inner()).clone());
        let stats = ScanStats {
            files_scanned: counters.files_scanned.load(Ordering::Relaxed),
            files_skipped: counters.files_skipped.load(Ordering::Relaxed),
            matches: counters.matches.load(Ordering::Relaxed),
            cancelled: counters.cancelled.load(Ordering::Relaxed),
            skipped_expected: counters.skipped_expected.load(Ordering::Relaxed),
            skipped_issues: counters.skipped_issues.load(Ordering::Relaxed),
            skips: ScanSkipCounts {
                max_file_size: counters.skip_max_file_size.load(Ordering::Relaxed),
                metadata: counters.skip_metadata.load(Ordering::Relaxed),
                io: counters.skip_io.load(Ordering::Relaxed),
                unsupported_syntax: counters.skip_unsupported_syntax.load(Ordering::Relaxed),
                binary: counters.skip_binary.load(Ordering::Relaxed),
            },
            issues: ScanIssueCounts {
                walk_errors: counters.warn_walk.load(Ordering::Relaxed),
                metadata_errors: counters.warn_metadata.load(Ordering::Relaxed),
                io_errors: counters.warn_io.load(Ordering::Relaxed),
            },
        };

        Ok(RawScanOutput {
            marks: output.marks,
            stats,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct SharedOutput {
    marks: Vec<Mark>,
}

#[derive(Debug)]
struct LocalOutput {
    marks: Vec<Mark>,
    shared: Arc<Mutex<SharedOutput>>,
}

#[derive(Clone, Debug)]
struct RawScanOutput {
    marks: Vec<Mark>,
    stats: ScanStats,
}

impl LocalOutput {
    fn new(shared: Arc<Mutex<SharedOutput>>) -> Self {
        Self {
            marks: Vec::new(),
            shared,
        }
    }
}

impl Drop for LocalOutput {
    fn drop(&mut self) {
        if self.marks.is_empty() {
            return;
        }
        let mut guard = self.shared.lock().unwrap_or_else(|err| err.into_inner());
        guard.marks.append(&mut self.marks);
    }
}
