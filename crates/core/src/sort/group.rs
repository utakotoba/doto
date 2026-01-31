use std::path::PathBuf;

use crate::model::Mark;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GroupKey {
    Mark(&'static str),
    Language(&'static str),
    Path(PathBuf),
    Folder(PathBuf),
}

#[derive(Debug)]
pub(crate) struct Group {
    pub(crate) items: Vec<Mark>,
    pub(crate) key: GroupKey,
}
