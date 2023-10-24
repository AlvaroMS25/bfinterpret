use clap::{Parser, Subcommand, Args, ArgGroup};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Run(RunCommand),
    Translate(TranslateCommand)
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("input_option")
        .required(true)
        .multiple(true)
        .args(&["input", "trailing"])
))]
pub struct RunCommand {
    #[clap(long, short = 'i')]
    input: Option<PathBuf>,
    trailing: Vec<String>
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("input_option")
        .required(true)
        .multiple(true)
        .args(&["input", "trailing"])
))]
pub struct TranslateCommand {
    #[clap(long, short = 'i')]
    input: Option<PathBuf>,
    #[clap(long, short = 'o')]
    output: Option<PathBuf>,
    trailing: Vec<String>
}
