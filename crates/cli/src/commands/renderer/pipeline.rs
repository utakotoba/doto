use std::collections::HashMap;
use std::path::{Path, PathBuf};

use doto_core::{Mark, SortStage};


#[derive(Debug)]
pub(crate) struct GroupNode {
    pub(crate) key: GroupKey,
    pub(crate) count: usize,
    pub(crate) items: Vec<Mark>,
    pub(crate) children: Vec<GroupNode>,
}

impl GroupNode {
    pub(crate) fn label(&self, roots: &[PathBuf]) -> String {
        match &self.key {
            GroupKey::Mark(value) => format!("mark: {value}"),
            GroupKey::Language(value) => format!("language: {value}"),
            GroupKey::Path(value) => format!("path: {}", display_path(value, roots)),
            GroupKey::Folder(value) => format!("folder: {}", display_path(value, roots)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum GroupKey {
    Mark(String),
    Language(String),
    Path(PathBuf),
    Folder(PathBuf),
}

pub(crate) fn build_group_tree(
    marks: &[Mark],
    pipeline: &[SortStage],
    roots: &[PathBuf],
) -> Vec<GroupNode> {
    if pipeline.is_empty() {
        return Vec::new();
    }
    build_groups(marks, pipeline, roots)
}

fn build_groups(marks: &[Mark], pipeline: &[SortStage], roots: &[PathBuf]) -> Vec<GroupNode> {
    let stage = &pipeline[0];
    let mut buckets = group_by_stage(stage, marks, roots);

    if pipeline.len() == 1 {
        return buckets
            .drain(..)
            .map(|bucket| GroupNode {
                key: bucket.key,
                count: bucket.items.len(),
                items: bucket.items,
                children: Vec::new(),
            })
            .collect();
    }

    buckets
        .drain(..)
        .map(|bucket| {
            let children = build_groups(bucket.items.as_slice(), &pipeline[1..], roots);
            let count = children.iter().map(|child| child.count).sum();
            GroupNode {
                key: bucket.key,
                count,
                items: Vec::new(),
                children,
            }
        })
        .collect()
}

struct Bucket {
    key: GroupKey,
    items: Vec<Mark>,
}

fn group_by_stage(stage: &SortStage, marks: &[Mark], roots: &[PathBuf]) -> Vec<Bucket> {
    match stage {
        SortStage::Mark(_) => group_by_key(marks, |mark| GroupKey::Mark(mark.mark.to_string())),
        SortStage::Language(_) => {
            group_by_key(marks, |mark| GroupKey::Language(mark.language.to_string()))
        }
        SortStage::Path(_) => group_by_key(marks, |mark| GroupKey::Path((*mark.path).clone())),
        SortStage::Folder(config) => group_by_key(marks, |mark| {
            GroupKey::Folder(folder_key(mark.path.as_ref(), roots, config.depth))
        }),
    }
}

fn group_by_key(
    marks: &[Mark],
    mut key_fn: impl FnMut(&Mark) -> GroupKey,
) -> Vec<Bucket> {
    let mut buckets: Vec<Bucket> = Vec::new();
    let mut index: HashMap<GroupKeyKey, usize> = HashMap::new();

    for mark in marks {
        let key = key_fn(mark);
        let map_key = GroupKeyKey::from(&key);
        if let Some(&idx) = index.get(&map_key) {
            buckets[idx].items.push(mark.clone());
        } else {
            let idx = buckets.len();
            buckets.push(Bucket {
                key,
                items: vec![mark.clone()],
            });
            index.insert(map_key, idx);
        }
    }

    buckets
}

#[derive(Hash, Eq, PartialEq)]
enum GroupKeyKey {
    Mark(String),
    Language(String),
    Path(PathBuf),
    Folder(PathBuf),
}

impl From<&GroupKey> for GroupKeyKey {
    fn from(value: &GroupKey) -> Self {
        match value {
            GroupKey::Mark(value) => GroupKeyKey::Mark(value.clone()),
            GroupKey::Language(value) => GroupKeyKey::Language(value.clone()),
            GroupKey::Path(value) => GroupKeyKey::Path(value.clone()),
            GroupKey::Folder(value) => GroupKeyKey::Folder(value.clone()),
        }
    }
}

fn folder_key(path: &Path, roots: &[PathBuf], depth: usize) -> PathBuf {
    if depth == 0 {
        return PathBuf::new();
    }

    if let Some(root) = roots.iter().find(|root| path.starts_with(root)) {
        if let Ok(rel) = path.strip_prefix(root) {
            if let Some(parent) = rel.parent() {
                let prefix = prefix_components(parent, depth);
                return root.join(prefix);
            }
        }
    }

    if let Some(parent) = path.parent() {
        return prefix_components(parent, depth);
    }

    PathBuf::new()
}

fn prefix_components(path: &Path, depth: usize) -> PathBuf {
    let mut key = PathBuf::new();
    for component in path.components().take(depth) {
        key.push(component.as_os_str());
    }
    key
}

fn display_path(path: &Path, roots: &[PathBuf]) -> String {
    let mut best: Option<&PathBuf> = None;
    for root in roots {
        if path.starts_with(root) {
            match best {
                Some(current) if current.components().count() >= root.components().count() => {}
                _ => best = Some(root),
            }
        }
    }
    if let Some(root) = best {
        if let Ok(rel) = path.strip_prefix(root) {
            if rel.as_os_str().is_empty() {
                return ".".to_string();
            }
            return rel.display().to_string();
        }
    }
    path.display().to_string()
}
