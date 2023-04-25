#[derive(Debug, Clone)]
pub enum DotPixel {
    DotPixel1x1 = 1,
    // 1 x 1
    DotPixel2x2,
    // 2 X 2
    DotPixel3x3,
    // 3 X 3
    DotPixel4x4,
    // 4 X 4
    DotPixel5x5,
    // 5 X 5
    DotPixel6x6,
    // 6 X 6
    DotPixel7x7,
    // 7 X 7
    DotPixel8x8,  // 8 X 8
}

pub const DOT_PIXEL_DFT: DotPixel = DotPixel::DotPixel1x1;

#[derive(Debug, PartialEq)]
pub enum DotStyle {
    DotFillAround = 1,
    // dot pixel 1 x 1
    DotFillRightup,   // dot pixel 2 X 2
}

pub const DOT_STYLE_DFT: DotStyle = DotStyle::DotFillAround;

#[derive(Debug, PartialEq)]
pub enum LineStyle {
    LineStyleSolid = 0,
    LineStyleDotted,
    DotFillAround,
}

#[derive(Debug, PartialEq)]
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

pub const MIRROR_IMAGE_DFT: MirrorImage = MirrorImage::MirrorNone;

// Color
type Color = u16;
pub const WHITE: Color = 0xFFFF;
pub const BLACK: Color = 0x0000;
pub const BLUE: Color = 0x001F;
pub const BRED: u16 = 0xF81F;
pub const GRED: Color = 0xFFE0;
pub const GBLUE: Color = 0x07FF;
pub const RED: Color = 0xF800;
pub const MAGENTA: Color = 0xF81F;
pub const GREEN: Color = 0x07E0;
pub const CYAN: Color = 0x7FFF;
pub const YELLOW: Color = 0xFFE0;
pub const BROWN: Color = 0xBC40;
pub const BRRED: Color = 0xFC07;
pub const GRAY: Color = 0x8430;

pub const IMAGE_BACKGROUND: Color = WHITE;
pub const FONT_FOREGROUND: Color = BLACK;
pub const FONT_BACKGROUND: Color = WHITE;

// rotate
type Rotate = u16;
pub const ROTATE_0: Rotate = 0;
pub const ROTATE_90: Rotate = 90;
pub const ROTATE_180: Rotate = 180;
pub const ROTATE_270: Rotate = 270;