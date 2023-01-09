use super::*;
use serde::Serialize;

/// A multi-value metrics aggregation that computes stats
/// over numeric values extracted from the aggregated
/// documents.
///
/// The stats that are returned consist of:
/// - `min`
/// - `max`
/// - `sum`
/// - `count`
/// - `avg`
#[derive(Debug, Clone)]
pub struct StatsAggregation {
    field: Field,
    missing_value: Option<NumericValue>,
}

impl StatsAggregation {
    /// Create a new stats aggregation for the given field
    pub fn new<F: Into<Field>>(field: F) -> Self {
        Self {
            field: field.into(),
            missing_value: None,
        }
    }

    /// Create a new stats aggregation for the given field
    /// and when a document is missing a value for use the
    /// provided value.  By default documents missing this
    /// field are ignored when calculating stats.
    pub fn new_with_missing<F, N>(field: F, numeric: N) -> Self
    where
        F: Into<Field>,
        N: Into<NumericValue>,
    {
        Self {
            field: field.into(),
            missing_value: Some(numeric.into()),
        }
    }
}

impl From<StatsAggregation> for Aggregation {
    fn from(value: StatsAggregation) -> Self {
        Self::Stats(value)
    }
}

impl Serialize for StatsAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        struct Params<'a> {
            field: &'a Field,
            #[serde(skip_serializing_if = "Option::is_none")]
            missing: &'a Option<NumericValue>,
        }

        let params = Params {
            field: &self.field,
            missing: &self.missing_value,
        };

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("stats", &params)?;
        map.end()
    }
}

impl AggregationData for StatsAggregation {}
