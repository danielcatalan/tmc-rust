#![no_std]
mod registers;
mod driver;
mod spi_status;

pub use registers::*;
pub use driver::{Operation, Tmc5160};
pub use spi_status::SpiStatus;

