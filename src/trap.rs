use core::fmt::Debug;
use core::arch::asm;

use crate::plic;
use crate::uart;

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
    sepc: u64, stval: u64, scause: u64, sstatus: u64,
    _trap_frame: &mut TrapFrame
) -> u64 {
    log::info!("ENTERED TRAP HANDLER...");
    log::info!("sepc:      {:#018x}", sepc);
    log::info!("stval:     {:#018x}", stval);
    log::info!("scause:    {:#018x}", scause);
    log::info!("sstatus:   {:#018x}", sstatus);
    //log::info!("trap:   {:#018x?}", trap_frame);

    let mut ret_pc = sepc;

    // Trap table for interrupts and sync exceptions
    let is_interrupt = (scause >> 63) != 0;
    let cause = scause & 0xff;
    if is_interrupt {
        match cause {
            1 => {
                // Supervisor software interrupt.
                log::info!("Supervisor software interrupt.");
                unsafe { asm! {
                    "csrci  sip, 1 << 1",
                }}
            },
            5 => {
                // Supervisor timer interrupt.
                log::info!("Supervisor timer interrupt.");
                unsafe { asm! {
                    "li     t0, 1 << 5",
                    "csrc   sip, t0",
                }}
            },
            9 => {
                // Supervisor external interrupt.
                log::info!("Supervisor external interrupt.");
                if let Some(int_id) = plic::claim() {
                    match int_id {
                        10 => {
                            let c = uart::get_char();
                            log::info!("RECV: {}/n", c);
                        },
                        _ => {
                            log::info!("Int ID: {} has no handler.", int_id);
                        }
                    }
                }
            },
            _ => {
                panic!("Invalid scause: {:#018x}", scause);
            }
        }
    } else {
        match cause {
            0 => {
                // Instruction address misaligned. 
                log::info!("Instruction address misaligned.");
            },
            1 => {
                // Instruction access fault.
                log::info!("Instruction access fault.");
            },
            2 => {
                // Illegal instruction.
                log::info!("Illegal instruction.");
            },
            3 => {
                // Breakpoint.
                log::info!("Breakpoint.");
            },
            4 => {
                // Load address misaligned.
                log::info!("Load address misaligned.");
            },
            5 => {
                // Load access fault.
                log::info!("Load access fault.");
                log::info!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            6 => {
                // Store/AMO address misaligned.
                log::info!("Store/AMO address misaligned.");
            },
            7 => {
                // Store/AMO access fault.
                log::info!("Store/AMO access fault.");
                log::info!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            8 => {
                // Environment call from U-mode.
                log::info!("Environment call from U-mode.");
            },
            9 => {
                // Environment call from S-mode.
                log::info!("Environment call from S-mode.");
            },
            12 => {
                // Instruction page fault.
                log::info!("Instruction page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            13 => {
                // Load page fault.
                log::info!("Load page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            15 => {
                // Store/AMO page fault.
                log::info!("Store/AMO page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            _ => {
                panic!("Invalid scause: {:#018x}", scause);
            }
        }
    }

    ret_pc
}