use super::*;
mod collection;
mod stats_result;
mod terms_result;

pub use collection::*;
pub use stats_result::*;
pub use terms_result::*;

/// A single aggregation result from Elasticsearch
#[derive(Debug, Clone)]
pub enum AggResult {
    /// when the term results are strings
    StringTerms(StringTerms),

    /// when the term results are numeric
    NumericTerms(NumericTerms),

    /// collecting stats for a field
    ///    min, max, sum, count and avg
    Stats(Stats),
}

impl AggResult {
    pub(super) fn str_identifier(&self) -> &'static str {
        match self {
            Self::NumericTerms(agg) => agg.str_identifier(),
            Self::StringTerms(agg) => agg.str_identifier(),
            Self::Stats(agg) => agg.str_identifier(),
        }
    }
}

/// Sanity Tag to ensure every aggregation
/// variant has the required functionality
pub trait AggResultData: private::SealedAggResultData {
    /// the tagged key name Elasticsearch will use
    /// as an identifier this is the the type to
    /// deserialize into
    const ES_KEY: &'static str;

    /// A nice name to represent the type for
    /// error reporting purposes
    const ID: &'static str;

    /// used to pull a reference back from the wrapped
    /// agg result enum
    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError>;

    /// attemps to unwrap the aggregation result it was placed
    /// in and pulls it back out to it's original data type
    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError>;

    #[doc(hidden)]
    fn str_identifier(&self) -> &'static str {
        Self::ID
    }
}

mod private {
    use super::*;
    use serde::Deserialize;

    /// Ensure `AggResultData` cannot be implemented
    /// outside of this crate for now.
    pub trait SealedAggResultData: Into<AggResult> + Deserialize<'static> + Sized {}

    impl SealedAggResultData for NumericTerms {}
    impl SealedAggResultData for StringTerms {}
    impl SealedAggResultData for Stats {}
}
