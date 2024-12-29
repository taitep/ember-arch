use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
#[error("invalid register index {index} (register indices go from 0-4 exclusive)")]
pub struct InvalidRegister {
    pub index: usize,
}

#[derive(Debug, Error, Clone, Copy)]
#[error("invalid flag index {index} (flag indices go from 0-4 exclusive)")]
pub struct InvalidFlag {
    pub index: usize,
}

#[derive(Debug, Error, Clone, Copy)]
pub enum EmberExecutionError {
    #[error(transparent)]
    InvalidOpcode(#[from] InvalidOpcode),
}

#[derive(Debug, Error, Clone, Copy)]
#[error("invalid opcode ({opcode:04b})")]
pub struct InvalidOpcode {
    opcode: u16,
}

impl From<u16> for InvalidOpcode {
    fn from(value: u16) -> Self {
        InvalidOpcode {
            opcode: value & 0xF000 >> 12,
        }
    }
}

impl InvalidOpcode {
    fn opcode(&self) -> u16 {
        self.opcode
    }
}
