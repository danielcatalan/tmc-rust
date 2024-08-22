pub use paste::paste;

pub trait Register {
    const ADDRESS: u8;
    fn get_address(&self) -> u8;
    fn get_bytes(&self) -> [u8; 4];
    fn from_bytes(data: [u8; 4]) -> Self;
}

macro_rules! fields {
    ($name:ident: $t:ty, <$shift:literal>; $($rest:tt)*) => {

        pub fn $name(&self) -> $t {
            ((self.raw_data >> $shift) & 0x01) as u8
        }

        paste!{
        pub fn [<set_ $name>](&mut self, value: u8){
            let value = (value & 0x01) << $shift;
            self.raw_data = self.raw_data | (value as u32);
        }
        }

        fields!($($rest)*);
    };

    () => {};
}

macro_rules! register {
    ($qual:vis struct $name:ident ($address: literal, RW) {$($f:tt)*}) => {
        $qual struct $name{
            raw_data: u32
        }

        impl $name{

            pub fn new() -> Self{
                $name{raw_data:0}
            }
            
            fields!( $($f)*);
        }

        impl Register for $name {
            const ADDRESS: u8 = $address;
            fn get_address(&self) -> u8 {
                Self::ADDRESS
            }

            fn get_bytes(&self) -> [u8; 4] {
                let mut values: [u8; 4] = [0; 4];
                let x = self.raw_data;
                values.copy_from_slice(&x.to_le_bytes());
                values
            }

            fn from_bytes(data: [u8; 4]) -> $name {
                $name::from_bytes(data)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    register! {
        struct MyRegister (0x00, RW) {
            slave_addr: u8, <0>;
            send_delay: u8, <2>;
        }
    }

    #[test]
    fn test_name() {
        let x = MyRegister{raw_data:0x00};
        assert_eq!(0, x.send_delay());
        assert_eq!(0, x.slave_addr());

        let x = MyRegister{raw_data:0x01};
        assert_eq!(0, x.send_delay());
        assert_eq!(1, x.slave_addr());

        let x = MyRegister{raw_data:0x04};
        assert_eq!(1, x.send_delay());
        assert_eq!(0, x.slave_addr());

        let x = MyRegister{raw_data:0x05};
        assert_eq!(1, x.send_delay());
        assert_eq!(1, x.slave_addr());
    }
}

pub(crate) use register;
pub(crate) use fields;