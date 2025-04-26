use std::pin::Pin;
use async_cron_scheduler::JobId;

#[derive(Debug)]
pub struct Cron {
    pub pattern: &'static str,
    pub runnable: fn(JobId) -> Pin<Box<dyn Future<Output = ()> + Send>>,
}

inventory::collect!(Cron);
