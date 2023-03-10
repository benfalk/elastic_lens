use super::*;
use serde::Serialize;

mod builder_trait;
mod collection;
mod filter_aggregation;
mod name;
mod stats_aggregation;
mod terms_aggregation;

pub use builder_trait::*;
pub use collection::*;
pub use filter_aggregation::*;
pub use name::*;
pub use stats_aggregation::*;
pub use terms_aggregation::*;

/// Enum to represent ever form of aggregation
/// that can be asked for from Elasticsearch.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Aggregation {
    /// A multi-bucket value source based aggregation where
    /// buckets are dynamically built - one per unique value.
    Terms(TermsAggregation),

    /// Numerical stats for a field:
    ///   min, max, sum, count and avg.
    Stats(StatsAggregation),

    /// A single bucket aggregation that narrows the
    /// set of documents to those that match a query
    Filter(FilterAggregation),
}

impl Serialize for Aggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Terms(agg) => agg.serialize(serializer),
            Self::Stats(agg) => agg.serialize(serializer),
            Self::Filter(agg) => agg.serialize(serializer),
        }
    }
}

/// Every data type wrapped by `Aggregation` must have this trait.
/// It acts as a sainity check to ensure they can be serialized
/// and wrapped into the enum.
pub trait AggregationData: private::SealedAgg {}

mod private {
    use super::*;
    use serde::Serialize;

    pub trait SealedAgg: Sized + Serialize + Into<Aggregation> {}

    impl SealedAgg for TermsAggregation {}
    impl SealedAgg for StatsAggregation {}
    impl SealedAgg for FilterAggregation {}
}
