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
    pub fn perform_operation<T: int::UInt>(&self, input_a: T, input_b: T, flags: Option<&mut Flags>) -> T {
        let input_a = input_a ^ if self.invert_a {T::MAX_VALUE} else {T::_0};
        let input_b = input_b ^ if self.invert_b {T::MAX_VALUE} else {T::_0};

        let mut carry = self.carry_in;

        let mut sum: T = T::_0;

        for bit in 0..T::BIT_COUNT {
            let a_bit = input_a & (T::_1 << bit) != T::_0;
            let b_bit = input_b & (T::_1 << bit) != T::_0;

            let sum_bit = if !self.or_mode {
                a_bit ^ b_bit ^ carry
            } else {
                (a_bit | b_bit) ^ carry
            };

            sum = sum | (if sum_bit {T::_1} else {T::_0}) << bit;

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
            flags.flag_sign = sum & T::_1 << (T::BIT_COUNT - 1) as u8 != T::_0;
            flags.flag_zero = sum == T::_0;
        }

        sum
    }
}
