use waveshare_lcd::config::dev_hardware_spi::HardwareSpi;
use waveshare_lcd::gui::types::{BLACK, BLUE, DOT_STYLE_DFT, DotStyle, DrawFill, GREEN, RED, ROTATE_270, WHITE};
use waveshare_lcd::lcd::lcd_2inch::LCD;
use waveshare_lcd::lcd::types::Inch;
use waveshare_lcd::gui::gui_paint::Paint;
use waveshare_lcd::gui::types::DotPixel::{DotPixel1x1, DotPixel2x2, DotPixel3x3, DotPixel4x4};
use waveshare_lcd::gui::types::DotStyle::DotFillAround;
use waveshare_lcd::gui::types::LineStyle::LineStyleSolid;


fn main() {
    println!("2inch LCD demo...\r\n");
    let spi = HardwareSpi::new("/dev/spidev1.0");
    let mut lcd = LCD::new(Inch::Lcd2inch {width:320, height:240 },  spi);
    lcd.init_dev();
    lcd.lcd_2in_clear(WHITE);
    lcd.sleep(10000);
    return;
    lcd.set_black(1010);

    let mut paint = Paint::new();
    let image = vec![0; 320 * 240 * 2];
    paint.paint_new_image(image, 320, 240, 90, WHITE, 16);
    paint.paint_clear(WHITE);
    paint.paint_set_rotate(ROTATE_270);
    println!("drawing...\r\n");

    paint.paint_draw_point(5, 10, BLACK, DotPixel1x1, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 25, BLACK, DotPixel2x2, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 40, BLACK, DotPixel3x3, DOT_STYLE_DFT);
    paint.paint_draw_point(5, 55, BLACK, DotPixel4x4, DOT_STYLE_DFT);
    lcd.sleep(500);

    paint.paint_draw_line(20, 10, 70, 60, RED, DotPixel1x1, LineStyleSolid);
    paint.paint_draw_line(70, 10, 20, 60, RED, DotPixel1x1, LineStyleSolid);
    paint.paint_draw_line(170, 15, 170, 55, RED, DotPixel1x1, LineStyleSolid);
    paint.paint_draw_line(150, 35, 190, 35, RED, DotPixel1x1, LineStyleSolid);

    paint.paint_draw_rectangle(20, 10, 70, 60, BLUE, DotPixel1x1, DrawFill::DrawFillEmpty);
    paint.paint_draw_rectangle(85, 10, 130, 60, BLUE, DotPixel1x1, DrawFill::DrawFillFull);


    paint.paint_draw_circle(170, 35, 20, GREEN, DotPixel1x1, DrawFill::DrawFillEmpty);
    paint.paint_draw_circle(170, 85, 20, GREEN, DotPixel1x1, DrawFill::DrawFillFull);

    lcd.lcd_2in_display(paint.image.clone());
    lcd.sleep(10000);

    lcd.lcd_2in_display(paint.image.clone());
    lcd.sleep(300);
}