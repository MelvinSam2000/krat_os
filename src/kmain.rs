#![no_std]
#![no_main]
#![feature(alloc_error_handler, fn_align, naked_functions)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

extern crate alloc;

use alloc::alloc::Layout;
use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::memlayout::*;
use crate::utils::logger;

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/mem.asm"));

/// Whenever there is a fatal kernel panic,
/// this function is called. It also prints
/// panic info.
#[panic_handler]
#[no_mangle]
extern "C" fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("FATAL - Kernel Panic:");
    log::error!("{}", info);
    loop {
        // Safety: Disabling interrupts and "halting" processor safely
        // Kernel termination therefore anything after this does not matter
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
extern "C" fn kmain(_hart_id: usize, fdt_ptr: usize) -> ! {
    drivers::uart::init(UART_BASE_ADDR);
    logger::init();
    kheap::init();
    fdt::init(fdt_ptr);

    drivers::plic::init(PLIC_BASE_ADDR);

    memlayout::print_sections();
    mm::init();
    trap::init();
    sched::init();
}

pub mod drivers;
pub mod fdt;
pub mod kheap;
pub mod memlayout;
pub mod mm;
pub mod proc;
pub mod riscv;
pub mod sched;
pub mod syscall;
pub mod trap;
pub mod utils;
