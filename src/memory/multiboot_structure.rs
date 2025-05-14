#[derive(Debug,Clone, Copy)]
#[repr(C)]
pub struct MultibootInfoStructure{

    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms:[u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,

}

#[derive(Debug, PartialEq,Clone, Copy,Eq)]
pub enum MultibootFlag {
    MemInfo,        // Bit 0: Memory information
    BootDevice,     // Bit 1: Boot device
    Cmdline,        // Bit 2: Command line
    Mods,          // Bit 3: Modules
    SymsAout,      // Bit 4: AOUT symbols
    SymsElf,       // Bit 5: ELF symbols
    Mmap,          // Bit 6: Memory map
    Drives,        // Bit 7: Drives
    ConfigTable,   // Bit 8: Config table
    BootLoaderName,// Bit 9: Bootloader name
    ApmTable,      // Bit 10: APM table
}

impl MultibootInfoStructure{
    const MAX_FLAGS: usize = 11;
    pub fn enabled_flags(self)->([Option<MultibootFlag>;Self::MAX_FLAGS],usize) {

        let mut flags = [None; Self::MAX_FLAGS];
        let mut count = 0;
        macro_rules! check_flag{
            ($bit:expr,$flag_name:ident)=>{
                if self.flags & (1 << $bit) != 0{
                    flags[count] =Some(MultibootFlag::$flag_name);
                    count += 1;
                }
            }
        }

       
        check_flag!(0, MemInfo);
        check_flag!(1, BootDevice);
        check_flag!(2, Cmdline);
        check_flag!(3, Mods);
        check_flag!(4, SymsAout);
        check_flag!(5, SymsElf);
        check_flag!(6, Mmap);
        check_flag!(7, Drives);
        check_flag!(8, ConfigTable);
        check_flag!(9, BootLoaderName);
        check_flag!(10, ApmTable);
        

        (flags,count)

    }
}


