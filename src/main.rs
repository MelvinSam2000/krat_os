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
    uart_print!("FATAL - Kernel Panic:\n");
    uart_print!("{}\n", info);
    loop { unsafe { asm!("wfi"); } }
    
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
fn kmain(_hart_id: u64, fdt_ptr: u64) {
    
    uart::init(UART_BASE_ADDR);
    fdt::init(fdt_ptr);
    
    plic::init(PLIC_BASE_ADDR);

    trap::init();
    kheap::init();
    // unsafe { memlayout::print_sections() };
    vmem::init();

    // configure UART interrupts for PLIC
    plic::set_threshold(0);
    plic::enable(10);
    plic::set_priority(10, 1);

    // testing clint


    unsafe {
        asm!{
            // enable all interrupt types
            "li     t0, (1 << 9) | (1 << 5) | (1 << 1)",
            "csrs   sie, t0",
            
            // global interrupt enable
            "li     t0, 1 << 1",
            "csrs   sstatus, t0",

            // call sbi sbi_set_time(20000000)
            // "li     a6, 0",
            // "li     a7, 0x54494d45",
            // "li     a0, 20000000",
            // "ecall"
        }
    }
  
  
    
    uart_print!("It works! :)\n");

    loop { unsafe { asm!("wfi"); } }
}

pub mod debug;
pub mod memlayout;
pub mod uart;
pub mod vmem;
pub mod kheap;
pub mod trap;
pub mod fdt;
pub mod plic;