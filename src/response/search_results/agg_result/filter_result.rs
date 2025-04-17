use super::*;
use serde::Deserialize;

/// Results for a filtered aggregation
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Filtered {
    /// number of documents that matched the filter
    #[serde(rename = "doc_count")]
    pub count: usize,

    /// any sub-aggregations if any were requested
    #[serde(flatten)]
    pub sub_aggs: AggResultCollection,
}

impl From<Filtered> for AggResult {
    fn from(value: Filtered) -> Self {
        Self::Filtered(value)
    }
}

impl AggResultData for Filtered {
    const ID: &'static str = "Filtered";
    const ES_KEY: &'static str = "filter";

    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError> {
        match data {
            AggResult::Filtered(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }

    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError> {
        match data {
            AggResult::Filtered(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }
}
