#![no_main]

use libfuzzer_sys::fuzz_target;
extern crate tmc5160_driver;
use tmc5160_driver::SpiStatus;

fuzz_target!(|data: u8| {
    let status = SpiStatus::from(data);
    // fuzzed code goes here
});
