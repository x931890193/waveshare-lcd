#[derive(Debug, Clone)]
pub enum DotPixel {
    DotPixel1x1 = 1,  // 1 x 1
    DotPixel2x2,      // 2 X 2
    DotPixel3x3,     // 3 X 3
    DotPixel4x4,     // 4 X 4
    DotPixel5x5,    // 5 X 5
    DotPixel6x6,    // 6 X 6
    DotPixel7x7,   // 7 X 7
    DotPixel8x8,  // 8 X 8
}

pub const DOT_PIXEL_DFT: DotPixel = DotPixel::DotPixel1x1;

#[derive(Debug)]
pub enum DotStyle {
    DotFillAround = 1, // dot pixel 1 x 1
    DotFillRightup,  // dot pixel 2 X 2
}

pub const DOT_STYLE_DFT: DotStyle = DotStyle::DOT_FILL_AROUND;

#[derive(Debug)]
pub enum LineStyle {
    LineStyleSolid = 0,
    LineStyleDotted,
    DotFillAround,
}

#[derive(Debug)]
pub enum DrawFill {
    DrawFillEmpty = 0,
    DrawFillFull,
}

#[derive(Debug)]
pub enum MirrorImage {
    MirrorNone,
    MirrorHorizontal,
    MirrorVertical,
    MirrorOrigin,
}

pub const WHITE: u16 = 0xFFFF;
pub const BLACK: u16 = 0x0000;
pub const BLUE: u16 = 0x001F;
pub const BRED: u16 = 0xF81F;
pub const GRED: u16 = 0xFFE0;
pub const GBLUE: u16 = 0x07FF;
pub const RED: u16 = 0xF800;
pub const MAGENTA: u16 = 0xF81F;
pub const GREEN: u16 = 0x07E0;
pub const CYAN: u16 = 0x7FFF;
pub const YELLOW: u16 = 0xFFE0;
pub const BROWN: u16 = 0xBC40;
pub const BRRED: u16 = 0xFC07;
pub const GRAY: u16 = 0x8430;

pub const IMAGE_BACKGROUND: u16 = WHITE;
pub const FONT_FOREGROUND: u16 = BLACK;
pub const FONT_BACKGROUND: u16 = WHITE;