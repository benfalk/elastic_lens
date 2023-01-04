/// Information that drives the client in how it connects
/// and interactions with an Elasticsearch server
#[derive(Debug, Clone, Default)]
pub struct Settings {
    pub(super) index: String,
    pub(super) host: String,
    pub(super) doc_type: Option<String>,
    pub(super) credentials: Option<Credentials>,
    pub(super) default_limit: Option<usize>,
}

/// Represents the username and password which
/// are used to connect to the Elasticsearch
/// server.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub(super) username: String,
    pub(super) password: String,
}
