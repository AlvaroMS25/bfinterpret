use clap::Parser;
use cli::Cli;
use interpreter::Interpreter;

use crate::instruction::{Instruction, InstructionTracker};

// https://gist.github.com/roachhd/dce54bec8ba55fb17d3a

mod cli;
mod error;
mod instruction;
mod interpreter;
mod outerpreter;

fn main() {
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
}
