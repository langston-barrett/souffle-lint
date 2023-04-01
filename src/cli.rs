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

#[derive(clap::ValueEnum, Debug, Default, Clone, PartialEq, Eq)]
pub enum Format {
    #[default]
    Default,
    Json,
    None,
    Oneline,
    Verbose,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Format::Default => write!(f, "default"),
            Format::Json => write!(f, "json"),
            Format::None => write!(f, "none"),
            Format::Oneline => write!(f, "oneline"),
            Format::Verbose => write!(f, "verbose"),
        }
    }
}

#[derive(clap::ValueEnum, Debug, Default, Clone, PartialEq, Eq)]
pub enum InfoFormat {
    #[default]
    Oneline,
    Verbose,
}

impl std::fmt::Display for InfoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InfoFormat::Oneline => write!(f, "oneline"),
            InfoFormat::Verbose => write!(f, "verbose"),
        }
    }
}

#[derive(clap::ValueEnum, Debug, Default, Clone, PartialEq, Eq)]
pub enum OnParseError {
    #[default]
    Ignore,
    Warn,
    Error,
}

impl std::fmt::Display for OnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OnParseError::Ignore => write!(f, "ignore"),
            OnParseError::Warn => write!(f, "warn"),
            OnParseError::Error => write!(f, "error"),
        }
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

        /// Behavior on parse errors
        #[arg(long, default_value_t = OnParseError::Ignore, value_name = "CHOICE")]
        on_parse_error: OnParseError,

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

        /// Format
        #[arg(long, value_enum, default_value_t = InfoFormat::Oneline, value_name = "FORMAT")]
        format: InfoFormat,

        /// Show help for a specific rule
        #[arg(default_value = None, value_name = "RULE")]
        rule: Option<String>,
    },
    /// Show the full manual
    #[cfg(feature = "man")]
    Man {},
    /// Print Soufflé Datalog files as S-expressions
    Sexp {
        /// Soufflé Datalog file(s) to print; if empty, parse from stdin
        #[arg(value_name = "DATALOG_SRC")]
        datalog_files: Vec<String>,
    },
}
