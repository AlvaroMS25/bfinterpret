use crate::error::Error;
use crate::instruction::{Instruction, InstructionTracker};

pub struct Interpreter {
    pub instructions: Vec<InstructionTracker>,
    pub pointer: u32,
    pub data: Vec<u8>
}

impl Interpreter {
    pub fn new(instructions: Vec<InstructionTracker>) -> Self {
        Self {
            instructions,
            pointer: 0,
            data: Vec::new()
        }
    }

    pub fn decrement_pointer(&mut self, idx: usize) -> Result<(), Error> {
        if let Some(value) = self.pointer.checked_sub(1) {
            self.pointer = value;
            Ok(())
        } else {
            Err(Error::PointerUnderflow(idx + 1))
        }
    }

    pub fn increment_pointer(&mut self, idx: usize) -> Result<(), Error> {
        if let Some(value) = self.pointer.checked_add(1) {
            self.pointer = value;
            Ok(())
        } else {
            Err(Error::PointerOverflow(idx + 1))
        }
    }

    pub fn run(&mut self) -> () {
        for t in self.instructions.iter() {

        }
    }
}
