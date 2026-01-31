//! Core scanning engine for tracking TODO/FIXME style marks in a workspace.
//!
//! The API is intentionally small: build a `ScanConfig`, then call `scan` or
//! `scan_grouped`. Results include counters for matches, skips, and issues.
//!
//! ## Basic scan
//! ```no_run
//! use doto_core::{scan, ScanConfig};
//!
//! let config = ScanConfig::builder()
//!     .root(".")
//!     .build();
//!
//! let result = scan(config)?;
//! println!("matches: {}", result.stats.matches);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Customize detection and filters
//! ```no_run
//! use doto_core::{
//!     scan, DimensionStage, DimensionValue, FilterConfig, FilterRule, ScanConfig, SortConfig,
//!     ValuePredicate,
//! };
//!
//! let filter = FilterConfig {
//!     rules: vec![
//!         FilterRule {
//!             stage: DimensionStage::Language(Default::default()),
//!             predicate: ValuePredicate::Allow {
//!                 values: vec![DimensionValue::Language("rs".into())],
//!             },
//!         }
//!     ],
//! };
//!
//! let config = ScanConfig::builder()
//!     .root(".")
//!     .regex(r"\b(?:TODO|FIXME)\b")
//!     .filter_config(filter)
//!     .sort_config(SortConfig::default())
//!     .build();
//!
//! let result = scan(config)?;
//! println!("files scanned: {}", result.stats.files_scanned);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
mod config;
mod constants;
mod control;
mod error;
mod filter;
mod model;
mod scanner;
mod sort;
mod syntax;
mod utils;

pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use error::ScanError;
pub use filter::{FilterConfig, FilterRule, ValuePredicate};
pub use model::{
    Dimension, DimensionValue, GroupNode, GroupTree, GroupedScanResult, Mark, ScanIssueCounts,
    ScanResult, ScanSkipCounts, ScanStats,
};
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
