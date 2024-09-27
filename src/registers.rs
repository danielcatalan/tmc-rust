mod bitstate;
mod convert;
mod general_config_registers;
mod ramp_generator_registers;
mod reg_macro;
mod traits;
mod velocity_dep_driver_registers;

pub(crate) mod utils;

pub use bitstate::BitState;
pub use general_config_registers::*;
pub use ramp_generator_registers::*;
pub use traits::*;
pub use velocity_dep_driver_registers::*;
