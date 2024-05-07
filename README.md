# retry (cli)

## Summary

*A small command line application that assists in retrying failed commands.*

`retry` is a command line tool written in [Rust](https://www.rust-lang.org/) intended to automatically re-run failed commands with a user configurable delay between tries.

## Features

* Do not retry if the command exits due to a signal (such the program not existing)
* Adds small random jitter if delays are specified
* Use multiple delay methods (Increase by Fibonacci sequence, Delay a fixed amount, or no delay at all)

## Usage

```
Usage: retry [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...

Options:
      --retries <RETRIES>    [default: 3]
      --duration <DURATION>  [default: 10]
      --method <METHOD>      [default: fibonacci] [possible values: fibonacci, fixed, no-delay]
  -h, --help                 Print help
  -V, --version              Print version
```

## Installation

```console
$ cargo install retry-cli
```

## Examples

Working successfully:
```console
$ retry echo hi
hi
$
```

The command fails with a signal
```console
$ retry cmd-does-not-exist
❯ ./target/release/retry cmd-does-not-exist
Error: Error("unable to execute: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }")
$
```

With a duration of a half second between tries, increasing following a Fibonacci sequence (default delay method)
```console
$ time retry --duration 50 false
failed, retrying...
failed, retrying...
failed, retrying...
retry failed: continued to fail after 3 retries

real    0m15.009s
user    0m0.008s
sys     0m0.000s
$
```

With a fixed duration of a half second between tries
```console
$ time retry --method fixed --duration 50 false
failed, retrying...
failed, retrying...
failed, retrying...
retry failed: continued to fail after 3 retries

real    0m5.016s
user    0m0.012s
sys     0m0.004s
$
```

With no delay at all
```console
$ time retry --method nodelay false
failed, retrying...
failed, retrying...
failed, retrying...
retry failed: continued to fail after 3 retries

real    0m0.007s
user    0m0.006s
sys     0m0.000s
$
```

Retrying 5 times
```console
$ retry --retries 5 false
failed, retrying...
failed, retrying...
failed, retrying...
failed, retrying...
failed, retrying...
retry failed: continued to fail after 5 retries
$
```
