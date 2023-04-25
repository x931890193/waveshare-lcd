use spidev::{Spidev, SpidevOptions, SpiModeFlags};

const BITS: u8 = 8;
const SPI_CPHA: u8 = 0x01;
const SPI_CPOL: u8 = 0x02;
const SPI_MODE_0: u8 = 0;
const SPI_MODE_1: u8 = 0 | SPI_CPHA;
const SPI_MODE_2: u8 = SPI_CPOL | 0;
const SPI_MODE_3: u8 = SPI_CPOL | SPI_CPHA;

enum SPIMode {
    SpiMode0(u8),  /*< CPOL = 0, CPHA = 0 */
    SpiMode1(u8),  /*< CPOL = 0, CPHA = 1 */
    SpiMode2(u8),  /*< CPOL = 1, CPHA = 0 */
    SpiMode3(u8)   /*< CPOL = 1, CPHA = 1 */
}

enum SPICSEN {
    Disable,
    Enable
}

enum SPIChipSelect {
    SpiCsModeLow(i32), /*< Chip Select 0 */
    SpiCsModeHigh(i32), /*< Chip Select 1 */
    SpiCsModeNone(i32), /*< No CS, control it yourself */
}

enum SPIBitOrder {
    SpiBitOrderLsbfirst = 0,  /*< LSB First */
    SpiBitOrderMsbfirst = 1   /*< MSB First */
}

enum BusMode {
    Spi3wireMode = 0,
    Spi4wireMode = 1
}

pub struct HardwareSpi{
    pub spi: Spidev
}

const SPI_CS_HIGH: u8 = 0x04;
//Chip select high
const SPI_LSB_FIRST: u8 = 0x08;
//LSB
const SPI_3WIRE: u8 = 0x10;
//3-wire mode SI and SO same line
const SPI_LOOP: u8 = 0x20;
//Loopback mode
const SPI_NO_CS: u8 = 0x40;
//A single device occupies one SPI bus, so there is no chip select
const SPI_READY: u8 = 0x80;                // Slave pull low to stop data transmission


impl HardwareSpi {
    // new HardwareSpi instance
    fn new(device_name: &str) -> Self {
        let mut spi = Spidev::open(device_name).expect(format!("open {} error", device_name).as_str());
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(10000000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options).expect(format!("spi configure {} error", device_name).as_str());
        HardwareSpi{
            spi
        }
    }

    /******************************************************************************
    function:   SPI port initialization
    parameter:
        device_name : Device name
    Info:
        /dev/spidev0.0
        /dev/spidev0.1
    ******************************************************************************/
    fn dev_hardware_spi_begin(&mut self, device_name: &str) {}

    fn dev_hardware_spi_begin_set(device_name: &str, mode: SPIMode, speed:i32) {

    }

    fn dev_hardware_spi_end() {}

    fn dev_hardware_spi_set_speed(speed: i32) {

    }

    fn dev_hardware_spi_transfer_byte(buf: &[u8]) {

    }

    fn dev_hardware_spi_transfer(buf: &[u8], len: u32) {

    }

    fn dev_hardware_spi_set_data_interval(us: u16) {

    }

    fn dev_hardware_spi_set_bus_mode(mode: BusMode) {

    }

    fn dev_hardware_spi_set_bit_order(order: SPIBitOrder) {

    }

    fn dev_hardware_spi_chip_select(cs_mode: SPIChipSelect) {

    }

    fn dev_hardware_spi_csen(en: SPICSEN) {}

    fn dev_hardware_spi_mode(mode: SPIMode) {

    }

}
