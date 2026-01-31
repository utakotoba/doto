use std::path::PathBuf;
use std::sync::Arc;

use crate::constants::DEFAULT_MARK_REGEX;
use crate::control::{CancellationToken, ProgressConfig, ProgressReporter};
use crate::filter::FilterConfig;
use crate::sort::{DimensionStage, SortConfig};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum DetectionConfig {
    Regex { pattern: String },
}

#[derive(Clone)]
pub struct ScanConfig {
    roots: Vec<PathBuf>,
    detection: DetectionConfig,
    include: Vec<String>,
    exclude: Vec<String>,
    follow_gitignore: bool,
    include_hidden: bool,
    builtin_excludes: bool,
    sort_config: SortConfig,
    filter_config: FilterConfig,
    max_file_size: Option<u64>,
    threads: Option<usize>,
    read_buffer_size: usize,
    progress: Option<ProgressConfig>,
    cancellation: Option<CancellationToken>,
}

impl ScanConfig {
    pub fn builder() -> ScanConfigBuilder {
        ScanConfigBuilder::new()
    }

    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn detection(&self) -> &DetectionConfig {
        &self.detection
    }

    pub fn include(&self) -> &[String] {
        &self.include
    }

    pub fn exclude(&self) -> &[String] {
        &self.exclude
    }

    pub fn follow_gitignore(&self) -> bool {
        self.follow_gitignore
    }

    pub fn include_hidden(&self) -> bool {
        self.include_hidden
    }

    pub fn builtin_excludes(&self) -> bool {
        self.builtin_excludes
    }

    pub fn sort_config(&self) -> &SortConfig {
        &self.sort_config
    }

    pub fn filter_config(&self) -> &FilterConfig {
        &self.filter_config
    }

    pub fn max_file_size(&self) -> Option<u64> {
        self.max_file_size
    }

    pub fn threads(&self) -> Option<usize> {
        self.threads
    }

    pub fn read_buffer_size(&self) -> usize {
        self.read_buffer_size
    }

    pub fn progress(&self) -> Option<&ProgressConfig> {
        self.progress.as_ref()
    }

    pub fn cancellation_token(&self) -> Option<&CancellationToken> {
        self.cancellation.as_ref()
    }
}

impl std::fmt::Debug for ScanConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScanConfig")
            .field("roots", &self.roots)
            .field("detection", &self.detection)
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("follow_gitignore", &self.follow_gitignore)
            .field("include_hidden", &self.include_hidden)
            .field("builtin_excludes", &self.builtin_excludes)
            .field("sort_config", &self.sort_config)
            .field("filter_config", &self.filter_config)
            .field("max_file_size", &self.max_file_size)
            .field("threads", &self.threads)
            .field("read_buffer_size", &self.read_buffer_size)
            .field("progress", &self.progress)
            .field("cancellation", &self.cancellation)
            .finish()
    }
}

#[derive(Clone)]
pub struct ScanConfigBuilder {
    roots: Vec<PathBuf>,
    detection: DetectionConfig,
    include: Vec<String>,
    exclude: Vec<String>,
    follow_gitignore: bool,
    include_hidden: bool,
    builtin_excludes: bool,
    sort_config: SortConfig,
    filter_config: FilterConfig,
    max_file_size: Option<u64>,
    threads: Option<usize>,
    read_buffer_size: usize,
    progress: Option<ProgressConfig>,
    cancellation: Option<CancellationToken>,
}

impl ScanConfigBuilder {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            detection: DetectionConfig::Regex {
                pattern: default_pattern().to_string(),
            },
            include: Vec::new(),
            exclude: Vec::new(),
            follow_gitignore: true,
            include_hidden: false,
            builtin_excludes: true,
            sort_config: SortConfig::default(),
            filter_config: FilterConfig::default(),
            max_file_size: None,
            threads: None,
            read_buffer_size: 64 * 1024,
            progress: None,
            cancellation: None,
        }
    }

    pub fn root(mut self, root: impl Into<PathBuf>) -> Self {
        self.roots.push(root.into());
        self
    }

    pub fn roots<I, P>(mut self, roots: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        self.roots.extend(roots.into_iter().map(Into::into));
        self
    }

    pub fn regex(mut self, pattern: impl Into<String>) -> Self {
        self.detection = DetectionConfig::Regex {
            pattern: pattern.into(),
        };
        self
    }

    pub fn include(mut self, pattern: impl Into<String>) -> Self {
        self.include.push(pattern.into());
        self
    }

    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        self.exclude.push(pattern.into());
        self
    }

    pub fn follow_gitignore(mut self, yes: bool) -> Self {
        self.follow_gitignore = yes;
        self
    }

    pub fn include_hidden(mut self, yes: bool) -> Self {
        self.include_hidden = yes;
        self
    }

    pub fn builtin_excludes(mut self, yes: bool) -> Self {
        self.builtin_excludes = yes;
        self
    }

    pub fn sort_config(mut self, sort_config: SortConfig) -> Self {
        self.sort_config = sort_config;
        self
    }

    pub fn sort_pipeline(mut self, pipeline: Vec<DimensionStage>) -> Self {
        self.sort_config = SortConfig::with_pipeline(pipeline);
        self
    }

    pub fn filter_config(mut self, filter_config: FilterConfig) -> Self {
        self.filter_config = filter_config;
        self
    }

    pub fn max_file_size(mut self, max_file_size: Option<u64>) -> Self {
        self.max_file_size = max_file_size;
        self
    }

    pub fn threads(mut self, threads: Option<usize>) -> Self {
        self.threads = threads;
        self
    }

    pub fn read_buffer_size(mut self, bytes: usize) -> Self {
        self.read_buffer_size = bytes.max(8 * 1024);
        self
    }

    pub fn progress_reporter<R>(mut self, reporter: R) -> Self
    where
        R: ProgressReporter + 'static,
    {
        self.progress = Some(ProgressConfig::new(Arc::new(reporter)));
        self
    }

    pub fn progress_reporter_arc(mut self, reporter: Arc<dyn ProgressReporter>) -> Self {
        self.progress = Some(ProgressConfig::new(reporter));
        self
    }

    pub fn cancellation_token(mut self, token: CancellationToken) -> Self {
        self.cancellation = Some(token);
        self
    }

    pub fn build(self) -> ScanConfig {
        ScanConfig {
            roots: self.roots,
            detection: self.detection,
            include: self.include,
            exclude: self.exclude,
            follow_gitignore: self.follow_gitignore,
            include_hidden: self.include_hidden,
            builtin_excludes: self.builtin_excludes,
            sort_config: self.sort_config,
            filter_config: self.filter_config,
            max_file_size: self.max_file_size,
            threads: self.threads,
            read_buffer_size: self.read_buffer_size,
            progress: self.progress,
            cancellation: self.cancellation,
        }
    }
}

impl std::fmt::Debug for ScanConfigBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScanConfigBuilder")
            .field("roots", &self.roots)
            .field("detection", &self.detection)
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("follow_gitignore", &self.follow_gitignore)
            .field("include_hidden", &self.include_hidden)
            .field("builtin_excludes", &self.builtin_excludes)
            .field("sort_config", &self.sort_config)
            .field("filter_config", &self.filter_config)
            .field("max_file_size", &self.max_file_size)
            .field("threads", &self.threads)
            .field("read_buffer_size", &self.read_buffer_size)
            .field("progress", &self.progress)
            .field("cancellation", &self.cancellation)
            .finish()
    }
}

impl Default for ScanConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn default_pattern() -> &'static str {
    DEFAULT_MARK_REGEX
}
