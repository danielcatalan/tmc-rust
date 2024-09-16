pub use crate::registers::convert::*;
pub use crate::registers::traits::*;
pub use crate::registers::utils::create_mask;
pub use paste::paste;

macro_rules! register {
    // Read/Write Register
    ($(#[$attr:meta])*
        $qual:vis struct $name:ident ($address: literal, $reg_type: ident) {$($f:tt)*}) => {
        register!(BASE, $(#[$attr])*, $qual, $name, $address, $reg_type, fields!($reg_type $($f)*); );
    };

    // // Read/Write-clear register
    // ($qual:vis struct $name:ident ($address: literal, RWC) {$($f:tt)*}) => {
    //     register!(BASE, $qual, $name, $address, fields!(RWC $($f)* ); );
    // };

    // Base for all Regiters
    (BASE, $(#[$attr:meta])*, $qual:vis, $name:ident, $address: literal, $reg_type: ident, $($f:tt)* ) => {
        $(#[$attr])*
        #[doc = "(Address="]
        #[doc = stringify!($address)]
        #[doc = ")"]
        $qual struct $name{
            raw_data: u32
        }

        impl $name{

            pub fn new() -> Self{
                $name{raw_data:0}
            }

            $($f)*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
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
        traits!($reg_type, $name);
    };
}

macro_rules! fields {
    // for regiters with no fields with 1bit representation
    ($reg_type: ident $(#[$attr:meta])* self: $t:ty | <$shift:literal>,) => {
        // eg: name: u8, <4>, ...
        fields($reg_type $(#[$attr])* value: $t | <$shift..$shift>,);
    };
    // for regiters with no fields with multi-bit representation
    ($reg_type: ident $(#[$attr:meta])* self: $t:ty | <$lsb:literal..$msb:literal>,) => {
        // eg: name: u8, <4>, ...

        // pub fn value(&self) -> $t {
        //     const MASK: u32 = create_mask($lsb, $msb);
        //     ((self.raw_data >> $lsb) & MASK) as u8
        // }
        fields!($reg_type $(#[$attr])* value: $t | <$lsb..$msb>,);


        // pub fn set_value(&mut self, value: u8){
        //     const BIT_MASK: u32 = 0x01 << $shift;
        //     let value = ((value as u32) & 0x01) << $shift;
        //     self.raw_data = (self.raw_data & !BIT_MASK) | value; // clear bits
        // }
    };

    ($reg_type: ident $(#[$attr:meta])* $name:ident: $t:ty | <$shift:literal>, $($rest:tt)* ) => {
        fields!($reg_type $(#[$attr])* $name: $t | <$shift..$shift>, $($rest)* );
    };


    (RWC $(#[$attr:meta])* $name:ident: $t:ty | <$lsb:literal..$msb:literal>, $($rest:tt)* ) => {
        // eg: "name: u8 | <4>,"

        getter!($(#[$attr])*, $name, $t, $lsb, $msb);

        fields!(RWC $($rest)* );
    };

    (RWC) => {};

    (RW $(#[$attr:meta])* $name:ident: $t:ty | <$lsb:literal..$msb:literal>, $($rest:tt)*) => {

        getter!($(#[$attr])*, $name, $t, $lsb, $msb);

        setter!($(#[$attr])*, $name, $t, $lsb, $msb);

        fields!(RW $($rest)*);
    };

    (RW) => {};

    (RO $(#[$attr:meta])* $name:ident: $t:ty | <$lsb:literal..$msb:literal>, $($rest:tt)*) => {

        getter!($(#[$attr])*, $name, $t, $lsb, $msb);

        fields!(RO $($rest)*);
    };

    (RO) => {};

    () => {};
}

macro_rules! getter {
    ($(#[$attr:meta])*, $name:ident, $t:ty , $lsb:literal, $msb:literal) => {
        // eg: name: u8, <4>, ...
        #[doc="Gets field `"]
        #[doc=stringify!($name)]
        #[doc="`.\n\n"]
        $(#[$attr])*
        pub fn $name(&self) -> $t {
            const MASK:u32 = create_mask($lsb, $msb);
            <$t>::from_u32((self.raw_data & MASK) >> $lsb)
        }
    };
}

macro_rules! setter {
    ($(#[$attr:meta])*, $name:ident, $t:ty , $lsb:literal, $msb:literal) => {
        // eg: name: u8, <4>, ...
        paste! {
            #[doc="Sets field `"]
            #[doc=stringify!($name)]
            #[doc="`.\n\n"]
            $(#[$attr])*
            pub fn [<set_ $name>](&mut self, value: $t){
                const BIT_MASK: u32 = create_mask($lsb,$msb);
                let value = value.to_u32() << $lsb;
                self.raw_data = (self.raw_data & !BIT_MASK) | value; // clear bits
            }
            }
    };

    (CLEAR $(#[$attr:meta])*, $name:ident, $t:ty , $lsb:literal, $msb:literal) => {
        // eg: name: u8, <4>, ...
        paste! {
            #[doc="Clears field `"]
            #[doc=stringify!($name)]
            #[doc="` by setting bit(s) to 1.\n\n"]
            $(#[$attr])*
            pub fn [<clear_ $name>](&mut self){
                const BIT_MASK: u32 = create_mask($lsb,$msb);

                self.raw_data = self.raw_data | BIT_MASK; // clear bits
            }
            }
    };
}

macro_rules! traits {
    (RW, $name:ident) => {
        impl ReadRegister for $name {}
        impl WriteRegister for $name {}
    };

    (RWC, $name:ident) => {
        impl ReadRegister for $name {}
        impl WriteRegister for $name {}
    };
    (RO, $name:ident) => {
        impl ReadRegister for $name {}
    };
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
        x.raw_data = 0x00;
        assert_eq!(0, x.a());
        assert_eq!(0, x.b());
        x.raw_data = 0x01;
        assert_eq!(1, x.a());
        assert_eq!(0, x.b());
        x.raw_data = 0b0100;
        assert_eq!(0, x.a());
        assert_eq!(1, x.b());
        x.raw_data = 0b0101;
        assert_eq!(1, x.a());
        assert_eq!(1, x.b());
    }
}
pub(crate) use fields;
pub(crate) use getter;
pub(crate) use register;
pub(crate) use setter;
pub(crate) use traits;
