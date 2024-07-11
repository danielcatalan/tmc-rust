use embedded_hal::spi::SpiDevice;

use crate::Register;
use crate::SpiStatus;

// for reference: https://www.analog.com/media/en/technical-documentation/data-sheets/TMC5160A_datasheet_rev1.17.pdf

pub struct Tmc5160<Spi>
where
    Spi: SpiDevice<u8>,
{
    pub spi: Spi,
}

pub enum Operation {
    Read = 0x00,
    Write = 0x80,
}

impl<Spi> Tmc5160<Spi>
where
    Spi: SpiDevice<u8>,
{
    #[inline(always)]
    pub fn write<Reg: Register>(&mut self, data: &Reg) -> Result<SpiStatus, Spi::Error> {
        let address = data.get_address();
        let tx_data = data.get_bytes();

        self.write_impl(address, tx_data)
    }

    #[inline(always)]
    pub fn read<Reg: Register>(&mut self) -> Result<(SpiStatus,Reg), Spi::Error> {
        let address = Reg::ADDRESS;
        let (status, miso_data) = self.read_impl(address)?;

        Ok((status, Reg::from_bytes(miso_data)))
    }

    fn write_impl(&mut self, address: u8, tx_data: [u8; 4]) -> Result<SpiStatus, Spi::Error> {
        let op = Operation::Write;
        let mosi_packet = create_mosi_packet(address, op, tx_data);
        let mut miso_packet: [u8; 5] = [0x00; 5];

        self.spi.transfer(&mut miso_packet, &mosi_packet)?;

        Ok(SpiStatus::from(miso_packet[0]))
    }

    fn read_impl(&mut self, address: u8) -> Result<(SpiStatus, [u8;4]), Spi::Error> {
        let op = Operation::Read;
        let tx_data: [u8;4] = [0x00;4];
        let mosi_packet = create_mosi_packet(address, op, tx_data);
        let mut miso_packet: [u8; 5] = [0x00; 5];

        self.spi.transfer(&mut miso_packet, &mosi_packet)?;

        Ok(parse_miso_packet(miso_packet))
    }
}

fn parse_miso_packet(data_bytes: [u8;5]) -> (SpiStatus, [u8;4]){
    let status = SpiStatus::from(data_bytes[0]);
    let mut data: [u8;4] = [0x00; 4];
    data.clone_from_slice(&data_bytes[1..]);
    data.reverse();
    (status, data)
}

fn create_mosi_packet(address: u8, op: Operation, tx_data: [u8; 4]) -> [u8; 5] {
    let mut buf: [u8; 5] = [0; 5];
    buf[0] = (op as u8) | address;

    let mut data_reversed = tx_data;
    data_reversed.reverse();
    buf[1..].copy_from_slice(&data_reversed);

    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VMAX, XACTUAL};

    #[test]
    fn test_write_packet() {
        /*
        action                  | data sent to TMC5160 | data received from TMC5160
        ----------------------------------------------|---------------------------
        write VMAX:= 0x00123456 | 0xA700123456         | 0xSS00ABCDEF
         */

        let reg = VMAX::new().with_value(0x123456);
        let addr = reg.get_address();
        let tx_data = reg.get_bytes();
        let op = Operation::Write;
        let packet = create_mosi_packet(addr, op, tx_data);
        assert_eq!(packet, [0xA7, 0x00, 0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_read_packet() {
        /*
        action                  | data sent to TMC5160 | data received from TMC5160
        ----------------------------------------------|---------------------------
        read XACTUAL            | 0x2100000000         | 0xSS & unused data
        read XACTUAL            | 0x2100000000         | 0xSS & XACTUAL
         */
        let reg = XACTUAL::new();
        let addr = reg.get_address();
        let tx_data = reg.get_bytes();
        let op = Operation::Read;
        let packet = create_mosi_packet(addr, op, tx_data);
        assert_eq!(packet, [0x21, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_parse_miso(){

        let miso_bytes:[u8; 5] = [0xA5, 0x12, 0x34, 0x56, 0x78];

        let (status, reg) = parse_miso_packet(miso_bytes); 
        let reg = XACTUAL::from_bytes(reg);
        assert_eq!(0x12345678, reg.value());

        assert_eq!(1, status.reset_flag());
        assert_eq!(0, status.driver_error());
        assert_eq!(1, status.sg2());
        assert_eq!(0, status.standstill());
        assert_eq!(0, status.velocity_reached());
        assert_eq!(1, status.position_reached());
        assert_eq!(0, status.status_stop_l());
        assert_eq!(1, status.status_stop_r());
    }
}
