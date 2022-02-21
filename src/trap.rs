use core::fmt::Debug;
use core::arch::asm;

use alloc::string::String;
use alloc::format;

use crate::plic;
use crate::uart;
use crate::uart_print;

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

    if log::log_enabled!(log::Level::Debug) {
        let mut msg = String::from("ENTERED TRAP HANDLER...\n");
        msg += &format!("\tsepc:      {:#018x}\n", sepc);
        msg += &format!("\tstval:     {:#018x}\n", stval);
        msg += &format!("\tscause:    {:#018x}\n", scause);
        msg += &format!("\tsstatus:   {:#018x}\n", sstatus);
        log::debug!("{}", msg);
        //log::info!("trap:   {:#018x?}", trap_frame);
    }

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
                log::debug!("Supervisor timer interrupt.");
                unsafe { asm! {
                    // add time
                    "csrr   t0, time",
                    "li     t1, 10000000",
                    "add    t0, t0, t1",
                    // call sbi sbi_set_time(time + 10000000)
                    "li     a6, 0",
                    "li     a7, 0x54494d45",
                    "mv     a0, t0",
                    "ecall",
                }}
            },
            9 => {
                // Supervisor external interrupt.
                log::debug!("Supervisor external interrupt.");
                if let Some(int_source) = plic::claim(1) {
                    match int_source {
                        10 => {
                            let c = uart::get_char();
                            if log::log_enabled!(log::Level::Debug) {
                                log::debug!("RECV: {}", c);
                            } else {
                                uart_print!("{}", c);
                            }
                        },
                        _ => {
                            log::info!("Int ID: {} has no handler.", int_source);
                        }
                    }
                    plic::complete(int_source, 1);
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
                panic!("Instruction address misaligned.");
            },
            1 => {
                // Instruction access fault.
                panic!("Instruction access fault.");
            },
            2 => {
                // Illegal instruction.
                panic!("Illegal instruction.");
            },
            3 => {
                // Breakpoint.
                log::info!("Breakpoint.");
            },
            4 => {
                // Load address misaligned.
                log::error!("Load address misaligned.");
            },
            5 => {
                // Load access fault.
                log::error!("Load access fault.");
                log::error!("Invalid access at {:#010x}", stval);
                ret_pc += 4;
            },
            6 => {
                // Store/AMO address misaligned.
                log::error!("Store/AMO address misaligned.");
            },
            7 => {
                // Store/AMO access fault.
                log::error!("Store/AMO access fault.");
                log::error!("Invalid access at {:#010x}", stval);
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