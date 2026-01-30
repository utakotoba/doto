use ignore::overrides::OverrideBuilder;

pub struct MarkPriority {
    pub mark: &'static str,
    pub priority: u8,
}

pub const DEFAULT_MARK_PRIORITIES: &[MarkPriority] = &[
    MarkPriority {
        mark: "ERROR",
        priority: 0,
    },
    MarkPriority {
        mark: "WARN",
        priority: 1,
    },
    MarkPriority {
        mark: "FIXME",
        priority: 2,
    },
    MarkPriority {
        mark: "TODO",
        priority: 3,
    },
    MarkPriority {
        mark: "NOTE",
        priority: 4,
    },
    MarkPriority {
        mark: "INFO",
        priority: 5,
    },
];

pub const DEFAULT_MARK_REGEX: &str = r"(?i)\b(?:ERROR|WARN|FIXME|TODO|NOTE|INFO)\b";

const DEFAULT_EXCLUDES: &[&str] = &[
    "node_modules/",
    "target/",
    "dist/",
    "build/",
    "out/",
    ".git/",
    ".hg/",
    ".svn/",
    ".idea/",
    ".vscode/",
    "vendor/",
    "Pods/",
    ".next/",
    ".nuxt/",
    ".svelte-kit/",
    "coverage/",
    ".cache/",
    ".tmp/",
    ".DS_Store",
];

pub fn apply_builtin_excludes(builder: &mut OverrideBuilder) -> Result<(), ignore::Error> {
    for pattern in DEFAULT_EXCLUDES {
        let glob = if pattern.starts_with('!') {
            pattern.to_string()
        } else {
            format!("!{pattern}")
        };
        builder.add(&glob)?;
    }
    Ok(())
}

pub fn normalize_mark(input: &str) -> Option<&'static str> {
    normalize_mark_bytes(input.as_bytes())
}

pub fn normalize_mark_bytes(input: &[u8]) -> Option<&'static str> {
    for entry in DEFAULT_MARK_PRIORITIES {
        let bytes = entry.mark.as_bytes();
        if bytes.len() != input.len() {
            continue;
        }
        let mut matches = true;
        for (left, right) in bytes.iter().zip(input) {
            if left.to_ascii_uppercase() != right.to_ascii_uppercase() {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(entry.mark);
        }
    }
    None
}
