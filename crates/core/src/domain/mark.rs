use std::path::PathBuf;
use std::sync::Arc;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Mark {
    pub path: Arc<PathBuf>,
    pub line: u32,
    pub column: u32,
    pub mark: &'static str,
    pub language: &'static str,
}
