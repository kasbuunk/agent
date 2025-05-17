# Haiku Generator Agent

An AI agent implementation in Rust that demonstrates the Model-Control-Protocol (MCP) pattern. This project serves as a learning exercise for building AI agents that can interact with language models and system resources.

## Project Overview

The agent implements a simple but illustrative workflow:
1. Connects to a local language model
2. Queries the model to generate haikus
3. Saves each haiku to the filesystem with timestamp-based filenames
4. Implements a controlled loop with timing

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

## Getting Started

(To be added: setup instructions, dependencies, and configuration details) 