#[derive(Debug)]
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
    DotPixel8x8,        // 8 X 8
}

#[derive(Debug)]
pub enum DotStyle {
    DotFillAround = 1,
    // dot pixel 1 x 1
    DotFillRightup,        // dot pixel 2 X 2
}


#[derive(Debug)]
pub enum LineStyle {
    LineStyleSolid = 0,
    LineStyleDotted,
}

#[derive(Debug)]
pub enum DrawFill {
    DrawFillEmpty = 0,
    DrawFillFull,
}

#[derive(Debug)]
pub enum MirrorImage {
    MirrorNone(u16),
    MirrorHorizontal(u16),
    MirrorVertical(u16),
    MirrorOrigin(u16),
}