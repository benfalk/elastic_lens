//! Any error in ElasticLens is captured here

/// This encapsulates every error which can be emitted
/// by ElasticLens.  While there are more fine-grained
/// error types in this crate they can all be converted
/// to this root type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any errors from the internal client adapter
    #[error("Internal Client Error: {0}")]
    Adapter(#[from] crate::client::AdapterError),

    /// Any errors related to building a client
    #[error("Error Building Client: {0}")]
    ClientBuild(#[from] crate::client::BuilderError),

    /// Any errors related to the client
    #[error("Client Error: {0}")]
    Client(#[from] crate::client::ClientError),

    /// Any errors related to bad access of aggregation
    /// data from the results.
    #[error("{0}")]
    AggResultAccess(#[from] crate::response::AggAccessError),
}
