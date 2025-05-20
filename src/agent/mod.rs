use crate::mcp_client::MCPClient;
use crate::model_client::ModelClient;
use anyhow::Result;
use rmcp::model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub arguments: Arguments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arguments {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub method: String,
    pub params: Params,
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
    mcp_client: MCPClient,
    context: String,
}

impl Agent {
    pub fn new(
        model: Box<dyn ModelClient>,
        mcp_client: MCPClient,
        initial_context: String,
    ) -> Self {
        Self {
            model,
            mcp_client,
            context: initial_context,
        }
    }

    pub async fn run_once(&mut self) -> Result<()> {
        // Ask model what actions to take
        let model_response = self.model.complete(&self.context).await?;

        // Parse the model's JSON response to get MCP requests
        let mcp_request: model::JsonRpcRequest =
            match serde_json::from_str(&model_response.response) {
                Ok(response) => response,
                Err(e) => {
                    // Log the invalid response for debugging
                    eprintln!("Failed to parse model response: {}", e);
                    eprintln!("Raw response: {}", model_response.response);
                    return Err(anyhow::anyhow!("Invalid JSON response from model"));
                }
            };

        // Execute each MCP request through the server
        self.mcp_client.do_request(mcp_request).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mcp_client, model_client::LocalOllamaClient};
    use serde_json::json;
    use std::fs;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn acceptance_test_agent_stores_model_response() -> Result<()> {
        // Create a temporary directory for test outputs
        let expected_path = "./nature_inspired.txt";

        // Initial prompt that specifies the task using MCP
        let initial_prompt = format!(
            "SYSTEM: You are an agent with MCP capabilities. You have access to the filesystem write_file call, where the method is 'tools/call'. You must ONLY output valid JSON, with NO explanations or thinking process.
HUMAN: Generate a haiku about nature and return it as the contents of a file named {} with the write_file command for the filesystem MCP server in the JSON-RPC format. For example:
{}

Requirements:
1. The haiku must follow 5-7-5 syllable pattern
2. Replace <first line>, <second line>, <third line> with your haiku
3. DO NOT include any text outside the JSON
4. DO NOT explain your thinking
5. DO NOT add any formatting or indentation
ASSISTANT: Output the JSON now:",

            expected_path,
        json!({
            "jsonrpc": "2.0",
            "method": "tools/call",  // Method name
            "params": {
                "name": "write_file",
                "arguments": {
                    "path": "my_path",
                    "content": "my_content"
                    },
            },
            "id": 1
        }),
        );

        let model = LocalOllamaClient::new("qwen3".to_string());
        let mut mcp_client = mcp_client::MCPClient::new();
        mcp_client.init().await?;
        let mut agent = Agent::new(Box::new(model), mcp_client, initial_prompt);

        // Run the agent once with a 60-second timeout
        match timeout(Duration::from_secs(60), agent.run_once()).await {
            Ok(result) => result?,
            Err(_) => anyhow::bail!("Agent timed out after 60 seconds"),
        }

        // Verify the file exists and has three lines (haiku structure)
        assert!(
            fs::exists(expected_path)?,
            "File should exist at the specified path"
        );
        let content = std::fs::read_to_string(expected_path)?;
        assert_eq!(
            content.lines().count(),
            3,
            "File should contain exactly three lines (haiku structure)"
        );

        Ok(())
    }
}

