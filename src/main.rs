pub mod agent;
pub mod model_client;

use crate::model_client::LocalOllamaClient;
use crate::agent::{Agent, MCPServer, MCPRequest};
use anyhow::Result;

struct FilesystemMCPServer;

impl MCPServer for FilesystemMCPServer {
    fn handle_request(&self, request: MCPRequest) -> Result<()> {
        match request.action.as_str() {
            "write_file" => {
                let args = request.args;
                let path = args["path"].as_str().unwrap();
                let content = args["content"].as_str().unwrap();
                
                if let Some(parent) = std::path::Path::new(path).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(path, content)?;
                Ok(())
            },
            _ => anyhow::bail!("Unsupported MCP action: {}", request.action),
        }
    }
}

#[cfg(test)]
mod tests {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = LocalOllamaClient::new("qwen3".to_string());
    let mcp_server = FilesystemMCPServer;
    let agent = Agent::new(
        Box::new(model),
        Box::new(mcp_server),
        "Generate a haiku about programming and store it in haikus/latest.txt".to_string(),
    );

    agent.run_once().await?;
    Ok(())
}
