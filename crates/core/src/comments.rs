use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum CommentSyntax {
    Line(&'static [u8]),
    Hash,
    CStyle { allow_backtick: bool },
}

impl CommentSyntax {
    pub fn for_path(path: &Path) -> Option<Self> {
        let ext = path.extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
        let name = path.file_name().map(|name| name.to_string_lossy().to_ascii_lowercase());
        if let Some(name) = name.as_deref() {
            if name == "makefile" {
                return Some(CommentSyntax::Line(b"#"));
            }
            if name == "dockerfile" {
                return Some(CommentSyntax::Line(b"#"));
            }
        }
        let ext = ext.as_deref()?;
        let syntax = match ext {
            "rs" | "c" | "h" | "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" | "java" | "kt"
            | "kts" | "swift" | "go" | "cs" | "scala" | "dart" => {
                CommentSyntax::CStyle {
                    allow_backtick: false,
                }
            }
            "js" | "jsx" | "ts" | "tsx" => CommentSyntax::CStyle {
                allow_backtick: true,
            },
            "py" | "rb" | "sh" | "bash" | "zsh" | "yml" | "yaml" | "toml" | "ini"
            | "cfg" | "conf" | "env" => {
                CommentSyntax::Hash
            }
            "lua" => CommentSyntax::Line(b"--"),
            "mk" => CommentSyntax::Line(b"#"),
            _ => return None,
        };
        Some(syntax)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockState {
    pub in_block: bool,
    in_string: Option<u8>,
    escape: bool,
}

pub fn find_comment_ranges(
    line: &[u8],
    state: &mut BlockState,
    syntax: CommentSyntax,
    mut on_range: impl FnMut(usize, usize),
) {
    match syntax {
        CommentSyntax::Line(token) => {
            if let Some(idx) = find_line_comment_start(line, token) {
                on_range(idx, line.len());
            }
        }
        CommentSyntax::Hash => {
            if let Some(idx) = find_line_comment_start(line, b"#") {
                on_range(idx, line.len());
            }
        }
        CommentSyntax::CStyle { allow_backtick } => {
            find_cstyle_ranges(line, state, allow_backtick, &mut on_range);
        }
    }
}

fn find_cstyle_ranges(
    line: &[u8],
    state: &mut BlockState,
    allow_backtick: bool,
    on_range: &mut impl FnMut(usize, usize),
) {
    let mut cursor = 0;
    let len = line.len();
    while cursor < len {
        if state.in_block {
            if let Some(end) = find_subslice_from(line, b"*/", cursor) {
                on_range(cursor, end + 2);
                state.in_block = false;
                cursor = end + 2;
            } else {
                on_range(cursor, len);
                return;
            }
        }

        let mut idx = cursor;
        while idx < len {
            let byte = line[idx];
            if state.in_string.is_some() {
                if state.escape {
                    state.escape = false;
                } else if byte == b'\\' {
                    state.escape = true;
                } else if Some(byte) == state.in_string {
                    state.in_string = None;
                }
                idx += 1;
                continue;
            }

            if byte == b'"' || byte == b'\'' || (allow_backtick && byte == b'`') {
                state.in_string = Some(byte);
                idx += 1;
                continue;
            }

            if idx + 1 < len && line[idx] == b'/' && line[idx + 1] == b'/' {
                on_range(idx, len);
                return;
            }
            if idx + 1 < len && line[idx] == b'/' && line[idx + 1] == b'*' {
                if let Some(end) = find_subslice_from(line, b"*/", idx + 2) {
                    on_range(idx, end + 2);
                    idx = end + 2;
                    continue;
                }
                on_range(idx, len);
                state.in_block = true;
                return;
            }

            idx += 1;
        }
        if allow_backtick {
            if state.in_string != Some(b'`') {
                state.in_string = None;
                state.escape = false;
            }
        } else {
            state.in_string = None;
            state.escape = false;
        }
        return;
    }
}

fn find_line_comment_start(line: &[u8], token: &[u8]) -> Option<usize> {
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let len = line.len();
    let mut idx = 0;
    while idx < len {
        let byte = line[idx];
        if let Some(delim) = in_string {
            if escape {
                escape = false;
            } else if byte == b'\\' {
                escape = true;
            } else if byte == delim {
                in_string = None;
            }
            idx += 1;
            continue;
        }

        if byte == b'"' || byte == b'\'' {
            in_string = Some(byte);
            idx += 1;
            continue;
        }

        if idx + token.len() <= len && &line[idx..idx + token.len()] == token {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn find_subslice_from(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() || start >= haystack.len() {
        return None;
    }
    let end = haystack.len() - needle.len();
    let mut idx = start;
    while idx <= end {
        if haystack[idx] == needle[0]
            && haystack[idx..idx + needle.len()] == needle[..]
        {
            return Some(idx);
        }
        idx += 1;
    }
    None
}
