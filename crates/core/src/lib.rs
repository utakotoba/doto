mod syntax;
mod config;
mod constants;
mod control;
mod dimension;
mod domain;
mod error;
mod filter;
mod scanner;
mod sort;

pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use dimension::{
    Dimension, DimensionStage, DimensionValue, FolderSortConfig, LanguageOrder, LanguageSortConfig,
    MarkPriorityOverride, MarkSortConfig, Order, PathSortConfig,
};
pub use domain::{GroupNode, GroupTree, GroupedScanResult, Mark, ScanResult, ScanStats, ScanWarning};
pub use error::ScanError;
pub use filter::{FilterConfig, FilterRule, ValuePredicate};
pub use scanner::Scanner;
pub use sort::{
    SortConfig,
};

pub fn scan(config: ScanConfig) -> Result<ScanResult, ScanError> {
    Scanner::new(config)?.scan()
}

pub fn scan_grouped(config: ScanConfig) -> Result<GroupedScanResult, ScanError> {
    Scanner::new(config)?.scan_grouped()
}
