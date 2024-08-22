use crate::utils::make_register;
use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::reg_macro::*;
pub use crate::reg_macro::Register;

register!{
    pub struct GCONF (0x00, RW) {
        recalibrate:    u8, <0>;
        faststandstill: u8, <1>;
        en_pwm_mode:    u8, <2>;
        multistep_filt: u8, <3>;
        // pub shaft: B1,
        // pub diag0_error: B1,
        // pub diag0_otpw: B1,
        // pub diag0_stall_step: B1,
        // pub diag1_stall_dir: B1,
        // pub diag1_index: B1,
        // pub diag1_onstate: B1,
        // pub diag1_steps_skipped: B1,
        // pub diag0_int_pushpull: B1,
        // pub diag1_poscomp_pushpull: B1,
        // pub small_hstesis: B1,
        // pub stop_enable: B1,
        // pub direct_mode: B1,
        // pub test_mode: B1,
        // #[skip]
        // __: B14,
    }
}

// make_register!(GCONF, 0x00);

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
    __: B29,
}
make_register!(GSTAT, 0x01);

/// Interface transmission counter. This register becomes
/// incremented with each successful UART interface write access.
/// It can be read out to check the serial transmission for lost
/// data. Read accesses do not change the content. Disabled in SPI
/// operation. The counter wraps around from 255 to 0
#[bitfield]
pub struct IFCNT {
    #[skip(setters)]
    pub value: B8,
    #[skip]
    __: B24,
}
make_register!(IFCNT, 0x02);

#[bitfield]
pub struct SLAVECONF {
    ///These eight bits set the address of unit for the UART
    /// interface. The address becomes incremented by one
    /// when the external address pin NEXTADDR is active.
    /// Range: 0-253 (254 cannot be incremented), default=0
    pub slave_addr: B8,
    pub send_delay: B4,
    #[skip]
    __: B20,
}
make_register!(SLAVECONF, 0x03);

/// Reads the state of all input pins available
#[bitfield]
pub struct IOIN {
    pub refl_step: B1,
    pub refr_dir: B1,
    pub encb_dcen_cfg4: B1,
    pub enca_dcin_cfg5: B1,
    pub drv_enn: B1,
    pub enc_n_dco_cfg6: B1,
    pub sd_mode: B1,
    pub swcomp_in: B1,
    #[skip]
    __: B16,
    ///VERSION: 0x30=first version of the IC
    /// Identical numbers mean full digital compatibility
    pub version: B8,
}
make_register!(IOIN, 0x04);

#[bitfield]
pub struct GlobalScaler {
    pub globalscaler: B8,
    #[skip]
    __: B24,
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
    __: B9,
}
make_register!(VMAX, 0x27);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gconf() {
        let mut reg1 = GCONF::new();
        reg1.set_recalibrate(1);
        let val = reg1.get_bytes();
        assert_eq!([0x01, 0x00, 0x00, 0x00], val);
        
        reg1.set_faststandstill(1);
        let val = reg1.get_bytes();
        assert_eq!([0x03, 0x00, 0x00, 0x00], val);
        // reg1.set_shaft(1);
        // reg1.set_diag1_stall_dir(1);
        // reg1.set_test_mode(1);


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
    fn test_ifcnt() {
        let data = [0x55, 0xFF, 0xFF, 0xFF];
        let reg = IFCNT::from_bytes(data);
        assert_eq!(0x55, reg.value());
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
