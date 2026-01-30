use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::constants::{DEFAULT_MARK_PRIORITIES, normalize_mark};
use crate::model::Mark;
use crate::sort::config::{
    FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride, MarkSortConfig,
    Order, PathSortConfig, SortStage,
};

#[derive(Debug)]
pub(crate) struct Group {
    pub(crate) items: Vec<Mark>,
    key: GroupKey,
}

#[derive(Debug)]
enum GroupKey {
    Mark(&'static str),
    Language(&'static str),
    Path(PathBuf),
    Folder(PathBuf),
}

pub(crate) fn group_for_stage(
    stage: &SortStage,
    items: Vec<Mark>,
    roots: &[PathBuf],
) -> Vec<Group> {
    match stage {
        SortStage::Mark(config) => group_by_mark(items, config),
        SortStage::Language(config) => group_by_language(items, config),
        SortStage::Path(config) => group_by_path(items, config),
        SortStage::Folder(config) => group_by_folder(items, config, roots),
    }
}

fn group_by_mark(items: Vec<Mark>, config: &MarkSortConfig) -> Vec<Group> {
    let mut map: HashMap<&'static str, Vec<Mark>> = HashMap::new();
    for mark in items {
        let Some(kind) = normalize_mark(mark.mark) else {
            continue;
        };
        map.entry(kind).or_default().push(mark);
    }
    let mut groups = map
        .into_iter()
        .map(|(key, items)| Group {
            key: GroupKey::Mark(key),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (GroupKey::Mark(a_key), GroupKey::Mark(b_key)) = (&a.key, &b.key) else {
            return std::cmp::Ordering::Equal;
        };
        let a_prio = mark_priority(a_key, &config.overrides).unwrap_or(u8::MAX);
        let b_prio = mark_priority(b_key, &config.overrides).unwrap_or(u8::MAX);
        a_prio.cmp(&b_prio).then_with(|| a_key.cmp(b_key))
    });
    groups
}

fn group_by_language(items: Vec<Mark>, config: &LanguageSortConfig) -> Vec<Group> {
    let mut map: HashMap<&'static str, Vec<Mark>> = HashMap::new();
    for mark in items {
        map.entry(mark.language).or_default().push(mark);
    }
    let mut groups = map
        .into_iter()
        .map(|(key, items)| Group {
            key: GroupKey::Language(key),
            items,
        })
        .collect::<Vec<_>>();

    match config.order {
        LanguageOrder::CountDescNameAsc => {
            groups.sort_by(|a, b| {
                let (GroupKey::Language(a_key), GroupKey::Language(b_key)) = (&a.key, &b.key)
                else {
                    return std::cmp::Ordering::Equal;
                };
                b.items
                    .len()
                    .cmp(&a.items.len())
                    .then_with(|| a_key.cmp(b_key))
            });
        }
        LanguageOrder::NameAsc => {
            groups.sort_by(|a, b| {
                let (GroupKey::Language(a_key), GroupKey::Language(b_key)) = (&a.key, &b.key)
                else {
                    return std::cmp::Ordering::Equal;
                };
                a_key.cmp(b_key)
            });
        }
    }

    groups
}

fn group_by_path(items: Vec<Mark>, config: &PathSortConfig) -> Vec<Group> {
    let mut map: HashMap<PathBuf, Vec<Mark>> = HashMap::new();
    for mark in items {
        map.entry((*mark.path).clone()).or_default().push(mark);
    }
    let mut groups = map
        .into_iter()
        .map(|(key, items)| Group {
            key: GroupKey::Path(key),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (GroupKey::Path(a_key), GroupKey::Path(b_key)) = (&a.key, &b.key) else {
            return std::cmp::Ordering::Equal;
        };
        match config.order {
            Order::Asc => a_key.cmp(b_key),
            Order::Desc => b_key.cmp(a_key),
        }
    });
    groups
}

fn group_by_folder(items: Vec<Mark>, config: &FolderSortConfig, roots: &[PathBuf]) -> Vec<Group> {
    let mut map: HashMap<PathBuf, Vec<Mark>> = HashMap::new();
    for mark in items {
        let key = folder_key(mark.path.as_ref(), roots, config.depth);
        map.entry(key).or_default().push(mark);
    }
    let mut groups = map
        .into_iter()
        .map(|(key, items)| Group {
            key: GroupKey::Folder(key),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (GroupKey::Folder(a_key), GroupKey::Folder(b_key)) = (&a.key, &b.key) else {
            return std::cmp::Ordering::Equal;
        };
        match config.order {
            Order::Asc => a_key.cmp(b_key),
            Order::Desc => b_key.cmp(a_key),
        }
    });
    groups
}

fn folder_key(path: &Path, roots: &[PathBuf], depth: usize) -> PathBuf {
    if depth == 0 {
        return PathBuf::new();
    }

    if let Some(root) = roots.iter().find(|root| path.starts_with(root)) {
        if let Ok(rel) = path.strip_prefix(root) {
            if let Some(parent) = rel.parent() {
                return prefix_components(parent, depth);
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

fn mark_priority(mark: &str, overrides: &[MarkPriorityOverride]) -> Option<u8> {
    for override_entry in overrides {
        if override_entry.mark.eq_ignore_ascii_case(mark) {
            return Some(override_entry.priority);
        }
    }
    for entry in DEFAULT_MARK_PRIORITIES {
        if entry.mark == mark {
            return Some(entry.priority);
        }
    }
    None
}
