use super::NumericValue;
use serde::Serialize;
use std::borrow::Cow;

/// Scalar Value
///
/// Simple data type that represents the common primitive
/// data that you normally will send to Elasticsearch.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue {
    /// string values, can be static or owned
    Text(Cow<'static, str>),
    /// decimal numbers
    Float(f64),
    /// whole numbers
    Integer(i64),
    /// true or false
    Boolean(bool),
}

impl From<&'static str> for ScalarValue {
    fn from(value: &'static str) -> Self {
        Self::Text(Cow::Borrowed(value))
    }
}

impl From<String> for ScalarValue {
    fn from(value: String) -> Self {
        Self::Text(Cow::Owned(value))
    }
}

impl From<f64> for ScalarValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for ScalarValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for ScalarValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<NumericValue> for ScalarValue {
    fn from(value: NumericValue) -> Self {
        match value {
            NumericValue::Float(float) => Self::Float(float),
            NumericValue::Integer(int) => Self::Integer(int),
        }
    }
}

impl Serialize for ScalarValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Text(cow) => serializer.serialize_str(cow),
            Self::Float(float) => serializer.serialize_f64(*float),
            Self::Integer(int) => serializer.serialize_i64(*int),
            Self::Boolean(boolean) => serializer.serialize_bool(*boolean),
        }
    }
}
