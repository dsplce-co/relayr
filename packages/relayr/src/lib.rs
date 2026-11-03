pub mod prelude;

pub use cron::Cron;
pub use cron::CronPattern;
pub use inventory;
pub use relayr_macros::cron;
use std::pin::Pin;

mod cron;

use async_cron_scheduler::*;
use chrono::TimeZone;
use lazy_static::lazy_static;
use smol::Timer;
use tokio::sync::Mutex;

type ErrorCallback =
    dyn FnMut(JobId, &'static str, anyhow::Error) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync;

lazy_static! {
    static ref ERROR_CALLBACK: Mutex<Option<Box<ErrorCallback>>> = Mutex::new(None);
}

pub async fn set_error_callback<F, Fut>(mut callback: F)
where
    F: FnMut(JobId, &'static str, anyhow::Error) -> Fut + std::marker::Send + std::marker::Sync + 'static,
    Fut: Future<Output = ()> + std::marker::Send + 'static,
{
    let boxed_cb: Box<ErrorCallback> =
        Box::new(move |job_id: JobId, job_name: &'static str, error: anyhow::Error| Box::pin(callback(job_id, job_name, error)));

    *ERROR_CALLBACK.lock().await = Some(boxed_cb);
}

pub async fn run<Tz>() where
    Tz: TimeZoneExt + 'static,
    <Tz as TimeZone>::Offset: Send + Sync + 'static,
{
    let (mut scheduler, sched_service) = Scheduler::<Tz>::launch(Timer::after);

    for cron in inventory::iter::<Cron> {
        let cron_pattern = &cron.pattern;
        let cron_expressions = cron_pattern.resolve().unwrap_or_else(|| panic!("Unable to resolve {cron_pattern}"));

        if cron_expressions.is_empty() {
            panic!("No cron expression found in {cron_pattern}");
        }

        for cron_expression in cron_expressions {
            let expression = Job::cron(cron_expression.as_str())
                .unwrap_or_else(|err| panic!("Invalid cron expression {cron_expression:?} in {cron_pattern}: {err}"));

            let job_within_runtime = move |job_id| {
                tokio::spawn(async move {
                    if let Err(err) = (cron.runnable)(job_id).await {
                        let mut error_callback = ERROR_CALLBACK.lock().await;

                        if let Some(ref mut callback) = *error_callback {
                            callback(job_id, cron.name, err).await;
                        }
                    }
                });
            };

            scheduler.insert(expression, job_within_runtime).await;
        }
    }

    sched_service.await;
}
