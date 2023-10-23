#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid character {1} at position {0}")]
    InvalidCharacter(usize, char),
    #[error("Unclosed loop detected at {0}")]
    UnclosedLoop(usize),
    #[error("Unexpected token `{1}` at position {0}")]
    UnexpectedToken(usize, char),
    #[error("Pointer underflow detected at position {0}")]
    PointerUnderflow(usize),
    #[error("Pointer overflow detected at position {0}")]
    PointerOverflow(usize),
    #[error(transparent)]
    IO(std::io::Error)
}

impl Error {
    pub fn set_position(mut self, position: usize) -> Self {
        match &mut self {
            Self::InvalidCharacter(p, _) => *p = position,
            Self::UnexpectedToken(p, _) => *p = position,
            _ => ()
        }

        self
    }
}
