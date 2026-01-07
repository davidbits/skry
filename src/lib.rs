//! # skry
//!
//! A deterministic, LSP-powered context assembler for LLMs.
//!
//! `skry` uses the Language Server Protocol (LSP) to navigate code the way a compiler does,
//! building context prompts by traversing the Abstract Syntax Tree (AST), resolving symbol
//! definitions, and walking dependency graphs.

pub mod assembly;
pub mod config;
pub mod lsp;
pub mod pipeline;
pub mod standby;

pub use config::Config;
