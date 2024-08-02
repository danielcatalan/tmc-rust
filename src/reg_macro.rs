macro_rules! fields {
    ($name:ident: $t:ty, <$shift:literal>; $($rest:tt)*) => {

        pub fn $name(&self) -> $t {
            ((self.0 >> $shift) & 0x01) as u8
        }

        fields!($($rest)*);
    };

    () => {};
}

macro_rules! register {
    ($qual:vis struct $name:ident (RW) {$($f:tt)*}) => {
        $qual struct $name(u32);

        impl $name{
            fn new() -> Self{
                $name(0)
            }
            
            fields!( $($f)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    register! {
        struct MyRegister (RW) {
            slave_addr: u8, <0>;
            send_delay: u8, <2>;
        }
    }

    #[test]
    fn test_name() {
        let x = MyRegister(0x00);
        assert_eq!(0, x.send_delay());
        assert_eq!(0, x.slave_addr());

        let x = MyRegister(0x01);
        assert_eq!(0, x.send_delay());
        assert_eq!(1, x.slave_addr());

        let x = MyRegister(0x04);
        assert_eq!(1, x.send_delay());
        assert_eq!(0, x.slave_addr());

        let x = MyRegister(0x05);
        assert_eq!(1, x.send_delay());
        assert_eq!(1, x.slave_addr());
    }
}

pub(crate) use register;
pub(crate) use fields;