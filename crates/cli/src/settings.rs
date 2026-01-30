use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use crate::cli::{ConfigFormat, ListArgs};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub roots: Vec<PathBuf>,
    pub regex: Option<String>,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub gitignore: Option<bool>,
    pub hidden: Option<bool>,
    pub max_file_size: Option<u64>,
    pub threads: Option<usize>,
    pub read_buffer_size: Option<usize>,
}

pub fn load_settings(
    config_path: Option<&PathBuf>,
    config_format: Option<ConfigFormat>,
) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder()
        .set_default("gitignore", true)?
        .set_default("hidden", false)?
        .set_default("read_buffer_size", 64 * 1024)?;

    if let Some(path) = config_path {
        builder = match config_format {
            Some(format) => builder.add_source(File::new(
                path.to_string_lossy().as_ref(),
                format.to_file_format(),
            )),
            None => builder.add_source(File::from(path.clone())),
        };
    }

    builder = builder.add_source(
        Environment::with_prefix("KODA")
            .separator("__")
            .try_parsing(true)
            .list_separator(",")
            .with_list_parse_key("roots")
            .with_list_parse_key("include")
            .with_list_parse_key("exclude"),
    );

    builder.build()?.try_deserialize()
}

pub fn apply_args(settings: &mut Settings, args: ListArgs) {
    if !args.roots.is_empty() {
        settings.roots = args.roots;
    }
    if let Some(regex) = args.regex {
        settings.regex = Some(regex);
    }
    if !args.include.is_empty() {
        settings.include = args.include;
    }
    if !args.exclude.is_empty() {
        settings.exclude = args.exclude;
    }
    if let Some(gitignore) = args.gitignore {
        settings.gitignore = Some(gitignore);
    }
    if let Some(hidden) = args.hidden {
        settings.hidden = Some(hidden);
    }
    if let Some(max_file_size) = args.max_file_size {
        settings.max_file_size = Some(max_file_size);
    }
    if let Some(threads) = args.threads {
        settings.threads = Some(threads);
    }
    if let Some(read_buffer_size) = args.read_buffer_size {
        settings.read_buffer_size = Some(read_buffer_size);
    }
}
