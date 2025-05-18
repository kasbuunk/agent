use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ModelResponse {
    pub response: String,
}

#[async_trait]
pub trait ModelClient {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse>;
}

pub struct LocalOllamaClient {
    model: String,
}

impl LocalOllamaClient {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

#[async_trait]
impl ModelClient for LocalOllamaClient {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse> {
        let client = reqwest::Client::new();
        
        let response = client.post("http://localhost:11434/api/generate")
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false  // Disable streaming for simplicity
            }))
            .send()
            .await?
            .text()
            .await?;

        // Parse the single response
        let ollama_response: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(ModelResponse {
            response: ollama_response["response"].as_str()
                .ok_or_else(|| anyhow::anyhow!("Missing response field"))?.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_model_responds_to_prompt() {
        let client = LocalOllamaClient::new("qwen3".to_string());
        let response = client.complete("Say hello").await.unwrap();
        assert!(!response.response.is_empty());
    }
}