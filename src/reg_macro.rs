pub use paste::paste;

pub trait Register {
    const ADDRESS: u8;
    fn get_address(&self) -> u8;
    fn get_bytes(&self) -> [u8; 4];
    fn from_bytes(data: [u8; 4]) -> Self;
}

macro_rules! register {
    // Read/Write Register
    ($qual:vis struct $name:ident ($address: literal, RW) {$($f:tt)*}) => {
        register!(BASE, $qual, $name, $address, fields!($($f)*); );
    };

    // Read/Write-clear register
    ($qual:vis struct $name:ident ($address: literal, RWC) {$($f:tt)*}) => {
        register!(BASE, $qual, $name, $address, fields!(RWC $($f)* ); );
    };
    // Base for all Regiters 
    (BASE, $qual:vis, $name:ident, $address: literal,  $($f:tt)* ) => {
        $qual struct $name{
            raw_data: u32
        }

        impl $name{

            pub fn new() -> Self{
                $name{raw_data:0}
            }

            $($f)*
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
                $name {
                    raw_data: u32::from_le_bytes(data)
                }
            }
        }
    };
}

macro_rules! fields {
    (RWC $name:ident: $t:ty | <$shift:literal>, $($rest:tt)* ) => {
        // eg: "name: u8 | <4>,"

        pub fn $name(&self) -> $t {
            ((self.raw_data >> $shift) & 0x01) as u8
        }

        paste! {
        pub fn [<clear_ $name>](&mut self,){
            const BIT_MASK: u32 = 0x01 << $shift;
            self.raw_data = (self.raw_data & !BIT_MASK) 
        }
        }

        fields!(RWC $($rest)* );
    };

    (RWC) => {};

    ($name:ident: $t:ty | <$shift:literal>, $($rest:tt)*) => {
        // eg: name: u8, <4>, ...

        pub fn $name(&self) -> $t {
            ((self.raw_data >> $shift) & 0x01) as u8
        }

        paste! {
        pub fn [<set_ $name>](&mut self, value: u8){
            const BIT_MASK: u32 = 0x01 << $shift;
            let value = ((value as u32) & 0x01) << $shift;
            self.raw_data = (self.raw_data & !BIT_MASK) | value; // clear bits
        }
        }

        fields!($($rest)*);
    };



    () => {};
}

#[cfg(test)]
mod tests {
    use super::*;
    register! {
        struct RwRegister (0x00, RW) {
            a: u8 | <0>,
            b: u8 | <2>,
        }
    }

    #[test]
    fn test_rw() {
        let mut x = RwRegister::new();
        x.raw_data = 0;
        assert_eq!(0, x.b());
        assert_eq!(0, x.a());

        x.set_a(1);
        assert_eq!(0x01, x.raw_data);
        assert_eq!(0, x.b());
        assert_eq!(1, x.a());

        x.set_a(0);
        x.set_b(1);
        assert_eq!(0x04, x.raw_data);
        assert_eq!(1, x.b());
        assert_eq!(0, x.a());

        x.set_a(1);
        x.set_b(1);
        assert_eq!(0x05, x.raw_data);
        assert_eq!(1, x.b());
        assert_eq!(1, x.a());
    }

    register! {
        struct RwcRegister (0x00, RWC) {
            a: u8 | <0>,
            b: u8 | <2>,
        }
    }
    #[test]
    fn test_rwc() {
        let mut x = RwcRegister::new();
        x.raw_data = 0x05;
        assert_eq!(1, x.a());
        assert_eq!(1, x.b());
        x.clear_a();
        assert_eq!(0, x.a());
        assert_eq!(1, x.b());
        x.raw_data = 0x05;
        x.clear_b();
        assert_eq!(1, x.a());
        assert_eq!(0, x.b());
        x.clear_a();
        assert_eq!(0, x.a());
        assert_eq!(0, x.b());
    }

}
pub(crate) use fields;
pub(crate) use register;
