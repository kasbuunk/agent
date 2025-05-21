# The Silicon Reptile Brain: Engineering an AI Agent

A Rust-based agent that uses JSON-RPC to interact with local resources through a filesystem service, demonstrating the principles of Model Context Protocol (MCP).

## Features

- Uses Ollama with qwen3 model for LLM interactions
- Implements JSON-RPC based file operations
- Follows strict TDD principles with comprehensive test coverage
- Demonstrates core MCP principles through practical implementation

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Install Ollama and the qwen3 model:
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

- `Agent`: Core business logic that coordinates between the model and filesystem operations
- `ModelClient`: Interface to the LLM (Ollama with qwen3)
- `MCPClient`: Handles JSON-RPC based file operations

## File Operations

This project uses JSON-RPC for file operations. The agent communicates with a filesystem service using standardized JSON-RPC requests.

### Usage

The agent generates JSON-RPC requests in the following format:

```json
{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "write_file",
        "arguments": {
            "path": "path/to/file",
            "content": "file content"
        }
    },
    "id": 1
}
```

## Project Overview

The agent implements a simple but illustrative workflow:
1. Connects to a local language model (Ollama with qwen3)
2. Processes prompts and receives responses
3. Interacts with the filesystem through JSON-RPC calls
4. Implements a controlled execution loop for continuous operation

## Core Components

### Implementation
- **MCPClient**: Implements JSON-RPC client for file operations
- **ModelClient**: Connects to Ollama for LLM capabilities
- **Agent**: Coordinates between model and file operations

### Main Features
- Continuous haiku generation loop
- Filesystem interaction through JSON-RPC
- Controlled timing with sleep intervals
- Structured prompt engineering
- UUID-based file management

### Technical Requirements
- Rust
- Ollama with qwen3 model

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
- Keep dependencies behind clean interfaces

### Example: Model Client Development
Here's how we developed the model client:
1. Started with just the core need: send prompt, get response
2. Wrote a test against a real model (no mocks initially)
3. Made no assumptions about transport/protocol until needed
4. Added complexity (like error handling) only when required

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

## Project Insights and Learnings

To comprehend what an AI agent is, how it works, and what it needs to interact with the world, I built this agent program in Rust. I learned extensively about the Model Context Protocol, which enables an LLM to have an effect on the world.

### Key Discoveries

As a simple but profound demonstration, I created a test where the agent prompts an LLM to write a haiku to the file system. The elegance lies in having no haiku-specific code—the intelligence resides entirely in the prompt. While I hardcoded the filesystem MCP server in the initialization, this approach could easily be extracted to configuration files or made dynamic.

The agent is not complete, but it has taught me enough about agent architecture to know when to stop pursuing perfection. Having built this foundation, I now understand what to look for in off-the-shelf agents.

### Conceptual Framework

Before implementation, I developed this analogy between agents and the human body:
- **Agent**: reptile brain (controller)
- **LLM**: neocortex (reasoning)
- **MCP Client**: nervous system (communication)
- **MCP Servers**: muscles (tools) and sensors (resources)

The agent module functions as a controller with access to both the model and MCP client. It currently initiates the MCP server (though ideally this would reside elsewhere), forwards prompts to the LLM, and processes tool calls into MCP requests.

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

## Future Directions

- Design communication channels for agent-to-agent interaction, including inbox/outbox systems that allow for human intervention and input
- Explore existing agent implementations to strengthen intuition and practical understanding through hands-on application
