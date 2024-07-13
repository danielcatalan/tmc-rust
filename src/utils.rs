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
