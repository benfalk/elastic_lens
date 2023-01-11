use super::*;
use crate::request::search::SearchBody;
use serde::Serialize;

/// An MSearch requires a "header" row above every request
/// row that is sent.  Since we really don't have anything
/// to send along for this row, this serves as a marketer
/// to the serialize to that fact.
pub(super) enum MsearchBody<'a> {
    EmptyMeta,
    Search(SearchBody<'a>),
}

impl<'a> Serialize for MsearchBody<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        match self {
            Self::Search(body) => body.serialize(serializer),
            Self::EmptyMeta => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            }
        }
    }
}

/// consumes the multisearch and extracts the search bodies
/// over to a set of empty headers and bodies and then wrapped
/// by `JsonBody` as that is the interface for the Elasticsearch
/// client
pub(super) fn multisearch_to_body(mut search: MultiSearch<'_>) -> Vec<JsonBody<MsearchBody<'_>>> {
    search
        .bodies
        .drain(..)
        .flat_map(|body| [MsearchBody::EmptyMeta, MsearchBody::Search(body)])
        .map(JsonBody::new)
        .collect()
}
