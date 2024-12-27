use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
#[error("invalid register index {index} (register indices go from 0-4 exclusive)")]
pub struct InvalidRegister {
    pub index: usize,
}
