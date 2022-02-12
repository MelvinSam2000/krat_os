#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::arch::global_asm;

use alloc::alloc::Layout;

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/mem.asm"));
global_asm!(include_str!("asm/trap.asm"));

#[panic_handler]
#[no_mangle]
extern "C" 
fn panic_handler(info: &PanicInfo) -> ! {
    uart_print!("FATAL - Kernel Panic:\n");
    uart_print!("{}\n", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[no_mangle]
extern "C"
fn kmain() {
    kheap::init();
    unsafe { memlayout::print_sections() };
    vmem::init();
    uart_print!("It works! :)\n");
    loop {}
}

pub mod debug;
pub mod memlayout;
pub mod uart;
pub mod vmem;
pub mod kheap;