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
};

const C_STYLE_JS: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"//"),
    block_comment: Some((b"/*", b"*/")),
    strings: C_STYLE_JS_STRINGS,
};

const HASH_SIMPLE: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: HASH_STRINGS,
};

const HASH_PY: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: PY_STRINGS,
};

const HASH_TOML: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: TOML_STRINGS,
};

const HASH_SHELL: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"#"),
    block_comment: None,
    strings: SHELL_STRINGS,
};

const LUA: SyntaxSpec = SyntaxSpec {
    line_comment: Some(b"--"),
    block_comment: None,
    strings: HASH_STRINGS,
};

pub fn syntax_for_path(path: &Path) -> Option<&'static SyntaxSpec> {
    let ext = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_ascii_lowercase());
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase());
    if let Some(name) = name.as_deref() {
        if name == "makefile" || name == "dockerfile" {
            return Some(&HASH_SIMPLE);
        }
    }
    let ext = ext.as_deref()?;
    match ext {
        "rs" | "c" | "h" | "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" | "java" | "kt"
        | "kts" | "swift" | "go" | "cs" | "scala" | "dart" => Some(&C_STYLE),
        "js" | "jsx" | "ts" | "tsx" => Some(&C_STYLE_JS),
        "py" => Some(&HASH_PY),
        "sh" | "bash" | "zsh" => Some(&HASH_SHELL),
        "toml" => Some(&HASH_TOML),
        "rb" | "yml" | "yaml" | "ini" | "cfg" | "conf" | "env" => Some(&HASH_SIMPLE),
        "lua" => Some(&LUA),
        "mk" => Some(&HASH_SIMPLE),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockState {
    pub in_block: bool,
    in_string: Option<usize>,
    escape: bool,
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
