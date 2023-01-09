use super::*;
use elasticsearch::{
    http::{transport::Transport, Url},
    Elasticsearch,
};
use serde::Serialize;

#[derive(Debug)]
pub struct ElasticsearchAdapter {
    es_client: Elasticsearch,
    settings: Settings,
}

#[async_trait::async_trait]
impl ClientAdapter for ElasticsearchAdapter {
    type Transport = Elasticsearch;

    fn try_new_from(settings: &Settings) -> Result<Self, AdapterError> {
        let mut url =
            Url::parse(&settings.host).map_err(|e| AdapterError::Internal(format!("{:?}", e)))?;

        if let Some(Credentials { username, password }) = &settings.credentials {
            // TODO: Don't throw these errors away
            url.set_username(username).ok();
            url.set_password(Some(password)).ok();
        }

        let transport = Transport::single_node(url.as_str())?;

        Ok(ElasticsearchAdapter {
            es_client: Elasticsearch::new(transport),
            settings: settings.clone(),
        })
    }

    fn borrow_transport(&self) -> &Self::Transport {
        &self.es_client
    }

    async fn get_by_id(&self, id: &str) -> Result<String, AdapterError> {
        use elasticsearch::GetParts;

        let parts = if let Some(doc_type) = &self.settings.doc_type {
            GetParts::IndexTypeId(&self.settings.index, doc_type, id)
        } else {
            GetParts::IndexId(&self.settings.index, id)
        };

        let response = self.es_client.get(parts).send().await?;

        match response.status_code().as_u16() {
            200 => Ok(response.text().await?),
            404 => Err(AdapterError::NotFound),
            code => Err(AdapterError::Internal(format!(
                "[{code}]: {}",
                response.text().await?
            ))),
        }
    }

    async fn search<B: Serialize + Sync>(&self, body: &B) -> Result<String, AdapterError> {
        use elasticsearch::SearchParts;

        let index = [self.settings.index.as_str()];
        let mut doc = [""];

        let parts = if let Some(doc_type) = self.settings.doc_type.as_ref() {
            doc[0] = doc_type.as_str();
            SearchParts::IndexType(&index, &doc)
        } else {
            SearchParts::Index(&index)
        };

        let response = self
            .es_client
            .search(parts)
            .body(body)
            .typed_keys(true)
            .send()
            .await?;

        match response.status_code().as_u16() {
            200 => Ok(response.text().await?),
            code => Err(AdapterError::Internal(format!(
                "[{code}]: {}",
                response.text().await?
            ))),
        }
    }
}

// Convert Error Types from host
impl From<elasticsearch::Error> for AdapterError {
    fn from(value: elasticsearch::Error) -> Self {
        AdapterError::Internal(format!("{:?}", value))
    }
}
