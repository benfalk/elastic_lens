use super::*;
use std::ops::Not;

mod field;

pub use field::*;

/// Tag used by SearchCondition to determine
/// if criterion is negated or not.
#[derive(Debug, Clone, Copy)]
pub enum Tag {
    /// Goes to positive bucket
    Positive,
    /// Goes to negative bucket
    Negative,
}

/// This is the struct that criterion get baked
/// down into before being unwrapped again and
/// placed into a search.  The tag is what keeps
/// track if the condition has been negated or not
#[derive(Debug, Clone)]
pub struct SearchCondition {
    pub(crate) criterion: Criterion,
    pub(crate) tag: Tag,
}

impl<C: Into<Criterion>> From<C> for SearchCondition {
    fn from(value: C) -> Self {
        SearchCondition {
            criterion: value.into(),
            tag: Tag::Positive,
        }
    }
}

impl Not for RangeFilterBuilder {
    type Output = SearchCondition;
    fn not(self) -> Self::Output {
        SearchCondition::from(self).not()
    }
}

impl Not for SearchCondition {
    type Output = Self;
    fn not(self) -> Self::Output {
        SearchCondition {
            criterion: self.criterion,
            tag: Tag::Negative,
        }
    }
}

impl Not for ExistsFilter {
    type Output = SearchCondition;
    fn not(self) -> Self::Output {
        SearchCondition {
            criterion: self.into(),
            tag: Tag::Negative,
        }
    }
}

impl Not for GeoDistanceFilter {
    type Output = SearchCondition;
    fn not(self) -> Self::Output {
        SearchCondition {
            criterion: self.into(),
            tag: Tag::Negative,
        }
    }
}
