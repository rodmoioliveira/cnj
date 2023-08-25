use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::{
    help::{Help::*, *},
    input,
};

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(author = clap::crate_authors!())]
#[clap(color = clap::ColorChoice::Never)]
#[clap(
    about = clap::crate_description!(),
    long_about = clap::crate_description!(),
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(
        color = clap::ColorChoice::Never,
        arg_required_else_help = true,
        about = completion_about(Short),
        long_about = completion_about(Long),
    )]
    Completion {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    #[clap(
        color = clap::ColorChoice::Never,
        about = validate_about(Short),
        long_about = validate_about(Long),
    )]
    Validate(ValidateArgs),
}

#[derive(Debug, Args, Clone, Serialize)]
pub struct ValidateArgs {
    #[clap(
        default_value = "-",
        default_missing_value = "-",
        value_name = "CNJ",
        value_delimiter = ' ',
        required = false,
        help = cnjs_help(Short),
        long_help = cnjs_help(Long)
    )]
    pub input: Vec<String>,

    #[arg(
        short,
        long,
        value_enum,
        required = false,
        default_value = "table",
        help = format_help(Short),
        long_help = format_help(Long)
    )]
    pub format: Format,
}

#[derive(Debug, ValueEnum, Clone, Copy, Serialize)]
pub enum Format {
    Csv,
    Json,
    Table,
    Vertical,
}

impl input::HasPositionalInputs for ValidateArgs {
    fn get_positional_input(&self) -> Vec<String> {
        self.input.clone()
    }
}
