// llmdoc/src/embeddings/http_provider.rs

use async_trait::async_trait;
use reqwest::Client;

use crate::embeddings::{Embedding, EmbeddingError, EmbeddingProvider, EmbeddingRequest, EmbeddingResponse};

pub struct HttpEmbeddingProvider {
    client: Client,
    url: String,
    api_key: Option<String>,
    model: Option<String>,
}

impl HttpEmbeddingProvider {
    pub fn new(url: String, api_key: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            url,
            api_key,
            model,
        }
    }
}

#[async_trait]
impl EmbeddingProvider for HttpEmbeddingProvider {
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Embedding>, EmbeddingError> {
        let request = EmbeddingRequest {
            texts,
            model: self.model.clone(),
        };
        let response = self.generate_embeddings_with_response(request).await?;
        Ok(response.embeddings)
    }

    async fn generate_embeddings_with_response(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse, EmbeddingError> {
        let mut request_builder = self.client.post(&self.url).json(&request);

        if let Some(api_key) = &self.api_key {
            request_builder = request_builder.bearer_auth(api_key);
        }

        let response = request_builder.send().await?.json::<EmbeddingResponse>().await?;
        Ok(response)
    }

    fn provider_name(&self) -> String {
        "http_embedding_provider".to_string()
    }
}