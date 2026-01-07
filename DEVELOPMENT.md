# Development Guide

This document provides information about the project structure and how to work with the codebase.

## Project Structure

```
skry/
├── src/
│   ├── lib.rs           # Library entry point, exports public modules
│   ├── main.rs          # CLI entry point with argument parsing
│   ├── config.rs        # Configuration parsing (.skry.toml)
│   ├── lsp.rs           # LSP client implementation
│   ├── pipeline.rs      # Intent parsing and dependency traversal
│   ├── assembly.rs      # Context assembly and token budgeting
│   └── standby.rs       # Daemon mode implementation
├── .skry.toml           # Example configuration file
├── Cargo.toml           # Rust dependencies and project metadata
└── README.md            # User-facing documentation
```

## Module Overview

### `config` Module
Handles loading and parsing `.skry.toml` configuration files. Supports:
- Token budget settings
- Pruning strategies (aggressive, bodies, full)
- LSP server mappings for different file extensions

### `lsp` Module
LSP client implementation that:
- Manages language server subprocesses
- Communicates via JSON-RPC
- Supports lifecycle management (start, stop)
- Placeholder for textDocument/definition and textDocument/references

### `pipeline` Module
Core processing pipeline that:
- Parses user intent (symbol resolution or search)
- Resolves entry points for LSP queries
- Traverses dependency graphs
- Currently contains placeholder implementations

### `assembly` Module
Context assembly engine that:
- Stitches code snippets into prompts
- Applies pruning strategies
- Enforces token budget limits
- Estimates token usage (rough approximation: 1 token ≈ 4 characters)

### `standby` Module
Daemon mode implementation for:
- Keeping LSP servers running between queries
- Reducing startup latency for large projects
- Handling background query processing
- Currently contains placeholder implementations

## Building and Testing

### Build the Project
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Run Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Generate Documentation
```bash
# Generate and open documentation
cargo doc --open --no-deps
```

### Run the CLI
```bash
# Show help
cargo run -- --help

# Generate context (one-shot mode)
cargo run -- generate --search "query" --depth 2

# Start daemon
cargo run -- standby

# Query daemon (in another terminal)
cargo run -- query --symbol "function_name"
```

## Development Status

This is the **initial scaffolding** for the project. The following areas need further implementation:

### High Priority
1. **LSP Communication**: Complete JSON-RPC implementation in `lsp.rs`
   - Initialize request/response
   - textDocument/definition
   - textDocument/references
   - Proper message framing and parsing

2. **Entry Point Resolution**: Implement actual file parsing and symbol search in `pipeline.rs`
   - Use tree-sitter or regex for symbol finding
   - Implement fuzzy search for natural language queries

3. **Graph Traversal**: Implement recursive dependency walking in `pipeline.rs`
   - Breadth-first or depth-first traversal
   - Cycle detection
   - Depth limiting

4. **Daemon IPC**: Implement inter-process communication for standby mode
   - Unix socket or named pipe
   - Message protocol
   - State persistence

### Medium Priority
5. **Smart Pruning**: Improve content pruning strategies in `assembly.rs`
   - Better AST-aware pruning
   - Signature extraction
   - Comment handling

6. **Token Estimation**: More accurate token counting
   - Integration with tiktoken or similar
   - Model-specific tokenization

### Nice to Have
7. **Language Support**: Add support for more languages
   - Python (pyright/pylsp)
   - Go (gopls)
   - Java (jdtls)

8. **Configuration**: Enhanced configuration options
   - Per-language settings
   - Custom pruning rules
   - Output formatting

## Code Style

- Follow Rust standard naming conventions
- Add documentation comments (`///`) for public items
- Write unit tests for new functionality
- Use `tracing` for logging, not `println!`
- Handle errors with `Result` and `anyhow`/`thiserror`

## Dependencies

Key dependencies used:
- **clap**: CLI argument parsing with derive macros
- **tokio**: Async runtime for subprocess management
- **lsp-types**: LSP protocol types
- **serde**: Serialization/deserialization
- **toml**: Configuration file parsing
- **anyhow/thiserror**: Error handling
- **tracing**: Structured logging

## Testing Philosophy

- Unit tests for individual functions and modules
- Integration tests for end-to-end workflows (to be added)
- Test both success and error cases
- Mock external dependencies (LSP servers)

## Contributing

When adding new features:
1. Update relevant module documentation
2. Add unit tests
3. Update this development guide if architecture changes
4. Ensure `cargo test` passes
5. Run `cargo clippy` for linting suggestions
6. Format code with `cargo fmt`
