//! This is a platform agnostic Rust driver for the W25 serial flash
//! memory devices from Winbond using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hals
//!
//! ## The devices
//!
//! Winbond's W25X and W25Q SpiFlash® Multi-I/O Memories feature the
//! popular Serial Peripheral Interface (SPI), densities from 512K-bit to
//! 512M-bit, small erasable sectors and the industry's highest performance.
//!
//! Datasheets:
//! - [W25Q64FW](https://www.winbond.com/resource-files/w25q64fw%20revn%2005182017%20sfdp.pdf)
//!
//! ## Usage
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples].
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
pub use hal::spi::{MODE_0, MODE_3};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<SpiError, PinError> {
    /// SPI communication error
    Spi(SpiError),
    /// Chip select pin set error
    Pin(PinError),
}

/// SPI interface
#[doc(hidden)]
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
}

/// SPI blocking read/write
pub trait ReadWrite {
    /// Final error type
    type Error;

    /// write payload then read
    fn write_read(
        &mut self,
        write_payload: &[u8],
        read_payload: &mut [u8],
    ) -> Result<(), Self::Error>;
}

impl<SPI, CS, SpiE, PinE> ReadWrite for SpiInterface<SPI, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = SpiE>
        + hal::blocking::spi::Transfer<u8, Error = SpiE>,
    CS: hal::digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<SpiE, PinE>;

    fn write_read(
        &mut self,
        write_payload: &[u8],
        mut read_payload: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        let result = self.spi.write(&write_payload).map_err(Error::Spi);
        result?;
        let result = self.spi.transfer(&mut read_payload).map_err(Error::Spi);
        self.cs.set_high().map_err(Error::Pin)?;
        result?;
        Ok(())
    }
}

struct Commands;
impl Commands {
    const JEDEC_ID: u8 = 0x9F;
}

/// W25 serial flash memory device
#[derive(Debug, Default)]
pub struct W25<DI> {
    iface: DI,
}

impl<SPI, CS> W25<SpiInterface<SPI, CS>> {
    /// Create new W25Q64 device
    pub fn new_w25q64(spi: SPI, cs: CS) -> Self {
        W25 {
            iface: SpiInterface { spi, cs },
        }
    }
}

impl<DI, SpiE, PinE> W25<DI>
where
    DI: ReadWrite<Error = Error<SpiE, PinE>>,
{
    /// Get the JEDEC ID
    pub fn get_jedec_id(&mut self) -> Result<[u8; 3], DI::Error> {
        let mut id = [0; 3];
        self.iface.write_read(&[Commands::JEDEC_ID], &mut id)?;
        Ok(id)
    }
}

mod private {
    use super::SpiInterface;
    pub trait Sealed {}

    impl<SPI, CS> Sealed for SpiInterface<SPI, CS> {}
}
