//!
//! Any iteractions between your code and the Elasticsearch server
//! are encapsulated under this module.  One major note is that you
//! are not working directly with an official client, although one may
//! be supported later.  What you are working with instead is a wrapper
//! around some third party library that is doing the heavy lifting.
//!

mod adapter;
mod builder;
#[cfg(feature = "official_client")]
mod official_adapter;
mod settings;

use std::borrow::{Borrow, Cow};

pub use adapter::*;
pub use builder::*;
pub use settings::*;

use crate::{
    request::{search::SearchTrait, MultiSearch, ScrollCursor, ScrollSearch},
    response::{MultiResponse, SearchResults},
};
use serde::de::DeserializeOwned;

/// The adapter which is used by default for the ClientBuilder
#[cfg(feature = "official_client")]
pub type DefaultAdapter = official_adapter::ElasticsearchAdapter;

/// Result for the client where the Error is always a ClientError
pub type ClientResult<T> = std::result::Result<T, ClientError>;

/// Error variants the client can emmit
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// Any errors from the client's adapter
    #[error("{0}")]
    Adapter(#[from] AdapterError),

    /// If the document cannot be deserialzed you get
    /// the underlying serde_json::Error as well as the
    /// payload that could not be deserialized.
    #[error("Client Deserialize Error: {0}")]
    Deserialize(serde_json::Error, String),

    /// If an expected scroll ID is missing while making a
    /// call to [Client::scroll] or [Client::scroll_search]
    #[error("Missing Scroll ID")]
    MissingScrollId,
}

/// Passes requests to the Elasticsearch server it has been configured
/// to connect to and parses the responses.  Both the requests and the
/// responses are traits and data-structures that make working with it
/// more pleasant.
#[derive(Debug, Clone)]
pub struct Client<T: ClientAdapter> {
    adapter: T,
    settings: Settings,
}

#[cfg(feature = "official_client")]
impl Client<DefaultAdapter> {
    /// Create a builder which drives the creation of a client
    pub fn default_builder() -> ClientBuilder<DefaultAdapter> {
        ClientBuilder::default()
    }
}

impl<T: ClientAdapter> Client<T> {
    /// Fetch a document by ID
    pub async fn get_by_id<D>(&self, id: &str) -> ClientResult<Option<D>>
    where
        D: DeserializeOwned,
    {
        use crate::response::single_document::SingleDocument;

        match self.adapter.get_by_id(id).await {
            Ok(data) => {
                let single: SingleDocument<D> = deserialze(data)?;
                Ok(single.doc)
            }
            Err(AdapterError::NotFound) => Ok(None),
            Err(other) => Err(ClientError::Adapter(other)),
        }
    }

    /// Execute a Search
    pub async fn search<D>(&self, search: &impl SearchTrait) -> ClientResult<SearchResults<D>>
    where
        D: DeserializeOwned,
    {
        let mut body = search.search_body();
        body.apply_defaults(&self.settings);

        match self.adapter.search(&body).await {
            Ok(data) => Ok(deserialze(data)?),
            Err(other) => Err(ClientError::Adapter(other)),
        }
    }

    /// Make several searches at once
    pub async fn multi_search<'a, D>(
        &self,
        search: impl Into<MultiSearch<'a>>,
    ) -> ClientResult<MultiResponse<D>>
    where
        D: DeserializeOwned,
    {
        let data = self.adapter.multi_search(search.into()).await?;
        Ok(deserialze(data)?)
    }

    /// Start a search that allows you to fetch
    /// all of the results.  This returns a tuple
    /// with the [ScrollCursor] first and the
    /// [SearchResults] second.  The cursor is used
    /// with [Client::scroll] to fetch additional
    /// results.
    pub async fn scroll_search<D>(
        &self,
        search: &impl SearchTrait,
    ) -> ClientResult<(ScrollCursor, SearchResults<D>)>
    where
        D: DeserializeOwned,
    {
        let duration = "1m";
        let scroll = ScrollSearch::new_with_duration(search, duration);
        let data = self.adapter.scroll_search(scroll).await?;
        let mut results: SearchResults<D> = deserialze(data)?;
        let cursor = ScrollCursor {
            scroll_id: results
                .take_scroll_id()
                .ok_or_else(|| ClientError::MissingScrollId)?,
            scroll: duration.into(),
        };
        Ok((cursor, results))
    }

    /// Fetch another page of results from a [ScrollCursor]
    /// that was started by [Client::scroll_search].
    pub async fn scroll<D>(&self, scroll: &mut ScrollCursor) -> ClientResult<SearchResults<D>>
    where
        D: DeserializeOwned,
    {
        let data = self.adapter.scroll(scroll).await?;
        let mut results: SearchResults<D> = deserialze(data)?;
        scroll.scroll_id = results
            .take_scroll_id()
            .ok_or_else(|| ClientError::MissingScrollId)?;
        Ok(results)
    }
}

fn deserialze<T: DeserializeOwned>(data: String) -> ClientResult<T> {
    serde_json::from_str(&data).map_err(|serde_err| ClientError::Deserialize(serde_err, data))
}
