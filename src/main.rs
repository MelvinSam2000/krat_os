#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use uart::Uart;

global_asm!(include_str!("asm/boot.asm"));

#[panic_handler]
#[no_mangle]
extern "C" 
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C"
fn kmain() {

    Uart::write_str("Hello World!\n");
    loop {}
}

pub mod uart;