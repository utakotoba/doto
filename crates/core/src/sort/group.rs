use crate::dimension::DimensionValue;
use crate::domain::Mark;

#[derive(Debug)]
pub(crate) struct Group {
    pub(crate) items: Vec<Mark>,
    pub(crate) key: DimensionValue,
}
