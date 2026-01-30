use std::path::PathBuf;

use std::error::Error;

use config::{Config as ConfigSource, ConfigError, Environment, File};
use serde::Deserialize;

use crate::cli::{ConfigFormat, ListArgs};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
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

pub fn load_config(
    config_path: Option<&PathBuf>,
    config_format: Option<ConfigFormat>,
) -> Result<Config, ConfigError> {
    let mut builder = ConfigSource::builder()
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

pub fn load_config_with_context(
    no_dotenv: bool,
    dotenv: Option<&PathBuf>,
    config_path: Option<&PathBuf>,
    config_format: Option<ConfigFormat>,
) -> Result<Config, Box<dyn Error>> {
    load_dotenv(no_dotenv, dotenv)?;
    let config = load_config(config_path, config_format)?;
    Ok(config)
}

pub fn apply_args(config: &mut Config, args: ListArgs) {
    if !args.roots.is_empty() {
        config.roots = args.roots;
    }
    if let Some(regex) = args.regex {
        config.regex = Some(regex);
    }
    if !args.include.is_empty() {
        config.include = args.include;
    }
    if !args.exclude.is_empty() {
        config.exclude = args.exclude;
    }
    if let Some(gitignore) = args.gitignore {
        config.gitignore = Some(gitignore);
    }
    if let Some(hidden) = args.hidden {
        config.hidden = Some(hidden);
    }
    if let Some(max_file_size) = args.max_file_size {
        config.max_file_size = Some(max_file_size);
    }
    if let Some(threads) = args.threads {
        config.threads = Some(threads);
    }
    if let Some(read_buffer_size) = args.read_buffer_size {
        config.read_buffer_size = Some(read_buffer_size);
    }
}

fn load_dotenv(no_dotenv: bool, dotenv: Option<&PathBuf>) -> Result<(), Box<dyn Error>> {
    if no_dotenv {
        return Ok(());
    }
    if let Some(path) = dotenv {
        dotenvy::from_path(path)?;
    } else {
        let _ = dotenvy::dotenv();
    }
    Ok(())
}
