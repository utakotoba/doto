use std::path::PathBuf;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;

use crate::config::ScanConfig;
use crate::error::ScanError;

pub fn build_walk_builder(config: &ScanConfig) -> Result<WalkBuilder, ScanError> {
    let mut roots = config.roots().iter();
    let first = roots.next().ok_or(ScanError::EmptyRoots)?;
    let mut builder = WalkBuilder::new(first);
    for root in roots {
        builder.add(root);
    }

    if let Some(threads) = config.threads() {
        builder.threads(threads);
    }

    let follow_gitignore = config.follow_gitignore();
    builder
        .git_ignore(follow_gitignore)
        .git_exclude(follow_gitignore)
        .git_global(follow_gitignore)
        .ignore(follow_gitignore)
        .require_git(false)
        .hidden(!config.include_hidden());

    if !config.include().is_empty() || !config.exclude().is_empty() {
        let override_base = config
            .roots()
            .first()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let mut overrides = OverrideBuilder::new(override_base);
        for include in config.include() {
            overrides.add(include)?;
        }
        for exclude in config.exclude() {
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
