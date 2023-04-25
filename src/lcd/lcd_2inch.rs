use crate::config::dev_hardware_spi::HardwareSpi;
use crate::config::dev_config::*;
use super::types::Inch;
use sysfs_gpio::{Pin, Direction};
use std::{thread, time};
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

    pub fn init_dev(&mut self) {
        self.pin_cs.set_direction(Direction::Out).expect("[init_dev] error ");
        self.pin_rst.set_direction(Direction::Out).expect("[init_dev] error ");
        self.pin_dc.set_direction(Direction::Out).expect("[init_dev] error ");
        self.pin_bl.set_direction(Direction::Out).expect("[init_dev] error ");
        self.pin_cs.set_value(1).expect("[init_dev] error ");
        self.pin_bl.set_value(1).expect("[init_dev] error ")
    }

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
            self.device.spi.transfer(&mut transfer).expect("[transfer_byte] error");
        }
        return rx_buf[0];
    }

    pub fn transfer(&mut self, buf: &[u8]) {
        let mut offset = 0;
        let mut remaining = buf.len();
        let max_transfer_size = ((10000000 / 8) as usize).min(4096);

        // 分多次发送数据，每次发送不超过最大传输长度
        while remaining > 0 {
            let chunk_size = std::cmp::min(max_transfer_size as usize, remaining);
            let chunk = &buf[offset..offset+chunk_size];
            let mut transfer = SpidevTransfer::write(chunk);
            self.device.spi.transfer(&mut transfer).expect("[transfer] error");

            offset += chunk_size;
            remaining -= chunk_size;
        }
    }

    pub fn set_black(&mut self, value: u16) {
        self.pin_bl.set_value(1).expect("[set_black], error");
    }

    pub fn sleep(&self, ms: u64) {
        let ten_millis = time::Duration::from_millis(ms);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
    }

    pub fn lcd_in_init(&mut self) {
        self.lcd_in_reset();
        self.lcd_in_write_command(0x36);
        self.lcd_in_write_data_byte(0x00);

        self.lcd_in_write_command(0x3A);
        self.lcd_in_write_data_byte(0x05);

        self.lcd_in_write_command(0x21);
        self.lcd_in_write_command(0x2A);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0x01);
        self.lcd_in_write_data_byte(0x3F);

        self.lcd_in_write_command(0x2B);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0xEF);

        self.lcd_in_write_command(0xB2);
        self.lcd_in_write_data_byte(0x0C);
        self.lcd_in_write_data_byte(0x0C);
        self.lcd_in_write_data_byte(0x00);
        self.lcd_in_write_data_byte(0x33);
        self.lcd_in_write_data_byte(0x33);

        self.lcd_in_write_command(0xB7);
        self.lcd_in_write_data_byte(0x35);

        self.lcd_in_write_command(0xBB);
        self.lcd_in_write_data_byte(0x1F);

        self.lcd_in_write_command(0xC0);
        self.lcd_in_write_data_byte(0x2C);

        self.lcd_in_write_command(0xC2);
        self.lcd_in_write_data_byte(0x01);

        self.lcd_in_write_command(0xC3);
        self.lcd_in_write_data_byte(0x12);

        self.lcd_in_write_command(0xC4);
        self.lcd_in_write_data_byte(0x20);

        self.lcd_in_write_command(0xC6);
        self.lcd_in_write_data_byte(0x0F);

        self.lcd_in_write_command(0xD0);
        self.lcd_in_write_data_byte(0xA4);
        self.lcd_in_write_data_byte(0xA1);

        self.lcd_in_write_command(0xE0);
        self.lcd_in_write_data_byte(0xD0);
        self.lcd_in_write_data_byte(0x08);
        self.lcd_in_write_data_byte(0x11);
        self.lcd_in_write_data_byte(0x08);
        self.lcd_in_write_data_byte(0x0C);
        self.lcd_in_write_data_byte(0x15);
        self.lcd_in_write_data_byte(0x39);
        self.lcd_in_write_data_byte(0x33);
        self.lcd_in_write_data_byte(0x50);
        self.lcd_in_write_data_byte(0x36);
        self.lcd_in_write_data_byte(0x13);
        self.lcd_in_write_data_byte(0x14);
        self.lcd_in_write_data_byte(0x29);
        self.lcd_in_write_data_byte(0x2D);

        self.lcd_in_write_command(0xE1);
        self.lcd_in_write_data_byte(0xD0);
        self.lcd_in_write_data_byte(0x08);
        self.lcd_in_write_data_byte(0x10);
        self.lcd_in_write_data_byte(0x08);
        self.lcd_in_write_data_byte(0x06);
        self.lcd_in_write_data_byte(0x06);
        self.lcd_in_write_data_byte(0x39);
        self.lcd_in_write_data_byte(0x44);
        self.lcd_in_write_data_byte(0x51);
        self.lcd_in_write_data_byte(0x0B);
        self.lcd_in_write_data_byte(0x16);
        self.lcd_in_write_data_byte(0x14);
        self.lcd_in_write_data_byte(0x2F);
        self.lcd_in_write_data_byte(0x31);
        self.lcd_in_write_command(0x21);

        self.lcd_in_write_command(0x11);

        self.lcd_in_write_command(0x29);
    }

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
           self.device.spi.transfer(&mut transfer).expect("[transfer] error");
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
           self.device.spi.transfer(&mut transfer).expect("[transfer] error");
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
           self.device.spi.transfer(&mut transfer).expect("[transfer] error");
        }

        let tx_buf = [data_low_byte];
        let mut rx_buf = [0; 1];
        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
           self.device.spi.transfer(&mut transfer).expect("[transfer] error");
        }
        self.pin_cs.set_value(0).expect(format!("[lcd_in_write_data_word]: pin {} error!", self.pin_cs.get_pin()).as_str());
    }

    pub fn lcd_2in_set_window(&mut self, x_start: u16, y_start: u16, x_end: u16, y_end: u16) {
        self.lcd_in_write_command(0x2a);
        self.lcd_in_write_data_byte((x_start >> 8) as u8);
        self.lcd_in_write_data_byte((x_start & 0xff) as u8);
        self.lcd_in_write_data_byte(((x_end - 1) >> 8) as u8);
        self.lcd_in_write_data_byte(((x_end - 1) & 0xff) as u8);

        self.lcd_in_write_command(0x2b);
        self.lcd_in_write_data_byte((y_start >> 8) as u8);
        self.lcd_in_write_data_byte((y_start & 0xff) as u8);

        self.lcd_in_write_data_byte(((y_end - 1) >> 8) as u8);
        self.lcd_in_write_data_byte(((y_end - 1) & 0xff) as u8);

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

    pub fn lcd_2in_clear(&mut self, color: u16) {
        let mut w = 0u16;
        let mut h = 0u16;
        match self.inch {
            Inch::Lcd2inch{width, height} => {
                w = width;
                h = height
            }
            _ => {}
        }
        let mut image = vec![0u16; w as usize];
        for i in 0..w as usize {
            image[i] = (color >> 8 | (color & 0xff) << 8);
        }
        self.lcd_2in_set_window(0, 0, w, h);
        self.pin_dc.set_value(1).expect("[lcd_2in_clear] error");
        for i in 0..h {
            self.transfer(&image.iter().map(|&x| x as u8).collect::<Vec<u8>>()[..])
        }

    }

    pub fn lcd_2in_clear_window(&mut self, x_start: u16, y_start: u16, x_end: u16, y_end: u16, color: u16) {
        self.lcd_2in_set_window(x_start, y_start, x_end - 1, y_end - 1);
    }

    pub fn lcd_2in_display(&mut self, image:  Vec<u16>) {
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
        self.lcd_2in_set_window(0, 0, width, height);
        self.pin_dc.set_value(1).expect(format!("[lcd_2in_display]: pin {} error!", self.pin_dc.get_pin()).as_str());
        let buf = &image.iter().map(|&x| x as u8).collect::<Vec<u8>>()[..];
        self.transfer(buf)
        // for i in 0..height {
        //     let start: usize = i as usize;
        //     let end: usize = (i * 2 * width) as usize;
        //     let buf = &image[start..end].iter().map(|&x| x as u8).collect::<Vec<u8>>()[..];
        //     self.transfer(buf)
        // }
    }

    pub fn lcd_2in_draw_paint(&mut self, x: u16, y: u16, color: u16) {
        self.lcd_in_set_cursor(x, y);
        self.lcd_in_write_data_word(color)
    }
}
