use ignore::overrides::OverrideBuilder;

const BUILTIN_EXCLUDES: &[&str] = &[
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
    for pattern in BUILTIN_EXCLUDES {
        let glob = if pattern.starts_with('!') {
            pattern.to_string()
        } else {
            format!("!{pattern}")
        };
        builder.add(&glob)?;
    }
    Ok(())
}
