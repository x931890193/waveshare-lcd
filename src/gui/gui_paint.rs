use crate::gui::types::LineStyle::{LineStyleDotted, LineStyleSolid};
use super::types::*;

pub struct PaintTime {
    pub year: u16,     //0000
    pub month: u8,     //1 - 12
    pub day: u8,        //1 - 30
    pub hour: u8,        //0 - 23
    pub min: u8,        //0 - 59
    pub sec: u8,        //0 - 59
}

#[derive(Debug)]
struct Paint {
    image: *mut u16,
    width: u16,
    height: u16,
    width_memory: u16,
    height_memory: u16,
    color: u16,
    rotate: u16,
    mirror: MirrorImage,
    width_byte: u16,
    height_byte: u16,
    depth: u16,
    mode: u8,
}


impl Paint {
    //  init and Clear
    pub fn paint_new_image(&mut self, image: *mut u16, width: u16, height: u16, rotate: u16, color: u16, depth: u16) {
        self.image = image;
        self.width_memory = width;
        self.height_memory = height;
        self.color = color;
        self.width_byte = width;
        self.height_byte = Height;
        self.depth = depth;
        self.rotate = rotate;
        self.mirror = MirrorImage::MirrorNone;
        self.width = width;
        self.height = height;
        if rotate != 0 && rotate != 180 {
            self.width = height;
            self.height = width;
        }
    }

    pub fn paint_select_image(&mut self, image: *mut u16) {
        self.image = image;
    }

    pub fn paint_set_rotate(&mut self, rotate: u16) {
        if rotate != 0 && rotate != 90 && rotate != 180 && rotate != 270 {
            println!("rotate = 0, 90, 180, 270\r\n");
            return;
        }
        self.rotate = rotate
    }

    pub fn paint_set_mirroring(&mut self, mirror: MirrorImage) {
        self.mirror = mirror;
    }

    pub fn paint_set_pixel(&mut self, x_point: u16, y_point: u16, color: u16) {
        if x_point > self.width || y_point > self.height {
            return;
        }
        let mut x = 0u16;
        let mut y = 0u16;
        match self.rotate {
            0 => {
                x = x_point;
                y = y_point;
            }
            90 => {
                x = self.width_memory - y_point - 1;
                y = x_point;
            }
            180 => {
                x = self.width_memory - x_point - 1;
                y = self.height - y_point - 1;
            }
            270 => {
                x = y_point;
                y = self.height_memory - x_point - 1;
            }
            _ => {
                eprint!("errror");
                return;
            }
        }
        match self.mirror {
            MirrorImage::MirrorNone => {}
            MirrorImage::MirrorHorizontal => {
                x = self.width_memory - X - 1;
            }
            MirrorImage::MirrorVertical => {
                y = self.height_memory - Y - 1;
            }
            MirrorImage::MirrorOrigin => {
                x = self.width_memory - X - 1;
                y = self.height_memory - Y - 1;
            }
        }
        if self.depth == 1 {
            let addr = x / 8 + y * self.width_byte;
            let rdata = self.image[addr];
            if color == BLACK {
                self.image[addr] = rdata & !(0x80 >> (X % 8));
            } else {
                self.image[addr] = rdata | (0x80 >> (X % 8));
            }
        } else {
            let addr = x + y * self.width_byte;
            self.image[addr] = ((Color << 8) & 0xff00) | (Color >> 8);
        }
    }

    pub fn paint_clear(&mut self, color: u16) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = x + y * self.WidthByte;
                self.image[addr] = color;
            }
        }
    }

    pub fn paint_clear_window(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16) {
        for y in y_start..y_send {
            for x in x_start..x_send {
                self.paint_set_pixel(x, y, color)
            }
        }
    }

    //Drawing
    pub fn paint_draw_point(&mut self, x_point: u16, y_point: u16, color: u16, dot_pixel: DotPixel, dot_style: DotStyle) {
        if x_point > self.width || y > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        if dot_style == DotStyle::DotFillAround {
            for xdir_num in 0..2 * &dot_pixel - 1 {
                for ydir_num in 0..2 * &dot_pixel - 1 {
                    if x_point + xdir_num - &dot_pixel < 0 || y_point + ydir_num - &dot_pixel < 0 {
                        break;
                    }
                    self.paint_set_pixel(x_point + xdir_num - &dot_pixel, y_point + ydir_num - &dot_pixel, color)
                }
            }
        } else {
            for xdir_num in 0..&dot_pixel - 1 {
                for ydir_num in 0..&dot_pixel - 1 {
                    self.paint_set_pixel(x_point + xdir_num - 1, y_point + ydir_num - 1, color)
                }
            }
        }
    }

    pub fn paint_draw_line(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16, line_width: DotPixel, line_style: LineStyle) {
        if x_point > self.width || y > self.height || x_send > self.width || y_send > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        let x_point = x_start;
        let y_point = y_start;
        let dx = {
            if x_send - x_start >= 0 {
                x_send - x_start
            } else {
                x_start - x_send
            }
        };
        let dy = {
            if y_send - y_start >= 0 {
                y_send - y_start
            } else {
                y_start - y_send
            }
        };
        let esp = dx + dy;
        let mut dotted_len = 0;
        loop {
            dotted_len += 1;
            if line_style == LineStyleDotted && dotted_len % 3 == 0 {
                self.paint_draw_point(x_point, y_point, IMAGE_BACKGROUND, line_width.clone(), DOT_STYLE_DFT)
            }
        }
    }

    pub fn paint_draw_rectangle(&mut self, x_start: u16, y_start: u16, x_send: u16, y_send: u16, color: u16, line_width: DotPixel, draw_fill: DrawFill) {
        if x_point > self.width || y > self.height || x_send > self.width || y_send > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        if draw_fill {
            for y_point in 0..y_send {
                self.paint_draw_line(x_start, y_point, x_send, y_send, color, line_width.clone(), LineStyleSolid)
            }
        } else {
            self.paint_draw_line(x_start, y_start, x_send, y_send, color, line_width.clone(), LINE_STYLE_SOLID);
            self.paint_draw_line(x_start, y_start, x_start, y_send, color, line_width.clone(), LINE_STYLE_SOLID);
            self.paint_draw_line(x_send, y_send, x_send, y_start, color, line_width.clone(), LINE_STYLE_SOLID);
            self.paint_draw_line(x_send, y_send, x_start, y_send, color, line_width.clone(), LINE_STYLE_SOLID);
        }
    }

    pub fn paint_draw_circle(&mut self, x_center: u16, y_center: u16, radius: u16, color: u16, dot_pixel: DotPixel, draw_fill: DrawFill) {
        if x_point > self.width || y > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        let mut x_current = 0u16;
        let mut y_current = radius;
        let mut esp = 3 - (radius << 1);
        if draw_fill == DrawFill::DrawFillFull {
            loop {
                if x_current > y_current {
                    break;
                } else {
                    for s_county in 0..y_current {
                        self.paint_draw_point(x_center + x_current, y_center + s_county, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - x_current, y_center + s_county, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - s_county, y_center + x_current, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - s_county, y_center - x_current, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - x_current, y_center - s_county, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + x_current, y_center - s_county, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + s_county, y_center - x_current, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + s_county, y_center + x_current, color, DOT_STYLE_DFT, DOT_STYLE_DFT);
                    }
                    if esp < 0 {
                        esp += 4 * x_current + 6;
                    } else {
                        esp += 10 + 4 * (x_current - y_current);
                        y_current -= 1;
                    }
                    x_current += 1;
                }
            }
        } else {
            loop {
                if x_current <= y_current {
                    break;
                }
                Paint_DrawPoint(X_Center + XCurrent, Y_Center + YCurrent, Color, Line_width, DOT_STYLE_DFT);//1
                Paint_DrawPoint(X_Center - XCurrent, Y_Center + YCurrent, Color, Line_width, DOT_STYLE_DFT);//2
                Paint_DrawPoint(X_Center - YCurrent, Y_Center + XCurrent, Color, Line_width, DOT_STYLE_DFT);//3
                Paint_DrawPoint(X_Center - YCurrent, Y_Center - XCurrent, Color, Line_width, DOT_STYLE_DFT);//4
                Paint_DrawPoint(X_Center - XCurrent, Y_Center - YCurrent, Color, Line_width, DOT_STYLE_DFT);//5
                Paint_DrawPoint(X_Center + XCurrent, Y_Center - YCurrent, Color, Line_width, DOT_STYLE_DFT);//6
                Paint_DrawPoint(X_Center + YCurrent, Y_Center - XCurrent, Color, Line_width, DOT_STYLE_DFT);//7
                Paint_DrawPoint(X_Center + YCurrent, Y_Center + XCurrent, Color, Line_width, DOT_STYLE_DFT);//0
                if esp < 0 {
                    esp += 4 * x_current + 6;
                } else {
                    esp += 10 + 4 * (x_current - y_current);
                    y_current -= 1;
                }
                x_current += 1;
            }
        }
    }

    //Display string
    pub fn paint_draw_char(&mut self, x_start: u16, y_start: u16, char: char, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_string_en(&mut self, x_start: u16, y_start: u16, text: &str, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_string_cn(&mut self, x_start: u16, y_start: u16, text: &str, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_num(&mut self, x_point: u16, y_point: u16, number: i32, font: SFont, c_foreground: u16, c_bckground: u16) {}

    pub fn paint_draw_float_num(&mut self, x_point: u16, y_point: u16, number: f32, decimal_point: u8, font: SFont, c_foreground: u16, c_background: u16) {}

    pub fn paint_draw_time(&mut self, x_start: u16, y_start: u16, p_time: PaintTime, font: SFont, c_foreground: u16, c_bckground: u16) {}

    //pic
    pub fn paint_draw_image(&mut self, image: &str, start_x: u16, start_y: u16, w: u16, h: u16) {
        for j in 0..h {
            for i in 0..w {
                if start_x + i < self.width_memory && start_x < self.height_memory {
                    self.paint_set_pixel(start_x + i, start_y + j, (image + j * w * 2 + i + i * 2 + 1) << 8 | (image + j*W_Image*2 + i*2))
                }
            }
        }
    }

    pub fn paint_draw_bit_map(&mut self, image_buffer: *const u8) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = x + y * self.width_byte;
                self.image[addr] = image_buffer[addr] as char;
            }
        }
    }
}