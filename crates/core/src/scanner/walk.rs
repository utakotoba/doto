use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;

use crate::constants::apply_builtin_excludes;
use crate::config::ScanConfig;
use crate::error::ScanError;

pub fn build_walk_builder(config: &ScanConfig, root: &Path) -> Result<WalkBuilder, ScanError> {
    let mut builder = WalkBuilder::new(root);

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

    if config.builtin_excludes() || !config.include().is_empty() || !config.exclude().is_empty() {
        let override_base = PathBuf::from(root);
        let mut overrides = OverrideBuilder::new(override_base);
        if config.builtin_excludes() {
            apply_builtin_excludes(&mut overrides)?;
        }
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
