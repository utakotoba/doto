use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct StringDelim {
    pub token: &'static [u8],
    pub multiline: bool,
    pub escape: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct SyntaxSpec {
    pub line_comment: Option<&'static [u8]>,
    pub block_comment: Option<(&'static [u8], &'static [u8])>,
    pub strings: &'static [StringDelim],
    pub raw_string: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct SyntaxInfo {
    pub spec: &'static SyntaxSpec,
    pub language: &'static str,
}

const C_STYLE_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: true,
    },
];

const C_STYLE_JS_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"`",
        multiline: true,
        escape: true,
    },
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: true,
    },
];

const HASH_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: true,
    },
];

const PY_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"\"\"\"",
        multiline: true,
        escape: true,
    },
    StringDelim {
        token: b"'''",
        multiline: true,
        escape: true,
    },
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: true,
    },
];

const TOML_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"\"\"\"",
        multiline: true,
        escape: true,
    },
    StringDelim {
        token: b"'''",
        multiline: true,
        escape: false,
    },
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: false,
    },
];

const SHELL_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"\"",
        multiline: true,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: true,
        escape: false,
    },
];

const C_STYLE: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"//"),
    block_comment: Some((b"/*", b"*/")),
    strings: C_STYLE_STRINGS,
    raw_string: false,
};

const C_STYLE_JS: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"//"),
    block_comment: Some((b"/*", b"*/")),
    strings: C_STYLE_JS_STRINGS,
    raw_string: false,
};

const HASH_SIMPLE: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: HASH_STRINGS,
    raw_string: false,
};

const HASH_PY: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: PY_STRINGS,
    raw_string: false,
};

const HASH_TOML: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: TOML_STRINGS,
    raw_string: false,
};

const HASH_SHELL: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: SHELL_STRINGS,
    raw_string: false,
};

const LUA: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"--"),
    block_comment: None,
    strings: HASH_STRINGS,
    raw_string: false,
};

const GO_STRINGS: &[StringDelim] = &[
    StringDelim {
        token: b"`",
        multiline: true,
        escape: false,
    },
    StringDelim {
        token: b"\"",
        multiline: false,
        escape: true,
    },
    StringDelim {
        token: b"'",
        multiline: false,
        escape: true,
    },
];

const GO: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"//"),
    block_comment: Some((b"/*", b"*/")),
    strings: GO_STRINGS,
    raw_string: false,
};

const RUST: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"//"),
    block_comment: Some((b"/*", b"*/")),
    strings: C_STYLE_STRINGS,
    raw_string: true,
};

pub fn syntax_for_path(path: &Path) -> Option<SyntaxInfo> {
    let ext = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_ascii_lowercase());
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase());
    if let Some(name) = name.as_deref() {
        if name == "makefile" {
            return Some(SyntaxInfo {
                spec: &HASH_SIMPLE,
                language: "makefile",
            });
        }
        if name == "dockerfile" {
            return Some(SyntaxInfo {
                spec: &HASH_SIMPLE,
                language: "dockerfile",
            });
        }
    }
    let ext = ext.as_deref()?;
    match ext {
        "rs" => Some(SyntaxInfo {
            spec: &RUST,
            language: "rs",
        }),
        "c" | "h" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "c",
        }),
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "cpp",
        }),
        "java" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "java",
        }),
        "kt" | "kts" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "kotlin",
        }),
        "swift" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "swift",
        }),
        "go" => Some(SyntaxInfo {
            spec: &GO,
            language: "go",
        }),
        "cs" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "cs",
        }),
        "scala" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "scala",
        }),
        "dart" => Some(SyntaxInfo {
            spec: &C_STYLE,
            language: "dart",
        }),
        "js" | "jsx" => Some(SyntaxInfo {
            spec: &C_STYLE_JS,
            language: "js",
        }),
        "ts" | "tsx" => Some(SyntaxInfo {
            spec: &C_STYLE_JS,
            language: "ts",
        }),
        "py" => Some(SyntaxInfo {
            spec: &HASH_PY,
            language: "py",
        }),
        "sh" | "bash" | "zsh" => Some(SyntaxInfo {
            spec: &HASH_SHELL,
            language: "sh",
        }),
        "toml" => Some(SyntaxInfo {
            spec: &HASH_TOML,
            language: "toml",
        }),
        "rb" => Some(SyntaxInfo {
            spec: &HASH_SIMPLE,
            language: "rb",
        }),
        "yml" | "yaml" => Some(SyntaxInfo {
            spec: &HASH_SIMPLE,
            language: "yaml",
        }),
        "ini" | "cfg" | "conf" | "env" => Some(SyntaxInfo {
            spec: &HASH_SIMPLE,
            language: "ini",
        }),
        "lua" => Some(SyntaxInfo {
            spec: &LUA,
            language: "lua",
        }),
        "mk" => Some(SyntaxInfo {
            spec: &HASH_SIMPLE,
            language: "make",
        }),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockState {
    pub in_block: bool,
    in_string: Option<usize>,
    escape: bool,
    raw_string_hash: Option<usize>,
}

pub fn find_comment_ranges(
    line: &[u8],
    state: &mut BlockState,
    spec: &SyntaxSpec,
    mut on_range: impl FnMut(usize, usize),
) {
    find_ranges(line, state, spec, &mut on_range);
}

fn find_ranges(
    line: &[u8],
    state: &mut BlockState,
    spec: &SyntaxSpec,
    on_range: &mut impl FnMut(usize, usize),
) {
    let len = line.len();
    let mut cursor = 0;

    if state.in_block {
        let Some((_, end_token)) = spec.block_comment else {
            state.in_block = false;
            state.in_string = None;
            state.escape = false;
            state.raw_string_hash = None;
            return;
        };
        if let Some(end) = find_subslice_from(line, end_token, cursor) {
            on_range(cursor, end + end_token.len());
            state.in_block = false;
            cursor = end + end_token.len();
        } else {
            on_range(cursor, len);
            return;
        }
    }

    let mut idx = cursor;
    while idx < len {
        if let Some(active) = state.in_string {
            let delim = &spec.strings[active];
            if delim.escape && state.escape {
                state.escape = false;
                idx += 1;
                continue;
            }
            if delim.escape && line[idx] == b'\\' {
                state.escape = true;
                idx += 1;
                continue;
            }
            if starts_with(line, delim.token, idx) {
                state.in_string = None;
                state.escape = false;
                idx += delim.token.len();
                continue;
            }
            idx += 1;
            continue;
        }

        if let Some(hash_count) = state.raw_string_hash {
            if let Some(end_idx) = find_raw_string_end(line, idx, hash_count) {
                idx = end_idx;
                state.raw_string_hash = None;
                continue;
            }
            return;
        }

        if spec.raw_string {
            if let Some((hash_count, consumed)) = parse_raw_string_start(line, idx) {
                state.raw_string_hash = Some(hash_count);
                idx += consumed;
                continue;
            }
        }

        if let Some(string_idx) = find_string_start(line, spec.strings, idx) {
            let delim = &spec.strings[string_idx];
            state.in_string = Some(string_idx);
            state.escape = false;
            idx += delim.token.len();
            continue;
        }

        if let Some(token) = spec.line_comment {
            if starts_with(line, token, idx) {
                on_range(idx, len);
                return;
            }
        }

        if let Some((start, end)) = spec.block_comment {
            if starts_with(line, start, idx) {
                if let Some(end_idx) = find_subslice_from(line, end, idx + start.len()) {
                    on_range(idx, end_idx + end.len());
                    idx = end_idx + end.len();
                    continue;
                }
                on_range(idx, len);
                state.in_block = true;
                return;
            }
        }

        idx += 1;
    }

    if let Some(active) = state.in_string {
        if !spec.strings[active].multiline {
            state.in_string = None;
            state.escape = false;
        }
    }
}

fn find_string_start(line: &[u8], strings: &[StringDelim], idx: usize) -> Option<usize> {
    for (i, delim) in strings.iter().enumerate() {
        if starts_with(line, delim.token, idx) {
            return Some(i);
        }
    }
    None
}

fn starts_with(haystack: &[u8], needle: &[u8], idx: usize) -> bool {
    idx + needle.len() <= haystack.len() && &haystack[idx..idx + needle.len()] == needle
}

fn find_subslice_from(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() || start >= haystack.len() {
        return None;
    }
    let end = haystack.len() - needle.len();
    let mut idx = start;
    while idx <= end {
        if haystack[idx] == needle[0] && haystack[idx..idx + needle.len()] == needle[..] {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn parse_raw_string_start(line: &[u8], idx: usize) -> Option<(usize, usize)> {
    if idx >= line.len() || line[idx] != b'r' {
        return None;
    }
    if idx > 0 && line[idx - 1].is_ascii_alphanumeric() {
        return None;
    }
    let mut pos = idx + 1;
    let mut hashes = 0usize;
    while pos < line.len() && line[pos] == b'#' {
        hashes += 1;
        pos += 1;
    }
    if pos < line.len() && line[pos] == b'"' {
        return Some((hashes, pos - idx + 1));
    }
    None
}

fn find_raw_string_end(line: &[u8], start: usize, hashes: usize) -> Option<usize> {
    if start >= line.len() {
        return None;
    }
    let mut idx = start;
    while idx < line.len() {
        if line[idx] == b'"' {
            let mut ok = true;
            for offset in 0..hashes {
                let pos = idx + 1 + offset;
                if pos >= line.len() || line[pos] != b'#' {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Some(idx + 1 + hashes);
            }
        }
        idx += 1;
    }
    None
}
