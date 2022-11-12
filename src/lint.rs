use std::io::Write;

use anyhow::{Context, Result};
use colored::Colorize;
use serde::Serialize;
use tracing::info_span;
use tree_sitter::{Language, Node, Query, QueryCursor, Tree};

use super::cli::Format;
use super::config::Rule;
use super::interactive::Interactive;

// TODO(lb): Maybe use ariadne or miette (internals) for formatting errors?

#[derive(Serialize, Clone, Eq, PartialEq, Debug)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl From<tree_sitter::Point> for Point {
    fn from(pt: tree_sitter::Point) -> Self {
        Point {
            row: pt.row,
            column: pt.column,
        }
    }
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug)]
pub struct Fragment<'a> {
    // NOTE: Can't have a Node<'a> field because Send is not implemented for *Tree
    pub node_text: &'a str,
    pub start: Point,
    pub end: Point,
    pub context: Option<Box<Fragment<'a>>>,
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug)]
pub struct Diagnostic<'a> {
    pub fragments: Vec<Fragment<'a>>,
    pub rule: &'a Rule,
    pub source: &'a str,
    pub source_file: &'a str,
}

fn lpad(s: &str, c: char, n: usize) -> String {
    let mut left = String::new();
    while left.len() + s.len() < n {
        left.push(c);
    }
    left + s
}

fn rpad(mut s: String, c: char, n: usize) -> String {
    while s.len() < n {
        s.push(c);
    }
    s
}

fn hint(color: bool) -> colored::ColoredString {
    if color {
        "hint".italic()
    } else {
        "hint".into()
    }
}

fn _note(color: bool) -> colored::ColoredString {
    if color {
        "note".bold().blue()
    } else {
        "note".into()
    }
}

fn warn(color: bool) -> colored::ColoredString {
    if color {
        "warn".bold().yellow()
    } else {
        "warn".into()
    }
}

// TODO(lb): Search through context and colorize node
impl<'a> Fragment<'a> {
    fn ctx_len(&self) -> usize {
        if let Some(ctx) = &self.context {
            ctx.node_text.len()
        } else {
            0
        }
    }

    fn display_pos(&self, source_filename: &str) -> String {
        format!(
            "{}:{}:{}-{}:{}",
            source_filename, self.start.row, self.start.column, self.end.row, self.end.column,
        )
    }

    fn display_oneline(&self, w: &mut impl Write, source_filename: &str) -> Result<()> {
        let ctx_len = self.ctx_len();
        let pos = self.display_pos(source_filename);
        assert!(ctx_len == 0 || ctx_len >= self.node_text.len());
        // writeln!(w,"{}  at {}", rpad(self.node_text.to_string(), ' ', ctx_len), pos);
        writeln!(
            w,
            "at {}: {}",
            pos,
            rpad(self.node_text.to_string(), ' ', ctx_len)
        )?;
        if let Some(ctx) = &self.context {
            // writeln!(w,"{}  in {}", ctx.node_text, ctx.display_pos(source_filename));
            writeln!(
                w,
                "in {}: {}",
                ctx.display_pos(source_filename),
                ctx.node_text
            )?;
        }
        Ok(())
    }

    fn display_multiline(&self, w: &mut impl Write, source_filename: &str) -> Result<()> {
        writeln!(w, "at {}:\n", self.display_pos(source_filename))?;
        let mut first = true;
        for line in self.node_text.lines() {
            let mut l = line.to_string();
            if first {
                first = false;
                l = lpad(line, ' ', self.start.column + line.len());
            }
            writeln!(w, "  {}", l)?;
        }
        writeln!(w)?;
        if let Some(ctx) = &self.context {
            writeln!(w, "in {}:\n", ctx.display_pos(source_filename))?;
            for line in ctx.node_text.lines() {
                writeln!(w, "  {}", line)?;
            }
            writeln!(w)?;
        }
        Ok(())
    }

    fn can_display_oneline(&self, source_filename: &str) -> bool {
        let pos = format!(
            "at {}:{}:{}-{}:{}",
            source_filename, self.start.row, self.start.column, self.end.row, self.end.column,
        );
        let ctx_len = self.ctx_len();
        let too_long = pos.len() + self.node_text.len() + " ".len() > 80 || ctx_len > 80;
        let newlines = self.node_text.contains('\n')
            || if let Some(ctx) = &self.context {
                ctx.node_text.contains('\n')
            } else {
                false
            };
        !too_long && !newlines
    }

    pub fn display(&self, w: &mut impl Write, source_filename: &str, verbose: bool) -> Result<()> {
        if !verbose && self.can_display_oneline(source_filename) {
            self.display_oneline(w, source_filename)
        } else {
            self.display_multiline(w, source_filename)
        }
    }
}

impl<'a> Diagnostic<'a> {
    fn display_default(&self, w: &mut impl Write, color: bool, verbose: bool) -> Result<()> {
        writeln!(w, "{}[{}] {}", warn(color), self.rule.name, self.rule.short)?;

        for frag in &self.fragments {
            frag.display(w, self.source_file, verbose)?;
        }

        if verbose {
            writeln!(
                w,
                "{}: Run `souffle-lint info {}` for more information.",
                hint(color),
                self.rule.name,
            )?;
        }
        Ok(())
    }

    fn display_oneline(&self, w: &mut impl Write, color: bool) -> Result<()> {
        write!(
            w,
            "{}[{}] {}:",
            if color {
                "warn".bold().yellow()
            } else {
                "warn".into()
            },
            self.rule.name,
            self.source_file,
        )?;
        let mut first = true;
        for frag in &self.fragments {
            if first {
                write!(
                    w,
                    "{}:{}-{}:{}",
                    frag.start.row, frag.start.column, frag.end.row, frag.end.column,
                )?;
                first = false;
            } else {
                write!(
                    w,
                    ",{}:{}-{}:{}",
                    frag.start.row, frag.start.column, frag.end.row, frag.end.column,
                )?;
            }
        }
        writeln!(w)?;
        Ok(())
    }

    pub fn display(
        &self,
        w: &mut impl Write,
        format: &Format,
        interactive: &Interactive,
    ) -> Result<()> {
        let span = info_span!("display");
        let _enter = span.enter();
        match format {
            Format::Default => self.display_default(w, From::from(interactive), false),
            Format::Json => {
                let mut diag = self.clone();
                diag.source = ""; // don't output whole source file
                Ok(writeln!(w, "{}", serde_json::to_string(&diag)?)?)
            }
            Format::None => Ok(()),
            Format::Oneline => self.display_oneline(w, From::from(interactive)),
            Format::Verbose => self.display_default(w, From::from(interactive), true),
        }
    }
}

// The context is the largest surrounding node that is textually different from
// the start node but smaller than the whole program, i.e., the surrounding
// top-level declaration.
fn context_node(start: Node) -> Option<Node> {
    let mut node = start;
    loop {
        node = match node.parent() {
            None => break,
            Some(p) => {
                if p.kind() == "program" {
                    break;
                }
                p
            }
        };
        if node.range() != start.range() {
            return Some(node);
        }
    }
    None
}

fn context_for_node<'a>(source: &'a str, start: Node<'a>) -> Result<Option<Fragment>> {
    match context_node(start) {
        None => Ok(None),
        Some(ancestor) => {
            let ancestor_text = ancestor
                .utf8_text(source.as_bytes())
                .context("Failed to retrieve node text")?;
            Ok(Some(Fragment {
                node_text: ancestor_text,
                start: Point::from(ancestor.start_position()),
                end: Point::from(ancestor.end_position()),
                context: None,
            }))
        }
    }
}

pub fn query<'a>(
    language: Language,
    source_file: &'a str,
    source: &'a str,
    tree: &'a Tree,
    rule: &'a Rule,
) -> Result<Vec<Diagnostic<'a>>> {
    let span = info_span!("query", rule.name);
    // let span = info_span!("query");
    let _enter = span.enter();
    let mut diags = Vec::new();
    for query_text in &rule.queries {
        // If the rule has significant captures, use those. Otherwise, add
        // a default one (@capture).
        let final_text = if rule.captures {
            query_text.clone()
        } else {
            query_text.clone() + "@capture"
        };

        let q = info_span!("new query")
            .in_scope(|| Query::new(language, &final_text))
            .with_context(|| format!("Failed to parse query {}", &final_text))?;
        let mut qc = QueryCursor::new();
        let matches = qc.matches(&q, tree.root_node(), source.as_bytes());
        for m in matches {
            assert!(!m.captures.is_empty());

            // If the rule has significant captures, use those. Otherwise, use
            // the default one (@capture) that was added above.
            let captures = if rule.captures {
                m.captures.iter().collect::<Vec<_>>()
            } else {
                vec![m.captures.iter().collect::<Vec<_>>()[0]]
            };

            let mut fragments = Vec::new();
            for c in captures {
                let node_text = c
                    .node
                    .utf8_text(source.as_bytes())
                    .context("Failed to retrieve node text")?;

                fragments.push(Fragment {
                    node_text,
                    start: Point::from(c.node.start_position()),
                    end: Point::from(c.node.end_position()),
                    context: context_for_node(source, c.node)
                        .context("Failed to retrieve context for node")?
                        .map(Box::new),
                });
            }
            diags.push(Diagnostic {
                fragments,
                rule,
                source,
                source_file,
            });
        }
    }
    Ok(diags)
}

#[cfg(test)]
mod tests {
    use super::super::config;
    use super::super::parse;
    use super::*;

    // #[test]
    // fn test_parse_config() {
    //     let config_text = include_str!("../conf.yaml");
    //     println!("{}", config_text);
    //     let _c: config::Config = serde_yaml::from_str(&config_text).unwrap();
    // }

    #[test]
    fn test_examples() -> Result<()> {
        let language = tree_sitter_souffle::language();
        for rule in config::default().rules {
            for ex in &rule.examples {
                let before_tree = parse::tree(&ex.before)?;
                let before_diags = query(language, "before.dl", &ex.before, &before_tree, &rule)?;
                assert!(!before_diags.is_empty());

                let after_tree = parse::tree(&ex.after)?;
                let after_diags = query(language, "after.dl", &ex.after, &after_tree, &rule)?;
                assert!(after_diags.is_empty());
            }
        }
        Ok(())
    }
}
