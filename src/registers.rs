use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

pub trait Register {
    const ADDRESS: u8;
    fn get_address(&self) -> u8;
    fn get_bytes(&self) -> [u8; 4];
    fn from_bytes(data: [u8; 4]) -> Self;
}

#[macro_export]
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

#[bitfield]
pub struct GCONF {
    pub recalibrate: B1,
    pub faststandstill: B1,
    pub en_pwm_mode: B1,
    pub multistep_filt: B1,
    pub shaft: B1,
    pub diag0_error: B1,
    pub diag0_otpw: B1,
    pub diag0_stall_step: B1,
    pub diag1_stall_dir: B1,
    pub diag1_index: B1,
    pub diag1_onstate: B1,
    pub diag1_steps_skipped: B1,
    pub diag0_int_pushpull: B1,
    pub diag1_poscomp_pushpull: B1,
    pub small_hstesis: B1,
    pub stop_enable: B1,
    pub direct_mode: B1,
    pub test_mode: B1,
    #[skip]
    reserved: B14,
}
make_register!(GCONF, 0x00);

/// Global status flags
#[bitfield]
pub struct GSTAT {
    /// Indicates that the IC has been reset
    pub reset: B1,
    /// Indicates, that the driver has been shut down
    /// due to overtemperature or short circuit detection.
    pub drv_err: B1,
    /// Indicates an undervoltage on the charge pump.
    pub uv_cp: B1,
    #[skip]
    reserved: B29,
}
make_register!(GSTAT, 0x01);

#[bitfield]
pub struct GlobalScaler {
    pub globalscaler: B8,
    #[skip]
    reserved: B24,
}

#[bitfield]
pub struct XACTUAL {
    pub value: B32,
}
make_register!(XACTUAL, 0x21);

#[bitfield]
pub struct VMAX {
    pub value: B23,
    #[skip]
    reserved: B9,
}
make_register!(VMAX, 0x27);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gconf() {
        let mut reg1 = GCONF::new();

        reg1.set_recalibrate(1);
        reg1.set_shaft(1);
        reg1.set_diag1_stall_dir(1);
        reg1.set_test_mode(1);
        let val = reg1.get_bytes();
        assert_eq!([0x11_u8, 0x01_u8, 0x02, 0x00], val);

        let addr = reg1.get_address();
        assert_eq!(0x00, addr);
    }

    #[test]
    fn test_gstat() {
        let mut reg = GSTAT::new();
        reg.set_reset(1);
        reg.set_drv_err(0);
        reg.set_uv_cp(1);

        let val = reg.get_bytes();
        assert_eq!([0x5_u8, 0x0_u8, 0x0, 0x00], val);

        let reg = GSTAT::from_bytes([0x02, 0x00, 0x00, 0x00]);
        assert_eq!(0, reg.reset());
        assert_eq!(1, reg.drv_err());
        assert_eq!(0, reg.uv_cp());

        let addr = reg.get_address();
        assert_eq!(0x01, addr);
    }

    #[test]
    fn test_gs() {
        let mut reg = GlobalScaler::new();
        reg.set_globalscaler(0x55);

        assert_eq!(0x55, reg.globalscaler());

        let data: [u8; 4] = reg.into_bytes();
        assert_eq!([0x55, 0x00, 0x00, 0x00], data);
    }
}
