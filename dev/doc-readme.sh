#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail
set -o noclobber
shopt -s inherit_errexit

index() {
  paste -d "" \
    <(
      cat dev/doc-readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/^ {1,}//g' |
        sed -E 's/(^#{1,}) (.+)/\1\[\2]/g' |
        sed 's/#/  /g' |
        sed -E 's/\[/- [/g'
    ) \
    <(
      cat dev/doc-readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/#//g' |
        sed -E 's/^ {1,}//g' |
        # https://www.gnu.org/software/grep/manual/html_node/Character-Classes-and-Bracket-Expressions.html
        sed -E "s1[][!#$%&'()*+,./:;<=>?@\\^_\`{|}~]11g" |
        sed -E 's/"//g' |
        sed 's/[A-Z]/\L&/g' |
        sed 's/ /-/g' |
        sed -E 's@(.+)@(#\1)@g'
    )
}

backlink() {
  sed -i -E '/^#{1,} [A-Z]/a\\n\[back^\](#index)' README.md
}

readme() {
  cat <<EOF >|README.md
# cnj

cnj is a CLI for [checking](#check)
and manipulating [CNJ numbers](https://atos.cnj.jus.br/atos/detalhar/atos-normativos?documento=119).

[![Build status](https://github.com/rodmoioliveira/cnj/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/cnj/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/cnj)](https://github.com/rodmoioliveira/cnj/releases)

# index

$(index)

# Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/cnj/releases)
for \`cnj\` are available for Windows, macOS and Linux.

# Building

\`cnj\` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build \`cnj\`, run:

\`\`\`
git clone git@github.com:rodmoioliveira/cnj.git
cd cnj
make rs-build
\`\`\`

# Commands

\`\`\`
cargo run -- --help

$(cargo run -- --help)
\`\`\`

# Subcommands

## Check

\`\`\`
cargo run -- check --help

$(cargo run -- check --help)
\`\`\`

Output example:

\`\`\`
cargo run -- check -oV 1234567-38.1011.1.21.3141 12345678910111213141

$(cargo run -- check -oV 1234567-38.1011.1.21.3141 12345678910111213141)
\`\`\`

## Completion

\`\`\`
cargo run -- completion --help

$(cargo run -- completion --help)
\`\`\`

# Dependencies

$(
    paste -d '@' \
      <(
        yq '.dependencies | keys[]' Cargo.toml |
          sort |
          sed -E 's@(.+)@- [\1](https://crates.io/crates/\1)@g'
      ) \
      <(
        yq '.dependencies | keys[]' Cargo.toml |
          sort |
          xargs -n1 bash -c 'cargo info $0 2>/dev/null | sed -n "2p"'
      ) |
      sed 's/@/ - /g'
  )

# Performance

$(cat benches/results.md)

# Make Recipes

\`\`\`
$(make help)
\`\`\`

# How to Release

$(cat RELEASE.md)
EOF

  sed -i -E '/^make\[[0-9]/d' README.md
  sed -i -E 's/cargo run --/cnj/g' README.md
  backlink
  dprint fmt README.md CHANGELOG.md
}

trap readme EXIT
