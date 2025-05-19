pub mod agent;
pub mod mcp_client;
pub mod model_client;

use crate::agent::Agent;
use crate::model_client::LocalOllamaClient;
use mcp_client::MCPClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = LocalOllamaClient::new("qwen3".to_string());
    let mcp_client = MCPClient::new();
    let mut agent = Agent::new(
        Box::new(model),
        mcp_client,
        "Generate a haiku about programming and store it in haikus/latest.txt".to_string(),
    );

    agent.run_once().await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
