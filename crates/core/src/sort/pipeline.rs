use std::path::PathBuf;

use crate::model::Mark;
use crate::sort::config::{DimensionStage, SortConfig};
use crate::sort::stages::group_for_stage;

pub fn apply_sort_pipeline(marks: Vec<Mark>, config: &SortConfig, roots: &[PathBuf]) -> Vec<Mark> {
    if marks.len() <= 1 || config.pipeline.is_empty() {
        return marks;
    }
    let mut output = Vec::with_capacity(marks.len());
    sort_recursive(&config.pipeline, roots, marks, &mut output);
    output
}

fn sort_recursive(
    stages: &[DimensionStage],
    roots: &[PathBuf],
    items: Vec<Mark>,
    output: &mut Vec<Mark>,
) {
    if stages.is_empty() {
        output.extend(items);
        return;
    }

    let stage = &stages[0];
    let mut groups = group_for_stage(stage, items, roots);

    for group in groups.drain(..) {
        sort_recursive(&stages[1..], roots, group.items, output);
    }
}
