# Haystacks

Reads needles from a file then searches the haystack from from stdin.

## Usage

```bash
$ haystacks-rs --help
rust-haystacks 1.0
Peter Coulton <petercoulton@gmail.com>

USAGE:
    haystacks-rs --needles <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --needles <FILE>
```

## Examples

```
$ cat ./haystack.txt | haystacks-rs --needles ./needles.txt
```

