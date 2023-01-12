use serde::Serialize;

use super::*;

/// How documents which are missing are treated
/// during a sort
#[derive(Debug, Clone)]
pub enum MissingValue {
    /// put them at the front
    First,
    /// put them at the back
    Last,
    /// use this value for the sort
    Custom(ScalarValue),
}

impl Serialize for MissingValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::First => "_first".serialize(serializer),
            Self::Last => "_last".serialize(serializer),
            Self::Custom(value) => value.serialize(serializer),
        }
    }
}
