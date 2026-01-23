use super::*;
use serde::Serialize;

/// Cardinality Aggregation
///
/// This is a request for the count of unique values
/// for a field.  It is not guaranteed to be exact,
/// and will always become less accurate as number of
/// terms increases.  The `precision_threshold` can be
/// increased to improve accuracy at the cost of more
/// memory usage on the Elasticsearch server.
///
#[derive(Debug, Clone)]
pub struct CardinalityAggregation {
    field: Field,
    precision_threshold: Option<u32>,
    missing: Option<ScalarValue>,
}

impl CardinalityAggregation {
    /// Create a cardinality aggregation for a field.
    pub fn new<F>(field: F) -> Self
    where
        F: Into<Field>,
    {
        Self {
            field: field.into(),
            precision_threshold: None,
            missing: None,
        }
    }

    /// Sets the precision threshold for the cardinality aggregation.
    pub fn with_precision_threshold(mut self, threshold: u32) -> Self {
        self.precision_threshold = Some(threshold);
        self
    }

    /// A value to use when a document's field is missing.
    pub fn use_for_missing_value<V>(mut self, value: V) -> Self
    where
        V: Into<ScalarValue>,
    {
        self.missing = Some(value.into());
        self
    }
}

impl AggregationData for CardinalityAggregation {}

impl From<CardinalityAggregation> for Aggregation {
    fn from(value: CardinalityAggregation) -> Self {
        Self::Cardinality(value)
    }
}

impl Serialize for CardinalityAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use ::serde::ser::SerializeStruct;

        #[derive(Serialize)]
        struct Params<'a> {
            field: &'a Field,
            #[serde(skip_serializing_if = "Option::is_none")]
            precision_threshold: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            missing: Option<&'a ScalarValue>,
        }

        let params = Params {
            field: &self.field,
            precision_threshold: self.precision_threshold,
            missing: self.missing.as_ref(),
        };

        let mut state = serializer.serialize_struct("cardinality", 1)?;
        state.serialize_field("cardinality", &params)?;
        state.end()
    }
}
