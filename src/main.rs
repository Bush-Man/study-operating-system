#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

use vga::writter::VgaWriter;


#[panic_handler]
fn panic_handler(_info:&PanicInfo)->!{
    loop{}
}

static HELLO:&[u8] = b"Hello world";


#[no_mangle]
pub extern "C" fn kernel_main()->!{
    let mut vga = VgaWriter::new();
    for &byte in HELLO{
        vga.write_char(byte);
    }

    loop{}
}


