mod stub;

use stub::*;
use tmc5160_driver::registers::VMAX;
use tmc5160_driver::Tmc5160;

type Driver = Tmc5160<StubSpiDevice>;

#[test]
fn integration_write_test() -> tmc5160_driver::error::Result<(), StubError> {
    // Setup Stub
    let mut spi_stub = StubSpiDevice::new();
    spi_stub.setup_miso([0x55, 0, 0, 0, 0]);

    spi_stub.expect_mosi([0x27 | 0x80, 0x00, 0xA, 0xBC, 0xDE]);

    // start test
    let mut driver = Driver { spi: spi_stub };

    let mut reg = VMAX::new();
    reg.set_value(0xABCDE);

    let status = driver.write(&reg)?;

    assert_eq!(0x55_u8, status.into());
    Ok(())
}
