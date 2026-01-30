mod file;
mod report;
mod stats;
mod walk;

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use ignore::WalkState;
use regex::bytes::Regex;

use crate::config::{DetectionConfig, ScanConfig};
use crate::control::SkipReason;
use crate::error::ScanError;
use crate::model::{Mark, ScanResult, ScanStats, ScanWarning};
use crate::scanner::file::{ScanOutcome, scan_file};
use crate::scanner::report::{
    is_cancelled, mark_cancelled, push_marks, push_warning, report_file_scanned,
    report_file_skipped,
};
use crate::scanner::stats::ScanCounters;
use crate::scanner::walk::build_walk_builder;

pub struct Scanner {
    config: ScanConfig,
    regex: Arc<Regex>,
}

impl Scanner {
    pub fn new(config: ScanConfig) -> Result<Self, ScanError> {
        if config.roots().is_empty() {
            return Err(ScanError::EmptyRoots);
        }

        let regex = match config.detection() {
            DetectionConfig::Regex { pattern } => Regex::new(pattern)?,
        };

        Ok(Self {
            config,
            regex: Arc::new(regex),
        })
    }

    pub fn scan(&self) -> Result<ScanResult, ScanError> {
        let builder = build_walk_builder(&self.config)?;
        let walker = builder.build_parallel();

        let marks = Arc::new(Mutex::new(Vec::<Mark>::new()));
        let warnings = Arc::new(Mutex::new(Vec::<ScanWarning>::new()));

        let counters = Arc::new(ScanCounters::default());

        let regex = Arc::clone(&self.regex);
        let config = self.config.clone();
        let progress = self
            .config
            .progress()
            .map(|progress| Arc::clone(progress.reporter()));
        let cancellation = self.config.cancellation_token().cloned();
        let marks_ref = Arc::clone(&marks);
        let warnings_ref = Arc::clone(&warnings);
        let counters_ref = Arc::clone(&counters);

        walker.run(move || {
            let regex = Arc::clone(&regex);
            let config = config.clone();
            let progress = progress.clone();
            let cancellation = cancellation.clone();
            let marks = Arc::clone(&marks_ref);
            let warnings = Arc::clone(&warnings_ref);
            let counters = Arc::clone(&counters_ref);

            Box::new(move |entry| {
                if is_cancelled(&cancellation) {
                    mark_cancelled(&counters.cancelled, &progress);
                    return WalkState::Quit;
                }

                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        push_warning(&warnings, &progress, None, err.to_string());
                        counters.files_skipped.fetch_add(1, Ordering::Relaxed);
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
                            return WalkState::Continue;
                        }
                        Err(err) => {
                            push_warning(
                                &warnings,
                                &progress,
                                Some(path.to_path_buf()),
                                err.to_string(),
                            );
                            report_file_skipped(&progress, path, SkipReason::Metadata);
                            counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                            return WalkState::Continue;
                        }
                        _ => {}
                    }
                }

                let mut local_marks = Vec::new();
                match scan_file(
                    path,
                    &regex,
                    &config,
                    &progress,
                    &cancellation,
                    &mut local_marks,
                ) {
                    Ok(ScanOutcome::Completed) => {
                        counters.files_scanned.fetch_add(1, Ordering::Relaxed);
                        report_file_scanned(&progress, path);
                        if !local_marks.is_empty() {
                            counters
                                .matches
                                .fetch_add(local_marks.len() as u64, Ordering::Relaxed);
                            push_marks(&marks, local_marks);
                        }
                    }
                    Ok(ScanOutcome::Skipped(reason)) => {
                        report_file_skipped(&progress, path, reason);
                        counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                    }
                    Ok(ScanOutcome::Cancelled) => {
                        if !local_marks.is_empty() {
                            counters
                                .matches
                                .fetch_add(local_marks.len() as u64, Ordering::Relaxed);
                            push_marks(&marks, local_marks);
                        }
                        mark_cancelled(&counters.cancelled, &progress);
                        return WalkState::Quit;
                    }
                    Err(err) => {
                        push_warning(
                            &warnings,
                            &progress,
                            Some(path.to_path_buf()),
                            err.to_string(),
                        );
                        report_file_skipped(&progress, path, SkipReason::Io);
                        counters.files_skipped.fetch_add(1, Ordering::Relaxed);
                    }
                }

                WalkState::Continue
            })
        });

        let marks = Arc::try_unwrap(marks)
            .map(|inner| inner.into_inner().unwrap_or_else(|err| err.into_inner()))
            .unwrap_or_else(|arc| arc.lock().unwrap_or_else(|err| err.into_inner()).clone());
        let warnings = Arc::try_unwrap(warnings)
            .map(|inner| inner.into_inner().unwrap_or_else(|err| err.into_inner()))
            .unwrap_or_else(|arc| arc.lock().unwrap_or_else(|err| err.into_inner()).clone());
        let stats = ScanStats {
            files_scanned: counters.files_scanned.load(Ordering::Relaxed),
            files_skipped: counters.files_skipped.load(Ordering::Relaxed),
            matches: counters.matches.load(Ordering::Relaxed),
            cancelled: counters.cancelled.load(Ordering::Relaxed),
        };

        Ok(ScanResult {
            marks,
            stats,
            warnings,
        })
    }
}
