# human-readable

Converts byte size into human-readable format

``` bash
hr [OPTIONS] [FILE]
```

## Arguments

```
[FILE]  File to read, if empty read from stdin
```

## Options

```
-d, --delimiter <DELIMITER>  Use <DELIMITER> as the field delimiter [default: "\t"]
-f, --fields <FIELDS>        Specify which fields to convert to human-readable format [default: 1]
-u, --unit <UNIT>            Specify which unit to use [default: auto] [possible values: auto, byte, kilo, mega, giga, tera, peta, exa]
-p, --precision <PRECISION>  Decimal precision of the output [default: 1]
-h, --help                   Print help
-V, --version                Print version
```

## Examples

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
