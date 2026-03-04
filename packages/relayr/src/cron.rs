use std::pin::Pin;
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

inventory::collect!(Cron);
