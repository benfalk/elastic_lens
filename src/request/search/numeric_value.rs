use serde::Serialize;

/// Certain functionality is only concerned with numeric
/// values; this is to cover those cases to ensure only
/// the correct values are used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericValue {
    /// decimal numbers
    Float(f64),
    /// whole numbers
    Integer(i64),
}

impl From<f64> for NumericValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for NumericValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl Serialize for NumericValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Float(float) => serializer.serialize_f64(*float),
            Self::Integer(int) => serializer.serialize_i64(*int),
        }
    }
}
