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
    Yellow = 14,  
    White = 15,
}

fn create_color_code(fg: VgaColor, bg: VgaColor) -> u8 {
    return (bg as u8) << 4 | (fg as u8);
}

struct VgaWriter{
    row: usize,
    column: usize,
    color_code: u8,
}

impl VgaWriter{
    fn new()->Self{
        VgaWriter{
            row:0,
            col:0,
            color_code: create_color_code(VgaColor::White, VgaColor::Black),
        }
    }
    
}

