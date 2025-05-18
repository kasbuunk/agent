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
        
        eprintln!("Sending prompt to model: {}", prompt);
        
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

        eprintln!("Raw model response: {}", response);
        
        // Parse the Ollama response
        let ollama_response: serde_json::Value = serde_json::from_str(&response)?;
        let raw_response = ollama_response["response"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing response field"))?;
        
        eprintln!("Model text response: {}", raw_response);
        
        // Extract JSON part from the response
        if let Some(json_start) = raw_response.find('{') {
            if let Some(json_end) = raw_response.rfind('}') {
                let json_str = &raw_response[json_start..=json_end];
                eprintln!("Extracted JSON: {}", json_str);
                return Ok(ModelResponse {
                    response: json_str.to_string()
                });
            }
        }
        
        // If no JSON found, return the raw response
        eprintln!("No JSON found in response");
        Ok(ModelResponse {
            response: raw_response.to_string()
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