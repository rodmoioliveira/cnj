# cnj

cnj is a CLI for [validating](https://github.com/rodmoioliveira/cnj#validate)
and manipulating [CNJ numbers](https://atos.cnj.jus.br/atos/detalhar/atos-normativos?documento=119).

[![Build status](https://github.com/rodmoioliveira/cnj/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/cnj/actions)

## Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/cnj/releases)
for `cnj` are available for Windows, macOS and Linux.

## Building

`cnj` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build `cnj`, run:

```
git clone git@github.com:rodmoioliveira/cnj.git
cd cnj
make install
```

## Commands

```
cnj --help

cnj is a CLI for validating and manipulating CNJ numbers

Usage: cnj <SUBCOMMAND>

Subcommands:
  completion  Generate auto-completion for shells
  validate    Check if a CNJ number has the correct validation digits
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Subcommands

### Validate

```
cnj validate --help

Check if a CNJ number has the correct validation digits according to the CNJ specification
available at https://github.com/rodmoioliveira/cnj/blob/main/spec/cnj_spec.pdf

Usage: cnj validate [OPTIONS] [CNJ]...

Arguments:
  [CNJ]...
          A list of space-separated CNJs. The provided CNJ can be in any of the following
          formats:
          
              with mask:       1234567-38.1011.1.21.3141
              without mask:    12345678910111213141
          
          Examples:
          
              cnj validate 1234567-38.1011.1.21.3141 12345678910111213141
              echo 1234567-38.1011.1.21.3141 12345678910111213141 | xargs cnj validate
              cat list_of_cnjs.csv | cnj validate -oC
              cnj validate -ojson < list_of_cnjs.csv
          
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
cnj validate 1234567-38.1011.1.21.3141 12345678910111213141

cnj                          nnnnnnn    dd    aaaa    j    tr    oooo    is_valid    v_dd    v_cnj
1234567-38.1011.1.21.3141    1234567    38    1011    1    21    3141    true        38      1234567-38.1011.1.21.3141
1234567-89.1011.1.21.3141    1234567    89    1011    1    21    3141    false       38      1234567-38.1011.1.21.3141
```

### Completion

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

## Performance

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cnj validate [ input_size=10^0 ]` | 42.8 ± 14.0 | 30.4 | 76.1 | 1.01 ± 0.43 |
| `cnj validate [ input_size=10^1 ]` | 42.3 ± 11.6 | 31.9 | 75.9 | 1.00 |
| `cnj validate [ input_size=10^2 ]` | 55.4 ± 18.1 | 34.1 | 82.1 | 1.31 ± 0.56 |
| `cnj validate [ input_size=10^3 ]` | 55.9 ± 22.2 | 33.1 | 97.8 | 1.32 ± 0.64 |
| `cnj validate [ input_size=10^4 ]` | 98.7 ± 19.3 | 78.1 | 122.7 | 2.33 ± 0.79 |
| `cnj validate [ input_size=10^5 ]` | 500.2 ± 17.4 | 481.4 | 530.9 | 11.82 ± 3.28 |
| `cnj validate [ input_size=10^6 ]` | 4852.7 ± 134.3 | 4671.5 | 5174.4 | 114.71 ± 31.72 |

## Make Recipes

```
make help

audit           Audit Cargo.lock
audit-fix       Update Cargo.toml to fix vulnerable dependency requirement
bash-all        Run all bash tests
bash-check      Check format bash code
bash-fmt        Format bash code
bash-lint       Check lint bash code
bench           Run benchmarks
build           Build binary
cargo-deps      Install cargo dependencies
changelog       Write CHANGELOG.mode
check           Run check
dev             Run check in watch mode
doc             Open app documentation
fix             Fix rust code
fmt             Format rust code
help            Display this help screen
install         Install binary
lint-fix        Fix lint rust code
lint            Lint rust code
outdated        Display when dependencies are out of date
readme          Write README.md
tests           Run tests
typos           Check typos
typos-fix       Fix typos
uninstall       Uninstall binary
update          Update dependencies
```
