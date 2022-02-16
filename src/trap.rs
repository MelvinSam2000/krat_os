use crate::uart_print;

use core::fmt::Debug;

#[derive(Debug)]
#[repr(C)]
struct TrapFrame {
    pub gregs: [u64; 32],
    pub fregs: [u64; 32],
    pub satp: u64,
}

static mut TRAP_FRAME: TrapFrame = TrapFrame {
    gregs: [0; 32],
    fregs: [0; 32],
    satp: 0,
};

pub fn init() {
    // Store trap frame into sscratch
    riscv::register::sscratch::write(
        unsafe { (&TRAP_FRAME as *const TrapFrame) as usize });
}


/// This function handles traps. The trap vector
/// jumps to this function after saving the trap
/// frame in order to be able to handle all supervisor
/// traps in Rust.
#[no_mangle]
extern "C"
fn trap_handler(
    sepc: u64, stval: u64, scause: u64, status: u64,
    trap_frame: &mut TrapFrame
) -> u64 {
    uart_print!("ENTERED TRAP HANDLER...\n");
    uart_print!("sepc:   {:#018x}\n", sepc);
    uart_print!("stval:  {:#018x}\n", stval);
    uart_print!("scause: {:#018x}\n", scause);
    uart_print!("status: {:#018x}\n", status);
    uart_print!("trap:   {:#018x?}\n", trap_frame);

    let mut ret_pc = sepc;

    // Trap table for interrupts and sync exceptions
    let is_interrupt = (scause >> 63) != 0;
    let cause = scause & 0xff;
    if is_interrupt {
        match cause {
            1 => {
                // Supervisor software interrupt.
                uart_print!("Supervisor software interrupt.\n");
            },
            4 => {
                // Supervisor timer interrupt.
                uart_print!("Supervisor timer interrupt.\n");
            },
            9 => {
                // Supervisor external interrupt.
                uart_print!("Supervisor external interrupt.\n");
            },
            _ => {
                panic!("Invalid scause: {}\n", scause);
            }
        }
    } else {
        match cause {
            0 => {
                // Instruction address misaligned. 
                uart_print!("Instruction address misaligned.\n");
            },
            1 => {
                // Instruction access fault.
                uart_print!("Instruction access fault.\n");
            },
            2 => {
                // Illegal instruction.
                uart_print!("Illegal instruction.\n");
            },
            3 => {
                // Breakpoint.
                uart_print!("Breakpoint.\n");
            },
            4 => {
                // Load address misaligned.
                uart_print!("Load address misaligned.\n");
            },
            5 => {
                // Load access fault.
                uart_print!("Load access fault.\n");
            },
            6 => {
                // Store/AMO address misaligned.
                uart_print!("Store/AMO address misaligned.\n");
            },
            7 => {
                // Store/AMO access fault.
                uart_print!("Store/AMO access fault.\n");
            },
            8 => {
                // Environment call from U-mode.
                uart_print!("Environment call from U-mode.\n");
            },
            9 => {
                // Environment call from S-mode.
                uart_print!("Environment call from S-mode.\n");
            },
            12 => {
                // Instruction page fault.
                uart_print!("Instruction page fault.\n");
                ret_pc += 4;
            },
            13 => {
                // Load page fault.
                uart_print!("Load page fault.\n");
                ret_pc += 4;
            },
            15 => {
                // Store/AMO page fault.
                uart_print!("\n");
                ret_pc += 4;
            },
            _ => {
                panic!("Invalid scause: {}\n", scause);
            }
        }
    }

    ret_pc
}