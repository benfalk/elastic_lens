//!
//! Any iteractions between your code and the Elasticsearch server
//! are encapsulated under this module.  One major note is that you
//! are not working directly with an offical client, although one may
//! be supported later.  What you are working with instead is a wrapper
//! around some third party library that is doing the heavy lifting.
//!

mod adapter;
mod builder;
mod offical_client_adapter;
mod settings;

pub use adapter::*;
pub use builder::*;
pub use settings::*;

use serde::de::DeserializeOwned;

/// The adapter which is used by default for the ClientBuilder
pub type DefaultAdapter = offical_client_adapter::ElasticsearchAdapter;

/// Error variants the client can emmit
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// Any errors from the client's adapter
    #[error("{0}")]
    Adapter(#[from] AdapterError),

    /// If the document cannot be deserialzed
    #[error("Client Deserialize Error: {0}")]
    Deserialize(#[from] serde_json::Error),
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

impl Client<DefaultAdapter> {
    /// Create a builder which drives the creation of a client
    pub fn default_builder() -> ClientBuilder<DefaultAdapter> {
        ClientBuilder::default()
    }
}

impl<T: ClientAdapter> Client<T> {
    /// Fetch a document by ID
    pub async fn get_by_id<D>(&self, id: &str) -> Result<Option<D>, ClientError>
    where
        D: DeserializeOwned,
    {
        use crate::response::single_document::SingleDocument;

        match self.adapter.get_by_id(id).await {
            Ok(data) => {
                let single: SingleDocument<D> = serde_json::from_str(&data)?;
                Ok(single.doc)
            }
            Err(AdapterError::NotFound) => Ok(None),
            Err(other) => Err(ClientError::Adapter(other)),
        }
    }
}
