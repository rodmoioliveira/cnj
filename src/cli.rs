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
    help_template = HELP_COMMAND,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
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
        about = check_about(Short),
        short_flag = 'C',
        long_about = check_about(Long),
    )]
    Check(CheckArgs),
}

#[derive(Debug, Args, Clone, Serialize)]
pub struct CheckArgs {
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
    pub output: Output,
}

#[derive(Debug, ValueEnum, Clone, Copy, Serialize)]
pub enum Output {
    /// [alias = C] output in Csv
    #[clap(alias = "C")]
    Csv,
    /// [alias = J] output in Json
    #[clap(alias = "J")]
    Json,
    /// [alias = T] output in Table
    #[clap(alias = "T")]
    Table,
    /// [alias = V] output in Vertical
    #[clap(alias = "V")]
    Vertical,
}

impl input::HasPositionalInputs for CheckArgs {
    fn get_positional_input(&self) -> Vec<String> {
        self.input.clone()
    }
}
