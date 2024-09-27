use super::reg_macro::*;

// Ramp Generator Motion Control Registers
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
