use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::model::{DimensionValue, Mark};
use crate::sort::DimensionStage;
use crate::utils::extract_dimension_value;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FilterConfig {
    pub rules: Vec<FilterRule>,
}

impl FilterConfig {
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn apply(&self, marks: Vec<Mark>, roots: &[PathBuf]) -> Vec<Mark> {
        if self.rules.is_empty() || marks.is_empty() {
            return marks;
        }

        marks
            .into_iter()
            .filter(|mark| self.allows(mark, roots))
            .collect()
    }

    fn allows(&self, mark: &Mark, roots: &[PathBuf]) -> bool {
        for rule in &self.rules {
            let value = extract_dimension_value(&rule.stage, mark, roots);
            if !rule.allows(value.as_ref()) {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilterRule {
    pub stage: DimensionStage,
    pub predicate: ValuePredicate,
}

impl FilterRule {
    fn allows(&self, value: Option<&DimensionValue>) -> bool {
        self.predicate.allows(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValuePredicate {
    Allow { values: Vec<DimensionValue> },
    Deny { values: Vec<DimensionValue> },
}

impl ValuePredicate {
    fn allows(&self, value: Option<&DimensionValue>) -> bool {
        match self {
            ValuePredicate::Allow { values } => {
                value.is_some_and(|value| contains_value(values, value))
            }
            ValuePredicate::Deny { values } => {
                !value.is_some_and(|value| contains_value(values, value))
            }
        }
    }
}

fn contains_value(values: &[DimensionValue], value: &DimensionValue) -> bool {
    values.iter().any(|candidate| value_eq(candidate, value))
}

fn value_eq(a: &DimensionValue, b: &DimensionValue) -> bool {
    match (a, b) {
        (DimensionValue::Mark(a), DimensionValue::Mark(b)) => a.eq_ignore_ascii_case(b),
        (DimensionValue::Language(a), DimensionValue::Language(b)) => a.eq_ignore_ascii_case(b),
        (DimensionValue::Path(a), DimensionValue::Path(b)) => a == b,
        (DimensionValue::Folder(a), DimensionValue::Folder(b)) => a == b,
        _ => false,
    }
}
