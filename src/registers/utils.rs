macro_rules! make_register {
    ($name: ident, $address: literal) => {
        impl Register for $name {
            const ADDRESS: u8 = $address;
            fn get_address(&self) -> u8 {
                Self::ADDRESS
            }

            fn get_bytes(&self) -> [u8; 4] {
                let mut values: [u8; 4] = [0; 4];
                values.copy_from_slice(&self.bytes);
                values
            }

            fn from_bytes(data: [u8; 4]) -> $name {
                $name::from_bytes(data)
            }
        }
    };
}

pub(crate) use make_register;

pub const fn create_mask(lsb: u8, msb: u8) -> u32 {
    let bits = (msb + 1) - lsb;
    let mask = 2u32.pow(bits as u32) - 1;
    mask << lsb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mask() {
        let mask = create_mask(0,0); // 0b0001
        assert_eq!(1, mask);

        let mask = create_mask(0,1); // 0b0011
        assert_eq!(3, mask);

        let mask = create_mask(2,2); // 0b0100
        assert_eq!(4, mask);

        let mask = create_mask(0,2); // 0b0111
        assert_eq!(7, mask);
    }
}