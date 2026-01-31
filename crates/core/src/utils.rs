use std::path::PathBuf;

use crate::constants::normalize_mark;
use crate::model::{DimensionValue, Mark};
use crate::sort::{DimensionStage, FolderSortConfig};

pub(crate) fn extract_dimension_value(
    stage: &DimensionStage,
    mark: &Mark,
    roots: &[PathBuf],
) -> Option<DimensionValue> {
    match stage {
        DimensionStage::Mark(_) => {
            normalize_mark(mark.mark).map(|kind| DimensionValue::Mark(kind.into()))
        }
        DimensionStage::Language(_) => Some(DimensionValue::Language(mark.language.into())),
        DimensionStage::Path(_) => Some(DimensionValue::Path((*mark.path).clone())),
        DimensionStage::Folder(config) => {
            let key = folder_key(mark.path.as_ref(), roots, config);
            Some(DimensionValue::Folder(key))
        }
    }
}

pub(crate) fn folder_key(path: &std::path::Path, roots: &[PathBuf], config: &FolderSortConfig) -> PathBuf {
    let depth = config.depth;
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

fn prefix_components(path: &std::path::Path, depth: usize) -> PathBuf {
    let mut key = PathBuf::new();
    for component in path.components().take(depth) {
        key.push(component.as_os_str());
    }
    key
}
