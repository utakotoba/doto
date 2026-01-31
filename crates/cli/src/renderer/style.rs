use colored::{ColoredString, Colorize};

use doto_core::DimensionValue;

#[derive(Clone, Copy, Debug)]
pub(crate) enum GroupStyle {
    Mark,
    Language,
    Path,
    Folder,
}

impl GroupStyle {
    pub(crate) fn apply(self, input: String) -> ColoredString {
        match self {
            GroupStyle::Mark => input.bold(),
            GroupStyle::Language => input.magenta().bold(),
            GroupStyle::Path => input.bright_black(),
            GroupStyle::Folder => input.bright_black(),
        }
    }
}

pub(crate) fn mark_styled(mark: &str) -> ColoredString {
    match mark {
        "ERROR" => mark.red().bold(),
        "WARN" => mark.yellow().bold(),
        "FIXME" => mark.red(),
        "TODO" => mark.cyan(),
        "NOTE" => mark.blue(),
        "INFO" => mark.green(),
        _ => mark.normal(),
    }
}

pub(crate) fn group_style_for(key: &DimensionValue) -> GroupStyle {
    match key {
        DimensionValue::Mark(_) => GroupStyle::Mark,
        DimensionValue::Language(_) => GroupStyle::Language,
        DimensionValue::Path(_) => GroupStyle::Path,
        DimensionValue::Folder(_) => GroupStyle::Folder,
    }
}

pub(crate) fn mark_header(mark: &str, text: &str) -> ColoredString {
    match mark {
        "ERROR" => text.red().bold(),
        "WARN" => text.yellow().bold(),
        "FIXME" => text.red().bold(),
        "TODO" => text.cyan().bold(),
        "NOTE" => text.blue().bold(),
        "INFO" => text.green().bold(),
        _ => text.bold(),
    }
}
