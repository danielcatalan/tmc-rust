use super::reg_macro::*;

// Velocity Dependent Driver Control Registers
register! { // IHoldIRun
    /// IHOLD_IRUN – Driver current control
    pub struct IHoldIRun (0x10, WO) {
        /// IHOLD \
        /// Standstill current (0=1/32…31=32/32) \
        /// In combination with StealthChop mode, setting
        /// IHOLD=0 allows to choose freewheeling or coil
        /// short circuit for motor stand still
        ihold: u8 | <0..4>,
        /// IRUN \
        /// Motor run current (0=1/32…31=32/32) \
        /// Hint: Choose sense resistors in a way, that normal
        /// IRUN is 16 to 31 for best microstep performance
        irun: u8 | <8..12>,
        /// IHOLDDELAY
        /// Controls the number of clock cycles for motor
        /// power down after a motion as soon as standstill is
        /// detected (stst=1) and TPOWERDOWN has expired.
        /// The smooth transition avoids a motor jerk upon
        /// power down. \
        /// 0: instant power down \
        /// 1..15: Delay per current reduction step in multiple
        /// of 2^18 clocks
        ihold_delay: u8 | <16..19>,
    }
}
