#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::arch::asm;
use alloc::alloc::Layout;

use crate::memlayout::*;

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/mem.asm"));
global_asm!(include_str!("asm/trap.asm"));

/// Whenever there is a fatal kernel panic,
/// this function is called. It also prints
/// panic info. 
#[panic_handler]
#[no_mangle]
extern "C" 
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("FATAL - Kernel Panic:");
    log::error!("{}", info);
    loop {
        unsafe {
            asm! {
                "csrci  sstatus, 1 << 1",
                "wfi",
            }
        }
    }
    
}

/// Error handler for kernel heap allocation.
/// If the heap allocation failed for any reason
/// it will print the layout and the kernel will
/// immediately panic.
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

/// Kernel main where all other modules are
/// initialized. This function is the first
/// function to be called when the kernel
/// enters Rust.
#[no_mangle]
extern "C"
fn kmain(_hart_id: u64, fdt_ptr: u64) -> ! {
    
    uart::init(UART_BASE_ADDR);
    logger::init();
    kheap::init();
    fdt::init(fdt_ptr);
    trap::init();
    
    plic::init(PLIC_BASE_ADDR);
    
    unsafe { memlayout::print_sections() };
    vmem::init();

    log::info!("It works! :D");

    sched::init();
    loop {}
}

pub mod logger;
pub mod macros;
pub mod memlayout;
pub mod uart;
pub mod vmem;
pub mod kheap;
pub mod trap;
pub mod fdt;
pub mod plic;
pub mod proc;
pub mod sched;