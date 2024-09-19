mod bitstate;
mod convert;
mod reg_macro;
mod traits;
pub(crate) mod utils;

pub use bitstate::BitState;
use reg_macro::*;
pub use traits::*;

register! { // GCONF
    /// Global Configurations flags
    pub struct GCONF (0x00, RW) {
        /// Zero crossing recalibration during driver disable
        ///
        /// 1: Zero crossing recalibration during driver disable
        /// (via DRV_ENN or via TOFF setting)
        recalibrate:            BitState | <0>,
        /// Timeout for step execution until standstill detection:
        ///
        /// 1: Short time: 2^18 clocks
        ///
        /// 0: Normal time: 2^20 clocks
        faststandstill:         BitState | <1>,
        /// 1: StealthChop voltage PWM mode enabled
        /// (depending on velocity thresholds). Switch from
        /// off to on state while in stand-still and at IHOLD=
        /// nominal IRUN current, only
        en_pwm_mode:            BitState | <2>,
        /// 1: Enable step input filtering for StealthChop
        /// optimization with external step source (default=1)
        multistep_filt:         BitState | <3>,
        /// 1: Inverse motor direction
        shaft:                  BitState | <4>,
        /// (only with SD_MODE=1)
        ///
        /// 1: Enable DIAG0 active on driver errors:
        /// Over temperature (ot), short to GND (s2g)
        ///
        /// DIAG0 always shows the reset-status, i.e., is active low
        /// during reset condition.
        diag0_error:            BitState | <5>,
        /// (only with SD_MODE=1)
        ///
        /// 1: Enable DIAG0 active on driver over temperature
        /// prewarning (otpw)
        diag0_otpw:             BitState | <6>,
        /// diag0_stall (with SD_MODE=1)
        ///
        /// 1: Enable DIAG0 active on motor stall (set
        /// TCOOLTHRS before using this feature)
        ///
        /// diag0_step (with SD_MODE=0)
        ///
        /// 0: DIAG0 outputs interrupt signal
        ///
        /// 1: Enable DIAG0 as STEP output (half frequency,
        /// dual edge triggered) for external STEP/DIR driver
        diag0_stall_step:       BitState | <7>,
        /// diag1_stall (with SD_MODE=1)
        ///
        /// 1: Enable DIAG1 active on motor stall (set
        /// TCOOLTHRS before using this feature)
        ///
        /// diag1_dir (with SD_MODE=0)
        ///
        /// 0: DIAG1 outputs position compare signal
        ///
        /// 1: Enable DIAG1 as DIR output for external STEP/DIR
        /// driver
        diag1_stall_dir:        BitState | <8>,
        ///diag1_index (only with SD_MODE=1)
        ///
        /// 1: Enable DIAG1 active on index position (microstep
        /// look up table position 0)
        diag1_index:            BitState | <9>,
        /// diag1_onstate (only with SD_MODE=1)
        ///
        /// 1: Enable DIAG1 active when chopper is on (for the
        /// coil which is in the second half of the fullstep)
        diag1_onstate:          BitState | <10>,
        ///diag1_steps_skipped (only with SD_MODE=1)
        ///
        /// 1: Enable output toggle when steps are skipped in
        /// DcStep mode (increment of LOST_STEPS). Do not
        /// enable in conjunction with other DIAG1 options.
        diag1_steps_skipped:    BitState | <11>,
        /// diag0_int_pushpull
        ///
        /// 0: SWN_DIAG0 is open collector output (active low)
        ///
        /// 1: Enable SWN_DIAG0 push pull output (active high)
        diag0_int_pushpull:     BitState | <12>,
        /// diag1_poscomp_pushpull
        ///
        /// 0: SWP_DIAG1 is open collector output (active low)
        ///
        /// 1: Enable SWP_DIAG1 push pull output (active high)
        diag1_poscomp_pushpull: BitState | <13>,
        /// small_hysteresis
        ///
        /// 0: Hysteresis for step frequency comparison is 1/16
        ///
        /// 1: Hysteresis for step frequency comparison is 1/32
        small_hystesis:         BitState | <14>,
        /// stop_enable
        ///
        /// 0:  Normal operation
        ///
        /// 1: Emergency stop: ENCA_DCIN stops the sequencer
        /// when tied high (no steps become executed by
        /// the sequencer, motor goes to standstill state).
        stop_enable:            BitState | <15>,
        /// direct_mode
        ///
        /// 0:  Normal operation
        ///
        /// 1: Motor coil currents and polarity directly
        /// programmed via serial interface: Register XTARGET
        /// (0x2D) specifies signed coil A current (bits 8..0)
        /// and coil B current (bits 24..16). In this mode, the
        /// current is scaled by IHOLD setting. Velocity based
        /// current regulation of StealthChop is not available
        /// in this mode. The automatic StealthChop current
        /// regulation will work only for low stepper motor
        /// velocities.
        direct_mode:            BitState | <16>,
        /// test_mode
        ///
        /// 0:  Normal operation
        ///
        /// 1: Enable analog test output on pin ENCN_DCO.
        /// IHOLD[1..0] selects the function of ENCN_DCO:
        /// 0…2: T120, DAC, VDDH
        ///
        /// Hint: Not for user, set to 0 for normal operation!
        test_mode:              BitState | <17>,
    }
}

register! { // GSTAT
    /// Global status flags
    pub struct GSTAT (0x01, RWC) {
        /// reset
        ///
        /// 1:  Indicates that the IC has been reset. All registers
        /// have been cleared to reset values.
        reset: BitState | <0>,
        /// drv_err
        ///
        /// 1:  Indicates, that the driver has been shut down
        /// due to overtemperature or short circuit detection.
        /// Read DRV_STATUS for details. The flag can only
        /// be cleared when the temperature is below the
        /// limit again.
        drv_err: BitState | <1>,
        /// uv_cp
        ///
        /// 1: Indicates an undervoltage on the charge pump.
        /// The driver is disabled during undervoltage. This
        /// flag is latched for information.
        uv_cp: BitState | <2>,
    }
}

register! { // IFCNT
    /// Interface transmission counter. This register becomes
    /// incremented with each successful UART interface write access.
    /// It can be read out to check the serial transmission for lost
    /// data. Read accesses do not change the content. Disabled in SPI
    /// operation. The counter wraps around from 255 to 0
    pub struct IFCNT (0x02, RO) {
        self: u8 | <0..7>,
    }
}

register! { // SLAVECONF
    /// UART Slave Configuration
    pub struct SLAVECONF (0x03, WO) {
        ///These eight bits set the address of unit for the UART
        /// interface. The address becomes incremented by one
        /// when the external address pin NEXTADDR is active.
        /// Range: 0-253 (254 cannot be incremented), default=0
        slave_addr: u8 | <0..7>,
        send_delay: u8 | <8..11>,
    }
}

register! { // IOIN
    /// Reads the state of all input pins available
    pub struct IOIN (0x04, RO) {
        refl_step:       BitState | <0>,
        refr_dir:        BitState | <1>,
        encb_dcen_cfg4:  BitState | <2>,
        enca_dcin_cfg5:  BitState | <3>,
        drv_enn:         BitState | <4>,
        enc_n_dco_cfg6:  BitState | <5>,
        sd_mode:         BitState | <6>,
        swcomp_in:       BitState | <7>,

        /// VERSION: 0x30=first version of the IC
        /// Identical numbers mean full digital compatibility
        version: u8 | <24..31>,
    }
}

register! { // OUTPUT
    /// Sets the IO output pin polarity in UART mode
    pub struct OUTPUT (0x04, WO) {
        /// In UART mode, SDO_CFG0 is an output. This bit
        /// programs the output polarity of this pin. Its main
        /// purpose it to use SDO_CFG0 as NAO next address
        /// output signal for chain addressing of multiple ICs.
        /// Hint: Reset Value is 1 for use as NAO to next IC in
        /// single wire chain
        value: BitState | <0>,
    }
}

register! { // GlobalScaler
    /// Global scaling of Motor current. This value is multiplied
    /// to the current scaling to adapt a drive to a certain
    /// motor type. This value should be chosen before tuning
    /// other settings because it also influences chopper
    /// hysteresis
    pub struct GlobalScaler (0x0B, WO){
        /// 0: Full Scale (or write 256)\
        /// 1 … 31: Not allowed for operation\
        /// 32 … 255: 32/256 … 255/256 of maximum current.\
        /// Hint: Values >128 rec
        value: u8 | <0..7>, // TODO: use concreate type instead of u8.
    }
}

register! { // XACTUAL
    pub struct XACTUAL (0x21, RW) {
        value: u32 | <0..31>,
    }
}

register! { // VMAX
    pub struct VMAX (0x27, RW) {
        value: u32 | <0..22>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gconf() {
        let mut reg1 = GCONF::new();
        assert_eq!(BitState::Zero, reg1.recalibrate());
        reg1.set_recalibrate(BitState::One);
        assert!(BitState::One == reg1.recalibrate());
        let val = reg1.get_bytes();
        assert_eq!([0x01, 0x00, 0x00, 0x00], val);

        assert!(BitState::Zero == reg1.faststandstill());
        reg1.set_faststandstill(BitState::One);
        assert!(BitState::One == reg1.faststandstill());
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
        reg.raw_data = 0x05;
        assert_eq!(BitState::One, reg.reset());
        assert_eq!(BitState::Zero, reg.drv_err());
        assert_eq!(BitState::One, reg.uv_cp());

        let val = reg.get_bytes();
        assert_eq!([0x5_u8, 0x0_u8, 0x0, 0x00], val);

        let reg = GSTAT::from_bytes([0x02, 0x00, 0x00, 0x00]);
        assert_eq!(BitState::Zero, reg.reset());
        assert_eq!(BitState::One, reg.drv_err());
        assert_eq!(BitState::Zero, reg.uv_cp());

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
        reg.set_value(0x55);

        assert_eq!(0x55, reg.value());

        let data: [u8; 4] = reg.get_bytes();
        assert_eq!([0x55, 0x00, 0x00, 0x00], data);
    }

    #[test]
    fn test_ioin() {
        let mut reg = IOIN::new();

        reg.raw_data = 0x340000A5;
        assert_eq!(BitState::One, reg.refl_step());
        assert_eq!(BitState::Zero, reg.refr_dir());
        assert_eq!(BitState::One, reg.encb_dcen_cfg4());
        assert_eq!(BitState::Zero, reg.enca_dcin_cfg5());
        assert_eq!(BitState::Zero, reg.drv_enn());
        assert_eq!(BitState::One, reg.enc_n_dco_cfg6());
        assert_eq!(BitState::Zero, reg.sd_mode());

        assert_eq!(0x34, reg.version());
    }
}
