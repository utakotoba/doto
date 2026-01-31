use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

use regex::bytes::Regex;

use crate::comments::{BlockState, SyntaxSpec, find_comment_ranges, syntax_for_path};
use crate::config::ScanConfig;
use crate::constants::{DEFAULT_MARK_REGEX, normalize_mark_bytes};
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
    let use_default_detection = is_default_detection(config);

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
            if !contains_mark_initial(&buf, start, end) {
                return;
            }

            if use_default_detection {
                if let Some(match_start) = leading_mark_pos(&buf, start, end, syntax.spec) {
                    if let Some((mark, _len)) = match_builtin_mark(&buf[match_start..end]) {
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
                }
                return;
            }

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

fn is_default_detection(config: &ScanConfig) -> bool {
    matches!(config.detection(), crate::config::DetectionConfig::Regex { pattern }
        if pattern == DEFAULT_MARK_REGEX)
}

fn is_leading_mark(
    line: &[u8],
    range_start: usize,
    range_end: usize,
    match_start: usize,
    spec: &SyntaxSpec,
) -> bool {
    leading_mark_pos(line, range_start, range_end, spec)
        .is_some_and(|pos| pos == match_start)
}

fn leading_mark_pos(
    line: &[u8],
    range_start: usize,
    range_end: usize,
    spec: &SyntaxSpec,
) -> Option<usize> {
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
            return (pos < range_end).then_some(pos);
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
    (pos < range_end).then_some(pos)
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

fn contains_mark_initial(line: &[u8], start: usize, end: usize) -> bool {
    if start >= end || end > line.len() {
        return false;
    }
    let hay = &line[start..end];
    memchr::memchr(b'E', hay).is_some()
        || memchr::memchr(b'W', hay).is_some()
        || memchr::memchr(b'F', hay).is_some()
        || memchr::memchr(b'T', hay).is_some()
        || memchr::memchr(b'N', hay).is_some()
        || memchr::memchr(b'I', hay).is_some()
}

fn match_builtin_mark(input: &[u8]) -> Option<(&'static str, usize)> {
    const MARKS: [(&str, &[u8]); 6] = [
        ("ERROR", b"ERROR"),
        ("WARN", b"WARN"),
        ("FIXME", b"FIXME"),
        ("TODO", b"TODO"),
        ("NOTE", b"NOTE"),
        ("INFO", b"INFO"),
    ];

    for (name, bytes) in MARKS {
        if input.starts_with(bytes) {
            let end = bytes.len();
            if end == input.len() || !is_word_char(input[end]) {
                return Some((name, end));
            }
        }
    }
    None
}

fn is_word_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
