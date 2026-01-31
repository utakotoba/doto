use std::borrow::Cow;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::constants::normalize_mark;
use crate::domain::Mark;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Mark,
    Language,
    Path,
    Folder,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DimensionValue {
    Mark(Cow<'static, str>),
    Language(Cow<'static, str>),
    Path(PathBuf),
    Folder(PathBuf),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DimensionStage {
    Mark(MarkSortConfig),
    Language(LanguageSortConfig),
    Path(PathSortConfig),
    Folder(FolderSortConfig),
}

impl DimensionStage {
    pub fn dimension(&self) -> Dimension {
        match self {
            DimensionStage::Mark(_) => Dimension::Mark,
            DimensionStage::Language(_) => Dimension::Language,
            DimensionStage::Path(_) => Dimension::Path,
            DimensionStage::Folder(_) => Dimension::Folder,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkSortConfig {
    pub overrides: Vec<MarkPriorityOverride>,
}

impl Default for MarkSortConfig {
    fn default() -> Self {
        Self {
            overrides: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageSortConfig {
    pub order: LanguageOrder,
}

impl Default for LanguageSortConfig {
    fn default() -> Self {
        Self {
            order: LanguageOrder::CountDescNameAsc,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageOrder {
    CountDescNameAsc,
    NameAsc,
}

impl Default for LanguageOrder {
    fn default() -> Self {
        Self::CountDescNameAsc
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PathSortConfig {
    pub order: Order,
}

impl Default for PathSortConfig {
    fn default() -> Self {
        Self { order: Order::Asc }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FolderSortConfig {
    pub depth: usize,
    pub order: Order,
}

impl Default for FolderSortConfig {
    fn default() -> Self {
        Self {
            depth: 1,
            order: Order::Asc,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkPriorityOverride {
    pub mark: String,
    pub priority: u8,
}

pub(crate) fn extract_dimension_value(
    stage: &DimensionStage,
    mark: &Mark,
    roots: &[PathBuf],
) -> Option<DimensionValue> {
    match stage {
        DimensionStage::Mark(_) => {
            normalize_mark(mark.mark).map(|kind| DimensionValue::Mark(kind.into()))
        }
        DimensionStage::Language(_) => Some(DimensionValue::Language(mark.language.into())),
        DimensionStage::Path(_) => Some(DimensionValue::Path((*mark.path).clone())),
        DimensionStage::Folder(config) => {
            let key = folder_key(mark.path.as_ref(), roots, config.depth);
            Some(DimensionValue::Folder(key))
        }
    }
}

pub(crate) fn folder_key(path: &std::path::Path, roots: &[PathBuf], depth: usize) -> PathBuf {
    if depth == 0 {
        return PathBuf::new();
    }

    if let Some(root) = roots.iter().find(|root| path.starts_with(root)) {
        if let Ok(rel) = path.strip_prefix(root) {
            if let Some(parent) = rel.parent() {
                return prefix_components(parent, depth);
            }
        }
    }

    if let Some(parent) = path.parent() {
        return prefix_components(parent, depth);
    }

    PathBuf::new()
}

fn prefix_components(path: &std::path::Path, depth: usize) -> PathBuf {
    let mut key = PathBuf::new();
    for component in path.components().take(depth) {
        key.push(component.as_os_str());
    }
    key
}
