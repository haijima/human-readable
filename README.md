# hrdbl: human-readable

[![CI Status](https://github.com/haijima/human-readable/workflows/CI/badge.svg)](https://github.com/haijima/human-readable/actions)

Converts byte size into human-readable format

[rust-doc](https://haijima.github.io/human-readable/hrdbl/)

## Usage

``` bash
hr [OPTIONS] [FILE]
```

### Arguments

```
[FILE]  File to read, if empty read from stdin
```

### Options

```
-d, --delimiter <DELIMITER>  Use <DELIMITER> as the field delimiter [default: "\t"]
-f, --fields <FIELDS>        Specify which fields to convert to human-readable format [default: 1]
-u, --unit <UNIT>            Specify which unit to use [possible values: byte, kilo, mega, giga, tera, peta, exa]
-p, --precision <PRECISION>  Decimal precision of the output [default: 1]
-v, --verbose...             More output per occurrence
-q, --quiet...               Less output per occurrence
-h, --help                   Print help
-V, --version                Print version
```

### Examples

``` bash
$ cat foo.txt
1024
1234567890
$ hr foo.txt
1.0K
1.1G

$ echo "1234567890" | hr
1.1G

$ du | tail | hr
60.0K   ./foo
63.4K   ./bar
123.4K  .
```

## Install

If you're a macOS user, you can install with [Homebrew](https://brew.sh/).

``` bash
brew install haijima/tap/hr
```

If you're a Rust user, you can install with [Cargo](https://doc.rust-lang.org/cargo/).

``` bash
cargo install --git https://github.com/haijima/human-readable.git
```

Or you can download precompiled binaries from [releases](https://github.com/haijima/human-readable/releases).
