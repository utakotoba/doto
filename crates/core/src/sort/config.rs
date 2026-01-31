use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SortConfig {
    pub pipeline: Vec<DimensionStage>,
}

impl SortConfig {
    pub fn with_pipeline(pipeline: Vec<DimensionStage>) -> Self {
        Self { pipeline }
    }
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            pipeline: vec![
                DimensionStage::Mark(MarkSortConfig::default()),
                DimensionStage::Language(LanguageSortConfig::default()),
            ],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DimensionStage {
    Mark(MarkSortConfig),
    Language(LanguageSortConfig),
    Path(PathSortConfig),
    Folder(FolderSortConfig),
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
