use serde::Deserialize;

/// Single document wrapper that is used to hold
/// the actual document type that's expecting to
/// be deserialized.
#[derive(Debug, Deserialize)]
pub(crate) struct SingleDocument<T> {
    #[serde(rename = "_source")]
    pub(crate) doc: Option<T>,
}
