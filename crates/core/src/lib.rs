mod syntax;
mod config;
mod constants;
mod control;
mod utils;
mod error;
mod filter;
mod model;
mod scanner;
mod sort;

pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use model::{
    Dimension, DimensionValue, GroupNode, GroupTree, GroupedScanResult, Mark, ScanResult,
    ScanStats, ScanWarning,
};
pub use error::ScanError;
pub use filter::{FilterConfig, FilterRule, ValuePredicate};
pub use scanner::Scanner;
pub use sort::{
    DimensionStage, FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride,
    MarkSortConfig, Order, PathSortConfig, SortConfig,
};

pub fn scan(config: ScanConfig) -> Result<ScanResult, ScanError> {
    Scanner::new(config)?.scan()
}

pub fn scan_grouped(config: ScanConfig) -> Result<GroupedScanResult, ScanError> {
    Scanner::new(config)?.scan_grouped()
}
