use clap::Parser;
use exponential_backoff::Backoff;
use std::{process::Command, thread::sleep, time::Duration};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error: {0}")]
    Error(String),
}

#[derive(Parser)]
#[command(
    author,
    version,
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Args {
    #[clap(long, default_value = "3")]
    retries: u32,

    /// minimum duration in tenths of a second
    #[clap(long, default_value = "10")]
    min_duration: u64,

    /// maximum duration in tenths of a second
    #[clap(long)]
    max_duration: Option<u64>,

    /// amount of randomization to add to the backoff
    #[clap(long, default_value = "0.3")]
    jitter: f32,

    /// backoff factor
    #[clap(long, default_value = "2")]
    factor: u32,

    #[clap(required = true)]
    command: Vec<String>,
}

fn main() -> Result<(), String> {
    let Args {
        retries,
        min_duration,
        max_duration,
        jitter,
        factor,
        mut command,
    } = Args::parse();

    let min_duration = Duration::from_millis(min_duration.saturating_mul(100));
    let max_duration = max_duration.map(|x| Duration::from_millis(x.saturating_mul(100)));
    let retries = retries.clamp(1, u32::MAX);

    let mut backoff = Backoff::new(retries, min_duration, max_duration);
    backoff.set_factor(factor);
    backoff.set_jitter(jitter);

    let mut cmd = Command::new(command.remove(0));
    if !command.is_empty() {
        cmd.args(command);
    }

    let mut backoff = backoff.iter();
    loop {
        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    return Ok(());
                }
                if let Some(duration) = backoff.next() {
                    eprintln!("failed, retrying...");
                    sleep(duration);
                } else {
                    break;
                }
            }
            Err(fatal) => return Err(format!("unable to execute: {fatal:?}")),
        }
    }

    Err(format!("continued to fail after {retries} retries"))
}
