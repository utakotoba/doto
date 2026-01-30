use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("no scan roots were provided")]
    EmptyRoots,
    #[error("invalid regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("invalid include/exclude pattern: {0}")]
    Overrides(#[from] ignore::Error),
}
