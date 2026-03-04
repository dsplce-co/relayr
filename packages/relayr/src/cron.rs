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
    Literal(&'static str),
    EnvironmentVariable(&'static str)
}

impl CronPattern {
    pub fn resolve(&self) -> Option<String> {
        match self {
            Self::Literal(literal_pattern) => Some(literal_pattern.to_string()),
            Self::EnvironmentVariable(variable_name) => std::env::var(variable_name).ok()
        }
    }
}

impl Display for CronPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvironmentVariable(variable_name) => write!(f, "cron pattern from environment variable ${variable_name}"),
            Self::Literal(literal_pattern) => write!(f, "cron pattern from the literal {literal_pattern}")
        }
    }
}

inventory::collect!(Cron);
