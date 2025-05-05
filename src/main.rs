#![no_std]
#![no_main]


pub mod vga;

use core::panic::PanicInfo;


#[panic_handler]
fn panic_handler(_info:&PanicInfo)->!{
    loop{}
}
#[no_mangle]
pub extern "C" fn _start()->!{
    loop{}
}

