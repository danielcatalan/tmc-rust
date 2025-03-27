use ftdi_embedded_hal as hal;
use ftdi_embedded_hal::libftd2xx::Ft232h;
use ftdi_embedded_hal::libftd2xx::TimeoutError;
use ftdi_embedded_hal::SpiDevice;

use tmc5160_driver::Tmc5160;

fn main() {
    let driver = foo();
}

fn foo() -> Tmc5160<Driver> {
    let device = Ft232h::with_serial_number("").unwrap();

    let hal = hal::FtHal::init_default(device).unwrap();

    let spi = hal.spi_device(3).unwrap();

    let driver = Driver { spi };
    Tmc5160 { spi: driver }
}

struct Driver {
    pub spi: SpiDevice<Ft232h>,
}

impl embedded_hal::spi::SpiDevice for Driver {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        let mut ref_spi = &self.spi;

        let result = ref_spi.transaction(operations);
        result
    }
}
impl embedded_hal::spi::ErrorType for Driver {
    type Error = ftdi_embedded_hal::Error<TimeoutError>;
}
