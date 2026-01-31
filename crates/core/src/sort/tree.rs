use std::path::PathBuf;

use crate::model::{GroupNode, GroupTree, Mark};
use crate::sort::config::{DimensionStage, SortConfig};
use crate::sort::stages::group_for_stage;

pub fn build_group_tree(marks: Vec<Mark>, config: &SortConfig, roots: &[PathBuf]) -> GroupTree {
    if marks.is_empty() || config.pipeline.is_empty() {
        return GroupTree {
            groups: Vec::new(),
            items: marks,
        };
    }

    let groups = build_groups(&config.pipeline, roots, marks);
    GroupTree {
        groups,
        items: Vec::new(),
    }
}

fn build_groups(stages: &[DimensionStage], roots: &[PathBuf], items: Vec<Mark>) -> Vec<GroupNode> {
    if stages.is_empty() {
        return Vec::new();
    }

    let stage = &stages[0];
    let groups = group_for_stage(stage, items, roots);
    let mut out = Vec::with_capacity(groups.len());
    let next_stages = &stages[1..];

    for group in groups {
        let count = group.items.len();
        if next_stages.is_empty() {
            out.push(GroupNode {
                key: group.key,
                count,
                groups: Vec::new(),
                items: group.items,
            });
            continue;
        }

        let children = build_groups(next_stages, roots, group.items);
        out.push(GroupNode {
            key: group.key,
            count,
            groups: children,
            items: Vec::new(),
        });
    }

    out
}
