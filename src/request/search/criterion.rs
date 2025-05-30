use super::*;
use serde::Serialize;

mod all_match;
mod any_match;
mod builder_trait;
mod exists_filter;
mod geo_distance_filter;
mod nested;
mod not_all;
mod range_filter;
mod term_filter;

pub use all_match::*;
pub use any_match::*;
pub use builder_trait::*;
pub use exists_filter::*;
pub use geo_distance_filter::*;
pub use nested::*;
pub use not_all::*;
pub use range_filter::*;
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
    /// at least one value from the target is found on the field
    Contains(TermFilter),

    /// selects by a range for a field
    Range(RangeFilter),

    /// if the document has any value for the field
    Exists(ExistsFilter),

    /// if the field is withing the distance of provided point/distance
    GeoDistance(GeoDistanceFilter),

    /// matches if at least one criterion is true
    AnyMatch(AnyMatch),

    /// matches if none of the criterion are true
    NotAll(NotAll),

    /// selects if all conditions are true, optimized to put
    /// not conditions together in a similar bucket
    AllMatch(AllMatch),

    /// sub-query designed to filter nested structures
    Nested(NestedFilter),
}

impl Criterion {
    pub(crate) fn is_usable(&self) -> bool {
        match self {
            Self::AllMatch(all) => all.has_data(),
            Self::NotAll(not_all) => not_all.has_data(),
            Self::AnyMatch(any) => any.has_data(),
            Self::Nested(query) => query.has_data(),
            _ => true,
        }
    }
}

impl Serialize for Criterion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Contains(filter) => filter.serialize(serializer),
            Self::Range(filter) => filter.serialize(serializer),
            Self::Exists(filter) => filter.serialize(serializer),
            Self::GeoDistance(filter) => filter.serialize(serializer),
            Self::AnyMatch(filters) => filters.serialize(serializer),
            Self::NotAll(filters) => filters.serialize(serializer),
            Self::AllMatch(filters) => filters.serialize(serializer),
            Self::Nested(filters) => filters.serialize(serializer),
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
    impl SealedCriterion for RangeFilter {}
    impl SealedCriterion for ExistsFilter {}
    impl SealedCriterion for GeoDistanceFilter {}
    impl SealedCriterion for AnyMatch {}
    impl SealedCriterion for NotAll {}
    impl SealedCriterion for AllMatch {}
}
