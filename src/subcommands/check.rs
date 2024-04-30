use crate::{
    cli::{CheckArgs, Cli},
    cnj, input,
    types::*,
};
use clap::CommandFactory;
use rayon::prelude::*;

pub async fn check(args: CheckArgs) -> Result<()> {
    assert_ne!(
        input::is_positional(&args.input),
        input::is_stdin(&args.input),
        "Can't have both positional inputs and stdin inputs"
    );

    if !input::is_valid(&args.input) {
        Cli::command()
            .find_subcommand_mut("check")
            .unwrap()
            .print_help()?;
        std::process::exit(1);
    }

    let cnjs: Vec<cnj::Cnj> = input::parse(args.input)
        .into_par_iter()
        .map(cnj::unmask)
        .filter(cnj::has_20_len)
        .map(cnj::new)
        .map(cnj::check_dd)
        .collect();

    cnj::print(cnjs, args.output)?;
    Ok(())
}
