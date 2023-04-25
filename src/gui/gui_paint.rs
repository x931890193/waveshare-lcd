use crate::gui::types::LineStyle::{LineStyleDotted, LineStyleSolid};
use super::types::*;
use crate::fonts::font::SFont;

pub struct PaintTime {
    pub year: u16,     //0000
    pub month: u8,     //1 - 12
    pub day: u8,        //1 - 30
    pub hour: u8,        //0 - 23
    pub min: u8,        //0 - 59
    pub sec: u8,        //0 - 59
}

#[derive(Debug)]
pub struct Paint {
    pub image: Vec<u16>,
    pub width: u16,
    pub height: u16,
    pub width_memory: u16,
    pub height_memory: u16,
    pub color: u16,
    pub rotate: u16,
    pub mirror: MirrorImage,
    pub width_byte: u16,
    pub height_byte: u16,
    pub depth: u16,
    pub mode: u8,
}


impl Paint {

    pub fn new() -> Self {
        Paint{
            image: vec![],
            width: 0,
            height: 0,
            width_memory: 0,
            height_memory: 0,
            color: 0,
            rotate: 0,
            mirror: MirrorImage::MirrorNone,
            width_byte: 0,
            height_byte: 0,
            depth: 0,
            mode: 0
        }
    }
    //  init and Clear
    pub fn paint_new_image(&mut self, image: Vec<u16>, width: u16, height: u16, rotate: u16, color: u16, depth: u16)  {
        self.image = image;
        self.width_memory = width;
        self.height_memory = height;
        self.color = color;
        self.width_byte = width;
        self.height_byte = height;
        self.depth = depth;
        self.rotate = rotate;
        self.mirror = MirrorImage::MirrorNone;
        self.width = width;
        self.height = height;
        if rotate != ROTATE_0 && rotate != ROTATE_180 {
            self.width = height;
            self.height = width;
        }
    }

    pub fn paint_select_image(&mut self, image: Vec<u16>) {
        self.image = image;
    }

    pub fn paint_set_rotate(&mut self, rotate: u16) {
        if rotate != ROTATE_0 && rotate != ROTATE_90 && rotate != ROTATE_180 && rotate != ROTATE_270 {
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
                eprint!("error");
                return;
            }
        }
        match self.mirror {
            MirrorImage::MirrorNone => {}
            MirrorImage::MirrorHorizontal => {
                x = self.width_memory - x - 1;
            }
            MirrorImage::MirrorVertical => {
                y = self.height_memory - y - 1;
            }
            MirrorImage::MirrorOrigin => {
                x = self.width_memory - x - 1;
                y = self.height_memory - y - 1;
            }
        }
        if self.depth == 1 {
            let addr = (x / 8) as usize + (y * self.width_byte) as usize;
            let rdata = self.image[addr];
            if color == BLACK {
                self.image[addr] = rdata & !(0x80 >> (x % 8));
            } else {
                self.image[addr] = rdata | (0x80 >> (x % 8));
            }
        } else {
            let addr = match x.checked_mul(self.width_byte).and_then(|n| n.checked_add(y)) {
                Some(result) => result as usize,
                None => {
                    // 处理溢出错误
                    return;
                }
            };
            self.image[addr] = ((color << 8) & 0xff00) | (color >> 8);
        }
    }

    pub fn paint_clear(&mut self, color: u16) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = match x.checked_mul(self.width_byte).and_then(|n| n.checked_add(y)) {
                    Some(result) => result as usize,
                    None => {
                        // 处理溢出错误
                        return;
                    }
                };
                self.image[addr] = color;
            }
        }
    }

    pub fn paint_clear_window(&mut self, x_start: u16, y_start: u16, x_end: u16, y_end: u16, color: u16) {
        for y in y_start..y_end {
            for x in x_start..x_end {
                self.paint_set_pixel(x, y, color)
            }
        }
    }

    //Drawing
    pub fn paint_draw_point(&mut self, x_point: u16, y_point: u16, color: u16, dot_pixel: DotPixel, dot_style: DotStyle) {
        if x_point > self.width || y_point > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        if dot_style == DotStyle::DotFillAround {
            for xdir_num in 0..2 * (dot_pixel.clone() as u16) - 1 {
                for ydir_num in 0..2 * dot_pixel.clone() as u16 - 1 {
                    if (x_point + xdir_num - (dot_pixel.clone() as u16)) < 0 || (y_point + ydir_num - (dot_pixel.clone() as u16)) < 0 {
                        break;
                    }
                    self.paint_set_pixel(x_point + xdir_num - dot_pixel.clone() as u16, y_point + ydir_num - dot_pixel.clone() as u16, color)
                }
            }
        } else {
            for xdir_num in 0..(dot_pixel.clone() as u16) - 1 {
                for ydir_num in 0..(dot_pixel.clone() as u16) - 1 {
                    self.paint_set_pixel(x_point + xdir_num - 1, y_point + ydir_num - 1, color)
                }
            }
        }
    }

    pub fn paint_draw_line(&mut self, x_start: u16, y_start: u16, x_end: u16, y_end: u16, color: u16, line_width: DotPixel, line_style: LineStyle) {
        if x_start > self.width || y_start > self.height || x_end > self.width || y_end > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        let mut x_point = x_start;
        let mut y_point = y_start;
        let dx = {
            if x_end >= x_start {
                x_end - x_start
            } else {
                x_start - x_end
            }
        } as i16;
        let dy = {
            if y_end >= y_start {
                y_end - y_start
            } else {
                y_start - y_end
            }
        } as i16;
        let x_add_way: i16 = {
            if x_start < x_end {
                1
            } else {
                -1
            }
        } as i16;

        let y_add_way: i16 = {
            if y_start < y_end {
                1
            } else {
                -1
            }
        };

        let mut esp = dx + dy;
        let mut dotted_len = 0;
        loop {
            dotted_len += 1;
            if line_style == LineStyleDotted && dotted_len % 3 == 0 {
                self.paint_draw_point(x_point, y_point, IMAGE_BACKGROUND, line_width.clone(), DOT_STYLE_DFT);
                dotted_len = 0;
            } else {
                self.paint_draw_point(x_point, y_point, color, line_width.clone(), DOT_STYLE_DFT)
            }
            if esp * 2 >= dy {
                if x_point == y_end {
                    break
                }
                esp += dy;
                x_point += x_add_way as u16;
            }
            if esp * 2 <= dy {
                if y_point == y_end {
                    break
                }
                esp += dx;
                y_point += y_add_way as u16;
            }
        }
    }

    pub fn paint_draw_rectangle(&mut self, x_start: u16, y_start: u16, x_end: u16, y_end: u16, color: u16, line_width: DotPixel, draw_fill: DrawFill) {
        if x_start > self.width || y_start > self.height || x_end > self.width || y_end > self.height {
            eprint!("Paint_DrawPoint Input exceeds the normal display range\r\n");
            return;
        }
        if draw_fill == DrawFill::DrawFillFull {
            for y_point in 0..y_end {
                self.paint_draw_line(x_start, y_point, x_end, y_end, color, line_width.clone(), LineStyleSolid)
            }
        } else {
            self.paint_draw_line(x_start, y_start, x_end, y_end, color, line_width.clone(), LineStyleSolid);
            self.paint_draw_line(x_start, y_start, x_start, y_end, color, line_width.clone(), LineStyleSolid);
            self.paint_draw_line(x_end, y_end, x_end, y_start, color, line_width.clone(), LineStyleSolid);
            self.paint_draw_line(x_end, y_end, x_start, y_end, color, line_width.clone(), LineStyleSolid);
        }
    }

    pub fn paint_draw_circle(&mut self, x_center: u16, y_center: u16, radius: u16, color: u16, dot_pixel: DotPixel, draw_fill: DrawFill) {
        if x_center > self.width || y_center > self.height {
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
                        self.paint_draw_point(x_center + x_current, y_center + s_county, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - x_current, y_center + s_county, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - s_county, y_center + x_current, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - s_county, y_center - x_current, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center - x_current, y_center - s_county, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + x_current, y_center - s_county, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + s_county, y_center - x_current, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
                        self.paint_draw_point(x_center + s_county, y_center + x_current, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
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
                self.paint_draw_point(x_center + x_current, y_center + y_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center - x_current, y_center + y_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center - y_current, y_center + x_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center - y_current, y_center - x_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center - x_current, y_center - y_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center + x_current, y_center - y_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center + y_current, y_center - x_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
                self.paint_draw_point(x_center + y_current, y_center + x_current, color, dot_pixel.clone(), DOT_STYLE_DFT);
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
    pub fn paint_draw_image(&mut self, image: &[u8], start_x: u16, start_y: u16, w: u16, h: u16) {
        for j in 0..h {
            for i in 0..w {
                if start_x + i < self.width_memory && start_x < self.height_memory {
                    let index = (j * w * 2 + i * 2) as usize;
                    let pixel = ((image[index + 1] as u16) << 8) | (image[index] as u16);

                    self.paint_set_pixel(start_x + i, start_y + j, pixel)
                }
            }
        }
    }

    pub fn paint_draw_bit_map(&mut self, image_buffer: Vec<u16>) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = (x + y * self.width_byte) as usize;
                self.image[addr] = image_buffer[addr];
            }
        }
    }
}