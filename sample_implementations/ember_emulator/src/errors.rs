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
pub enum EmberExecutionError {}
