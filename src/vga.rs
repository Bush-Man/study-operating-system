
/// repr(u8) is used to ensure that the enum is represented as a single byte.
/// This is important because VGA color codes are 8-bit values.
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,  // Typically called Yellow instead of LightBrown in VGA
    White = 15,
}