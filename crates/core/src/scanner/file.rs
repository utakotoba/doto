use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

use regex::bytes::Regex;

use crate::comments::{BlockState, find_comment_ranges, syntax_for_path};
use crate::config::ScanConfig;
use crate::constants::normalize_mark;
use crate::control::{CancellationToken, ProgressReporter, SkipReason};
use crate::model::Mark;
use crate::scanner::report::is_cancelled;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanOutcome {
    Completed,
    Skipped(SkipReason),
    Cancelled,
}

pub fn scan_file(
    path: &Path,
    regex: &Regex,
    config: &ScanConfig,
    progress: &Option<Arc<dyn ProgressReporter>>,
    cancellation: &Option<CancellationToken>,
    output: &mut Vec<Mark>,
) -> io::Result<ScanOutcome> {
    let Some(syntax) = syntax_for_path(path) else {
        return Ok(ScanOutcome::Skipped(SkipReason::UnsupportedSyntax));
    };

    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(config.read_buffer_size(), file);
    let mut buf = Vec::with_capacity(4096);
    let mut line_no: u32 = 0;
    let path = Arc::new(path.to_path_buf());
    let mut block_state = BlockState::default();

    loop {
        if is_cancelled(cancellation) {
            return Ok(ScanOutcome::Cancelled);
        }
        buf.clear();
        let read = reader.read_until(b'\n', &mut buf)?;
        if read == 0 {
            break;
        }
        line_no = line_no.saturating_add(1);

        find_comment_ranges(&buf, &mut block_state, syntax.spec, |start, end| {
            for found in regex.find_iter(&buf[start..end]) {
                let raw = &buf[start + found.start()..start + found.end()];
                let Ok(raw_str) = std::str::from_utf8(raw) else {
                    continue;
                };
                let Some(mark) = normalize_mark(raw_str) else {
                    continue;
                };
                let entry = Mark {
                    path: Arc::clone(&path),
                    line: line_no,
                    column: (start + found.start() + 1) as u32,
                    mark: mark.to_string(),
                    language: syntax.language,
                };
                if let Some(progress) = progress.as_deref() {
                    progress.on_match(&entry);
                }
                output.push(entry);
            }
        });
    }

    Ok(ScanOutcome::Completed)
}
