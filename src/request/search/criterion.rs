use super::*;
use serde::Serialize;

mod builder_trait;
mod term_filter;

pub use builder_trait::*;
pub use term_filter::*;

/// Conditions for a Search
///
/// This encapsulates the different kinds of conditions that
/// a search has.  It is non-exhaustive because there is no
/// way I plan to cover them all for awhile and there is also
/// the fact that more could be added with newer versions of
/// Elasticsearch.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Criterion {
    /// at least one value from the target is found
    Contains(TermFilter),
}

impl Serialize for Criterion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Contains(termfilter) => termfilter.serialize(serializer),
        }
    }
}

/// Every data type wrapped by `Criterion` must have this trait.
/// It acts as a sainity check to ensure they can be serialized
/// and wrapped into the enum.
pub trait CriterionData: private::SealedCriterion {}

mod private {
    use super::*;
    use crate::request::search::Criterion;
    use serde::Serialize;

    pub trait SealedCriterion: Sized + Serialize + Into<Criterion> {}

    impl SealedCriterion for TermFilter {}
}
