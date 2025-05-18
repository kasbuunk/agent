# Agent

A Rust implementation that demonstrates the Model-Control-Protocol (MCP) pattern. This project serves as a learning exercise for building AI agents that can interact with language models and system resources.

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

(To be added: setup instructions, dependencies, and configuration details) 