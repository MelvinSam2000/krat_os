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
    log::info!("===========================");
    log::info!("TEXT:      {:#010x} .. {:#010x}", TEXT_START, TEXT_END);
    log::info!("RODATA:    {:#010x} .. {:#010x}", RODATA_START, RODATA_END);
    log::info!("DATA:      {:#010x} .. {:#010x}", DATA_START, DATA_END);
    log::info!("BSS:       {:#010x} .. {:#010x}", BSS_START, BSS_END);
    log::info!("KSTACK:    {:#010x} .. {:#010x}", KSTACK_START, KSTACK_END);
    log::info!("KHEAP:     {:#010x} .. {:#010x}", KHEAP_START, KSTACK_END);
    log::info!("UMEMORY:   {:#010x} .. {:#010x}", UMEMORY_START, UMEMORY_END);
    log::info!("===========================");
}