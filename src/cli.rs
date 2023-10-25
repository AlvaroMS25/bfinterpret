use clap::{Parser, Subcommand, Args, ArgGroup};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Run(RunCommand),
    Translate(TranslateCommand)
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("run_input_option")
        .required(true)
        .multiple(true)
        .args(&["input", "trailing"])
))]
pub struct RunCommand {
    #[clap(long, short = 'i')]
    pub input: Option<PathBuf>,
    pub trailing: Vec<String>
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("translate_input_option")
        .required(true)
        .multiple(true)
        .args(&["input", "trailing"])
))]
pub struct TranslateCommand {
    #[clap(long, short = 'i')]
    pub input: Option<PathBuf>,
    #[clap(long, short = 'o')]
    pub output: Option<PathBuf>,
    pub trailing: Vec<String>
}
