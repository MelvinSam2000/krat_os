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
    uart_print!("FATAL - Kernel Panic:\n");
    uart_print!("{}\n", info);
    loop {}
}

#[no_mangle]
extern "C"
fn kmain() {

    uart_print!("It works! :D\n");
    unsafe {
        let mut p_alloc = PhysicalPageAllocator::new();
        let p1 = p_alloc.alloc().unwrap();
        let p2 = p_alloc.alloc().unwrap();
        uart_print!("P1 0x{:x}\n", p1 as usize);
        uart_print!("P2 0x{:x}\n", p2 as usize);
        p_alloc.dealloc(p1).unwrap();
    }
    loop {}
}

pub mod debug;
pub mod uart;
pub mod pages;