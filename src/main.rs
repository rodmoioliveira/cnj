use clap::{CommandFactory, Parser};
use cnj::{
    cli::{Cli, Subcommands::*},
    subcommands,
    types::*,
};

// The Rust standard library suppresses the default SIGPIPE behavior, so that
// writing to a closed pipe doesn't kill the process.
//
// See:
// https://stackoverflow.com/questions/65755853/simple-word-count-rust-program-outputs-valid-stdout-but-panicks-when-piped-to-he
// https://github.com/BurntSushi/ripgrep/commit/3065a8c9c839f7e722a73e8375f2e41c7e084737
#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {}

#[tokio::main]
async fn main() -> Result<()> {
    reset_sigpipe();
    let args = Cli::parse();

    match args.subcommand {
        Completion { shell } => subcommands::completions(shell, &mut Cli::command()),
        Validate(args) => subcommands::validate_par(args).await?,
    }
    Ok(())
}
