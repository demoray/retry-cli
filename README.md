# retry (cli)

## Summary

*A small command line application that assists in retrying failed commands.*

`retry` is a command line tool written in [Rust](https://www.rust-lang.org/) intended to automatically re-run failed commands with a user configurable delay between tries.

## Usage

```
Usage: retry [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...

Options:
      --retries <RETRIES>            [default: 3]
      --min-duration <MIN_DURATION>  minimum duration in tenths of a second [default: 10]
      --max-duration <MAX_DURATION>  maximum duration in tenths of a second
      --jitter <JITTER>              amount of randomization to add to the backoff [default: 0.3]
      --factor <FACTOR>              backoff factor [default: 2]
  -h, --help                         Print help
  -V, --version                      Print version
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

The command fails to execute
```console
$ retry cmd-does-not-exist
Error: "unable to execute: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"
$
```

The command executes, but fails
```console
$ retry false
failed, retrying...
failed, retrying...
failed, retrying...
failed, retrying...
Error: "continued to fail after 3 retries"
$
```
