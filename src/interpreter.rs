use std::io::Read;
use crate::error::Error;
use crate::instruction::{Instruction, InstructionTracker};

pub struct Interpreter<'a> {
    pub instructions: &'a Vec<InstructionTracker>,
    pub pointer: usize,
    pub data: Vec<u8>
}

impl<'a> Interpreter<'a> {
    pub fn new(instructions: &'a Vec<InstructionTracker>) -> Self {
        Self {
            instructions,
            pointer: 0,
            data: vec![0; 32]
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
            if self.data.len() <= self.pointer {
                self.data.push(0);
            }
            Ok(())
        } else {
            Err(Error::PointerOverflow(idx + 1))
        }
    }

    fn increase_value(&mut self) -> Result<(), Error> {
        self.data[self.pointer] = match self.data[self.pointer].checked_add(1) {
            None => u8::MIN,
            Some(v) => v
        };

        Ok(())
    }

    fn decrease_value(&mut self) -> Result<(), Error> {
        self.data[self.pointer] = match self.data[self.pointer].checked_sub(1) {
            None => u8::MAX,
            Some(v) => v
        };

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.execute(self.instructions.iter())
    }

    fn execute<I>(&mut self, iter: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a InstructionTracker>
    {
        for t in iter {
            //println!("Pointer before instruction: {}", self.pointer);
            match &t.instruction {
                Instruction::MoveRight => self.increment_pointer(t.position)?,
                Instruction::MoveLeft => self.decrement_pointer(t.position)?,
                Instruction::Inc => self.increase_value()?,
                Instruction::Dec => self.decrease_value()?,
                Instruction::Input => {
                    let mut buf = [0u8];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.data[self.pointer] = buf[0];
                },
                Instruction::Output => print!("{}", self.data[self.pointer] as char),
                Instruction::Loop(inner) => {
                    while self.data[self.pointer] != 0 {
                        self.execute(inner.iter())?;
                    }
                },
                _ => continue
            }
        }

        Ok(())
    }
}
