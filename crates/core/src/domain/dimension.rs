use std::borrow::Cow;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Mark,
    Language,
    Path,
    Folder,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DimensionValue {
    Mark(Cow<'static, str>),
    Language(Cow<'static, str>),
    Path(PathBuf),
    Folder(PathBuf),
}
