#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail

readme() {
  cat <<EOF >README.md
# cnj

cnj is a CLI for [checking](https://github.com/rodmoioliveira/cnj#check)
and manipulating [CNJ numbers](https://atos.cnj.jus.br/atos/detalhar/atos-normativos?documento=119).

[![Build status](https://github.com/rodmoioliveira/cnj/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/cnj/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/cnj)](https://github.com/rodmoioliveira/cnj/releases)

## Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/cnj/releases)
for \`cnj\` are available for Windows, macOS and Linux.

## Building

\`cnj\` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build \`cnj\`, run:

\`\`\`
git clone git@github.com:rodmoioliveira/cnj.git
cd cnj
make install
\`\`\`

## Commands

\`\`\`
cargo run -- --help

$(cargo run -- --help)
\`\`\`

## Subcommands

### Check

\`\`\`
cargo run -- check --help

$(cargo run -- check --help)
\`\`\`

Output example:

\`\`\`
cargo run -- check -oV 1234567-38.1011.1.21.3141 12345678910111213141

$(cargo run -- check -oV 1234567-38.1011.1.21.3141 12345678910111213141)
\`\`\`

### Completion

\`\`\`
cargo run -- completion --help

$(cargo run -- completion --help)
\`\`\`

## Performance

$(cat benches/results.md)
EOF

  sd '(make\[1\]:.+\n)' '' README.md
  sd 'cargo run --' 'cnj' README.md
}

trap readme EXIT
