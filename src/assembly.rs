//! Context assembly and token budgeting
//!
//! Stitches code snippets into final prompts while respecting token limits.

use crate::config::PruningStrategy;
use crate::pipeline::DependencyGraph;
use anyhow::Result;
use tracing::debug;

/// Context assembler that builds prompts from dependency graphs
pub struct Assembler {
    token_budget: usize,
    pruning_strategy: PruningStrategy,
}

impl Assembler {
    /// Create a new assembler with a token budget and pruning strategy
    pub fn new(token_budget: usize, pruning_strategy: PruningStrategy) -> Self {
        Self {
            token_budget,
            pruning_strategy,
        }
    }

    /// Assemble a context prompt from a dependency graph
    pub fn assemble(&self, graph: &DependencyGraph, intent: &str) -> Result<String> {
        debug!(
            "Assembling context (budget: {} tokens, strategy: {:?})",
            self.token_budget, self.pruning_strategy
        );

        let mut output = String::new();

        // Add the user's intent/query
        output.push_str("# User Intent\n\n");
        output.push_str(intent);
        output.push_str("\n\n");

        // Add context information
        output.push_str("# Context\n\n");
        output.push_str(
            "The following code snippets were retrieved by traversing the dependency graph.\n",
        );
        output.push_str(&format!("Graph depth: {}\n", graph.depth));
        output.push_str(&format!("Total nodes: {}\n\n", graph.nodes.len()));

        // Add code snippets
        output.push_str("# Code Snippets\n\n");

        for (i, node) in graph.nodes.iter().enumerate() {
            output.push_str(&format!("## {} - {}\n\n", i + 1, node.symbol));
            output.push_str(&format!("Location: {}\n\n", node.location.as_str()));
            output.push_str("```\n");

            // Apply pruning strategy
            let content = self.prune_content(&node.content);
            output.push_str(&content);

            output.push_str("\n```\n\n");

            // Check if we're approaching the token budget
            if self.estimate_tokens(&output) > self.token_budget {
                debug!("Token budget exceeded, truncating output");
                output.push_str("... [output truncated due to token budget] ...\n");
                break;
            }
        }

        Ok(output)
    }

    /// Prune content according to the configured strategy
    fn prune_content(&self, content: &str) -> String {
        match self.pruning_strategy {
            PruningStrategy::Aggressive => {
                // TODO: Implement proper language-specific pruning logic
                // Requires dynamic logic detection based on file extension/language
                // Should use AST parsing or language-specific patterns for each supported language
                // For now, returns full content as placeholder
                content.to_string()
            }
            PruningStrategy::Bodies => {
                // TODO: Implement proper function body removal
                // Should parse the AST and extract only signatures
                // For now, returns full content as placeholder
                content.to_string()
            }
            PruningStrategy::Full => {
                // Keep everything
                content.to_string()
            }
        }
    }

    /// Estimate the number of tokens in the text
    /// Uses a rough approximation: 1 token ≈ 4 characters
    fn estimate_tokens(&self, text: &str) -> usize {
        text.len() / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{DependencyGraph, DependencyNode};
    use lsp_types::Uri;
    use std::str::FromStr;

    #[test]
    fn test_assembler_creation() {
        let assembler = Assembler::new(4096, PruningStrategy::Aggressive);
        assert_eq!(assembler.token_budget, 4096);
    }

    #[test]
    fn test_token_estimation() {
        let assembler = Assembler::new(4096, PruningStrategy::Full);
        let text = "a".repeat(400);
        assert_eq!(assembler.estimate_tokens(&text), 100);
    }

    #[test]
    fn test_assemble_empty_graph() {
        let assembler = Assembler::new(4096, PruningStrategy::Full);
        let graph = DependencyGraph {
            nodes: Vec::new(),
            depth: 0,
        };

        let result = assembler.assemble(&graph, "Test intent").unwrap();
        assert!(result.contains("Test intent"));
        assert!(result.contains("Total nodes: 0"));
    }

    #[test]
    fn test_assemble_with_nodes() {
        let assembler = Assembler::new(4096, PruningStrategy::Full);
        let graph = DependencyGraph {
            nodes: vec![DependencyNode {
                symbol: "test_function".to_string(),
                location: Uri::from_str("file:///test.rs").unwrap(),
                content: "fn test_function() {}".to_string(),
                references: Vec::new(),
            }],
            depth: 1,
        };

        let result = assembler.assemble(&graph, "Find test function").unwrap();
        assert!(result.contains("test_function"));
        assert!(result.contains("file:///test.rs"));
    }
}
