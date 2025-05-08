use core::ptr::write_volatile;


const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;     
const VGA_BUFFER_BASE_ADDRESS:*mut u16 = 0xb8000 as *mut u16;
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
    // Write a character to the VGA buffer
   pub fn write_char(&mut self,char:u8){
        match char{
            b'\n' =>{
                if self.row >= VGA_BUFFER_HEIGHT || self.column >= VGA_BUFFER_WIDTH {
                    self.scroll_up();
                }
                
            },
            _ =>{
              unsafe{
                if self.column >= VGA_BUFFER_WIDTH{
                    self.column = 0;
                    self.row +=1;
                    
                }
                let offset = (self.row * VGA_BUFFER_WIDTH + self.column) as isize;
                let ascii_char = ((self.color_code as u16) << 8) | char as u16;
                let position = VGA_BUFFER_BASE_ADDRESS.offset(offset);
                //TODO: Check if the position is valid before writing
            
                    write_volatile(position, ascii_char);
                
                
                self.column += 1;
                
              }
            }
        }


    }

    // Write a string to the VGA buffer
    pub fn write_str(&mut self,string:&str){

        for byte in string.bytes(){
            match byte{
                0x20..=0x7e => self.write_char(byte),
                _ => self.write_char(0xFF),
            }
        }

    }
    // Clear row
    pub fn clear_row(&mut self,row:usize){
        if row >= VGA_BUFFER_HEIGHT{
            return;
        }
        
        for column in 0..VGA_BUFFER_WIDTH{
         let offset = row * VGA_BUFFER_WIDTH + column;
         let position = unsafe {VGA_BUFFER_BASE_ADDRESS.offset(offset as isize)};
         let blank_char = b' ';
         let blank_color = create_color_code(VgaColor::Black, VgaColor::Black);
         let blank = ((blank_color as u16) << 8 )| blank_char as u16;
         unsafe{

             write_volatile(position, blank);
            }
       
        
        }

    }
    pub fn scroll_up(&mut self){

    }

    
    
}