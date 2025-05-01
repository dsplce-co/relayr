use std::pin::Pin;
use async_cron_scheduler::JobId;
use anyhow::Result;

#[derive(Debug)]
pub struct Cron {
    pub pattern: &'static str,
    pub runnable: fn(JobId) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    pub name: &'static str,
}

inventory::collect!(Cron);
