use super::*;
use serde::Deserialize;

/// When the terms are strings
pub type StringTerms = TermResults<String>;

/// When the term results are numeric
pub type NumericTerms = TermResults<i64>;

/// Term Aggregation Results from Elasticsearch
#[derive(Debug, Clone, Deserialize)]
pub struct TermResults<T> {
    /// unique term counts, this is not guaranteed
    /// to be all values, it will only be the top
    /// number of values; by default this is ten.
    #[serde(rename = "buckets")]
    pub terms: Vec<TermCount<T>>,

    /// The count of all documents not included in
    /// the terms reported on by these results
    #[serde(rename = "sum_other_doc_count")]
    pub other_doc_count: usize,

    /// The number of documents that didn’t make it
    /// into the top terms. If this is greater
    /// than 0, you can be sure that the terms agg had
    /// to throw away some buckets, either because they
    /// didn’t fit into size on the coordinating node
    /// or they didn’t fit into shard_size on the data
    /// node
    pub doc_count_error_upper_bound: usize,
}

/// Count for a particular term and any
/// sub-aggregations it may have
#[derive(Debug, Clone, Deserialize)]
pub struct TermCount<T> {
    /// counted term in all documents
    #[serde(rename = "key")]
    pub term: T,

    /// document count containing term
    #[serde(rename = "doc_count")]
    pub count: usize,

    /// Any sub-aggregations if any where requested
    #[serde(flatten)]
    pub sub_aggs: AggResultCollection,
}

impl From<StringTerms> for AggResult {
    fn from(value: StringTerms) -> Self {
        Self::StringTerms(value)
    }
}

impl From<NumericTerms> for AggResult {
    fn from(value: NumericTerms) -> Self {
        Self::NumericTerms(value)
    }
}

impl AggResultData for StringTerms {
    const ES_KEY: &'static str = "sterms";
    const ID: &'static str = "StringTermResults";

    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError> {
        match data {
            AggResult::StringTerms(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }

    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError> {
        match data {
            AggResult::StringTerms(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }
}

impl AggResultData for NumericTerms {
    const ES_KEY: &'static str = "lterms";
    const ID: &'static str = "NumericTermResults";

    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError> {
        match data {
            AggResult::NumericTerms(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }

    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError> {
        match data {
            AggResult::NumericTerms(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }
}
