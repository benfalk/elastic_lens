use super::*;
use serde::Deserialize;

/// Results for a cardinality aggregation
#[derive(Debug, Clone, Deserialize, Default, Copy)]
pub struct Cardinality {
    /// the cardinality count value
    #[serde(rename = "value")]
    pub count: usize,
}

impl From<Cardinality> for AggResult {
    fn from(value: Cardinality) -> Self {
        Self::Cardinality(value)
    }
}

impl AggResultData for Cardinality {
    const ID: &'static str = "Cardinality";
    const ES_KEY: &'static str = "cardinality";

    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError> {
        match data {
            AggResult::Cardinality(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }

    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError> {
        match data {
            AggResult::Cardinality(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }
}
