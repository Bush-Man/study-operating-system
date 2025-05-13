use core::{cell::UnsafeCell, fmt::{self, Write}};

use super::writter::VgaWriter;

pub struct GlobalWriter {
    inner: UnsafeCell<VgaWriter>
}

impl GlobalWriter{
   const fn new()->Self{
        GlobalWriter{
            inner:UnsafeCell::new( VgaWriter::new())
        }
    }
    pub fn write(&self, args:fmt::Arguments){
        unsafe{
            let writer = &mut *self.inner.get();
            writer.write_fmt(args).unwrap_or(());
        }
        
             
        

    }
    
}

unsafe impl Sync for GlobalWriter {}

pub static  GLOBAL_WRITER: GlobalWriter = GlobalWriter::new();


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::global_writer::GLOBAL_WRITER.write(format_args!($($arg)*));
        
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}