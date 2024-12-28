use crate::Flags;

#[derive(Debug, Clone, Copy, Default)]
pub struct ALUSettings {
    pub invert_a: bool,
    pub invert_b: bool,
    pub flood_carry: bool,
    pub carry_in: bool,
    pub or_mode: bool,
}

impl From<u8> for ALUSettings {
    /// expects the data to be in the LSBs
    fn from(value: u8) -> Self {
        Self {
            invert_a: (value & 0b10000u8) != 0,
            invert_b: (value & 0b01000u8) != 0,
            flood_carry: (value & 0b00100u8) != 0,
            carry_in: (value & 0b00010u8) != 0,
            or_mode: (value & 0b00001u8) != 0,
        }
    }
}

impl ALUSettings {
    fn perform_operation(&self, input_a: u16, input_b: u16, flags: Option<&mut Flags>) -> u16 {
        let input_a = input_a ^ self.invert_a as u16 * 0xFFFF;
        let input_b = input_b ^ self.invert_b as u16 * 0xFFFF;

        let mut carry = self.carry_in;

        let mut sum = 0u16;

        for bit in 0..16usize {
            let a_bit = input_a & (1 << bit) != 0;
            let b_bit = input_b & (1 << bit) != 0;

            let sum_bit = if !self.or_mode {
                a_bit ^ b_bit ^ carry
            } else {
                (a_bit | b_bit) ^ carry
            };

            sum |= (sum_bit as u16) << bit;

            carry = if self.flood_carry {
                true
            } else if !self.or_mode {
                [a_bit, b_bit, carry].iter().filter(|x| **x).count() >= 2
            } else {
                a_bit & b_bit & carry
            }
        }

        if let Some(flags) = flags {
            flags.flag_carry = carry;
            flags.flag_sign = sum & 0b10000000_00000000 != 0;
            flags.flag_zero = sum == 0;
        }

        sum
    }

    fn perform_operation_8bit(&self, input_a: u8, input_b: u8, flags: Option<&mut Flags>) -> u8 {
        let input_a = input_a ^ self.invert_a as u8 * 0xFF;
        let input_b = input_b ^ self.invert_b as u8 * 0xFF;

        let mut carry = self.carry_in;

        let mut sum = 0u8;

        for bit in 0..8usize {
            let a_bit = input_a & (1 << bit) != 0;
            let b_bit = input_b & (1 << bit) != 0;

            let sum_bit = if !self.or_mode {
                a_bit ^ b_bit ^ carry
            } else {
                (a_bit | b_bit) ^ carry
            };

            sum |= (sum_bit as u8) << bit;

            carry = if self.flood_carry {
                true
            } else if !self.or_mode {
                [a_bit, b_bit, carry].iter().filter(|x| **x).count() >= 2
            } else {
                a_bit & b_bit & carry
            }
        }

        if let Some(flags) = flags {
            flags.flag_carry = carry;
            flags.flag_sign = sum & 0b10000000 != 0;
            flags.flag_zero = sum == 0;
        }

        sum
    }
}
