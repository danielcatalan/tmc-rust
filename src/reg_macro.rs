macro_rules! fields {
    ($name:ident; $t:ty, <$shift:literal>; $($rest:tt)*) => {
        pub fn $name(&self) -> $t {
            ((self.0 >> $shift) & 0xFF) as u8
        }

        fields!($($rest)*);
    };

    () => {};
}

macro_rules! register {
    ($qual:vis $name:ident {$($f:tt)*}) => {
        $qual struct $name(u32);

        impl $name{
            fields!( $($f)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    register! {
        MyRegister {
            slave_addr; u8, <0>;
            send_delay; u8, <8>;
        }
    }

    #[test]
    fn test_name() {
        let x = MyRegister(0x0305);

        assert_eq!(3, x.send_delay());
        assert_eq!(5, x.slave_addr());
    }
}

pub(crate) use register;
