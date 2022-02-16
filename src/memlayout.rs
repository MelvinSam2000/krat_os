use crate::uart_print;

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

pub static UART_BASE_ADDR: usize = 0x1000_0000;

/// Helper function. 
/// Prints the memory layout as specified in the linker script.
#[cfg(debug_assertions)]
pub unsafe fn print_sections() {
    uart_print!("===========================\n");
    uart_print!("TEXT:      {:#010x} .. {:#010x}\n", TEXT_START, TEXT_END);
    uart_print!("RODATA:    {:#010x} .. {:#010x}\n", RODATA_START, RODATA_END);
    uart_print!("DATA:      {:#010x} .. {:#010x}\n", DATA_START, DATA_END);
    uart_print!("BSS:       {:#010x} .. {:#010x}\n", BSS_START, BSS_END);
    uart_print!("KSTACK:    {:#010x} .. {:#010x}\n", KSTACK_START, KSTACK_END);
    uart_print!("KHEAP:     {:#010x} .. {:#010x}\n", KHEAP_START, KSTACK_END);
    uart_print!("UMEMORY:   {:#010x} .. {:#010x}\n", UMEMORY_START, UMEMORY_END);
    uart_print!("===========================\n");
}