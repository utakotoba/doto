use std::path::PathBuf;

use std::error::Error;

use config::{Config as ConfigSource, ConfigError, Environment, File};
use serde::Deserialize;

use doto_core::{
    FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride, MarkSortConfig,
    Order, PathSortConfig, SortConfig, SortStage,
};

use crate::cli::{Cli, ConfigFormat, SortLangOrderArg, SortOrderArg};

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
    pub sort: Option<SortConfig>,
    pub file_header: bool,
}

pub fn load_config(
    config_path: Option<&PathBuf>,
    config_format: Option<ConfigFormat>,
) -> Result<Config, ConfigError> {
    let mut builder = ConfigSource::builder()
        .set_default("gitignore", true)?
        .set_default("hidden", false)?
        .set_default("read_buffer_size", 64 * 1024)?
        .set_default("file_header", true)?;

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

pub fn apply_args(config: &mut Config, args: &Cli) {
    if !args.roots.is_empty() {
        config.roots = args.roots.clone();
    }
    if let Some(regex) = &args.regex {
        config.regex = Some(regex.clone());
    }
    if !args.include.is_empty() {
        config.include = args.include.clone();
    }
    if !args.exclude.is_empty() {
        config.exclude = args.exclude.clone();
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
    if args.no_file_header {
        config.file_header = false;
    }
}

pub fn resolve_sort_config(
    base: Option<SortConfig>,
    args: &Cli,
) -> Result<(Option<SortConfig>, Vec<String>), Box<dyn Error>> {
    let mut warnings = Vec::new();
    let mut config = if let Some(pipeline_raw) = &args.sort {
        let pipeline = parse_pipeline(pipeline_raw)?;
        SortConfig::with_pipeline(pipeline)
    } else if let Some(base) = base {
        base
    } else if has_sort_options(args) {
        SortConfig::default()
    } else {
        return Ok((None, warnings));
    };

    if let Some(raw) = &args.sort_mark_priority {
        let overrides = parse_mark_overrides(raw)?;
        let applied = apply_mark_overrides(&mut config.pipeline, overrides);
        if applied == 0 {
            warnings.push("sort-mark-priority provided but pipeline has no mark stage".to_string());
        }
    }

    if let Some(order) = args.sort_lang_order {
        let applied = apply_language_order(&mut config.pipeline, order);
        if applied == 0 {
            warnings.push("sort-lang-order provided but pipeline has no language stage".to_string());
        }
    }

    if let Some(order) = args.sort_path_order {
        let applied = apply_path_order(&mut config.pipeline, order);
        if applied == 0 {
            warnings.push("sort-path-order provided but pipeline has no path stage".to_string());
        }
    }

    if args.sort_folder_depth.is_some() || args.sort_folder_order.is_some() {
        let applied = apply_folder_options(
            &mut config.pipeline,
            args.sort_folder_depth,
            args.sort_folder_order,
        );
        if applied == 0 {
            warnings
                .push("sort-folder-* provided but pipeline has no folder stage".to_string());
        }
    }

    Ok((Some(config), warnings))
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

fn has_sort_options(args: &Cli) -> bool {
    args.sort_mark_priority.is_some()
        || args.sort_lang_order.is_some()
        || args.sort_path_order.is_some()
        || args.sort_folder_depth.is_some()
        || args.sort_folder_order.is_some()
}

fn parse_pipeline(raw: &str) -> Result<Vec<SortStage>, Box<dyn Error>> {
    let mut pipeline = Vec::new();
    for token in raw.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let stage = match token {
            "mark" => SortStage::Mark(MarkSortConfig::default()),
            "language" => SortStage::Language(LanguageSortConfig::default()),
            "path" => SortStage::Path(PathSortConfig::default()),
            "folder" => SortStage::Folder(FolderSortConfig::default()),
            _ => {
                return Err(format!("unknown sort stage '{token}'").into());
            }
        };
        pipeline.push(stage);
    }
    if pipeline.is_empty() {
        return Err("sort pipeline is empty".into());
    }
    Ok(pipeline)
}

fn parse_mark_overrides(raw: &str) -> Result<Vec<MarkPriorityOverride>, Box<dyn Error>> {
    let mut overrides = Vec::new();
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(overrides);
    }
    for pair in trimmed.split(',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }
        let Some((mark, prio)) = pair.split_once('=') else {
            return Err(format!("invalid mark priority '{pair}' (expected NAME=NUM)").into());
        };
        let mark = mark.trim();
        let prio = prio.trim();
        if mark.is_empty() {
            return Err(format!("invalid mark priority '{pair}' (empty mark)").into());
        }
        let priority: u8 = prio
            .parse()
            .map_err(|_| format!("invalid priority '{prio}' in '{pair}'"))?;
        overrides.push(MarkPriorityOverride {
            mark: mark.to_string(),
            priority,
        });
    }
    Ok(overrides)
}

fn apply_mark_overrides(pipeline: &mut [SortStage], overrides: Vec<MarkPriorityOverride>) -> usize {
    let mut applied = 0;
    for stage in pipeline {
        if let SortStage::Mark(config) = stage {
            config.overrides = overrides.clone();
            applied += 1;
        }
    }
    applied
}

fn apply_language_order(pipeline: &mut [SortStage], order: SortLangOrderArg) -> usize {
    let mut applied = 0;
    for stage in pipeline {
        if let SortStage::Language(config) = stage {
            config.order = match order {
                SortLangOrderArg::Count => LanguageOrder::CountDescNameAsc,
                SortLangOrderArg::Name => LanguageOrder::NameAsc,
            };
            applied += 1;
        }
    }
    applied
}

fn apply_path_order(pipeline: &mut [SortStage], order: SortOrderArg) -> usize {
    let mut applied = 0;
    for stage in pipeline {
        if let SortStage::Path(config) = stage {
            config.order = map_order(order);
            applied += 1;
        }
    }
    applied
}

fn apply_folder_options(
    pipeline: &mut [SortStage],
    depth: Option<usize>,
    order: Option<SortOrderArg>,
) -> usize {
    let mut applied = 0;
    for stage in pipeline {
        if let SortStage::Folder(config) = stage {
            if let Some(depth) = depth {
                config.depth = depth;
            }
            if let Some(order) = order {
                config.order = map_order(order);
            }
            applied += 1;
        }
    }
    applied
}

fn map_order(order: SortOrderArg) -> Order {
    match order {
        SortOrderArg::Asc => Order::Asc,
        SortOrderArg::Desc => Order::Desc,
    }
}
