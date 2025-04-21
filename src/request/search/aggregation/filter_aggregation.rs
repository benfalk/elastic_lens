use super::*;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

/// A single bucket aggregation that narrows the
/// set of documents to those that match a query
#[derive(Debug, Clone, Serialize)]
pub struct FilterAggregation {
    #[serde(serialize_with = "serialize_allmatch_for_filter")]
    #[serde(rename = "filter")]
    query: AllMatch,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aggs")]
    sub_aggs: Option<AggCollection>,
}

impl FilterAggregation {
    /// Create a new filter aggergation with the
    /// provided query and no sub-aggregations
    pub fn new(query: AllMatch) -> Self {
        Self {
            query,
            sub_aggs: None,
        }
    }

    /// Create a new filter aggregation with the
    /// provided query and sub-aggregations
    pub fn new_with_sub_aggs(query: AllMatch, sub_aggs: AggCollection) -> Self {
        Self {
            query,
            sub_aggs: Some(sub_aggs),
        }
    }
}

impl From<FilterAggregation> for Aggregation {
    fn from(value: FilterAggregation) -> Self {
        Self::Filter(value)
    }
}

impl AggregationData for FilterAggregation {}

fn serialize_allmatch_for_filter<S>(allmatch: &AllMatch, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Debug, Default, Serialize)]
    struct EmptyBool {
        bool: HashMap<(), ()>,
    }

    if allmatch.has_data() {
        allmatch.serialize(serializer)
    } else {
        EmptyBool::default().serialize(serializer)
    }
}
