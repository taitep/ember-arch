mod errors;

#[derive(Debug)]
pub struct Ember {
    ram: [u8; u16::MAX as usize + 1],
    regs: [Register; 4],
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
}

/// Represents a single register and contains its data (or marks it as a zero register)
#[derive(Debug, Clone, Copy)]
pub enum Register {
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
