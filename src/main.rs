pub mod agent;
pub mod mcp_client;
pub mod model_client;

use mcp_client::MCPClient;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = model_client::LocalOllamaClient::new("qwen3".to_string());
    let mut mcp_client = MCPClient::new();
    mcp_client.init().await?;

    let file_prefix = "./haiku-";
    let initial_prompt = format!(
            "SYSTEM: You are an agent with MCP capabilities. You have access to the filesystem write_file call, where the method is 'tools/call'. You must ONLY output valid JSON, with NO explanations or thinking process.
HUMAN: Generate a haiku about nature and return it as the contents of a file prefixed with {}, in the current directory, with a correct unique random uuid after the prefix and ending in a .txt extension, with the write_file command for the filesystem MCP server in the JSON-RPC format. For example:
{}

Requirements:
1. The haiku must follow 5-7-5 syllable pattern
2. Replace <first line>, <second line>, <third line> with your haiku
3. DO NOT include any text outside the JSON
4. DO NOT explain your thinking
5. DO NOT add any formatting or indentation
ASSISTANT: Output the JSON now:",

            file_prefix,
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
    let mut agent = agent::Agent::new(Box::new(model), mcp_client, initial_prompt);

    loop {
        match agent.run_once().await {
            Ok(_) => {}
            Err(err) => eprintln!("error running agent: {}", err),
        };
    }
}
