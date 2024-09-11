use crate::registers::convert::Convert;

#[derive(PartialEq)]
pub enum BitState {
    Zero,
    One,
}

impl Convert for BitState {
    fn from_u32(value: u32) -> Self {
        if (value & 0x01) == 1 {
            BitState::One
        } else {
            BitState::Zero
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }
}
