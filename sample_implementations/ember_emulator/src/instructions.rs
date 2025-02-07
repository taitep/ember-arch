#![allow(dead_code)]

use crate::{
    alu::ALUSettings,
    errors::{self, InvalidOpcode},
};

pub use Instruction::*;

#[derive(Debug, Clone)]
pub enum Instruction {
    Operation {
        a_register: usize,
        destination_register: usize,
        operation_type: OperationType,
        source_mode: OperationSourceMode,
        is_8bit: bool,
    },
    Load {
        destination_register: usize,
        offset_config: OffsetConfig,
        addressing_mode: LoadAddressingMode,
        is_big_endian: bool,
        is_8bit: bool,
    },
    Store {
        source_register: usize,
        offset_config: OffsetConfig,
        addressing_mode: StoreAddressingMode,
        is_big_endian: bool,
        is_8bit: bool,
    },
    Jump {
        offset_config: OffsetConfig,
        condition: Condition,
        is_subroutine_call: bool,
    },
    StackRead {
        destination_register: usize,
        offset_config: OffsetConfig,
        stack_pointer_change: u16,
    },
    StackWrite {
        source_register: usize,
        offset_config: OffsetConfig,
        stack_pointer_change: u16,
    },
    MoreSpecific(MoreSpecificInstructionOpcode),
}

impl TryFrom<u16> for Instruction {
    type Error = errors::InvalidOpcode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match (value >> 12) & 0b1111 {
            0b0000 => Ok(Operation {
                a_register: ((value >> 10) & 0b11) as usize,
                destination_register: ((value >> 10) & 0b11) as usize,
                operation_type: if (value >> 1) & 0b1 == 0 {
                    OperationType::ALU(ALUSettings::from(((value >> 3) & 0b11111) as u8))
                } else {
                    todo!()
                },
                source_mode: match (value >> 8) & 0b11 {
                    0b00 => OperationSourceMode::Immediate,
                    0b01 => OperationSourceMode::Register,
                    0b10 => OperationSourceMode::Memory {
                        is_big_endian: false,
                    },
                    0b11 => OperationSourceMode::Memory {
                        is_big_endian: true,
                    },
                    _ => unreachable!(),
                },
                is_8bit: ((value >> 2) & 0b1) != 0,
            }),

            0b0001 => Ok(Operation {
                // Comparison is just operation but the result goes in the zero register
                a_register: ((value >> 10) & 0b11) as usize,
                destination_register: 0,
                operation_type: if (value >> 1) & 0b1 == 0 {
                    OperationType::ALU(ALUSettings::from(((value >> 3) & 0b11111) as u8))
                } else {
                    todo!()
                },
                source_mode: match (value >> 8) & 0b11 {
                    0b00 => OperationSourceMode::Immediate,
                    0b01 => OperationSourceMode::Register,
                    0b10 => OperationSourceMode::Memory {
                        is_big_endian: false,
                    },
                    0b11 => OperationSourceMode::Memory {
                        is_big_endian: true,
                    },
                    _ => unreachable!(),
                },
                is_8bit: ((value >> 2) & 0b1) != 0,
            }),

            0b0010 => Ok(Load {
                destination_register: ((value >> 10) & 0b11) as usize,
                offset_config: OffsetConfig {
                    register: ((value >> 8) & 0b11) as usize,
                    shift_amount: (value >> 1) & 0b111,
                },
                addressing_mode: match (value >> 6) & 0b11 {
                    0b00 => LoadAddressingMode::Immediate,
                    0b01 => LoadAddressingMode::DirectAddress,
                    0b10 => LoadAddressingMode::Register,
                    0b11 => LoadAddressingMode::AddressRegister,
                    _ => unreachable!(),
                },
                is_big_endian: ((value >> 5) & 1) != 0,
                is_8bit: ((value >> 4) & 1) != 0,
            }),

            0b0011 => Ok(Store {
                source_register: ((value >> 10) & 0b11) as usize,
                offset_config: OffsetConfig {
                    register: ((value >> 8) & 0b11) as usize,
                    shift_amount: (value >> 1) & 0b111,
                },
                addressing_mode: match ((value >> 7) & 1) != 0 {
                    false => StoreAddressingMode::DirectAddress,
                    true => StoreAddressingMode::AddressRegister,
                },
                is_big_endian: ((value >> 5) & 1) != 0,
                is_8bit: ((value >> 4) & 1) != 0,
            }),

            0b0100 => Ok(Jump {
                offset_config: OffsetConfig {
                    register: ((value >> 10) & 0b11) as usize,
                    shift_amount: (value >> 5) & 0b111,
                },
                condition: Condition {
                    flag: ((value >> 2) & 0b11) as usize,
                    inverted: ((value >> 4) & 1) != 0,
                },
                is_subroutine_call: ((value >> 1) & 1) != 0,
            }),

            0b0110 => Ok(StackRead {
                destination_register: ((value >> 10) & 0b11) as usize,
                offset_config: OffsetConfig {
                    register: ((value >> 8) & 0b11) as usize,
                    shift_amount: (value >> 5) * 0b111,
                },
                stack_pointer_change: (value & 0b11) << ((value >> 2) & 0b111),
            }),

            0b0111 => Ok(StackWrite {
                source_register: ((value >> 10) & 0b11) as usize,
                offset_config: OffsetConfig {
                    register: ((value >> 8) & 0b11) as usize,
                    shift_amount: (value >> 5) & 0b111,
                },
                stack_pointer_change: (value & 0b11) << ((value >> 2) & 0b111),
            }),

            0b1111 => Ok(MoreSpecific(MoreSpecificInstructionOpcode::from(value))),

            0b1000..0b1111 | 0b0101 => Err(InvalidOpcode::from(value)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperationSourceMode {
    Immediate,
    Register,
    Memory { is_big_endian: bool },
}

#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    #[allow(clippy::upper_case_acronyms)]
    ALU(ALUSettings),
}

#[derive(Debug, Clone, Copy)]
pub struct OffsetConfig {
    register: usize,
    shift_amount: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadAddressingMode {
    Immediate,
    DirectAddress,
    Register,
    AddressRegister,
}

#[derive(Debug, Clone, Copy)]
pub enum StoreAddressingMode {
    DirectAddress,
    AddressRegister,
}

#[derive(Debug, Clone, Copy)]
pub struct Condition {
    flag: usize,
    inverted: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct MoreSpecificInstructionOpcode(u16);

impl MoreSpecificInstructionOpcode {
    fn opcode(&self) -> u16 {
        self.0
    }
}

impl From<u16> for MoreSpecificInstructionOpcode {
    fn from(value: u16) -> Self {
        Self(value & 0x0FFF)
    }
}
