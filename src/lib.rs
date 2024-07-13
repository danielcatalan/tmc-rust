#![no_std]
mod driver;
mod registers;
mod spi_status;
mod utils;

pub use driver::{Operation, Tmc5160};
pub use registers::*;
pub use spi_status::SpiStatus;
