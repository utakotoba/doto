use crate::domain::{DimensionValue, Mark};

#[derive(Debug)]
pub(crate) struct Group {
    pub(crate) items: Vec<Mark>,
    pub(crate) key: DimensionValue,
}
