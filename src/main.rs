use std::fs;
use std::io::{Read, Write};
use std::process;

use anyhow::{Context, Result};
use atty::Stream;
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

mod cli;
mod config;
mod filter;
mod interactive;
mod lint;
mod parse;

#[cfg(feature = "man")]
mod man;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAIL_LINT: i32 = 1;
const EXIT_OTHER: i32 = 2;

fn read_datalog_file(datalog_file: &str) -> Result<String> {
    let span = info_span!("read_datalog_file");
    let _enter = span.enter();
    fs::read_to_string(datalog_file)
        .with_context(|| format!("Failed to read Datalog file {}", datalog_file))
}

fn parse_datalog(datalog_code: &str) -> Result<tree_sitter::Tree> {
    let span = info_span!("parse_datalog");
    let _enter = span.enter();
    parse::tree(datalog_code)
}

fn read_config(path: &str) -> Result<config::Config> {
    let span = info_span!("read_config");
    let _enter = span.enter();
    let config_text =
        fs::read_to_string(path).with_context(|| format!("Failed to read config file {}", path))?;
    Ok(serde_yaml::from_str(&config_text)?)
}

fn merge_configs(configs: &Vec<String>, no_default_rules: bool) -> Result<config::Config> {
    let mut all_configs = if no_default_rules {
        Vec::new()
    } else {
        vec![config::default()]
    };
    all_configs.reserve(configs.len());
    for config_path in configs {
        all_configs.push(read_config(config_path)?);
    }
    Ok(config::Config::merge(all_configs))
}

struct DatalogSrc(String);

fn warn_on_parse_errors(node: &tree_sitter::Node) {
    if node.has_error() {
        eprintln!("
[WARN] souffle-lint could not parse part of this file.

You can see which part of the file caused the error with the `sexp` subcommand.

This may be due to a bug in the tree-sitter-souffle parser. You can view the
known bugs here:

    https://github.com/langston-barrett/tree-sitter-souffle/issues?q=is%3Aissue+is%3Aopen+label%3Abug

If this doesn't look like one of those, please file an issue:

    https://github.com/langston-barrett/tree-sitter-souffle/issues/new
");
    }
}

fn lint_src(
    language: tree_sitter::Language,
    datalog_file: &str,
    datalog_source: DatalogSrc,
    rules: &Vec<config::Rule>,
    format: &cli::Format,
    interactive: &interactive::Interactive,
    no_fail: bool,
) -> Result<bool> {
    let DatalogSrc(src) = datalog_source;
    let tree = parse_datalog(&src)?;
    warn_on_parse_errors(&tree.root_node());

    let mut fail = false;
    let diagnostics: Vec<Result<Vec<_>>> = rules
        .par_iter()
        .map(|r| lint::query(language, datalog_file, &src, &tree, r))
        .collect();
    let span = info_span!("print_diagnostics");
    let _enter = span.enter();
    // https://nnethercote.github.io/perf-book/io.html#locking
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for diags in diagnostics {
        for diag in diags? {
            if !no_fail {
                fail = true;
            }
            diag.display(&mut lock, format, interactive)
                .context("Failed to print diagnostics")?;
        }
    }
    Ok(fail)
}

// TODO(lb): Check for duplicated rule IDs
// TODO(lb): Warn on punctuation in short
// TODO(lb): Warn on lack of examples
// TODO(lb): Warn on empty short description
// TODO(lb): Warn on empty long description
fn check_config(config: &config::Config) {
    let mut err = false;
    for rule in &config.rules {
        if rule.queries.is_empty() {
            err = true;
            eprintln!("Error: No queries for rule {}", rule.name);
        }
    }
    if err {
        process::exit(EXIT_OTHER);
    }
}

fn lint(
    configs: &Vec<String>,
    no_default_rules: bool,
    datalog_files: &Vec<String>,
    format: &cli::Format,
    mut filt: filter::RuleFilter,
    interactive: &interactive::Interactive,
    no_fail: bool,
) -> Result<i32> {
    let config = merge_configs(configs, no_default_rules)?;
    check_config(&config);
    filt.ignore.extend(config.ignore);
    let rules = filter::filter(config.rules, &filt);
    if rules.is_empty() {
        eprintln!("No rules specified! Returning immediately without parsing.");
        return Ok(EXIT_OTHER);
    }

    let language = tree_sitter_souffle::language();

    // TODO(lb): parallelize this loop?
    let mut exit = EXIT_SUCCESS;
    if datalog_files.is_empty()
        && lint_src(
            language,
            "<stdin>",
            DatalogSrc(stdin_string()?),
            &rules,
            format,
            interactive,
            no_fail,
        )
        .context("Failed to lint stdin")?
    {
        exit = EXIT_FAIL_LINT;
    }
    for datalog_file in datalog_files {
        if lint_src(
            language,
            datalog_file,
            DatalogSrc(read_datalog_file(datalog_file)?),
            &rules,
            format,
            interactive,
            no_fail,
        )
        .with_context(|| format!("Failed to lint file {}", datalog_file))?
        {
            exit = EXIT_FAIL_LINT;
        }
    }
    Ok(exit)
}

fn info_rule(
    rule: &config::Rule,
    w: &mut impl Write,
    interactive: &interactive::Interactive,
) -> Result<()> {
    writeln!(w)?;
    writeln!(
        w,
        "{}: {}",
        if From::from(interactive) {
            rule.name.bold()
        } else {
            rule.name.as_str().into()
        },
        rule.short
    )?;
    if let Some(l) = &rule.long {
        writeln!(w, "\n{}", l)?;
    }
    for ex in &rule.examples {
        writeln!(w, "Example:")?;
        let before_lines: Vec<_> = ex.before.lines().collect();
        let after_lines: Vec<_> = ex.after.lines().collect();
        if before_lines.len() > 1 || after_lines.len() > 1 {
            writeln!(w, "\n  Before:\n")?;
            for line in before_lines {
                writeln!(w, "    {}", line)?;
            }
            writeln!(w, "\n  After:\n")?;
            for line in after_lines {
                writeln!(w, "    {}", line)?;
            }
        } else {
            write!(w, "  Before: {}", ex.before)?;
            write!(w, "  After:  {}", ex.after)?;
        }
    }
    Ok(())
}

fn info(
    configs: &Vec<String>,
    rule_name: &Option<String>,
    interactive: &interactive::Interactive,
) -> Result<i32> {
    let config = merge_configs(configs, false)?;
    // https://nnethercote.github.io/perf-book/io.html#locking
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for rule in config.rules {
        match rule_name {
            Some(name) => {
                if &rule.name == name {
                    info_rule(&rule, &mut lock, interactive)?;
                }
            }
            None => info_rule(&rule, &mut lock, interactive)?,
        }
    }
    Ok(EXIT_SUCCESS)
}

fn print_as_sexp(datalog_source: &str) -> Result<()> {
    let tree = parse_datalog(datalog_source)?;
    println!("{}", tree.root_node().to_sexp());
    Ok(())
}

fn stdin_string() -> Result<String> {
    let mut stdin_str: String = String::new();
    std::io::stdin().read_to_string(&mut stdin_str)?;
    Ok(stdin_str)
}

fn sexp(datalog_files: &Vec<String>) -> Result<i32> {
    if datalog_files.is_empty() {
        print_as_sexp(&stdin_string()?)?;
    }
    for datalog_file in datalog_files {
        print_as_sexp(&read_datalog_file(datalog_file)?)?;
    }
    Ok(EXIT_SUCCESS)
}

fn main() -> Result<()> {
    let mut args = cli::Args::parse();
    args.interactive = args.interactive && atty::is(Stream::Stdout);

    if args.trace {
        // TODO(lb): Make this less verbose, drop time
        tracing_subscriber::fmt::fmt()
            .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
            .with_target(false)
            .with_level(false)
            .init();
    }
    let span = info_span!("main");
    let _enter = span.enter();

    process::exit(match args.command {
        cli::Cmd::Lint {
            config,
            datalog_files,
            format,
            only,
            ignore,
            slow,
            no_default_rules,
            no_fail,
        } => lint(
            &config,
            no_default_rules,
            &datalog_files,
            &format,
            filter::RuleFilter { only, ignore, slow },
            &From::from(args.interactive),
            no_fail,
        )?,
        cli::Cmd::Info { config, rule } => info(&config, &rule, &From::from(args.interactive))?,
        #[cfg(feature = "man")]
        cli::Cmd::Man {} => {
            if man::man()?.success() {
                EXIT_SUCCESS
            } else {
                EXIT_OTHER
            }
        }
        cli::Cmd::Sexp { datalog_files } => sexp(&datalog_files)?,
    })
}
