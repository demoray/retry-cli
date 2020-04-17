#[macro_use]
extern crate clap;
extern crate retry;

use clap::{App, AppSettings, Arg};
use retry::delay::{jitter, Fibonacci, Fixed, NoDelay};
use retry::OperationResult;
use std::error::Error;
use std::process::Command;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Methods {
        Fibonacci,
        Fixed,
        NoDelay,
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = App::new("retry")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .args(&[
            Arg::with_name("retries")
                .long("retries")
                .takes_value(true)
                .help("specify maximum number of times to retry")
                .default_value("3"),
            Arg::with_name("duration")
                .long("duration")
                .takes_value(true)
                .help("duration (specified in tenths of a second)")
                .default_value("10"),
            Arg::with_name("method")
                .long("method")
                .takes_value(true)
                .possible_values(&Methods::variants())
                .default_value("Fibonacci")
                .case_insensitive(true)
                .help("retry methods"),
        ])
        .setting(AppSettings::AllowExternalSubcommands)
        .get_matches();

    let retries = value_t!(args.value_of("retries"), usize)?;
    let duration = value_t!(args.value_of("duration"), u64)? * 100;
    let method = value_t!(args.value_of("method"), Methods)?;

    let mut command = match args.subcommand {
        Some(x) => {
            let mut cmd = Command::new(x.name);
            for arg in x
                .matches
                .values_of("")
                .map_or(Vec::new(), Iterator::collect)
            {
                cmd.arg(arg);
            }
            cmd
        }
        None => {
            return Err(From::from("no command provided"));
        }
    };

    let func = |current_try: u64| match command.status() {
        Ok(status) => {
            if status.success() {
                OperationResult::Ok(())
            } else {
                if current_try <= retries as u64 {
                    eprintln!("failed, retrying...");
                }
                OperationResult::Retry(format!("continued to fail after {} retries", retries))
            }
        }
        Err(fatal) => OperationResult::Err(format!("unable to execute: {:?}", fatal)),
    };

    let res = match method {
        Methods::Fibonacci => retry::retry_with_index(
            Fibonacci::from_millis(duration).map(jitter).take(retries),
            func,
        ),
        Methods::Fixed => {
            retry::retry_with_index(Fixed::from_millis(duration).map(jitter).take(retries), func)
        }
        Methods::NoDelay => retry::retry_with_index(NoDelay.take(retries), func),
    };

    match res {
        Ok(_) => Ok(()),
        Err(err) => match err {
            retry::Error::Internal(error) => Err(From::from(error)),
            retry::Error::Operation { error, .. } => Err(From::from(error)),
        },
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("retry failed: {}", err);
        std::process::exit(1);
    }
}
