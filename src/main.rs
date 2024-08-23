use clap::Parser;
use duration_string::DurationString;
use exponential_backoff::Backoff;
use std::{process::Command, thread::sleep};

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
    attempts: u32,

    /// minimum duration
    ///
    /// Examples: `10ms`, `2s`, `5m 30s`, or `1h10m`
    #[clap(long, default_value = "10ms")]
    min_duration: DurationString,

    /// maximum duration
    ///
    /// Examples: `10ms`, `2s`, `5m 30s`, or `1h10m`
    #[clap(long)]
    max_duration: Option<DurationString>,

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
        attempts,
        min_duration,
        max_duration,
        jitter,
        factor,
        mut command,
    } = Args::parse();

    let mut backoff = Backoff::new(attempts, min_duration.into(), max_duration.map(Into::into));
    backoff.set_factor(factor);
    backoff.set_jitter(jitter);

    let mut cmd = Command::new(command.remove(0));
    if !command.is_empty() {
        cmd.args(command);
    }

    for duration in backoff {
        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    return Ok(());
                }
                if let Some(duration) = duration {
                    eprintln!("failed, retrying...");
                    sleep(duration);
                }
            }
            Err(fatal) => return Err(format!("unable to execute: {fatal:?}")),
        }
    }

    Err(format!("continued to fail after {attempts} attempts"))
}
