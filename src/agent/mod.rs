use crate::model_client::ModelClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub action: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub mcp_requests: Vec<MCPRequest>,
}

pub trait MCPServer {
    fn handle_request(&self, request: MCPRequest) -> Result<()>;
}

pub struct Agent {
    model: Box<dyn ModelClient>,
    mcp_server: Box<dyn MCPServer>,
    context: String,
}

impl Agent {
    pub fn new(
        model: Box<dyn ModelClient>,
        mcp_server: Box<dyn MCPServer>,
        initial_context: String,
    ) -> Self {
        Self {
            model,
            mcp_server,
            context: initial_context,
        }
    }

    pub async fn run_once(&self) -> Result<()> {
        // Ask model what actions to take
        let response = self.model.complete(&self.context).await?;

        // Parse the model's JSON response to get MCP requests
        let agent_response: ModelResponse = match serde_json::from_str(&response.response) {
            Ok(response) => response,
            Err(e) => {
                // Log the invalid response for debugging
                eprintln!("Failed to parse model response: {}", e);
                eprintln!("Raw response: {}", response.response);
                return Err(anyhow::anyhow!("Invalid JSON response from model"));
            }
        };

        // Execute each MCP request through the server
        for request in agent_response.mcp_requests {
            self.mcp_server.handle_request(request)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_client::LocalOllamaClient;
    use std::path::PathBuf;
    use tokio::time::{timeout, Duration};

    struct TestMCPServer {
        temp_dir: PathBuf,
    }

    impl MCPServer for TestMCPServer {
        fn handle_request(&self, request: MCPRequest) -> Result<()> {
            match request.action.as_str() {
                "write_file" => {
                    let args = request.args;
                    let path = self.temp_dir.join(args["path"].as_str().unwrap());
                    let content = args["content"].as_str().unwrap();

                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(path, content)?;
                    Ok(())
                }
                _ => anyhow::bail!("Unsupported MCP action: {}", request.action),
            }
        }
    }

    #[tokio::test]
    async fn acceptance_test_agent_stores_model_response() -> Result<()> {
        // Create a temporary directory for test outputs
        let temp_dir = tempfile::tempdir()?;
        let expected_path = "haikus/2024-03-17/nature_inspired.txt";

        // Initial prompt that specifies the task using MCP
        let initial_prompt = format!(
            "SYSTEM: You are a JSON-only response bot. You must ONLY output valid JSON, with NO explanations or thinking process.
HUMAN: Generate a haiku about nature and return it in this EXACT JSON format:
{{
    \"mcp_requests\": [
        {{
            \"action\": \"write_file\",
            \"args\": {{
                \"path\": \"{}\",
                \"content\": \"<first line>\\n<second line>\\n<third line>\"
            }}
        }}
    ]
}}
Requirements:
1. The haiku must follow 5-7-5 syllable pattern
2. Replace <first line>, <second line>, <third line> with your haiku
3. DO NOT include any text outside the JSON
4. DO NOT explain your thinking
5. DO NOT add any formatting or indentation
ASSISTANT: Output the JSON now:",
            expected_path
        );

        let model = LocalOllamaClient::new("qwen3".to_string());
        let mcp_server = TestMCPServer {
            temp_dir: temp_dir.path().to_path_buf(),
        };
        let agent = Agent::new(Box::new(model), Box::new(mcp_server), initial_prompt);

        // Run the agent once with a 60-second timeout
        match timeout(Duration::from_secs(60), agent.run_once()).await {
            Ok(result) => result?,
            Err(_) => anyhow::bail!("Agent timed out after 60 seconds"),
        }

        // Verify the file exists and has three lines (haiku structure)
        let full_path = temp_dir.path().join(expected_path);
        assert!(
            full_path.exists(),
            "File should exist at the specified path"
        );
        let content = std::fs::read_to_string(full_path)?;
        assert_eq!(
            content.lines().count(),
            3,
            "File should contain exactly three lines (haiku structure)"
        );

        Ok(())
    }
}

