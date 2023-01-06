use super::*;
use serde::Serialize;

/// This is the underlying data used to represent a term
/// filter in Elasticsearch.  It is normally wrapped in
/// the `Criterion` enum.
#[derive(Debug, Clone)]
pub struct TermFilter {
    field: Field,
    value: ScalarValue,
}

impl TermFilter {
    /// create a new term filter
    pub fn new(field: Field, value: ScalarValue) -> Self {
        Self { field, value }
    }
}

impl CriterionData for TermFilter {}

impl From<TermFilter> for Criterion {
    fn from(value: TermFilter) -> Self {
        Self::Contains(value)
    }
}

impl Serialize for TermFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        struct FieldValuePair<'a> {
            pair: &'a TermFilter,
        }

        #[derive(Serialize)]
        struct Term<'a> {
            term: FieldValuePair<'a>,
        }

        impl<'a> Serialize for FieldValuePair<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&self.pair.field, &self.pair.value)?;
                map.end()
            }
        }

        let term = Term {
            term: FieldValuePair { pair: self },
        };

        term.serialize(serializer)
    }
}
