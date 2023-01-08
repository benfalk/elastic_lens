use super::*;
use serde::Serialize;

/// Terms Aggregation
///
/// This is a request for the top terms for a field
/// along with addtional sub-aggregations for each
/// term in the aggregation.
#[derive(Debug, Clone)]
pub struct TermsAggregation {
    field: Field,
    size: Option<u16>,
    sub_agg: Option<AggCollection>,
}

impl TermsAggregation {
    /// Create a terms aggregation for a field.
    /// Leaves the bucket size to default for Elasticsearch;
    /// which is typically the top 10 terms.
    pub fn new<F>(field: F) -> Self
    where
        F: Into<Field>,
    {
        Self {
            field: field.into(),
            size: None,
            sub_agg: None,
        }
    }

    /// Create a terms aggregation for a field with a maximum
    /// bucket size set instead of being left up to the default
    /// for Elasticsearch.
    pub fn new_with_size<F>(field: F, size: u16) -> Self
    where
        F: Into<Field>,
    {
        Self {
            size: Some(size),
            ..Self::new(field)
        }
    }
}

impl AggregationData for TermsAggregation {}

impl From<TermsAggregation> for Aggregation {
    fn from(value: TermsAggregation) -> Self {
        Self::Terms(value)
    }
}

impl Serialize for TermsAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Params<'a> {
            field: &'a Field,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<u16>,
        }

        #[derive(Serialize)]
        struct Agg<'a> {
            terms: Params<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aggs: &'a Option<AggCollection>,
        }

        let agg = Agg {
            terms: Params {
                field: &self.field,
                size: self.size,
            },
            aggs: &self.sub_agg,
        };

        agg.serialize(serializer)
    }
}
