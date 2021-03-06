use alloc::string::String;
use alloc::format;

extern "C" {
    pub static TEXT_START: usize;
    pub static TEXT_END: usize;
    pub static RODATA_START: usize;
    pub static RODATA_END: usize;
    pub static DATA_START: usize;
    pub static DATA_END: usize;
    pub static BSS_START: usize;
    pub static BSS_END: usize;

    pub static KSTACK_START: usize;
    pub static KSTACK_END: usize;
    pub static KHEAP_START: usize;
    pub static KHEAP_END: usize;
    pub static UMEMORY_START: usize;
    pub static UMEMORY_END: usize;
}

pub static PLIC_BASE_ADDR: usize = 0x0c00_0000;
pub static UART_BASE_ADDR: usize = 0x1000_0000;


/// Helper function. 
/// Prints the memory layout as specified in the linker script.
#[cfg(debug_assertions)]
pub unsafe fn print_sections() {
    let mut out = String::from("Memory layout: \n");
    out += &format!("===================================\n");
    out += &format!("TEXT:      {:#010x} .. {:#010x}\n", TEXT_START, TEXT_END);
    out += &format!("RODATA:    {:#010x} .. {:#010x}\n", RODATA_START, RODATA_END);
    out += &format!("DATA:      {:#010x} .. {:#010x}\n", DATA_START, DATA_END);
    out += &format!("BSS:       {:#010x} .. {:#010x}\n", BSS_START, BSS_END);
    out += &format!("KSTACK:    {:#010x} .. {:#010x}\n", KSTACK_START, KSTACK_END);
    out += &format!("KHEAP:     {:#010x} .. {:#010x}\n", KHEAP_START, KHEAP_END);
    out += &format!("UMEMORY:   {:#010x} .. {:#010x}\n", UMEMORY_START, UMEMORY_END);
    out += &format!("===================================");
    log::info!("{}", out);
}