#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use crate::pages::allocator::PhysicalPageAllocator;

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/mem.asm"));
global_asm!(include_str!("asm/trap.asm"));

#[panic_handler]
#[no_mangle]
extern "C" 
fn panic_handler(info: &PanicInfo) -> ! {
    uart::write_str("FATAL - Kernel Panic:\n").unwrap();
    uart::write_fmt(format_args!("{}\n", info)).unwrap();
    loop {}
}

#[no_mangle]
extern "C"
fn kmain() {

    uart::write_str("It works! :D\n").unwrap();
    let p_alloc = PhysicalPageAllocator::new();
    p_alloc.print_pages();
    loop {}
}

pub mod uart;
pub mod pages;