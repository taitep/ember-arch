use crate::{
    alu::ALUSettings,
    errors::{self, InvalidOpcode},
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Operation {
        destination_register: usize,
        operation_type: OperationType,
        source_mode: OperationSourceMode,
        is_8bit: bool,
    },
}

impl TryFrom<u16> for Instruction {
    type Error = errors::InvalidOpcode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match (value >> 12) & 0b1111 {
            0b0000 => Ok(Instruction::Operation {
                destination_register: ((value >> 10) & 0b11) as usize,
                operation_type: if (value >> 1) & 0b1 == 0 {
                    OperationType::ALU(ALUSettings::from(((value >> 3) & 0b11111) as u8))
                } else {
                    todo!()
                },
                source_mode: match (value >> 8) & 0b11 {
                    0b00 => OperationSourceMode::Immediate,
                    0b01 => OperationSourceMode::Register,
                    0b10 => OperationSourceMode::Memory { is_big_endian: false },
                    0b11 => OperationSourceMode::Memory { is_big_endian: true },
                    _ => unreachable!(),
                },
                is_8bit: ((value >> 2) & 0b1) != 0,
            }),
            0b0001 => todo!(), // Comparison
            0b0010 => todo!(), // Load
            0b0011 => todo!(), // Store
            0b0100 => todo!(), // Jump
            0b0110 => todo!(), // Stack Read
            0b0111 => todo!(), // Stack Write
            0b1111 => todo!(), // More Specific Instructions

            0b1000..0b1111 | 0b0101 => Err(InvalidOpcode::from(value)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperationSourceMode {
    Immediate,
    Register,
    Memory { is_big_endian: bool },
}

#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    ALU(ALUSettings),
}
