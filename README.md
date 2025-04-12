# cnj

cnj is a CLI for [checking](#check)
and manipulating [CNJ numbers](https://atos.cnj.jus.br/atos/detalhar/atos-normativos?documento=119).

[![Build status](https://github.com/rodmoioliveira/cnj/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/cnj/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/cnj)](https://github.com/rodmoioliveira/cnj/releases)

# index

- [Installation](#installation)
- [Building](#building)
- [Commands](#commands)
- [Subcommands](#subcommands)
  - [Check](#check)
  - [Completion](#completion)
- [Dependencies](#dependencies)
- [Performance](#performance)
- [Make Recipes](#make-recipes)
- [How to Release](#how-to-release)

# Installation

[back^](#index)

Archives of [precompiled binaries](https://github.com/rodmoioliveira/cnj/releases)
for `cnj` are available for Windows, macOS and Linux.

# Building

[back^](#index)

`cnj` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build `cnj`, run:

```
git clone git@github.com:rodmoioliveira/cnj.git
cd cnj
make rs-build
```

# Commands

[back^](#index)

```
cnj --help

cnj is a CLI for validating and manipulating CNJ numbers

Usage: cnj <SUBCOMMAND>

Subcommands:
  completion  Generate auto-completion for shells
  check, -C   Check if a CNJ number has the correct validation digits
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Subcommands

[back^](#index)

## Check

[back^](#index)

```
cnj check --help

Check if a CNJ number has the correct validation digits according to the CNJ specification
available at https://github.com/rodmoioliveira/cnj/blob/main/spec/cnj_spec.pdf

Usage: cnj {check|-C} [OPTIONS] [CNJ]...

Arguments:
  [CNJ]...
          A list of space-separated CNJs. The provided CNJ can be in any of the following
          formats:
          
              with mask:       1234567-38.1011.1.21.3141
              without mask:    12345678910111213141
          
          Examples:
          
              cnj check 1234567-38.1011.1.21.3141 12345678910111213141
              echo 1234567-38.1011.1.21.3141 12345678910111213141 | xargs cnj check
              cat list_of_cnjs.csv | cnj -CoJ
              cnj check -ojson < list_of_cnjs.csv
          
          [default: -]

Options:
  -o, --output <OUTPUT>
          Change the output format
          
          [default: table]

          Possible values:
          - csv:      [alias = C] output in Csv
          - json:     [alias = J] output in Json
          - table:    [alias = T] output in Table
          - vertical: [alias = V] output in Vertical

  -h, --help
          Print help (see a summary with '-h')
```

Output example:

```
cnj check -oV 1234567-38.1011.1.21.3141 12345678910111213141

------------------------------------------------------
     cnj: 1234567-38.1011.1.21.3141
 nnnnnnn: 1234567
      dd: 38
    aaaa: 1011
       j: 1
      tr: 21
    oooo: 3141
is_valid: true
    v_dd: 38
   v_cnj: 1234567-38.1011.1.21.3141
------------------------------------------------------
     cnj: 1234567-89.1011.1.21.3141
 nnnnnnn: 1234567
      dd: 89
    aaaa: 1011
       j: 1
      tr: 21
    oooo: 3141
is_valid: false
    v_dd: 38
   v_cnj: 1234567-38.1011.1.21.3141
------------------------------------------------------
```

## Completion

[back^](#index)

```
cnj completion --help

Generate auto-completion for several shells:

    cnj completion bash > cnj.bash
    cnj completion fish > cnj.fish
    cnj completion zsh > _cnj
    cnj completion powershell > _cnj.ps1

For bash, move cnj.bash to $XDG_CONFIG_HOME/bash_completion or /etc/bash_completion.d/.

For fish, move cnj.fish to $HOME/.config/fish/completions/.

For zsh, move _cnj to one of your $fpath directories.

For PowerShell, add . _cnj.ps1 to your PowerShell profile (note the leading period).
If the _cnj.ps1 file is not on your PATH, do . /path/to/_cnj.ps1 instead.

Usage: cnj completion <SHELL>

Arguments:
  <SHELL>
          [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help
          Print help (see a summary with '-h')
```

# Dependencies

[back^](#index)

- [clap](https://crates.io/crates/clap) - A simple to use, efficient, and full-featured Command Line Argument Parser
- [clap_complete](https://crates.io/crates/clap_complete) - Generate shell completion scripts for your clap::Command
- [csv](https://crates.io/crates/csv) - Fast CSV parsing with support for serde.
- [grep-cli](https://crates.io/crates/grep-cli) - Utilities for search oriented command line applications.
- [libc](https://crates.io/crates/libc) - Raw FFI bindings to platform libraries like libc.
- [once_cell](https://crates.io/crates/once_cell) - Single assignment cells and lazy values.
- [rayon](https://crates.io/crates/rayon) - Simple work-stealing parallelism for Rust
- [regex](https://crates.io/crates/regex) - An implementation of regular expressions for Rust. This implementation uses
- [serde](https://crates.io/crates/serde) - A generic serialization/deserialization framework
- [serde_json](https://crates.io/crates/serde_json) - A JSON serialization file format
- [tokio](https://crates.io/crates/tokio) - An event-driven, non-blocking I/O platform for writing asynchronous I/O

# Performance

[back^](#index)

| Command                         |      Mean [ms] | Min [ms] | Max [ms] |       Relative |
| :------------------------------ | -------------: | -------: | -------: | -------------: |
| `cnj check [ input_size=10^0 ]` |    42.8 ± 14.0 |     30.4 |     76.1 |    1.01 ± 0.43 |
| `cnj check [ input_size=10^1 ]` |    42.3 ± 11.6 |     31.9 |     75.9 |           1.00 |
| `cnj check [ input_size=10^2 ]` |    55.4 ± 18.1 |     34.1 |     82.1 |    1.31 ± 0.56 |
| `cnj check [ input_size=10^3 ]` |    55.9 ± 22.2 |     33.1 |     97.8 |    1.32 ± 0.64 |
| `cnj check [ input_size=10^4 ]` |    98.7 ± 19.3 |     78.1 |    122.7 |    2.33 ± 0.79 |
| `cnj check [ input_size=10^5 ]` |   500.2 ± 17.4 |    481.4 |    530.9 |   11.82 ± 3.28 |
| `cnj check [ input_size=10^6 ]` | 4852.7 ± 134.3 |   4671.5 |   5174.4 | 114.71 ± 31.72 |

# Make Recipes

[back^](#index)

```
bash-all               Run all bash tests
bash-check             Check format bash code
bash-deps              Install bash dependencies
bash-fmt               Format bash code
bash-lint              Check lint bash code
comments-tidy          Tidy comments within code
doc-changelog          Write CHANGELOG.md
doc-readme             Write README.md
dprint-check           Dprint check
dprint-fmt             Dprint format
help                   Display this help screen
makefile-descriptions  Check if all Makefile rules have descriptions
rs-audit               Audit Cargo.lock
rs-audit-fix           Update Cargo.toml to fix vulnerable dependency requirement
rs-build               Build binary
rs-cargo-deps          Install cargo dependencies
rs-check               Run check
rs-dev                 Run check in watch mode
rs-doc                 Open app documentation
rs-fix                 Fix rust code
rs-fmt                 Format rust code
rs-fmt-fix             Format fix rust code
rs-install             Install binary
rs-lint                Lint rust code
rs-lint-fix            Fix lint rust code
rs-outdated            Display when dependencies are out of date
rs-tests               Run tests
rs-uninstall           Uninstall binary
rs-update-cargo        Update dependencies
rs-update-rustup       Update rust
typos                  Check typos
typos-fix              Fix typos
```

# How to Release

[back^](#index)

To generate a new version, you need to follow these steps:

1. In the `main` branch, you must bump the version inside the `Cargo.toml` file.
2. Run `make rs-check` so that the version is changed in the `Cargo.lock` file.
3. Run the command `git add -A && git commit -m "release: bump version"`.
4. Run the command `git tag -a <your.new.version> -m "version <your.new.version>"`.
5. Run the command `make doc-changelog && make doc-readme`.
6. Run the command `git add -A && git commit -m "release: <your.new.version>"`.
7. Run `git push` to `main`.
