#[derive(Debug)]
pub enum Inch {
    Lcd0inch96{width: u16, height: u16}, // 160 * 90
    Lcd1inch03{width: u16, height: u16}, // 240 * 240
    Lcd1inch08{width: u16, height: u16}, // 160 * 128
    Lcd1inch14{width: u16, height: u16}, // 240 * 135
    Lcd1inch28{width: u16, height: u16}, // 240 * 240
    Lcd1inch64{width: u16, height: u16}, // 230 * 320
    Lcd2inch{width: u16, height: u16},   // 240 * 320
    Lcd2inch4{width: u16, height: u16},  // 240 * 320

}