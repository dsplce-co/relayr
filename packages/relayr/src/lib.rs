pub mod prelude;
pub use cron::Cron;
pub use relayr_macros::cron;

mod cron;

use async_cron_scheduler::*;
use chrono::offset::Local;
use smol::Timer;

pub async fn run() {
    let (mut scheduler, sched_service) = Scheduler::<Local>::launch(Timer::after);

    for cron in inventory::iter::<Cron> {
        let job = Job::cron(cron.pattern).unwrap();
        scheduler.insert(job, cron.runnable).await;
    }

    sched_service.await;
}
