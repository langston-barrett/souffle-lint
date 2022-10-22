use tracing::info_span;

use super::config::Rule;

#[allow(clippy::module_name_repetitions)]
pub struct RuleFilter {
    pub only: Option<String>,
    pub ignore: Vec<String>,
    pub slow: bool,
}

pub fn filter(rules: Vec<Rule>, filt: &RuleFilter) -> Vec<Rule> {
    let span = info_span!("filter_rules");
    let _enter = span.enter();
    rules
        .into_iter()
        .filter(|r| {
            if let Some(ref name) = filt.only {
                &r.name == name
            } else {
                for pfx in &filt.ignore {
                    if r.name.starts_with(pfx) {
                        return false;
                    }
                }
                filt.slow || !r.slow
            }
        })
        .collect()
}
