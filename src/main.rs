use std::fs::File;
use std::io::Write;
use clap::Parser;
use cli::{Cli, RunCommand};
use error::Error;
use interpreter::Interpreter;

use crate::{instruction::{Instruction, InstructionTracker}, cli::SubCommand};
use crate::cli::TranslateCommand;
use crate::translator::Translator;

// https://gist.github.com/roachhd/dce54bec8ba55fb17d3a

mod bfdef;
mod cli;
mod error;
mod instruction;
mod interpreter;
mod translator;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommand::Run(run) => run_interpreter(run),
        SubCommand::Translate(translate) => run_translator(translate)
    }
}

fn run_interpreter(command: RunCommand) -> Result<(), Error> {
    let instructions = match command.input {
        Some(path) => std::fs::read_to_string(path)?,
        None => command.trailing.join("")
    };

    let parsed = InstructionTracker::parse(&instructions)?;
    let mut interpreter = Interpreter::new(&parsed);
    interpreter.run()
}

fn run_translator(command: TranslateCommand) -> Result<(), Error> {
    let output = match command.input {
        Some(path) => std::fs::read_to_string(path)?,
        None => command.trailing.join(" ")
    };

    let translator = Translator::new(output);
    let instructions = translator.run_string();

    if let Some(path) = command.output {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        file.write_all(instructions.as_bytes())?;
    } else {
        println!("{instructions}");
    }

    Ok(())
}
