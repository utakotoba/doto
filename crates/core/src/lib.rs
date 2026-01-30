mod comments;
mod config;
mod constants;
mod control;
mod error;
mod model;
mod scanner;
mod sort;

pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use error::ScanError;
pub use model::{Mark, ScanResult, ScanStats, ScanWarning};
pub use scanner::Scanner;
pub use sort::{
    FilenameSortConfig, FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride,
    MarkSortConfig, Order, PathSortConfig, SortConfig, SortStage,
};

pub fn scan(config: ScanConfig) -> Result<ScanResult, ScanError> {
    Scanner::new(config)?.scan()
}
