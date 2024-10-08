use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiDevice};

#[derive(Debug)]
pub struct StubSpiDevice {
    stub_miso: [u8; 5],
    stub_mosi: [u8; 5],
    stub_error: Option<StubError>,
}

impl StubSpiDevice {
    pub fn new() -> Self {
        StubSpiDevice {
            stub_miso: [0; 5],
            stub_mosi: [0; 5],
            stub_error: None,
        }
    }

    pub fn setup_miso(&mut self, data: [u8; 5]) {
        self.stub_miso = data;
    }

    pub fn expect_mosi(&mut self, data: [u8; 5]) {
        self.stub_mosi = data;
    }
}

impl ErrorType for StubSpiDevice {
    type Error = StubError;
}

impl SpiDevice<u8> for StubSpiDevice {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        // check stub error
        match &self.stub_error {
            None => {}
            Some(err) => {
                return Err(err.clone());
            }
        }
        // check stub operations
        for op in operations {
            match op {
                Operation::Read(_) => todo!(),
                Operation::Write(_) => todo!(),
                Operation::Transfer(miso_packet, mosi_packet) => {
                    // Print MOSI packet
                    {
                        let mosi_packet = *mosi_packet;
                        print!("Tx => [ ");
                        for b in mosi_packet {
                            print!("{b:02X} ")
                        }
                        println!("]");
                    }

                    // Do Assertion
                    assert_eq!(self.stub_mosi, *mosi_packet);

                    miso_packet.copy_from_slice(&self.stub_miso[..]);

                    {
                        let miso_packet: &[u8] = miso_packet;
                        print!("Rx <= [ ");
                        for b in miso_packet {
                            print!("{b:02X} ")
                        }
                        println!("]");
                    }
                }
                Operation::TransferInPlace(_) => todo!(),
                Operation::DelayNs(_) => todo!(),
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StubError {}

impl embedded_hal::spi::Error for StubError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}
