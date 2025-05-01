use std::time::Duration;
use chrono::Local;
use tokio::time::sleep;
use relayr::prelude::*;

#[cron("1/1 * * * * *")]
async fn print_message_every_second(_: JobId) -> anyhow::Result<()> {
    println!("🖤 Hello from relayr 0.4.0!");

    sleep(Duration::from_secs(3)).await;
    Err(anyhow::anyhow!("Something went wrong"))?;

    Ok(())
}

#[tokio::main]
async fn main() {
    relayr::set_error_callback(|job_id: JobId, job_name: &'static str, error: anyhow::Error| async move {
        println!("Error in job {:?} ({:?}):", job_name, job_id);
        println!("> {:?}\n", error);
    }).await;

    relayr::run::<Local>().await
}
