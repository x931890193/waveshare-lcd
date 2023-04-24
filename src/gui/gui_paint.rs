use std::io::ErrorKind::Other;
use super::types::*;

pub struct PaintTime {
    pub year: u16,  //0000
    pub month: u8,  //1 - 12
    pub day: u8,    //1 - 30
    pub hour: u8,   //0 - 23
    pub min: u8,    //0 - 59
    pub sec: u8,   //0 - 59
}

#[derive(Debug)]
struct Paint {
    image: u16,
    width: u16,
    height: u16,
    width_memory: u16,
    height_memory: u16,
    color: u16,
    rotate: u16,
    mirror: u16,
    width_byte: u16,
    height_byte: u16,
    depth: u16,
    mode: u8,
}


impl Paint {
    //  init and Clear
    pub fn paint_new_image(&mut self, image: u16, width: u16, height: u16, rotate: u16, color: u16, depth: u16) {
        self.image = image;
        self.width_memory = width;
        self.height_memory = height;
        self.color = color;
        self.WidthByte = width;
        self.HeightByte = Height;
        self.depth = depth;
        self.rotate = rotate;
        self.mirror = MirrorImage::MirrorNone(0x00).0;
        self.width = height;
        self.width = width;
        self.height = height;
        if rotate != 0 && rotate != 180 {
            self.width = height;
            self.height = width;
        }
    }

    pub fn paint_select_image(&mut self, image: u16) {
       self.image = image;
    }

    pub fn paint_set_rotate(&mut self, rotate: u16) {
        if rotate != 0 && rotate != 90 && rotate != 180 && rotate != 270 {
            println!("rotate = 0, 90, 180, 270\r\n");
            return;
        }
        self.rotate = rotate
    }

    pub fn paint_set_mirroring(&mut self, mirror: u8) {}

    pub fn paint_set_pixel(&mut self, x_point: u16, y_point: u16, color: u16) {}

    pub fn paint_clear(&mut self, color: u16) {}

    pub fn paint_clear_window(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16) {}

    //Drawing
    pub fn paint_draw_point(&mut self, x_point: u16, y_point: u16, color: u16, dot_pixel: DotPixel, dot_style: DotStyle) {}

    pub fn paint_draw_line(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16, line_width: DotPixel, line_style: LineStyle) {}

    pub fn paint_draw_rectangle(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16, line_width: DotPixel, draw_fill: DrawFill) {}

    pub fn paint_draw_circle(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16, dot_pixel: DotPixel, draw_fill: DrawFill) {}

    //Display string
    pub fn paint_draw_char(&mut self, x_start: u16, y_start: u16, char: char, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_string_en(&mut self, x_start: u16, y_start: u16, text: &str, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_string_cn(&mut self, x_start: u16, y_start: u16, text: &str, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_num(&mut self, x_point: u16, y_point: u16, number: i32, font: SFont, c_foreground: u16, c_bckground: u16) {}

    pub fn paint_draw_float_num(&mut self, x_point: u16, y_point: u16, number: f32, decimal_point: u8, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_time(&mut self, x_start: u16, y_start: u16, p_time: PaintTime, font: SFont, c_foreground: u16, c_bckground: u16) {}

    //pic
    pub fn paint_draw_image(&mut self, image: &str, start_x: u16, start_y: u16, end_x: u16, end_y: u16 ) {

    }
}