use alloc::string::String;
use core::fmt::Write;

extern "C" {
    pub static TEXT_START: usize;
    pub static TEXT_END: usize;
    pub static RODATA_START: usize;
    pub static RODATA_END: usize;
    pub static DATA_START: usize;
    pub static DATA_END: usize;
    pub static BSS_START: usize;
    pub static BSS_END: usize;

    pub static KHEAP_START: usize;
    pub static KHEAP_END: usize;
    pub static KSTACK_START: usize;
    pub static KSTACK_END: usize;
    pub static UMEMORY_START: usize;
}
pub static mut UMEMORY_END: usize = 0;

pub static PLIC_BASE_ADDR: usize = 0x0c00_0000;
pub static UART_BASE_ADDR: usize = 0x1000_0000;

/// Helper function. 
/// Prints the memory layout as specified in the linker script.
#[rustfmt::skip]
pub fn print_sections() {
    // Safety: Extern variables are well defined by the linker script
    unsafe {
        let mut out = String::from("Linker Memory layout: \n");
        writeln!(out, "=======================================").unwrap();
        writeln!(out, "TEXT:          {:#018x}..{:#018x}", TEXT_START, TEXT_END).unwrap();
        writeln!(out, "RODATA:        {:#018x}..{:#018x}", RODATA_START, RODATA_END).unwrap();
        writeln!(out, "DATA:          {:#018x}..{:#018x}", DATA_START, DATA_END).unwrap();
        writeln!(out, "BSS:           {:#018x}..{:#018x}", BSS_START, BSS_END).unwrap();
        writeln!(out, "KHEAP:         {:#018x}..{:#018x}", KHEAP_START, KHEAP_END).unwrap();
        writeln!(out, "KSTACK:        {:#018x}..{:#018x}", KSTACK_START, KSTACK_END).unwrap();
        writeln!(out, "USER PAGES:    {:#018x}..{:#018x}", UMEMORY_START, UMEMORY_END).unwrap();
        writeln!(out, "=======================================").unwrap();
        log::info!("{}", out);
    }
}
