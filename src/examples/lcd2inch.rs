use waveshare_lcd::config::dev_hardware_spi::HardwareSpi;
use waveshare_lcd::gui::types::WHITE;
use waveshare_lcd::lcd::lcd_2inch::LCD;
use waveshare_lcd::lcd::types::Inch;

fn main() {
    println!("2inch LCD demo...\r\n");
    let spi = HardwareSpi::new("/dev/spidev1.0");
    let mut lcd = LCD::new(Inch::Lcd2inch {width:320, height:240 },  spi);
    lcd.lcd_in_init();
    lcd.lcd_2in_clear(WHITE)
}