use std::io::Read;
use crate::error::Error;
use crate::instruction::{Instruction, InstructionTracker};

pub struct Interpreter<'a> {
    pub instructions: &'a Vec<InstructionTracker>,
    pub pointer: u32,
    pub data: Vec<u8>
}

impl<'a> Interpreter<'a> {
    pub fn new(instructions: &'a Vec<InstructionTracker>) -> Self {
        Self {
            instructions,
            pointer: 0,
            data: Vec::new()
        }
    }

    fn decrement_pointer(&mut self, idx: usize) -> Result<(), Error> {
        if let Some(value) = self.pointer.checked_sub(1) {
            self.pointer = value;
            Ok(())
        } else {
            Err(Error::PointerUnderflow(idx + 1))
        }
    }

    fn increment_pointer(&mut self, idx: usize) -> Result<(), Error> {
        if let Some(value) = self.pointer.checked_add(1) {
            self.pointer = value;
            Ok(())
        } else {
            Err(Error::PointerOverflow(idx + 1))
        }
    }

    fn increase_value(&mut self, idx: usize) -> Result<(), Error> {
        todo!()
    }

    fn decrease_value(&mut self, idx: usize) -> Result<(), Error> {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.execute(self.instructions.iter())
    }

    fn execute<I>(&mut self, iter: I) -> Result<(), Error>
    where
        I: Iterator<Item = InstructionTracker>
    {
        for t in self.instructions.iter() {
            match &t.instruction {
                Instruction::MoveRight => self.increment_pointer(t.position)?,
                Instruction::MoveLeft => self.decrement_pointer(t.position)?,
                Instruction::Inc => self.increase_value(t.position)?,
                Instruction::Dec => self.decrease_value(t.position)?,
                Instruction::Input => {
                    let mut buf = [0u8];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.data[self.pointer] = buf[0];
                },
                Instruction::Output => print!("{}", self.data[self.pointer] as char),
                Instruction::Loop(inner) => self.execute(inner.iter())
                _ => continue
            }
        }

        Ok(())
    }
}
