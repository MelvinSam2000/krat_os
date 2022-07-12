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

    pub static TRAMP_VECTOR: usize;
    pub static TRAMP_FRAME: usize;

    pub static KHEAP_START: usize;
    pub static KHEAP_END: usize;
    pub static KSTACK_START: usize;
    pub static KSTACK_END: usize;
    pub static UMEMORY_START: usize;
}
pub static mut UMEMORY_END: usize = 0;

pub const TRAMP_VADDR: usize = 0x0000_1000;

pub static PLIC_BASE_ADDR: usize = 0x0c00_0000;
pub static UART_BASE_ADDR: usize = 0x1000_0000;

/// Helper function. 
/// Prints the memory layout as specified in the linker script.
#[rustfmt::skip]
pub fn print_sections() -> Option<()> {
    // Safety: Extern variables are well defined by the linker script
    unsafe {
        let mut out = String::from("Memory layout: \n");
        writeln!(out, "=======================================").ok()?;
        writeln!(out, "TEXT:          {:#010x} .. {:#010x}", TEXT_START, TEXT_END).ok()?;
        writeln!(out, "TRAMP VECTOR:  {:#010x}", TRAMP_VECTOR).ok()?;
        writeln!(out, "TRAMP FRAME:   {:#010x}", TRAMP_FRAME).ok()?;
        writeln!(out, "RODATA:        {:#010x} .. {:#010x}", RODATA_START, RODATA_END).ok()?;
        writeln!(out, "DATA:          {:#010x} .. {:#010x}", DATA_START, DATA_END).ok()?;
        writeln!(out, "BSS:           {:#010x} .. {:#010x}", BSS_START, BSS_END).ok()?;
        writeln!(out, "KHEAP:         {:#010x} .. {:#010x}", KHEAP_START, KHEAP_END).ok()?;
        writeln!(out, "KSTACK:        {:#010x} .. {:#010x}", KSTACK_START, KSTACK_END).ok()?;
        writeln!(out, "USER PAGES:    {:#010x} .. {:#010x}", UMEMORY_START, UMEMORY_END).ok()?;
        writeln!(out, "=======================================").ok()?;
        log::info!("{}", out);
    }
    Some(())
}
