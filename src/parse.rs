use anyhow::{Context, Result};
use tree_sitter::{Parser, Tree};

pub fn tree(datalog_source: &str) -> Result<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_souffle::language())
        .context("Failed to set tree-sitter parser language")?;

    parser
        .parse(datalog_source, None)
        .context("Failed to parse Datalog code")
}
