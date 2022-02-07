#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("asm/boot.asm"));
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
    loop {}
}

pub mod uart;