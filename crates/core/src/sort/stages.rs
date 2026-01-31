use std::collections::HashMap;
use std::path::PathBuf;

use crate::constants::{DEFAULT_MARK_PRIORITIES, normalize_mark};
use crate::model::{DimensionValue, Mark};
use crate::sort::config::{
    DimensionStage, FolderSortConfig, LanguageOrder, LanguageSortConfig, MarkPriorityOverride,
    MarkSortConfig, Order, PathSortConfig,
};
use crate::sort::group::Group;
use crate::utils::folder_key;

pub(crate) fn group_for_stage(
    stage: &DimensionStage,
    items: Vec<Mark>,
    roots: &[PathBuf],
) -> Vec<Group> {
    match stage {
        DimensionStage::Mark(config) => group_by_mark(items, config),
        DimensionStage::Language(config) => group_by_language(items, config),
        DimensionStage::Path(config) => group_by_path(items, config),
        DimensionStage::Folder(config) => group_by_folder(items, config, roots),
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
            key: DimensionValue::Mark(key.into()),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (DimensionValue::Mark(a_key), DimensionValue::Mark(b_key)) = (&a.key, &b.key) else {
            return std::cmp::Ordering::Equal;
        };
        let a_prio = mark_priority(a_key.as_ref(), &config.overrides).unwrap_or(u8::MAX);
        let b_prio = mark_priority(b_key.as_ref(), &config.overrides).unwrap_or(u8::MAX);
        a_prio
            .cmp(&b_prio)
            .then_with(|| a_key.as_ref().cmp(b_key.as_ref()))
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
            key: DimensionValue::Language(key.into()),
            items,
        })
        .collect::<Vec<_>>();

    match config.order {
        LanguageOrder::CountDescNameAsc => {
            groups.sort_by(|a, b| {
                let (DimensionValue::Language(a_key), DimensionValue::Language(b_key)) =
                    (&a.key, &b.key)
                else {
                    return std::cmp::Ordering::Equal;
                };
                b.items
                    .len()
                    .cmp(&a.items.len())
                    .then_with(|| a_key.as_ref().cmp(b_key.as_ref()))
            });
        }
        LanguageOrder::NameAsc => {
            groups.sort_by(|a, b| {
                let (DimensionValue::Language(a_key), DimensionValue::Language(b_key)) =
                    (&a.key, &b.key)
                else {
                    return std::cmp::Ordering::Equal;
                };
                a_key.as_ref().cmp(b_key.as_ref())
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
            key: DimensionValue::Path(key),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (DimensionValue::Path(a_key), DimensionValue::Path(b_key)) = (&a.key, &b.key) else {
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
        let key = folder_key(mark.path.as_ref(), roots, config);
        map.entry(key).or_default().push(mark);
    }
    let mut groups = map
        .into_iter()
        .map(|(key, items)| Group {
            key: DimensionValue::Folder(key),
            items,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| {
        let (DimensionValue::Folder(a_key), DimensionValue::Folder(b_key)) = (&a.key, &b.key)
        else {
            return std::cmp::Ordering::Equal;
        };
        match config.order {
            Order::Asc => a_key.cmp(b_key),
            Order::Desc => b_key.cmp(a_key),
        }
    });
    groups
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
