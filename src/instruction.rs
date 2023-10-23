use std::process::id;
use crate::error::Error;

#[derive(Debug)]
pub struct InstructionTracker {
    pub position: usize,
    pub instruction: Instruction
}

#[derive(Debug)]
pub enum Instruction {
    MoveRight, // >
    MoveLeft, // <
    Inc, // +
    Dec, // -
    StartLoop, // [ - Not used on execution
    EndLoop, // ] - Not used on execution
    Input, // ,
    Output, // .
    Loop(Vec<InstructionTracker>), // - Only used in execution
    None // Any other char
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::MoveRight,
            '<' => Self::MoveLeft,
            '+' => Self::Inc,
            '-' => Self::Dec,
            '[' => Self::StartLoop,
            ']' => Self::EndLoop,
            ',' => Self::Input,
            '.' => Self::Output,
            _ => Self::None
        }
    }
}

impl InstructionTracker {
    pub fn parse(source: &str) -> Result<Vec<Self>, Error> {
        let mut out = Vec::with_capacity(source.chars().count());
        let mut iter = source.chars().enumerate();

        while let Some((idx, inst)) = iter.next() {
            let instruction = Instruction::from(inst);

            match instruction {
                Instruction::None | Instruction::Loop(_) => continue,
                Instruction::StartLoop => {
                    out.push(Self::parse_inner_loop(idx, &mut iter)?)
                },
                Instruction::EndLoop => return Err(Error::UnexpectedToken(idx+1, inst)),
                other => out.push(Self {
                    position: idx,
                    instruction: other
                })
            }
        }


        Ok(out)
    }

    fn parse_inner_loop<T>(start: usize, iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = (usize, char)>
    {
        let mut out = Vec::new();
        while let Some((idx, inst)) = iter.next() {
            let instruction = Instruction::from(inst);

            match instruction {
                Instruction::None | Instruction::Loop(_) => continue,
                Instruction::EndLoop => return Ok(Self {
                    position: start,
                    instruction: Instruction::Loop(out)
                }),
                Instruction::StartLoop => {
                    out.push(Self::parse_inner_loop(idx, iter)?)
                },
                ins => out.push(Self {
                    position: idx,
                    instruction: ins
                })
            }
        }

        Err(Error::UnclosedLoop(start+1))
    }
}