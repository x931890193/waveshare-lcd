use std::io::Write;
use anyhow::Result;

use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use sysfs_gpio::{Direction, Error, Pin};

const GPIOCHIP_BASE: u64 = 0;
const LCD_CS: u64 = GPIOCHIP_BASE + 49;
const LCD_RST: u64 = GPIOCHIP_BASE + 42;
const LCD_DC: u64 = GPIOCHIP_BASE + 44;
const LCD_BL: u64 = GPIOCHIP_BASE + 51;

const SPI_MODE: SpiModeFlags = SpiModeFlags::SPI_MODE_0;
const SPI_BITS_PER_WORD: u8 = 8;
const SPI_SPEED: u32 = 10_000_000; // 10 MHz

fn init_gpio() -> Result<(Pin, Pin, Pin, Pin)>   {
    let cs = Pin::new(LCD_CS);
    let rst = Pin::new(LCD_RST);
    let dc = Pin::new(LCD_DC);
    let bl = Pin::new(LCD_BL);

    cs.export().unwrap();
    rst.export().unwrap();
    dc.export().unwrap();
    bl.export().unwrap();

    cs.set_direction(Direction::Out).unwrap();
    rst.set_direction(Direction::Out).unwrap();
    dc.set_direction(Direction::Out).unwrap();
    bl.set_direction(Direction::Out).unwrap();
    Ok((cs, dc, rst, bl))
}

fn reset(rst: &Pin) {
    rst.set_value(0).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
    rst.set_value(1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
}

fn init_display(spi: &mut Spidev, cs: &Pin, dc: &Pin) {
    cs.set_value(0).unwrap();

    spi.write(&[0x00, 0xAE]).unwrap(); // 关闭显示
    spi.write(&[0x00, 0xD5, 0x50]).unwrap(); // 设置显示时钟分频
    spi.write(&[0x00, 0xA8, 0x1F]).unwrap(); // 设置多路复用率
    spi.write(&[0x00, 0xD3, 0x00]).unwrap(); // 设置显示偏移
    spi.write(&[0x00, 0x40 | 0x00]).unwrap(); // 设置起始行
    spi.write(&[0x00, 0x8D, 0x14]).unwrap(); // 设置充电泵电压
    spi.write(&[0x00, 0x20, 0x00]).unwrap(); // 设置内存地址模式（水平模式）
    spi.write(&[0x00, 0xA0 | 0x00]).unwrap(); // 设置段重定向（正常模式）
    spi.write(&[0x00, 0xC8]).unwrap(); // 设置COM扫描方向（正常模式）
    spi.write(&[0x00, 0xDA, 0x02]).unwrap(); // 设置COM引脚硬件配置
    spi.write(&[0x00, 0x81, 0xCF]).unwrap(); // 设置对比度
    spi.write(&[0x00, 0xD9, 0xF1]).unwrap(); // 设置预充电周期
    spi.write(&[0x00, 0xDB, 0x40]).unwrap(); // 设置VCOMH电压
    spi.write(&[0x00, 0xA4]).unwrap(); // 全部点亮
    spi.write(&[0x00, 0xA6]).unwrap(); // 设置显示方式（正常模式）
    spi.write(&[0x00, 0xAF]).unwrap(); // 打开显示

    cs.set_value(1).unwrap();
}

fn set_pos(spi: &mut Spidev, cs: &Pin, dc: &Pin, x: u8, y: u8) {
    cs.set_value(0).unwrap();
    spi.write(&[0x00, 0x21, x, 127]).unwrap(); // 设置列地址范围
    spi.write(&[0x00, 0x22, y, 63]).unwrap(); // 设置行地址范围

    cs.set_value(1).unwrap();
}

fn fill_screen(spi: &mut Spidev, cs: &Pin, dc: &Pin, color: u8) {
    set_pos(spi, cs, dc, 0, 0);
    cs.set_value(0).unwrap();
    for _ in 0..(128 * 64 / 8) {
        spi.write(&[color]).unwrap();
    }
    cs.set_value(1).unwrap();
}

fn main(){
    let (cs, dc, rst, bl) = init_gpio().unwrap();
    let mut spi = Spidev::open("/dev/spidev1.0").unwrap();
    let mut binding = SpidevOptions::new();
    let options = binding
        .bits_per_word(SPI_BITS_PER_WORD)
        .max_speed_hz(SPI_SPEED)
        .mode(SPI_MODE);
    spi.configure(&options).unwrap();
    reset(&rst);
    init_display(&mut spi, &cs, &dc);
    fill_screen(&mut spi, &cs, &dc, 0xff); // 填充白色
}
