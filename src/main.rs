use crate::instruction::{Instruction, InstructionTracker};

// https://gist.github.com/roachhd/dce54bec8ba55fb17d3a

mod cli;
mod error;
mod instruction;
mod interpreter;

fn main() {
    let a = "+-<>,.[..-[--].]";
    let instructions = InstructionTracker::parse(a).unwrap();
    println!("Instructions {instructions:#?}");
}
