# Agent

A Rust-based agent that uses the Model Context Protocol (MCP) to interact with local resources.

## Features

- Uses Ollama with qwen3 model for LLM interactions
- Integrates with official MCP server implementation for file operations
- Follows strict TDD principles with comprehensive test coverage

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Configure the MCP server:
   - Copy `claude_desktop_config.json` to:
     - MacOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
     - Windows: `%APPDATA%/Claude/claude_desktop_config.json`

3. Install Ollama and the qwen3 model:
```bash
ollama pull qwen3
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

Note: The acceptance test has a 60-second timeout to account for model response time.

## Architecture

The project follows a clean architecture with the following components:

- `Agent`: Core business logic that coordinates between the model and MCP server
- `ModelClient`: Interface to the LLM (Ollama with qwen3)
- MCP Integration: Uses official MCP server implementation for file operations

## MCP Server Integration

This project uses the official Model Context Protocol (MCP) server implementation for file operations. The MCP server provides a standardized way for AI models to interact with the filesystem while maintaining security and proper access controls.

### Configuration

The MCP server is configured in `claude_desktop_config.json`. By default, it uses the official filesystem server with access to the current directory.

### Usage

The agent generates MCP requests in the following format:

```json
{
    "mcp_requests": [
        {
            "action": "write_file",
            "args": {
                "path": "path/to/file",
                "content": "file content"
            }
        }
    ]
}
```

These requests are handled by the official MCP server, which provides proper security and access controls.

## Project Overview

The agent implements a simple but illustrative workflow:
1. Connects to a local language model
2. Processes prompts and receives responses
3. Interacts with system resources through MCP
4. Implements a controlled execution loop

## Core Components

### MCP Implementation
- **Client**: Implements the MCP client protocol
- **Model Interface**: Connects to a local language model
- **Tool Usage**: Demonstrates filesystem operations through MCP server

### Main Features
- Continuous haiku generation loop
- Filesystem interaction for saving outputs
- Controlled timing with sleep intervals
- Structured prompt engineering
- Timestamp-based file management

### Technical Requirements
- Rust
- Local language model setup (to be configured)
- MCP server implementation (to be configured)

## Agent State and Memory Considerations

The agent maintains state through its context, which represents the ongoing "memory" of the agent. This is a crucial design consideration with several important aspects:

### Context Evolution
- The context is not just a static mission or prompt
- It evolves over time as the agent:
  - Interacts with the model
  - Receives feedback from humans
  - Makes progress on tasks
  - Interacts with other agents

### Types of Context
1. **Conversation History**
   - Full history of interactions
   - May need compression/summarization for token limits
   - Critical for maintaining coherent dialogue

2. **Task State**
   - Current progress on objectives
   - Pending items (like a backlog)
   - Completed actions and their outcomes

3. **Environmental Knowledge**
   - Understanding of available tools
   - Access permissions and constraints
   - System state and resources

### Future Considerations
- Implementing more sophisticated context management
- Adding context compression strategies
- Supporting multi-agent context sharing
- Developing context persistence mechanisms
- Adding human feedback integration

## Project Goals

1. **Learning Objectives**
   - Understand MCP client implementation
   - Practice AI agent development in Rust
   - Learn model interaction patterns
   - Implement tool usage through MCP

2. **Technical Implementation**
   - Clean architecture following Rust best practices
   - Error handling and logging
   - Configuration management
   - Proper resource cleanup

## Development Plan

### Phase 1: Basic Structure
- Set up project structure
- Implement basic MCP client structure
- Create model interaction interface

### Phase 2: Core Functionality
- Implement haiku generation loop
- Add filesystem operations
- Integrate timing controls

### Phase 3: Model Integration
- Configure local model connection
- Implement prompt engineering
- Add error handling

### Phase 4: Polish
- Add logging and monitoring
- Implement graceful shutdown
- Add configuration options

## Development Philosophy

### Test-Driven Development (TDD)
We follow a strict TDD approach with these principles:
1. Write the minimal test that verifies core behavior
2. Start with `todo!()` implementations to see the test fail
3. Implement the minimal code to make the test pass
4. Avoid premature abstractions or assumptions

### Interface Design
- Start with the simplest possible interface
- Let complexity emerge from real requirements
- Don't make assumptions about implementation details too early
- Keep dependencies (like HTTP, filesystem) behind clean interfaces

### Example: Model Client Development
Here's how we developed the model client:
1. Started with just the core need: send prompt, get response
2. Wrote a test against a real model (no mocks initially)
3. Made no assumptions about transport/protocol until needed
4. Added complexity (like error handling) only when required

Bad:
```rust
// Too specific, assumes too much
async fn test_model_formats_json_and_uses_http() {
    assert_eq!(response.format, "specific_format");
    assert_eq!(response.metadata.timestamp, "2024-03-17");
}
```

Good:
```rust
// Tests core behavior only
async fn test_local_model_responds_to_prompt() {
    let client = LocalOllamaClient::new("qwen3".to_string());
    let response = client.complete("Say hello").await.unwrap();
    assert!(!response.response.is_empty());
}
```

### Incremental Development
1. Start with real, working examples
2. Keep changes small and focused
3. Commit working increments
4. Document learnings as you go

## Development Learnings

### TDD Best Practices
1. **Complete One TDD Cycle**
   - Write a failing test
   - Make it pass with minimal implementation
   - Only then refactor for better design
   - Don't start refactoring before the test passes

2. **Test Implementation**
   - Start with test-local implementations (like `ExternalMCPServer` in tests)
   - Move to production implementations only after test passes
   - Keep test implementations simple and focused

3. **Debugging Approach**
   - Add detailed logging at key points
   - Test components in isolation (e.g., model responses)
   - Use appropriate timeouts based on actual response times
   - Verify external dependencies (model server, MCP server)

### MCP Architecture Insights
1. **Component Separation**
   - Keep model client separate from MCP server
   - Use clean interfaces (traits) for each component
   - Follow adapter pattern for external services

2. **Protocol Design**
   - Use JSON-RPC for standardized communication
   - Handle both success and error responses
   - Include proper request/response validation
   - Maintain clear protocol boundaries

3. **Error Handling**
   - Proper error propagation through Result types
   - Descriptive error messages
   - Timeout handling for external services
   - Resource cleanup (e.g., process handling)

### Model Integration
1. **Response Handling**
   - Extract JSON from model responses
   - Handle thinking/XML-like tags
   - Validate response format
   - Use appropriate timeouts

2. **Prompt Engineering**
   - Clear system instructions
   - Explicit format requirements
   - No ambiguity in expected output
   - Include validation rules

### Refactoring Guidelines
1. **When to Refactor**
   - After tests pass
   - When patterns emerge from implementation
   - To reduce duplication
   - To improve separation of concerns

2. **Refactoring Steps**
   - One change at a time
   - Keep tests passing
   - Document design decisions
   - Update tests to match new structure

### Common Pitfalls
1. **Avoid Premature Abstraction**
   - Start with concrete implementations
   - Let patterns emerge naturally
   - Refactor only when needed
   - Keep it simple initially

2. **Testing Gotchas**
   - Don't mix test and production code
   - Handle external dependencies properly
   - Use appropriate timeouts
   - Clean up resources

3. **Development Flow**
   - Complete TDD cycles
   - Commit working states
   - Document learnings
   - Maintain clean history

## Getting Started

### Prerequisites
- Rust
- [Ollama](https://ollama.ai) installed
- qwen3 model pulled and running:
```bash
# First time setup
ollama pull qwen3

# Running the model (required for tests)
ollama run qwen3
```

### Running Tests
1. Ensure qwen3 is running in Ollama
2. Run the tests: `cargo test`

## Next Steps
1. Extract MCP server into its own module
2. Add more MCP actions beyond file operations
3. Improve error handling and logging
4. Add configuration management
5. Implement proper context management

## Architecture Overview

### Current State
- Model client capable of prompting and receiving responses
- Basic error handling and timeouts
- Clean separation between model and agent concerns

### Proposed Next Steps

#### Component Architecture
1. **Agent**
   - Acts as a "dumb pipe"
   - Coordinates between model and MCP client
   - No decision-making logic
   - Passes model responses to appropriate MCP servers

2. **Model Client** (✓ Implemented)
   - Handles model communication
   - Sends prompts, receives responses
   - Error handling and timeouts

3. **MCP Client** (To be implemented)
   - Implements MCP protocol
   - Discovers and communicates with MCP servers
   - Handles filesystem operations through MCP server
   - Maintains clean protocol boundaries

4. **MCP Filesystem Server** (To be implemented)
   - Implements filesystem operations
   - Follows MCP specification
   - Provides controlled access to file system

#### Development Approach

1. **Acceptance Test First**
   ```rust
   #[test]
   async fn test_agent_can_write_haiku_to_file() {
       let agent = Agent::new(
           ModelClient::new("qwen3"),
           MCPClient::new()
       );
       
       // Agent coordinates model and filesystem interaction
       let result = agent.run_once().await;
       
       // Verify haiku was written through MCP
       assert!(result.is_ok());
       // Verify file exists and contains haiku
   }
   ```

2. **Guided Implementation**
   - Start with failing acceptance test
   - Implement MCP filesystem server first
   - Then MCP client with filesystem capabilities
   - Finally, agent coordination logic

3. **Key Principles**
   - Agent remains "dumb" - just coordinates
   - Model makes decisions about tool usage
   - Clean protocol boundaries
   - Clear separation of concerns

#### Benefits of This Approach
1. Clear direction through acceptance test
2. Natural emergence of components
3. Protocol-driven development
4. Testable boundaries
5. Follows single responsibility principle

#### Risks and Considerations
1. Need to ensure model responses follow MCP format
2. Protocol versioning and compatibility
3. Error handling across boundaries
4. Testing strategy for each component