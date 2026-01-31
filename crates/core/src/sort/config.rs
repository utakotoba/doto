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

pub use crate::dimension::{
    DimensionStage, FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride,
    MarkSortConfig, Order, PathSortConfig,
};
