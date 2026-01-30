mod control;
mod comments;
mod config;
mod error;
mod model;
mod scanner;

pub use control::{CancellationToken, ProgressReporter, SkipReason};
pub use config::{DetectionConfig, ScanConfig, ScanConfigBuilder};
pub use error::ScanError;
pub use model::{Mark, ScanResult, ScanStats, ScanWarning};
pub use scanner::Scanner;

pub fn scan(config: ScanConfig) -> Result<ScanResult, ScanError> {
    Scanner::new(config)?.scan()
}
