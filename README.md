# 🔮 skry

> **scry** *(intransitive verb)*: To see or predict the future by means of a crystal ball.  
> **skry** *(noun)*: A deterministic, LSP-powered context assembler for LLMs.

> [!IMPORTANT]
> **Status: Ideation Phase**
> This project is currently in the architectural design and prototyping stage. The specifications below represent the target design goals, not the current state of the implementation.

---

## 👁️ Overview

**skry** is a command-line tool that allows you to "gaze" deeply into your codebase. Unlike standard RAG (Retrieval-Augmented Generation) tools that rely on fuzzy vector similarity, `skry` uses the **Language Server Protocol (LSP)** to navigate code the way a compiler does.

It builds context prompts for Large Language Models (LLMs) by traversing the Abstract Syntax Tree (AST), resolving symbol definitions, and walking dependency graphs. The result is a strictly relevant, hallucination-resistant context window.

### Why "skry"?
The name is derived from *scrying*, the act of peering into a reflective medium to reveal things that are distant or hidden. Similarly, `skry` peers into your LSP server to reveal distant dependencies and hidden interface contracts that simple text search would miss.

---

## ✨ Features

*   **Deterministic Retrieval:** No vector databases, no "vibes-based" matching. Context is built via precise symbol resolution.
*   **LSP Orchestration:** Acts as a generic LSP Client that speaks JSON-RPC to `clangd`, `rust-analyzer`, and `typescript-language-server`.
*   **Token Budgeting:** Automatically prunes deep dependency trees to fit within specific context window limits.
*   **Output Agnostic:** Does not call LLM APIs directly. It outputs formatted text to `stdout`, ready to be piped to `xclip`, `llm`, or saved to a file.

---

## 🏗️ Architecture

`skry` is built in **Rust** as a high-performance LSP Orchestrator. It does not re-implement parsing logic; instead, it launches existing language servers as subprocesses and manages their lifecycle.

### The Pipeline
1.  **Intent Parsing:** Determines if the user is asking for a specific symbol or a general search.
2.  **Entry Point:** Identifies the file/symbol to start traversing.
3.  **LSP Query:** Sends `textDocument/definition` and `textDocument/references` via JSON-RPC.
4.  **Graph Traversal:** Recursively finds dependencies (up to a configured depth).
5.  **Assembly:** Stitches code snippets into a final prompt.

---

## 🚀 Usage

`skry` operates in two primary modes: **One-Shot** (for quick queries) and **Standby** (for interactive sessions).

### 1. One-Shot Mode
Spins up the LSP, resolves the context, prints the output, and shuts down. Good for CI/CD or single scripts.

```bash
# Explicit symbol resolution
skry generate --file src/main.rs --symbol ServerConfig --intent "Refactor to singleton"

# Natural language search (uses grep/fuzzy finding to locate entry point)
skry generate --search "auth_middleware" --depth 2
```

### 2. Standby Mode
Keeps the LSP server process running in the background. This avoids the high latency of re-indexing large projects (like C++ codebases) for every query.

```bash
# Start the daemon
skry standby

# In another terminal (queries hit the running daemon)
skry query --symbol "User::login"
```

---

## 🛠️ Prerequisites & Configuration

`skry` requires the underlying Language Servers to be installed on your machine.

| Language | Required Binary | Notes |
| :--- | :--- | :--- |
| **Rust** | `rust-analyzer` | Usually installed via rustup. |
| **C++** | `clangd` | Part of LLVM tools. |
| **TypeScript** | `typescript-language-server` | **Required wrapper.** Pure `tsserver` is not supported. |

### Root Discovery
By default, `skry` treats the **current working directory** as the project root (`rootUri`).
*   To override: pass `--root /path/to/project`.

### Configuration (`.skry.toml`)
You can define project-specific rules in a config file:

```toml
[general]
token_budget = 4096
pruning_strategy = "aggressive" # Options: aggressive, bodies, full

[lsp]
# Map extensions to executables
rs = "rust-analyzer"
cpp = "clangd"
ts = "typescript-language-server --stdio"
```

---

## 🔮 Future Vision

*   **Summarization Engine:** Integration with local LLMs to replace large dependency function bodies with semantic summaries.
*   **LSP-Native Flavor:** Direct support for `tsserver` protocol without wrappers (config-driven).
*   **Graph Visualization:** Output a Graphviz/Mermaid diagram of the retrieved context.

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
