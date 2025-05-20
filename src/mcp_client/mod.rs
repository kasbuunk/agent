use anyhow::Result;
use rmcp::model;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

pub struct MCPClient {
    server_process: Option<tokio::process::Child>,
}

impl Drop for MCPClient {
    fn drop(&mut self) {
        if let Some(mut child) = self.server_process.take() {
            // Try to kill the process on drop
            let _ = child.kill();
        }
    }
}

impl MCPClient {
    pub fn new() -> Self {
        Self {
            server_process: None,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        let child = Command::new("npx")
            .arg("-y")
            .arg("@modelcontextprotocol/server-filesystem")
            .arg(".") // Or /tmp
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped()) // Capture stderr as well
            .spawn()?;

        // Store the child process
        self.server_process = Some(child);

        // Wait a moment for the server to initialize
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        println!("MCP server initialized");
        Ok(())
    }

    pub async fn do_request(&mut self, request: model::JsonRpcRequest) -> Result<()> {
        let child = self
            .server_process
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP server not initialized"))?;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdin"))?;

        // Debug output to see what we're sending
        let request_json = json!(request);
        println!("Sending request: {}", request_json);

        // Send the request
        stdin.write_all(request_json.to_string().as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        // Get the response with timeout protection
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            // Set a timeout for reading the response
            let read_future = reader.read_line(&mut line);
            match tokio::time::timeout(std::time::Duration::from_secs(5), read_future).await {
                Ok(result) => {
                    result?;
                    println!("Received response: {}", line);
                    if line.is_empty() {
                        return Err(anyhow::anyhow!("Empty response from MCP server"));
                    }

                    let response: serde_json::Value = serde_json::from_str(&line)?;

                    // Check for errors in the response
                    if let Some(error) = response.get("error") {
                        return Err(anyhow::anyhow!("MCP server error: {}", error));
                    }
                }
                Err(_) => {
                    return Err(anyhow::anyhow!("Timeout waiting for MCP server response"));
                }
            }
        }
        Ok(())
    }

    pub async fn write_file(&mut self, path: &str, content: &str) -> Result<()> {
        let child = self
            .server_process
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP server not initialized"))?;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdin"))?;

        // Create the MCP request - fixed method name and parameters structure
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",  // Method name
            "params": {
                "name": "write_file",
                "arguments": {
                    "path": path,
                    "content": content
                    },
            },
            "id": 1
        });

        // Debug output to see what we're sending
        println!("Sending request: {}", request.to_string());

        // Send the request
        stdin.write_all(request.to_string().as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        // Get the response with timeout protection
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            // Set a timeout for reading the response
            let read_future = reader.read_line(&mut line);
            match tokio::time::timeout(std::time::Duration::from_secs(5), read_future).await {
                Ok(result) => {
                    result?;
                    println!("Received response: {}", line);
                    if line.is_empty() {
                        return Err(anyhow::anyhow!("Empty response from MCP server"));
                    }

                    let response: serde_json::Value = serde_json::from_str(&line)?;

                    // Check for errors in the response
                    if let Some(error) = response.get("error") {
                        return Err(anyhow::anyhow!("MCP server error: {}", error));
                    }
                }
                Err(_) => {
                    return Err(anyhow::anyhow!("Timeout waiting for MCP server response"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, fs};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_execute_request_to_mcp_server() -> Result<()> {
        // Create a test file with a unique name
        let test_file_path = format!("./test_{}.txt", Uuid::new_v4());
        let test_content = "Hello through MCP!";

        println!("Test file path: {}", test_file_path);

        // Create and initialize the MCP client
        let mut client = MCPClient::new();
        client.init().await?;

        // Print server info to debug
        println!("Starting file write operation...");

        let id = 42;
        // Create parameters as a serde_json::Map
        let mut params_map = serde_json::Map::new();
        params_map.insert("name".to_string(), json!("write_file"));
        params_map.insert(
            "arguments".to_string(),
            json!({
                "path": test_file_path,
                "content": test_content
            }),
        );

        let mcp_request: model::JsonRpcRequest = model::JsonRpcRequest {
            jsonrpc: model::JsonRpcVersion2_0,
            id: model::NumberOrString::Number(id),
            request: model::Request {
                method: "tools/call".to_string(),
                params: Some(model::WithMeta {
                    _meta: None,
                    inner: params_map,
                }),
            },
        };

        // Attempt to do the mcp request.
        match client.do_request(mcp_request).await {
            Ok(_) => println!("mcp request successful"),
            Err(e) => println!("Error doing request: {}", e),
        }

        // Check if file exists before reading
        if !std::path::Path::new(&test_file_path).exists() {
            println!("File does not exist after write operation!");
            return Err(anyhow::anyhow!("File was not created"));
        }

        // Verify the file was written with correct content
        let written_content = fs::read_to_string(&test_file_path)?;
        println!("File content: {}", written_content);
        assert_eq!(written_content, test_content);

        // Clean up
        let _ = std::fs::remove_file(&test_file_path);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_file_through_mcp() -> Result<()> {
        // Create a test file with a unique name
        let test_file_path = format!("./test_{}.txt", Uuid::new_v4());
        let test_content = "Hello through MCP!";

        println!("Test file path: {}", test_file_path);

        // Create and initialize the MCP client
        let mut client = MCPClient::new();
        client.init().await?;

        // Print server info to debug
        println!("Starting file write operation...");

        // Attempt to write the file through MCP
        match client.write_file(&test_file_path, test_content).await {
            Ok(_) => println!("File written successfully"),
            Err(e) => println!("Error writing file: {}", e),
        }

        // Check if file exists before reading
        if !std::path::Path::new(&test_file_path).exists() {
            println!("File does not exist after write operation!");
            return Err(anyhow::anyhow!("File was not created"));
        }

        // Verify the file was written with correct content
        let written_content = fs::read_to_string(&test_file_path)?;
        println!("File content: {}", written_content);
        assert_eq!(written_content, test_content);

        // Clean up
        let _ = std::fs::remove_file(&test_file_path);

        Ok(())
    }
}
