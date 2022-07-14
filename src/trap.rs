use alloc::string::String;
use core::arch::asm;
use core::fmt::Debug;
use core::fmt::Write;

use riscv::register::mtvec::TrapMode;

use crate::drivers::plic;
use crate::drivers::uart;
use crate::sched::sched;
use crate::syscall::prep_syscall;
use crate::uart_print;

extern "C" {
    fn trap_vector();
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub gregs: [usize; 32],
    pub fregs: [usize; 32],
    pub pc: usize,
    pub satp: usize,
}

static mut TRAP_FRAME: TrapFrame = TrapFrame {
    gregs: [0; 32],
    fregs: [0; 32],
    pc: 0,
    satp: 0,
};

pub fn init() {
    unsafe {
        let trap_vector_addr = (trap_vector as *const ()) as usize;
        log::debug!("TRAP VECTOR ADDRESS: {:#018x}", trap_vector_addr);

        // Configure trap vector
        riscv::register::stvec::write(trap_vector_addr, TrapMode::Direct);

        // Store trap frame into sscratch
        riscv::register::sscratch::write((&TRAP_FRAME as *const _) as usize);

        TRAP_FRAME.satp = riscv::register::satp::read().bits();

        // Enable interrupts
        asm! {
            // enable all interrupt types
            "li     t0, (1 << 9) | (1 << 5) | (1 << 1)",
            "csrs   sie, t0",
            // global interrupt enable
            "csrsi  sstatus, 1 << 1",
        }
    }
}

/// This function handles traps. The trap vector
/// jumps to this function after saving the trap
/// frame in order to be able to handle all supervisor
/// traps in Rust.
#[no_mangle]
extern "C" fn trap_handler(
    trap_frame: &mut TrapFrame,
    scause: usize,
    stval: usize,
    sstatus: usize,
) -> usize {
    if log::log_enabled!(log::Level::Debug) {
        let mut msg = String::from("ENTERED TRAP HANDLER...\n");
        writeln!(msg, "\tstval:     {:#018x}", stval).unwrap();
        writeln!(msg, "\tscause:    {:#018x}", scause).unwrap();
        writeln!(msg, "\tsstatus:   {:#018x}", sstatus).unwrap();
        writeln!(msg, "\ttrap:      {:#018x?}", trap_frame).unwrap();
        log::debug!("{}", msg);
    }

    // Trap table for interrupts and sync exceptions
    let is_interrupt = (scause >> 63) != 0;
    let cause = scause & 0xff;
    if is_interrupt {
        match cause {
            1 => {
                // Supervisor software interrupt.
                log::info!("Supervisor software interrupt.");
                // Safety: Clearing software interrupt bit
                unsafe {
                    asm! {
                        "csrci  sip, 1 << 1",
                    }
                }
            }
            5 => {
                // Supervisor timer interrupt.
                log::debug!("Supervisor timer interrupt.");
                sched(trap_frame);
            }
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
                        }
                        _ => {
                            log::info!("Int ID: {} has no handler.", int_source);
                        }
                    }
                    plic::complete(int_source, 1);
                }
            }
            _ => {
                panic!("Invalid scause: {:#018x}", scause);
            }
        }
    } else {
        match cause {
            0 => {
                // Instruction address misaligned.
                panic!("Instruction address misaligned.");
            }
            1 => {
                // Instruction access fault.
                panic!("Instruction access fault.");
            }
            2 => {
                // Illegal instruction.
                panic!("Illegal instruction.");
            }
            3 => {
                // Breakpoint.
                log::info!("Breakpoint.");
            }
            4 => {
                // Load address misaligned.
                panic!("Load address misaligned at {:#018x}", stval);
            }
            5 => {
                // Load access fault.
                log::error!("Load access fault.");
                log::error!("Invalid access at {:#018x}", stval);
                trap_frame.pc += 4;
            }
            6 => {
                // Store/AMO address misaligned.
                panic!("Store/AMO address misaligned at {:#018x}", stval);
            }
            7 => {
                // Store/AMO access fault.
                log::error!("Store/AMO access fault.");
                log::error!("Invalid access at {:#018x}", stval);
                trap_frame.pc += 4;
            }
            8 => {
                // Environment call from U-mode.
                log::info!("Environment call from U-mode.");
                // ra = do_syscall(a7, a0-a6)
                trap_frame.gregs[1] = prep_syscall(trap_frame.gregs[17], &trap_frame.gregs[10..16]);
            }
            9 => {
                // Environment call from S-mode.
                log::info!("Environment call from S-mode.");
            }
            12 => {
                // Instruction page fault.
                log::info!("Instruction page fault.");
                log::info!("Invalid access at {:#018x}", stval);
                trap_frame.pc += 4;
            }
            13 => {
                // Load page fault.
                log::info!("Load page fault.");
                log::info!("Invalid access at {:#018x}", stval);
                trap_frame.pc += 4;
            }
            15 => {
                // Store/AMO page fault.
                log::info!("Store/AMO page fault.");
                log::info!("Invalid access at {:#018x}", stval);
                trap_frame.pc += 4;
            }
            _ => {
                panic!("Invalid scause: {:#018x}", scause);
            }
        }
    }

    trap_frame.pc
}
