> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# relayr

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![crates.io Downloads](https://img.shields.io/crates/d/relayr?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/relayr)
[![crates.io Size](https://img.shields.io/crates/size/relayr?style=for-the-badge)](https://crates.io/crates/relayr)
[![docs.rs](https://img.shields.io/docsrs/relayr?style=for-the-badge)](https://docs.rs/relayr)
[![License](https://img.shields.io/crates/l/relayr.svg?style=for-the-badge)](https://crates.io/crates/relayr)
[![crates.io](https://img.shields.io/crates/v/relayr?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/relayr)

🏃‍♂️ Effortless delegated cron jobs — scheduled tasks in Rust, made simple.

`relayr` lets you register cron jobs across your codebase without the manual boilerplate. Annotate a function with a macro and it gets auto-discovered and scheduled at runtime — no central list to keep in sync, no giant match block to grow.

_This crate is a wrapper around [`async-cron-scheduler`](https://crates.io/crates/async-cron-scheduler) to use it in a delegated flavour. If you're not after the delegated way of defining your cron jobs, you're probably better off using that directly._

⸻

## 🖤 Features

- **One macro is the whole registration.** Slap `#[cron("...")]` on a function and you're done — define the job right where it lives, not in some central registry you'll forget to update.
- **No manual wiring.** Jobs are discovered at compile time via [`inventory`](https://crates.io/crates/inventory), so there's no match block and no "ah, I forgot to add it to the list".
- **Typos fail the build, not prod.** Literal cron patterns are validated at compile time — a malformed schedule won't sneak past you to surface at 3am.
- **Fully async.** Built on top of `tokio`; your jobs are plain `async fn`s.
- **Schedules from the environment.** Point a job at an env var instead of a literal and change its cadence without a recompile.
- **One callback catches everything.** Register a single error handler and whatever any job throws lands there.
- **Timezone-aware.** Run under `Local`, `Utc`, or any `chrono-tz` zone.

⸻

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
- [🧪 Usage](#-usage)
  - [Define a job](#define-a-job)
  - [Schedule from an environment variable](#schedule-from-an-environment-variable)
  - [Handle errors with a callback](#handle-errors-with-a-callback)
  - [How it works](#how-it-works)
- [🛠️ Requirements](#%EF%B8%8F-requirements)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

Add it with cargo:

```bash
cargo add relayr
```

Or drop it into your `Cargo.toml` by hand:

```toml
relayr = "0.4"
```

That pulls in the core scheduler, `inventory`, and the macro support — nothing else to wire up.

⸻

## 🧪 Usage

### Define a job

Annotate any `async fn` that takes a `JobId` and returns `anyhow::Result<()>`. The cron pattern accepts an optional leading seconds field, so `1/1 * * * * *` means "every second":

```rust
use relayr::prelude::*;
use chrono::Local;

#[cron("1/1 * * * * *")]
async fn print_every_second(_: JobId) -> anyhow::Result<()> {
    println!("🖤 Hello from relayr!");
    Ok(())
}

#[tokio::main]
async fn main() {
    relayr::run::<Local>().await
}
```

That's it. When `relayr::run()` starts it picks up every function decorated with `#[cron(...)]` and schedules it — no registration step, no boilerplate. Swap `Local` for `Utc` or any `chrono-tz` zone to run on a different timezone.

### Schedule from an environment variable

Sometimes you don't want the cadence baked into the binary — staging should sweep hourly, prod every five minutes, that sort of thing. Pass a bare identifier (an env var name) instead of a string literal, and `relayr` resolves the pattern at runtime:

```rust
use relayr::prelude::*;

// reads the cron pattern from the CLEANUP_SCHEDULE environment variable at startup
#[cron(CLEANUP_SCHEDULE)]
async fn cleanup(_: JobId) -> anyhow::Result<()> {
    // ...
    Ok(())
}
```

One caveat worth knowing: literal patterns are validated at compile time, but env-var ones can only be checked once the value is read — so if `CLEANUP_SCHEDULE` is unset or malformed when `relayr::run()` starts, it'll panic rather than silently skip the job.

### Handle errors with a callback

Jobs return a `Result`, and each one runs in its own task — so a single job blowing up never takes the scheduler (or the others) down with it. Register one callback before `run()` and every error lands there, tagged with the job's id and name:

```rust
use relayr::prelude::*;
use chrono::Local;

#[cron("1/1 * * * * *")]
async fn flaky(_: JobId) -> anyhow::Result<()> {
    anyhow::bail!("something went wrong");
}

#[tokio::main]
async fn main() {
    relayr::set_error_callback(|job_id: JobId, job_name: &'static str, error: anyhow::Error| async move {
        eprintln!("job {job_name:?} ({job_id:?}) failed: {error:?}");
    })
    .await;

    relayr::run::<Local>().await
}
```

`job_name` is the function's name, so you get readable logs for free without naming anything twice.

### How it works

- You annotate functions with `#[cron("cron pattern")]` (or `#[cron(ENV_VAR_NAME)]`).
- Under the hood the macro registers your function in a global `inventory`.
- When `relayr::run::<Tz>()` is called, it:
  - spins up a scheduler for the timezone you pass,
  - iterates over every discovered job,
  - resolves each pattern (literal or from the environment) and inserts it.

No manual wiring. No giant match blocks. Just clean, delegated jobs.

⸻

## 🛠️ Requirements

- **Rust (2024 edition)**: `relayr` uses the 2024 edition, so a reasonably recent toolchain is needed.
- **An async runtime**: jobs run on `tokio`, which comes bundled as a dependency — you just need `#[tokio::main]` (or your own runtime) at the entry point.

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/relayr](https://github.com/dsplce-co/relayr)<br>
📦 **Crate**: [https://crates.io/crates/relayr](https://crates.io/crates/relayr)

PRs welcome! Let's make scheduled Rust ✨clean and effortless.

⸻

## 📄 License

MIT or Apache-2.0, at your option.
