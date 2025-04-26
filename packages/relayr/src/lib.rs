pub mod prelude;

pub use cron::Cron;
pub use inventory;
pub use relayr_macros::cron;

mod cron;

use async_cron_scheduler::*;
use chrono::offset::Local;
use smol::Timer;

pub async fn run() {
    let (mut scheduler, sched_service) = Scheduler::<Local>::launch(Timer::after);

    for cron in inventory::iter::<Cron> {
        let expression = Job::cron(cron.pattern).unwrap();

        let job_within_runtime = |job_id| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on((cron.runnable)(job_id))
            })
        };

        scheduler.insert(expression, job_within_runtime).await;
    }

    sched_service.await;
}
