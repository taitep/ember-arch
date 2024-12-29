mod alu;
mod errors;
mod instructions;

#[derive(Debug)]
pub struct Ember {
    ram: [u8; u16::MAX as usize + 1],
    regs: [Register; 4],
    flags: Flags,
    exec_state: ExecutionState,
    pub pc: u16,
}

/// Main CPU state struct. Contains all current machine state including RAM.
impl Ember {
    pub fn new() -> Ember {
        Ember {
            ram: [0; u16::MAX as usize + 1],
            regs: [
                Register::ZeroReg,
                Register::ValueReg(0),
                Register::ValueReg(0),
                Register::ValueReg(0),
            ],
            flags: Flags::default(),
            exec_state: ExecutionState::Running,
            pc: 0,
        }
    }

    pub fn read_reg(&self, id: usize) -> Option<u16> {
        self.regs.get(id).map(|r| r.get())
    }

    pub fn write_reg(&mut self, id: usize, value: u16) -> Result<(), errors::InvalidRegister> {
        if let Some(reg) = self.regs.get_mut(id) {
            reg.set(value);
            Ok(())
        } else {
            Err(errors::InvalidRegister { index: id })
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value
    }

    pub fn read_ram_little_endian(&self, address: u16) -> u16 {
        self.ram[address as usize] as u16 | (self.ram[address.wrapping_add(1) as usize] as u16) << 8
    }

    pub fn write_ram_little_endian(&mut self, address: u16, value: u16) {
        self.ram[address as usize] = value as u8;
        self.ram[address.wrapping_add(1) as usize] = (value >> 8) as u8;
    }

    pub fn exec_state(&self) -> ExecutionState {
        self.exec_state
    }

    pub fn get_flags(&self) -> Flags {
        self.flags
    }

    pub fn get_flags_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }
}

/// Represents a single register and contains its data (or marks it as a zero register)
#[derive(Debug, Clone, Copy)]
enum Register {
    ZeroReg,
    ValueReg(u16),
}

impl Register {
    pub fn get(&self) -> u16 {
        match self {
            Register::ZeroReg => 0,
            Register::ValueReg(x) => *x,
        }
    }

    pub fn set(&mut self, value: u16) {
        if let Register::ValueReg(x) = self {
            *x = value;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionState {
    Running,
    Halted {
        error: Option<errors::EmberExecutionError>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub flag_carry: bool,
    pub flag_zero: bool,
    pub flag_sign: bool,
    pub flag8_carry: bool,
    pub flag8_zero: bool,
    pub flag8_sign: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            flag_carry: false,
            flag_zero: false,
            flag_sign: false,
            flag8_carry: false,
            flag8_zero: false,
            flag8_sign: false,
        }
    }
}

impl Flags {
    pub fn get(&self, index: usize, is_8bit: bool) -> Option<bool> {
        match index {
            0 => Some(true),
            1 => Some(if is_8bit {
                self.flag8_carry
            } else {
                self.flag_carry
            }),
            2 => Some(if is_8bit {
                self.flag8_zero
            } else {
                self.flag_zero
            }),
            3 => Some(if is_8bit {
                self.flag8_sign
            } else {
                self.flag_sign
            }),
            _ => None,
        }
    }

    pub fn set(
        &mut self,
        index: usize,
        value: bool,
        is_8bit: bool,
    ) -> Result<(), errors::InvalidFlag> {
        match index {
            0 => Ok(()),
            1 => {
                if is_8bit {
                    self.flag8_carry = value;
                } else {
                    self.flag_carry = value;
                }
                Ok(())
            }
            2 => {
                if is_8bit {
                    self.flag8_zero = value;
                } else {
                    self.flag_zero = value;
                }
                Ok(())
            }
            3 => {
                if is_8bit {
                    self.flag8_sign = value;
                } else {
                    self.flag_sign = value;
                }
                Ok(())
            }
            _ => Err(errors::InvalidFlag { index }),
        }
    }
}
