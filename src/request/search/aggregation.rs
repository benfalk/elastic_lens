use super::*;
use serde::Serialize;

mod builder_trait;
mod collection;
mod name;
mod terms_aggregation;

pub use builder_trait::*;
pub use collection::*;
pub use name::*;
pub use terms_aggregation::*;

/// Enum to represent ever form of aggregation
/// that can be asked for from Elasticsearch.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Aggregation {
    /// A multi-bucket value source based aggregation where
    /// buckets are dynamically built - one per unique value.
    Terms(TermsAggregation),
}

impl Serialize for Aggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Terms(agg) => agg.serialize(serializer),
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
}
