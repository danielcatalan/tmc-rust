use crate::registers::utils::create_mask;

// pub struct SpiStatus{
//     pub reset_flag: B1,
//     pub driver_error: B1,
//     pub sg2: B1,
//     pub standstill: B1,
//     pub velocity_reached: B1,
//     pub position_reached: B1,
//     pub status_stop_l: B1,
//     pub status_stop_r: B1,
// }

pub struct SpiStatus {
    raw_data: u8,
}
impl SpiStatus {
    pub fn reset_flag(&self) -> u8 {
        const SHIFT: u8 = 0;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
    pub fn driver_error(&self) -> u8 {
        const SHIFT: u8 = 1;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
    pub fn sg2(&self) -> u8 {
        const SHIFT: u8 = 2;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }

    pub fn standstill(&self) -> u8 {
        const SHIFT: u8 = 3;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
    pub fn velocity_reached(&self) -> u8 {
        const SHIFT: u8 = 4;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
    pub fn position_reached(&self) -> u8 {
        const SHIFT: u8 = 5;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
    pub fn status_stop_l(&self) -> u8 {
        const SHIFT: u8 = 6;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }

    pub fn status_stop_r(&self) -> u8 {
        const SHIFT: u8 = 7;
        const MASK: u8 = create_mask(SHIFT, SHIFT) as u8;
        (self.raw_data & MASK) >> SHIFT
    }
}

impl From<u8> for SpiStatus {
    fn from(value: u8) -> Self {
        SpiStatus { raw_data: value }
    }
}

impl From<SpiStatus> for u8 {
    fn from(value: SpiStatus) -> Self {
        value.raw_data
    }
}
