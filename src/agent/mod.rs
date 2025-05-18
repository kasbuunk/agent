use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
    pub storage_path: String,
}

#[async_trait]
pub trait ModelClient {
    async fn complete(&self, prompt: &str) -> Result<ModelResponse>;
}

pub struct Agent {
    model: Box<dyn ModelClient>,
    context: String, // Ongoing conversation/task context that evolves over time
}

impl Agent {
    pub fn new(model: Box<dyn ModelClient>, initial_context: String) -> Self {
        Self { 
            model, 
            context: initial_context,
        }
    }

    pub async fn run_once(&self) -> Result<()> {
        // Ask model what to generate and where to store it
        let response = self.model.complete(&self.context).await?;
        
        // Ensure parent directories exist
        if let Some(parent) = std::path::Path::new(&response.storage_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Write response to file
        tokio::fs::write(response.storage_path, response.content).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct TestModel {
        responses: Arc<Mutex<Vec<ModelResponse>>>,
    }

    #[async_trait]
    impl ModelClient for TestModel {
        async fn complete(&self, _prompt: &str) -> Result<ModelResponse> {
            let mut responses = self.responses.lock().await;
            Ok(responses.remove(0))
        }
    }

    #[tokio::test]
    async fn test_agent_follows_model_storage_instructions() {
        // Create a temporary directory for test outputs
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file_path = temp_dir.path().join("test_output.txt");
        
        // Set up test model with a predetermined response that includes storage instructions
        let responses = Arc::new(Mutex::new(vec![
            ModelResponse {
                content: "Test response from model".to_string(),
                storage_path: test_file_path.to_string_lossy().to_string(),
            },
        ]));
        
        let model = TestModel { responses };
        let agent = Agent::new(
            Box::new(model),
            "Initial context: The agent needs to generate and store content appropriately".to_string(),
        );

        // Run one iteration of the agent
        agent.run_once().await.unwrap();

        // Verify the file was created where the model specified
        assert!(test_file_path.exists(), "File should exist at model-specified path");
        
        let file_content = std::fs::read_to_string(test_file_path).unwrap();
        assert_eq!(file_content, "Test response from model");
    }
} 