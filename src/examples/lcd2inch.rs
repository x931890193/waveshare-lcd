use waveshare_lcd::config::dev_hardware_spi::HardwareSpi;
use waveshare_lcd::gui::types::{BLACK, DOT_STYLE_DFT, DotStyle, RED, ROTATE_270, WHITE};
use waveshare_lcd::lcd::lcd_2inch::LCD;
use waveshare_lcd::lcd::types::Inch;
use waveshare_lcd::gui::gui_paint::Paint;
use waveshare_lcd::gui::types::DotPixel::{DotPixel1x1, DotPixel2x2, DotPixel3x3, DotPixel4x4};

fn main() {
    println!("2inch LCD demo...\r\n");
    let spi = HardwareSpi::new("/dev/spidev1.0");
    let mut lcd = LCD::new(Inch::Lcd2inch {width:320, height:240 },  spi);

    lcd.lcd_in_init();
    lcd.lcd_2in_clear(WHITE);
    lcd.set_black(1010);

    let mut paint = Paint::new();
    let image = Vec::with_capacity((320 * 240 * 2) as usize);
    paint.paint_new_image(image, 320, 240, 90, WHITE, 16);
    paint.paint_clear(WHITE);
    paint.paint_set_rotate(ROTATE_270);
    println!("drawing...\r\n");

    paint.paint_draw_point(5, 10, BLACK, DotPixel1x1, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 25, BLACK, DotPixel2x2, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 40, BLACK, DotPixel3x3, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 55, BLACK, DotPixel4x4, DOT_STYLE_DFT);

    lcd.lcd_2in_display(paint.image.clone());
    lcd.sleep(300);

    lcd.lcd_2in_display(paint.image.clone());
    lcd.sleep(300);
}