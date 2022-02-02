#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[no_mangle]
unsafe extern "C"
fn _start() {
    asm!("nop");
    kmain();
}

#[panic_handler]
extern "C" 
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C"
fn kmain() {
    loop {}
}
