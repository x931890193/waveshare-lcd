use std::io::Write;
use crate::config::dev_hardware_spi::HardwareSpi;
use crate::config::dev_config::*;
use super::types::Inch;
use sysfs_gpio::Pin;
use std::{thread, time};
use std::borrow::Borrow;
use std::slice::from_mut;
use spidev::SpidevTransfer;


pub struct LCD {
    pub device: HardwareSpi,
    pub inch: Inch,
    pin_cs: Pin,
    pin_rst: Pin,
    pin_dc: Pin,
    pin_bl: Pin,
}

impl LCD {
    pub fn new(inch: Inch, spi: HardwareSpi) -> LCD {
        return LCD {
            device: spi,
            inch,
            pin_cs: Pin::new(LCD_CS as u64),
            pin_rst: Pin::new(LCD_RST as u64),
            pin_dc: Pin::new(LCD_DC as u64),
            pin_bl: Pin::new(LCD_BL as u64),
        };
    }

    pub fn transfer_byte(&mut self, buf: u8) -> u8 {
        let tx_buf = [buf];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }
        return rx_buf[0];
    }

    pub fn transfer(&mut self, buf: u8, len: u32) {
        let tx_buf = [buf];
        let mut rx_buf = Vec::with_capacity(len as usize);
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }
    }

    pub fn sleep(&self, ms: u64) {
        let ten_millis = time::Duration::from_millis(ms);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
    }

    pub fn lcd_in_init(&mut self) {}

    pub fn lcd_in_reset(&self) {
        self.pin_cs.set_value(1).expect(format!("[lcd_in_init]: pin {} error!", self.pin_cs.get_pin()).as_str());
        self.sleep(100);
        self.pin_rst.set_value(0).expect(format!("[lcd_in_init]: pin {} error!", self.pin_rst.get_pin()).as_str());
        self.sleep(100);
        self.pin_rst.set_value(1).expect(format!("[lcd_in_init]: pin {} error!", self.pin_rst.get_pin()).as_str());
        self.sleep(100);
    }

    pub fn lcd_in_write_command(&mut self, data: u8) -> u8 {
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_command]: pin {} error!", self.pin_cs.get_pin()).as_str());
        self.pin_dc.set_value(0).expect(format!("[lcd_in_write_command]: pin {} error!", self.pin_dc.get_pin()).as_str());
        let tx_buf = [data];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }
        return rx_buf[0];
    }

    pub fn lcd_in_write_data_byte(&self, data: u8) {
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_data_byte]: pin {} error!", self.pin_cs.get_pin()).as_str());
        self.pin_dc.set_value(0).expect(format!("[lcd_in_write_data_byte]: pin {} error!", self.pin_dc.get_pin()).as_str());
        let tx_buf = [data];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_data_byte]: pin {} error!", self.pin_cs.get_pin()).as_str());
    }

    pub fn lcd_in_write_data_word(&self, data: u16) {
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_data_word]: pin {} error!", self.pin_cs.get_pin()).as_str());
        self.pin_dc.set_value(0).expect(format!("[lcd_in_write_data_word]: pin {} error!", self.pin_dc.get_pin()).as_str());
        let data_high_byte: u8 = ((data >> 8) & 0xff) as u8;
        let data_low_byte = (data & 0xff) as u8;

        let tx_buf = [data_high_byte];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }

        let tx_buf = [data_low_byte];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.device.spi.transfer(&mut transfer)?;
        }
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_data_word]: pin {} error!", self.pin_cs.get_pin()).as_str());
    }

    pub fn lcd_2in_set_window(&mut self, x_start: u16, y_start: u16, x_send: u16, y_sned: u16) {
        self.lcd_in_write_command(0x2a);
        self.lcd_in_write_data_byte((x_start >> 8) as u8);
        self.lcd_in_write_data_byte(((x_send - 1) >> 8) as u8);
        self.lcd_in_write_data_byte(((x_send - 1) >> 0xff) as u8);

        self.lcd_in_write_command(0x2b);
        self.lcd_in_write_data_byte((x_start >> 8) as u8);
        self.lcd_in_write_data_byte(((x_send - 1) >> 8) as u8);
        self.lcd_in_write_data_byte(((x_send - 1) >> 0xff) as u8);

        self.lcd_in_write_command(0x2c);
    }
    pub fn lcd_in_set_cursor(&mut self, x: u16, y: u16) {
        self.lcd_in_write_command(0x2a);
        self.lcd_in_write_command((x >> 8) as u8);
        self.lcd_in_write_command(x as u8);
        self.lcd_in_write_command((x >> 8) as u8);
        self.lcd_in_write_command(x as u8);

        self.lcd_in_write_command(0x2b);
        self.lcd_in_write_command((y >> 8) as u8);
        self.lcd_in_write_command(y as u8);
        self.lcd_in_write_command((y >> 8) as u8);
        self.lcd_in_write_command(y as u8);

        self.lcd_in_write_command(0x2c);
    }

    pub fn lcd_2in_clear(&self, color: u16) {

    }

    pub fn lcd_2in_clear_window(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16) {
        self.lcd_2in_set_window(x_start, y_start, x_send - 1, y_send - 1);
    }

    pub fn lcd_2in_display(&mut self, image: u16) {
        let mut width = 0;
        let mut height = 0;
        match self.inch {
            Inch::Lcd1inch03{width:x, height: y}
            | Inch::Lcd1inch08 { width: x, height: y }
            | Inch::Lcd1inch14 { width: x, height: y }
            | Inch::Lcd1inch28 { width: x, height: y }
            | Inch::Lcd1inch64 { width: x, height: y }
            | Inch::Lcd2inch { width: x, height: y }
            | Inch::Lcd2inch4 { width: x, height: y }
            => {
                width = x;
                height = y
            }
            _ => {
                eprint!("errir width, height!")
            }
        }
        self.pin_dc.set_value(1).expect(format!("[lcd_2in_display]: pin {} error!", self.pin_dc.get_pin()).as_str());
        self.lcd_2in_set_window(0, 0, width, height  )
    }

    pub fn lcd_2in_draw_paint(&mut self, x: u16, y: u16, color: u16) {
        self.lcd_in_set_cursor(x, y);
        self.lcd_in_write_data_word(color)
    }
}