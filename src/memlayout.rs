use crate::uart_print;

extern "C" {
    pub static TEXT_START: usize;
    pub static RODATA_START: usize;
    pub static DATA_START: usize;
    pub static BSS_START: usize;

    pub static KSTACK_START: usize;
    pub static KHEAP_START: usize;
    pub static UMEMORY_START: usize;
}

pub static UART_BASE_ADDR: usize = 0x1000_0000;

#[cfg(debug_assertions)]
pub unsafe fn print_sections() {
    uart_print!("===========================\n");
    uart_print!("TEXT START: {:#10x}\n", TEXT_START);
    uart_print!("RODATA START: {:#10x}\n", RODATA_START);
    uart_print!("DATA START: {:#10x}\n", DATA_START);
    uart_print!("BSS START: {:#10x}\n", BSS_START);
    uart_print!("KSTACK START: {:#10x}\n", KSTACK_START);
    uart_print!("KHEAP START: {:#10x}\n", KHEAP_START);
    uart_print!("UMEMORY START: {:#10x}\n", UMEMORY_START);
    uart_print!("===========================\n");
}