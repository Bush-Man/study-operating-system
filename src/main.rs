#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

use vga::writter::VgaWriter;

static UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
static NUMBERS: &str = "0123456789";
static SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?/~";
static WHITESPACE: &str = " ";
static CONTROL: &str = "\t\r\n";
static FLOATS: &str = "3.14 -0.001 42.0";
static WORLD:&str = "Wörld!";

#[panic_handler]
fn panic_handler(_info:&PanicInfo)->!{
    loop{}
}


#[no_mangle]
pub extern "C" fn kernel_main()->!{
    let mut vga = VgaWriter::new();
    vga.write_string(FLOATS);
   
    loop{}
}


