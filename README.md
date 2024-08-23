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
      --attempts <ATTEMPTS>
          [default: 3]

      --min-duration <MIN_DURATION>
          minimum duration

          Examples: `10ms`, `2s`, `5m 30s`, or `1h10m`

          [default: 10ms]

      --max-duration <MAX_DURATION>
          maximum duration

          Examples: `10ms`, `2s`, `5m 30s`, or `1h10m`

      --jitter <JITTER>
          amount of randomization to add to the backoff

          [default: 0.3]

      --factor <FACTOR>
          backoff factor

          [default: 2]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
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
Error: "continued to fail after 3 attempts"
$
```
