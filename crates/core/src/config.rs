use std::path::PathBuf;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum DetectionConfig {
    Regex { pattern: String },
}

#[derive(Clone, Debug)]
pub struct ScanConfig {
    roots: Vec<PathBuf>,
    detection: DetectionConfig,
    include: Vec<String>,
    exclude: Vec<String>,
    follow_gitignore: bool,
    include_hidden: bool,
    max_file_size: Option<u64>,
    threads: Option<usize>,
    read_buffer_size: usize,
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

    pub fn max_file_size(&self) -> Option<u64> {
        self.max_file_size
    }

    pub fn threads(&self) -> Option<usize> {
        self.threads
    }

    pub fn read_buffer_size(&self) -> usize {
        self.read_buffer_size
    }
}

#[derive(Clone, Debug)]
pub struct ScanConfigBuilder {
    roots: Vec<PathBuf>,
    detection: DetectionConfig,
    include: Vec<String>,
    exclude: Vec<String>,
    follow_gitignore: bool,
    include_hidden: bool,
    max_file_size: Option<u64>,
    threads: Option<usize>,
    read_buffer_size: usize,
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
            max_file_size: None,
            threads: None,
            read_buffer_size: 64 * 1024,
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

    pub fn build(self) -> ScanConfig {
        ScanConfig {
            roots: self.roots,
            detection: self.detection,
            include: self.include,
            exclude: self.exclude,
            follow_gitignore: self.follow_gitignore,
            include_hidden: self.include_hidden,
            max_file_size: self.max_file_size,
            threads: self.threads,
            read_buffer_size: self.read_buffer_size,
        }
    }
}

impl Default for ScanConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn default_pattern() -> &'static str {
    r"(?i)\b(?:TODO|FIXME|HACK|NOTE|BUG|XXX)\b"
}
