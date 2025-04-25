# relayr

🏃‍♂️ Effortless delegated cron jobs — scheduled tasks in Rust, made simple.

relayr makes it easy to register cron jobs across your codebase without manual boilerplate. Just annotate functions with a macro, and relayr will auto-discover and schedule them at runtime!

*This crate is a wrapper around [`async-cron-scheduler`](https://crates.io/crates/async-cron-scheduler) to use it in a delegated flavour. If you aren't looking for the delegated way of defining your cron jobs it's probably better for you to use that.*

⸻

## 🖤 Features

✅ Register cron jobs with a simple macro<br>
✅ Fully async<br>
✅ No need to manually wire up each job<br>
✅ Registration happens at compile time thanks to [`inventory`](https://crates.io/crates/inventory)<br>
✅ Validates cron patterns at compile time<br>

⸻

## 📦 Installation

Add to your Cargo.toml:

```toml
relayr = "0.2.0"
```

This will bring in the core scheduler, inventory, and macro support.

⸻

## 🧪 Example

```rust
use relayr::prelude::*;

#[relayr::cron("1/1 * * * * *")]
fn print_every_second(_: JobId) {
    println!("🖤 Hello from relayr!");
}

#[tokio::main]
async fn main() {
    relayr::run().await
}
```

## ✅ That’s it!

When `relayr::run() starts`, it automatically picks up all functions decorated with `#[relayr::cron(...)]` and schedules them.

⸻

## 🧠 How it Works
- You annotate functions with `#[relayr::cron("cron pattern")]`.
- Under the hood, the macro registers your function in a global inventory.
- When `relayr::run()` is called:
  - It spins up a scheduler.
  - It iterates over all discovered Cron items.
  - It inserts them into the scheduler automatically.

No manual wiring. No giant match blocks. Just clean, delegated jobs.

⸻

## 📁 Repo & Contributions

📦 **Crate:** [https://crates.io/crates/relayr](https://crates.io/crates/relayr)<br>
🛠️ **Repo:** [https://github.com/dsplce-co/relayr](https://github.com/dsplce-co/relayr)

PRs welcome! Let’s make scheduled Rust ✨clean and effortless.

⸻

## 📄 License

MIT or Apache-2.0, at your option.
