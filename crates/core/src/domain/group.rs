use crate::domain::Mark;
use crate::dimension::DimensionValue;

#[derive(Clone, Debug)]
pub struct GroupTree {
    pub groups: Vec<GroupNode>,
    pub items: Vec<Mark>,
}

impl GroupTree {
    pub fn total(&self) -> usize {
        if self.groups.is_empty() {
            return self.items.len();
        }
        self.groups.iter().map(|group| group.count).sum()
    }
}

#[derive(Clone, Debug)]
pub struct GroupNode {
    pub key: DimensionValue,
    pub count: usize,
    pub groups: Vec<GroupNode>,
    pub items: Vec<Mark>,
}
