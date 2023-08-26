use std::io;

use clap::Command;
use clap_complete::{generate, shells, Generator, Shell};

fn print<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, env!("CARGO_PKG_NAME"), &mut io::stdout());
}

pub fn completions(shell: Shell, cmd: &mut Command) {
    match shell {
        Shell::Bash => print(shells::Bash, cmd),
        Shell::Elvish => print(shells::Elvish, cmd),
        Shell::Fish => print(shells::Fish, cmd),
        Shell::PowerShell => print(shells::PowerShell, cmd),
        Shell::Zsh => print(shells::Zsh, cmd),
        _ => unreachable!(),
    };
}
