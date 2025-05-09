#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

use vga::writter::VgaWriter;
use core::fmt::Write;

static UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
static NUMBERS: &str = "0123456789";
static SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?/~\n";
static WHITESPACE: &str = " ";
static CONTROL: &str = "\t\r\n";
static FLOATS: &str = "3.14 -0.001 42.0\n";
static WORLD:&str = "Wörld!";

#[panic_handler]
fn panic_handler(_info:&PanicInfo)->!{
    loop{}
}


#[no_mangle]
pub extern "C" fn kernel_main()->!{
    let mut vga_writer = VgaWriter::new();
    
    // write!(vga_writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    // write!(vga_writer,"{}", UPPERCASE).unwrap();
    write!(vga_writer,"{}", WORLD).unwrap();

    loop{}
}


