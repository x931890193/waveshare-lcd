
pub const GPIOCHIP_BASE: u8 = 0;
pub const LCD_CS: u8 = GPIOCHIP_BASE + 49;
pub const LCD_RST: u8 = GPIOCHIP_BASE + 42;
pub const LCD_DC: u8 = GPIOCHIP_BASE + 44;
pub const LCD_BL: u8 = GPIOCHIP_BASE + 51;


fn fill_screen(spi: &mut Spidev, cs: &Pin, dc: &Pin, color: u8) {
    set_pos(spi, cs, dc, 0, 0);
    cs.set_value(0).unwrap();
    for _ in 0..(128 * 64 / 8) {
        spi.write(&[color]).unwrap();
    }
    cs.set_value(1).unwrap();
}
