#![no_std]
mod driver;

pub mod registers;
mod spi_status;

pub use driver::{Operation, Result, Tmc5160};
pub use spi_status::SpiStatus;
