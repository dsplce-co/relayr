use async_cron_scheduler::JobId;

#[derive(Debug)]
pub struct Cron {
    pub pattern: &'static str,
    pub runnable: fn(JobId),
}

inventory::collect!(Cron);
