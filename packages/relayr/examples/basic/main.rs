use relayr::prelude::*;

#[relayr::cron("1/1 * * * * *")]
fn print_message_every_second(_: JobId) {
    println!("🖤 Hello from relayr!");
}

#[tokio::main]
async fn main() {
    relayr::run().await
}
