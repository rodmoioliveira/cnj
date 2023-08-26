use clap::CommandFactory;

use crate::{
    cli::{Cli, ValidateArgs},
    cnj, input,
    types::*,
};

pub async fn validate(args: ValidateArgs) -> Result<()> {
    assert_ne!(
        input::is_positional(&args.input),
        input::is_stdin(&args.input),
        "Can't have both positional inputs and stdin inputs"
    );

    if !input::is_valid(&args.input) {
        Cli::command()
            .find_subcommand_mut("validate")
            .unwrap()
            .print_help()?;
        std::process::exit(1);
    }

    let cnjs: Vec<cnj::Cnj> = input::get(args.clone())
        .into_iter()
        .map(cnj::unmask)
        .filter(cnj::has_20_len)
        .map(cnj::new)
        .map(cnj::validate_dd)
        .collect();

    cnj::print(cnjs, args.output)?;
    Ok(())
}

pub async fn validate_par(args: ValidateArgs) -> Result<()> {
    use rayon::prelude::*;

    assert_ne!(
        input::is_positional(&args.input),
        input::is_stdin(&args.input),
        "Can't have both positional inputs and stdin inputs"
    );

    if !input::is_valid(&args.input) {
        Cli::command()
            .find_subcommand_mut("validate")
            .unwrap()
            .print_help()?;
        std::process::exit(1);
    }

    let cnjs: Vec<cnj::Cnj> = input::get(args.clone())
        .into_par_iter()
        .map(cnj::unmask)
        .filter(cnj::has_20_len)
        .map(cnj::new)
        .map(cnj::validate_dd)
        .collect();

    cnj::print(cnjs, args.output)?;
    Ok(())
}
