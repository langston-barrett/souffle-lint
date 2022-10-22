use clap::{Parser, Subcommand};

/// Lint Soufflé Datalog code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Cmd,

    /// Control interactive features (color, spinners)
    #[arg(long, default_value_t = true)]
    pub interactive: bool,

    /// Print the duration of various steps (for developers)
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Format {
    Default,
    None,
    Oneline,
    Verbose,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Format::Default => write!(f, "default"),
            Format::None => write!(f, "none"),
            Format::Oneline => write!(f, "oneline"),
            Format::Verbose => write!(f, "verbose"),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Default
    }
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Lint files
    Lint {
        /// Additional configuration file(s)
        #[arg(long, value_name = "CONFIG")]
        config: Vec<String>,

        /// Exit with status 0 even if there were warnings
        #[arg(long, default_value_t = false)]
        no_fail: bool,

        /// Format for diagnostic output
        #[arg(long, value_enum, default_value_t = Format::Default, value_name = "FORMAT")]
        format: Format,

        /// Only use this rule
        #[arg(long, default_value = None, value_name = "RULE")]
        only: Option<String>,

        /// Ignore this rule
        #[arg(long, value_name = "RULE")]
        ignore: Vec<String>,

        /// Disable the default (built-in) rules
        #[arg(long, default_value_t = false)]
        no_default_rules: bool,

        /// Enable slow rules
        #[arg(long, default_value_t = false)]
        slow: bool,

        /// Soufflé Datalog file(s) to lint; if empty, parse from stdin
        #[arg(value_name = "DATALOG_SRC")]
        datalog_files: Vec<String>,
    },
    /// Show descriptions of rules
    Info {
        /// Additional configuration file(s)
        #[arg(long, value_name = "CONFIG")]
        config: Vec<String>,

        /// Show help for a specific rule
        #[arg(default_value = None, value_name = "RULE")]
        rule: Option<String>,
    },
    /// Print Soufflé Datalog files as S-expressions
    Sexp {
        /// Soufflé Datalog file(s) to print; if empty, parse from stdin
        #[arg(value_name = "DATALOG_SRC")]
        datalog_files: Vec<String>,
    },
}
