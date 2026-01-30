use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

use regex::bytes::Regex;

use crate::comments::{BlockState, SyntaxSpec, find_comment_ranges, syntax_for_path};
use crate::config::ScanConfig;
use crate::constants::normalize_mark_bytes;
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
                let match_start = start + found.start();
                if !is_leading_mark(&buf, start, end, match_start, syntax.spec) {
                    continue;
                }
                let raw = &buf[match_start..start + found.end()];
                let Some(mark) = normalize_mark_bytes(raw) else {
                    continue;
                };
                let entry = Mark {
                    path: Arc::clone(&path),
                    line: line_no,
                    column: (match_start + 1) as u32,
                    mark,
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

fn is_leading_mark(
    line: &[u8],
    range_start: usize,
    range_end: usize,
    match_start: usize,
    spec: &SyntaxSpec,
) -> bool {
    let mut pos = range_start;
    let mut allow_block_marker = false;

    if let Some(token) = spec.line_comment {
        if starts_with(line, token, pos) {
            pos += token.len();
            if token.first() == Some(&b'/') {
                while pos < range_end && (line[pos] == b'/' || line[pos] == b'!') {
                    pos += 1;
                }
            } else if token.first() == Some(&b'#') {
                if pos < range_end && line[pos] == b'!' {
                    pos += 1;
                }
            }
            pos = skip_ws(line, pos, range_end);
            return match_start == pos;
        }
    }

    if let Some((start, _)) = spec.block_comment {
        if starts_with(line, start, pos) {
            pos += start.len();
            allow_block_marker = true;
        } else if range_start == 0 {
            allow_block_marker = true;
        }
    }

    if allow_block_marker {
        if pos < range_end && (line[pos] == b'*' || line[pos] == b'!') {
            pos += 1;
        }
    }
    pos = skip_ws(line, pos, range_end);
    match_start == pos
}

fn skip_ws(line: &[u8], mut pos: usize, end: usize) -> usize {
    while pos < end && line[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

fn starts_with(line: &[u8], needle: &[u8], idx: usize) -> bool {
    idx + needle.len() <= line.len() && &line[idx..idx + needle.len()] == needle
}
