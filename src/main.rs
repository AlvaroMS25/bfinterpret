use clap::Parser;
use cli::{Cli, RunCommand};
use error::Error;
use interpreter::Interpreter;

use crate::{instruction::{Instruction, InstructionTracker}, cli::SubCommand};

// https://gist.github.com/roachhd/dce54bec8ba55fb17d3a

mod cli;
mod error;
mod instruction;
mod interpreter;
mod outerpreter;

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
        SubCommand::Translate(translate) => todo!()
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
