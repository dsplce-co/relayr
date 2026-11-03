use std::{fmt::Display, pin::Pin};
use async_cron_scheduler::JobId;
use anyhow::Result;

#[derive(Debug)]
pub struct Cron {
    pub pattern: CronPattern,
    pub runnable: fn(JobId) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    pub name: &'static str,
}

#[derive(Debug)]
pub enum CronPattern {
    Lit(&'static str),
    EnvVar(&'static str)
}

impl CronPattern {
    pub fn resolve(&self) -> Option<Vec<String>> {
        match self {
            Self::Lit(literal_pattern) => Some(split_patterns(literal_pattern)),
            Self::EnvVar(variable_name) => std::env::var(variable_name).ok().map(|value| split_patterns(&value)),
        }
    }
}

pub(crate) fn split_patterns(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut patterns = Vec::new();
    let mut current = String::new();

    for (index, &character) in chars.iter().enumerate() {
        let touches_whitespace = index.checked_sub(1).is_none_or(|i| chars[i].is_whitespace())
            || chars.get(index + 1).is_none_or(|next| next.is_whitespace());

        if character == ',' && touches_whitespace {
            patterns.push(std::mem::take(&mut current));
        } else {
            current.push(character);
        }
    }

    patterns.push(current);

    patterns
        .into_iter()
        .map(|pattern| pattern.trim().to_string())
        .filter(|pattern| !pattern.is_empty())
        .collect()
}

impl Display for CronPattern {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvVar(variable_name) => write!(formatter, "cron pattern from environment variable {variable_name}"),
            Self::Lit(literal_pattern) => write!(formatter, "cron pattern from the literal {literal_pattern}")
        }
    }
}

inventory::collect!(Cron);
