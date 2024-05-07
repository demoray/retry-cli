use clap::{Parser, ValueEnum};
use retry::{
    delay::{jitter, Fibonacci, Fixed, NoDelay},
    retry_with_index, OperationResult,
};
use std::{process::Command, time::Duration};

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
    retries: usize,
    #[clap(long, default_value = "10")]
    duration: u64,
    #[clap(long, default_value = "fibonacci")]
    method: Method,
    #[clap(required = true)]
    command: Vec<String>,
}

#[derive(PartialEq, Debug, ValueEnum, Clone, Copy)]
pub enum Method {
    Fibonacci,
    Fixed,
    NoDelay,
}

fn retry_time(
    method: Method,
    retries: usize,
    duration: Duration,
) -> Box<dyn Iterator<Item = Duration>> {
    match method {
        Method::Fibonacci => Box::new(Fibonacci::from(duration).map(jitter).take(retries)),
        Method::Fixed => Box::new(Fixed::from(duration).map(jitter).take(retries)),
        Method::NoDelay => Box::new(NoDelay.map(jitter).take(retries)),
    }
}

fn main() -> Result<(), Error> {
    let Args {
        retries,
        duration,
        method,
        mut command,
    } = Args::parse();
    let duration = Duration::from_millis(duration.saturating_mul(100));

    let mut cmd = Command::new(command.remove(0));
    if !command.is_empty() {
        cmd.args(command);
    }

    let func = |current_try: u64| match cmd.status() {
        Ok(status) => {
            if status.success() {
                OperationResult::Ok(())
            } else {
                if current_try <= retries as u64 {
                    eprintln!("failed, retrying...");
                }
                OperationResult::Retry(format!("continued to fail after {retries} retries"))
            }
        }
        Err(fatal) => OperationResult::Err(format!("unable to execute: {fatal:?}")),
    };

    retry_with_index(retry_time(method, retries, duration), func).map_err(|e| Error::Error(e.error))
}
