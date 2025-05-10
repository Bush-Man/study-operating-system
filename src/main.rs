#![no_std]
#![no_main]

mod vga;
mod memory;
use core::panic::PanicInfo;

use memory::multiboot_structure::MultibootInfoStructure;
use vga::writter::VgaWriter;
use core::fmt::Write;

const MULTIBOOT_MAGIC_NUMBER: u32 = 0x2BADB002;


static WORLD:&str = "Wörld!";

#[panic_handler]
fn panic_handler(_info:&PanicInfo)->!{
    loop{}
}


#[no_mangle]
pub extern "C" fn kernel_main(multiboot_magic: u32, multiboot_info_ptr: &MultibootInfoStructure)->!{


    if MULTIBOOT_MAGIC_NUMBER != multiboot_magic{
        panic!("Invalid Multiboot Magic Number");
    }

    let mut vga_writer = VgaWriter::new();
    


    
    // write!(vga_writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    // write!(vga_writer,"{}", UPPERCASE).unwrap();
    write!(vga_writer,"{:#x} Magic Number\n",multiboot_magic).unwrap();
    write!(vga_writer,"{:?} Multiboot Info Address\n",multiboot_info_ptr).unwrap();

    loop{}
}


