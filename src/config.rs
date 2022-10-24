use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Config {
    #[serde(default)] // Empty
    pub ignore: Vec<String>,
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Rule {
    pub name: String,
    pub short: String,
    #[serde(default)] // None
    pub long: Option<String>,
    #[serde(default)] // false
    pub captures: bool,
    #[serde(default)] // false
    pub slow: bool,
    /// Should be non-empty
    pub queries: Vec<String>,
    pub examples: Vec<Example>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Example {
    pub before: String,
    pub after: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ignore: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn merge(configs: Vec<Config>) -> Config {
        let mut c = Config::new();
        for config in &configs {
            c.rules.extend(config.rules.clone());
        }
        for config in &configs {
            c.ignore.extend(config.ignore.clone());
        }
        c
    }
}

const DEFAULT_CONFIG: &str = include_str!("../config/default.yml");

pub fn default() -> Config {
    let config: Config =
        serde_yaml::from_str(DEFAULT_CONFIG).expect("Failed to parse default configuration!");
    config
}
