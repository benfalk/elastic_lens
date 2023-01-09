use super::*;
use serde::Deserialize;

/// Thes are numerical stats collected for a numeric field
/// or calculation
#[derive(Debug, Clone, Deserialize, Copy)]
pub struct Stats {
    /// the number of items counted for stats
    pub count: usize,

    /// the minimum value found
    pub min: f64,

    /// the maximum value found
    pub max: f64,

    /// the average value found
    pub avg: f64,

    /// the sum total of all values
    pub sum: f64,
}

impl From<Stats> for AggResult {
    fn from(value: Stats) -> Self {
        Self::Stats(value)
    }
}

impl AggResultData for Stats {
    const ES_KEY: &'static str = "stats";
    const ID: &'static str = "Stats";

    fn borrow_agg_result(data: &AggResult) -> Result<&Self, AggAccessError> {
        match data {
            AggResult::Stats(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }

    fn unwrap_inner(data: AggResult) -> Result<Self, AggAccessError> {
        match data {
            AggResult::Stats(agg) => Ok(agg),
            wrong_type => Err(AggAccessError::WrongType {
                expected: Self::ID,
                got: wrong_type.str_identifier(),
            }),
        }
    }
}
