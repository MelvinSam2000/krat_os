use core::arch::asm;

use crate::uart_print;

#[repr(C)]
struct TrapFrame {
    _nothing_yet: u64,
}

/// This function handles traps. The trap vector
/// jumps to this function after saving the trap
/// frame in order to be able to handle all supervisor
/// traps in Rust.
#[no_mangle]
extern "C"
fn trap_handler(
    sepc: u64, stval: u64, scause: u64, status: u64,
    _trap_frame: &TrapFrame
) {
    uart_print!("ENTERED TRAP HANDLER...\n");
    uart_print!("sepc:   {:#018x}\n", sepc);
    uart_print!("stval:  {:#018x}\n", stval);
    uart_print!("scause: {:#018x}\n", scause);
    uart_print!("status: {:#018x}\n", status);
    unsafe { loop { asm!("wfi"); } }
}