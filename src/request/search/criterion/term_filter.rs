use super::*;
use serde::Serialize;

/// This is the underlying data used to represent a term
/// or terms filter in Elasticsearch.  It is normally wrapped in
/// the `Criterion` enum.
#[derive(Debug, Clone)]
pub struct TermFilter {
    field: Field,
    value: ValueType,
}

#[derive(Debug, Clone)]
enum ValueType {
    Single(ScalarValue),
    Many(Vec<ScalarValue>),
}

impl ValueType {
    fn key(&self) -> &'static str {
        match self {
            Self::Single(_) => "term",
            Self::Many(_) => "terms",
        }
    }
}

impl TermFilter {
    /// create a new term filter
    pub fn single<F, V>(field: F, value: V) -> Self
    where
        F: Into<Field>,
        V: Into<ScalarValue>,
    {
        Self {
            field: field.into(),
            value: ValueType::Single(value.into()),
        }
    }

    /// create a term filter for many different values
    pub fn many<F, V, S>(field: F, values: V) -> Self
    where
        F: Into<Field>,
        S: Into<ScalarValue>,
        V: IntoIterator<Item = S>,
    {
        let values = values.into_iter().map(Into::into).collect();
        Self {
            field: field.into(),
            value: ValueType::Many(values),
        }
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

        impl Serialize for ValueType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    Self::Single(value) => value.serialize(serializer),
                    Self::Many(values) => values.serialize(serializer),
                }
            }
        }

        struct FieldValuePair<'a> {
            pair: &'a TermFilter,
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

        let pair = FieldValuePair { pair: self };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(self.value.key(), &pair)?;
        map.end()
    }
}
