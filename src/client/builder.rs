use super::*;

/// Builds a client which can be used to work with Elasticsearch
#[derive(Debug)]
pub struct ClientBuilder<T: ClientAdapter> {
    index: Option<String>,
    host: Option<String>,
    doc_type: Option<String>,
    credentials: Option<Credentials>,
    default_limit: Option<usize>,
    adapter: Option<T>,
}

/// Building a client can fail; and as such
/// these are the conditions in which they can
/// fail.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Variant if the underlying adapter creation fails
    #[error("Adapter Setup Failed: {0}")]
    AdapterSetup(#[from] adapter::AdapterError),

    /// Details which fields are missing or invalid
    #[error("Missing or Bad Data Fields: {0}")]
    BadData(String),
}

impl<T: ClientAdapter> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self {
            index: None,
            host: None,
            doc_type: None,
            credentials: None,
            default_limit: None,
            adapter: None,
        }
    }
}

impl<T: ClientAdapter> ClientBuilder<T> {
    /// Set Host
    ///
    /// This must be the http url scheme and full http path
    /// to the root of the Elasticsearch server.
    ///
    /// Example: `"https://localhost:8080/"`
    ///
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Set Index
    ///
    /// This is the index ( or indexes separated by commas ) that
    /// the client will be performing requests against.
    ///
    /// Examples:
    ///
    /// - `"my_index"`
    /// - `"current,last_month"`
    ///
    pub fn index<S: Into<String>>(mut self, index: S) -> Self {
        self.index = Some(index.into());
        self
    }

    /// Set Document Type
    ///
    /// Although it's depricated; some still use this for legacy
    /// purposes.  If this is you consider getting away from that
    /// sooner than later.
    ///
    pub fn doc_type<S: Into<String>>(mut self, doc_type: S) -> Self {
        self.doc_type = Some(doc_type.into());
        self
    }

    /// Set Credentials
    ///
    /// More and more Elasticsearch servers are setup with credential
    /// requirements to access and update data.  If your server falls
    /// does this is how you would set them.
    ///
    pub fn credentials<S, V>(mut self, username: S, password: V) -> Self
    where
        S: Into<String>,
        V: Into<String>,
    {
        let credentials = Credentials {
            username: username.into(),
            password: password.into(),
        };
        self.credentials = Some(credentials);
        self
    }

    /// Default Limit
    ///
    /// Optional limit to apply for any searches which do not specify
    /// a limit.  This is a convenience to get a different default result
    /// set count.
    ///
    pub fn default_limit(mut self, limit: usize) -> Self {
        self.default_limit = Some(limit);
        self
    }

    /// Use Adapter
    ///
    /// If you want to supply a specially setup adapter for use that goes
    /// beyond the scope of this project this provides a way to do that.
    /// Providing this essentially ignores most of the other provided
    /// settings for creating an adapter.  If you use this please make sure
    /// you understand what settings are still used and how.
    ///
    pub fn use_adapter(mut self, adapter: T) -> Self {
        self.adapter = Some(adapter);
        self
    }

    /// Build Client
    ///
    /// Consumes the builder and attempts to create a new client.  This
    /// can fail by failing to provide certain fields or if the adapter
    /// has problems initializing.
    ///
    pub fn build(mut self) -> Result<Client<T>, BuilderError> {
        let settings = self.build_settings()?;

        let adapter = if let Some(adapter) = self.adapter {
            adapter
        } else {
            T::try_new_from(&settings)?
        };

        Ok(Client { adapter, settings })
    }

    /// Helper method to extract settings from the builder structure.
    /// This is mainly just to keep `build()` a little cleaner.
    fn build_settings(&mut self) -> Result<Settings, BuilderError> {
        let mut missing_fields = vec![];

        if self.host.is_none() {
            missing_fields.push("HOST");
        }

        if self.index.is_none() {
            missing_fields.push("INDEX");
        }

        if !missing_fields.is_empty() {
            return Err(BuilderError::BadData(missing_fields.join(",")));
        }

        Ok(Settings {
            host: self.host.take().unwrap(),
            index: self.index.take().unwrap(),
            doc_type: self.doc_type.take(),
            credentials: self.credentials.take(),
            default_limit: self.default_limit.take(),
        })
    }
}
