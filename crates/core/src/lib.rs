mod comments;
mod config;
mod control;
mod error;
mod model;
mod builtin;
mod scanner;

pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use error::ScanError;
pub use model::{Mark, ScanResult, ScanStats, ScanWarning};
pub use scanner::Scanner;

pub fn scan(config: ScanConfig) -> Result<ScanResult, ScanError> {
    Scanner::new(config)?.scan()
}
