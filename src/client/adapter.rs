use super::*;
use crate::request::MultiSearch;
use serde::Serialize;

/// Every error that can be emmited by an adapter
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    /// Misc errors from the adapter
    #[error("Internal Adapter Error: {0}")]
    Internal(String),

    /// General error to let the caller know document is missing
    #[error("Document not found")]
    NotFound,
}

/// This trait is to avoid direct calls to a third party crate **AND** be
/// able to test interactions by mocking the adapter.  Currently this trait
/// is sealed as the API is extremely unstable right now... and it may remain
/// that way for the foreseeable future.
#[async_trait::async_trait]
pub trait ClientAdapter: private::SealedClientAdapter {
    /// the actual library making the underlying calls.
    type Transport;

    /// Given settings try to create a new instance of a client
    fn try_new_from(settings: &Settings) -> Result<Self, AdapterError>;

    /// an excape-hatch in case you need to make a low level request
    fn borrow_transport(&self) -> &Self::Transport;

    /// Fetch a document by id, returns the raw body response
    async fn get_by_id(&self, id: &str) -> Result<String, AdapterError>;

    /// Given a body that can serialize execute a search
    /// against the configured index and possible doc type
    async fn search<B: Serialize + Sync>(&self, body: &B) -> Result<String, AdapterError>;

    /// Performs multiple searches at once
    async fn multi_search<'a>(&self, mut searches: MultiSearch<'a>)
        -> Result<String, AdapterError>;
}

mod private {
    use crate::client::offical_adapter::ElasticsearchAdapter;

    pub trait SealedClientAdapter: Send + Sync + Sized {}

    impl SealedClientAdapter for ElasticsearchAdapter {}
}
