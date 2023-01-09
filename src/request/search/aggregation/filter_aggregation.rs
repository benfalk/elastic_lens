use super::*;
use serde::Serialize;

/// A single bucket aggregation that narrows the
/// set of documents to those that match a query
#[derive(Debug, Clone, Serialize)]
pub struct FilterAggregation {
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
