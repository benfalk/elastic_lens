use serde::Serialize;
use std::borrow::Cow;

/// Every aggregation in an Elasticsearch request is given
/// a name so that when the data comes back you can match
/// each aggregation dimension in the response from the
/// request you gave it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(Cow<'static, str>);

impl From<&'static str> for Name {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
