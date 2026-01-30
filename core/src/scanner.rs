use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use ignore::overrides::OverrideBuilder;
use ignore::{WalkBuilder, WalkState};
use regex::bytes::Regex;

use crate::config::{DetectionConfig, ScanConfig};
use crate::error::ScanError;
use crate::model::{Mark, ScanResult, ScanStats, ScanWarning};

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
        let builder = self.walk_builder()?;
        let walker = builder.build_parallel();

        let marks = Arc::new(Mutex::new(Vec::<Mark>::new()));
        let warnings = Arc::new(Mutex::new(Vec::<ScanWarning>::new()));

        let files_scanned = Arc::new(AtomicU64::new(0));
        let files_skipped = Arc::new(AtomicU64::new(0));
        let matches = Arc::new(AtomicU64::new(0));

        let regex = Arc::clone(&self.regex);
        let config = self.config.clone();
        let marks_ref = Arc::clone(&marks);
        let warnings_ref = Arc::clone(&warnings);
        let files_scanned_ref = Arc::clone(&files_scanned);
        let files_skipped_ref = Arc::clone(&files_skipped);
        let matches_ref = Arc::clone(&matches);

        walker.run(move || {
            let regex = Arc::clone(&regex);
            let config = config.clone();
            let marks = Arc::clone(&marks_ref);
            let warnings = Arc::clone(&warnings_ref);
            let files_scanned = Arc::clone(&files_scanned_ref);
            let files_skipped = Arc::clone(&files_skipped_ref);
            let matches = Arc::clone(&matches_ref);

            Box::new(move |entry| {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        push_warning(&warnings, None, err.to_string());
                        files_skipped.fetch_add(1, Ordering::Relaxed);
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
                            files_skipped.fetch_add(1, Ordering::Relaxed);
                            return WalkState::Continue;
                        }
                        Err(err) => {
                            push_warning(&warnings, Some(path.to_path_buf()), err.to_string());
                            files_skipped.fetch_add(1, Ordering::Relaxed);
                            return WalkState::Continue;
                        }
                        _ => {}
                    }
                }

                let mut local_marks = Vec::new();
                match scan_file(path, &regex, &config, &mut local_marks) {
                    Ok(()) => {
                        files_scanned.fetch_add(1, Ordering::Relaxed);
                        if !local_marks.is_empty() {
                            matches.fetch_add(local_marks.len() as u64, Ordering::Relaxed);
                            push_marks(&marks, local_marks);
                        }
                    }
                    Err(err) => {
                        push_warning(&warnings, Some(path.to_path_buf()), err.to_string());
                        files_skipped.fetch_add(1, Ordering::Relaxed);
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
            files_scanned: files_scanned.load(Ordering::Relaxed),
            files_skipped: files_skipped.load(Ordering::Relaxed),
            matches: matches.load(Ordering::Relaxed),
        };

        Ok(ScanResult {
            marks,
            stats,
            warnings,
        })
    }

    fn walk_builder(&self) -> Result<WalkBuilder, ScanError> {
        let mut roots = self.config.roots().iter();
        let first = roots.next().ok_or(ScanError::EmptyRoots)?;
        let mut builder = WalkBuilder::new(first);
        for root in roots {
            builder.add(root);
        }

        if let Some(threads) = self.config.threads() {
            builder.threads(threads);
        }

        let follow_gitignore = self.config.follow_gitignore();
        builder
            .git_ignore(follow_gitignore)
            .git_exclude(follow_gitignore)
            .git_global(follow_gitignore)
            .ignore(follow_gitignore)
            .hidden(!self.config.include_hidden());

        if !self.config.include().is_empty() || !self.config.exclude().is_empty() {
            let override_base = self
                .config
                .roots()
                .first()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            let mut overrides = OverrideBuilder::new(override_base);
            for include in self.config.include() {
                overrides.add(include)?;
            }
            for exclude in self.config.exclude() {
                let pattern = if exclude.starts_with('!') {
                    exclude.to_string()
                } else {
                    format!("!{exclude}")
                };
                overrides.add(&pattern)?;
            }
            builder.overrides(overrides.build()?);
        }

        Ok(builder)
    }
}

fn scan_file(
    path: &Path,
    regex: &Regex,
    config: &ScanConfig,
    output: &mut Vec<Mark>,
) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(config.read_buffer_size(), file);
    let mut buf = Vec::with_capacity(4096);
    let mut line_no: u32 = 0;
    let path = Arc::new(path.to_path_buf());

    loop {
        buf.clear();
        let read = reader.read_until(b'\n', &mut buf)?;
        if read == 0 {
            break;
        }
        line_no = line_no.saturating_add(1);

        for found in regex.find_iter(&buf) {
            let column = (found.start() + 1) as u32;
            let mark = String::from_utf8_lossy(&buf[found.start()..found.end()]).into_owned();
            output.push(Mark {
                path: Arc::clone(&path),
                line: line_no,
                column,
                mark,
            });
        }
    }

    Ok(())
}

fn push_marks(store: &Arc<Mutex<Vec<Mark>>>, mut marks: Vec<Mark>) {
    if marks.is_empty() {
        return;
    }
    let mut guard = store.lock().unwrap_or_else(|err| err.into_inner());
    guard.append(&mut marks);
}

fn push_warning(store: &Arc<Mutex<Vec<ScanWarning>>>, path: Option<PathBuf>, message: String) {
    let mut guard = store.lock().unwrap_or_else(|err| err.into_inner());
    guard.push(ScanWarning { path, message });
}
