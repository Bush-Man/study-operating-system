
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;     
const VGA_BUFFER:*mut u16 = 0xb8000 as *mut u16;
/// repr(u8) is used to ensure that the enum fields are represented as a single byte.
/// VGA color codes are 8-bit values.
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
#[repr(C)]
pub struct VgaCharacter{
     asci_char:u8,
     color_code:u8
}

pub struct VgaWriter{
    row: usize,
    column: usize,
    color_code: u8,
}


fn create_color_code(fg: VgaColor, bg: VgaColor) -> u8 {
    return (bg as u8) << 4 | (fg as u8);
}


impl VgaWriter{
    pub fn new()->Self{
        VgaWriter{
            row:0,
            column:0,
            color_code: create_color_code(VgaColor::White, VgaColor::Black),
        }
    }
   pub fn write_char(&mut self,char:u8){
        match char{
            b'\n' =>{
                if self.row >= VGA_BUFFER_HEIGHT || self.column >= VGA_BUFFER_WIDTH {
                    self.scroll_up();
                }
                
            },
            _ =>{
              unsafe{
                let index = self.row * VGA_BUFFER_WIDTH + self.column;
                let vga_buffer = VGA_BUFFER.offset(index as isize);
                *vga_buffer = ((self.color_code as u16) << 8) | char as u16;
                self.column += 1;
                if self.column >= VGA_BUFFER_WIDTH{
                    self.column = 0;
                    self.row +=1;

                }

                
              }
            }
        }

    }
    fn scroll_up(&mut self){

    }
    
}