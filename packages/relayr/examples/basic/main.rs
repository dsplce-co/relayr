use relayr::prelude::*;

#[relayr::cron("1/1 * * * * *")]
async fn print_message_every_second(_: JobId) {
    println!("🖤 Hello from relayr 0.3.0!");
}

#[tokio::main]
async fn main() {
    relayr::run().await
}
