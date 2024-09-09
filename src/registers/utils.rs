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
