#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use uart::Uart;

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/trap.asm"));

#[panic_handler]
#[no_mangle]
extern "C" 
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C"
fn kmain() {

    let mut uart = Uart::new();
    uart.write_str("It works! :D\n").unwrap();
    loop {}
}

pub mod uart;