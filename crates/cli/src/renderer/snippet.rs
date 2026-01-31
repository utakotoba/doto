use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct SnippetCache {
    files: HashMap<PathBuf, Vec<String>>,
}

impl SnippetCache {
    pub(crate) fn line_for(&mut self, path: &Path, line: u32) -> Option<&str> {
        if line == 0 {
            return None;
        }
        let entry = self
            .files
            .entry(path.to_path_buf())
            .or_insert_with(|| read_lines(path));
        entry
            .get(line.saturating_sub(1) as usize)
            .map(String::as_str)
    }
}

fn read_lines(path: &Path) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };
    contents.lines().map(|line| line.to_string()).collect()
}
