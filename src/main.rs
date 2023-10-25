use clap::Parser;
use cli::{Cli, RunCommand};
use error::Error;
use interpreter::Interpreter;

use crate::{instruction::{Instruction, InstructionTracker}, cli::SubCommand};
use crate::cli::TranslateCommand;
use crate::translator::Translator;

// https://gist.github.com/roachhd/dce54bec8ba55fb17d3a

mod cli;
mod error;
mod instruction;
mod interpreter;
mod translator;

fn main() -> Result<(), Error> {
    /*
    let a = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;
    //let a = "+++++>-[-]";
    let instructions = InstructionTracker::parse(a).unwrap();
    println!("Instructions: {instructions:#?}");
    let mut interpreter = Interpreter::new(&instructions);
    interpreter.run().unwrap();
    println!("Mem: {:?}", interpreter.data);
    */

    let cli = Cli::parse();
    println!("{cli:#?}");

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
        None => command.trailing.join("")
    };

    let translator = Translator::new(output);
    let instructions = translator.run_string();
    println!("Translated: {instructions}");

    Ok(())
}
