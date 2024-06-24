use modular_bitfield::{self, bitfield, specifiers::*};


#[bitfield]
pub struct SpiStatus{
    pub reset_flag: B1,
    pub driver_error: B1,
    pub sg2: B1,
    pub standstill: B1,
    pub velocity_reached: B1,
    pub position_reached: B1,
    pub status_stop_l: B1,
    pub status_stop_r: B1,
}

impl From<u8> for SpiStatus{
    fn from(value: u8) -> Self{
        let data: [u8;1] = [value];
        SpiStatus::from_bytes(data)
    }
}