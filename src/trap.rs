use riscv::register::mtvec::TrapMode;

use core::fmt::Debug;
use core::arch::asm;
use alloc::string::String;
use alloc::format;

use crate::drivers::plic;
use crate::drivers::uart;
use crate::uart_print;
use crate::sched::sched;
use crate::syscall::do_syscall;
use crate::memlayout::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub gregs: [u64; 32],
    pub fregs: [u64; 32],
    pub pc: u64,
    pub satp: u64,
    pub trap_handler_ptr: u64,
    pub kern_satp: u64,
}

#[allow(unreachable_code, dead_code)]
#[link_section = ".tramp.frame"]
static mut TRAP_FRAME: TrapFrame = TrapFrame {
    gregs: [0; 32],
    fregs: [0; 32],
    pc: 0,
    satp: 0,
    trap_handler_ptr: 0,
    kern_satp: 0,
};

pub fn init() {

    unsafe {

        let sscratch_val = TRAMP_VADDR + (TRAMP_FRAME - TRAMP_VECTOR);
        let fptr = (trap_handler as *const ()) as u64;
        
        // load trap handler function pointer to trap frame
        asm! {
            "mv     t0, {}", 
            "mv     t1, {}", 
            "sd     t0, 528(t1)",
            in(reg) fptr, in(reg) sscratch_val,
        }

        // load kernel root page to trap frame
        asm! {
            "csrr   t0, satp",
            "sd     t0, 536(t1)",
        }
    
        // Configure trap vector
        riscv::register::stvec::write(TRAMP_VADDR, TrapMode::Direct);

        // Store trap frame into sscratch
        riscv::register::sscratch::write(sscratch_val);

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


#[naked]
#[no_mangle]
#[link_section = ".tramp.vector"]
#[repr(align(4))]
// Safety:: Is an assembly function
unsafe extern "C" fn trap_vector() -> ! {
    // asm!(include_str!("asm/trap.asm"), options(noreturn));
    
    asm!("
        // swap sscratch with x31
        csrrw   x31, sscratch, x31

        // store general purpose registers into trap frame (except x31)
        sd      x1, 8(x31)
        sd      x2, 16(x31)
        sd      x3, 24(x31)
        sd      x4, 32(x31)
        sd      x5, 40(x31)
        sd      x6, 48(x31)
        sd      x7, 56(x31)
        sd      x8, 64(x31)
        sd      x9, 72(x31)
        sd      x10, 80(x31)
        sd      x11, 88(x31)
        sd      x12, 96(x31)
        sd      x13, 104(x31)
        sd      x14, 112(x31)
        sd      x15, 120(x31)
        sd      x16, 128(x31)
        sd      x17, 136(x31)
        sd      x18, 144(x31)
        sd      x19, 152(x31)
        sd      x20, 160(x31)
        sd      x21, 168(x31)
        sd      x22, 176(x31)
        sd      x23, 184(x31)
        sd      x24, 192(x31)
        sd      x25, 200(x31)
        sd      x26, 208(x31)
        sd      x27, 216(x31)
        sd      x28, 224(x31)
        sd      x29, 232(x31)
        sd      x30, 240(x31)

        // store x31
        csrr    t0, sscratch
        sd      t0, 248(x31)

        // store fp registers into trap frame
        fsd     f0, 256(x31)
        fsd     f1, 264(x31)
        fsd     f2, 272(x31)
        fsd     f3, 280(x31)
        fsd     f4, 288(x31)
        fsd     f5, 296(x31)
        fsd     f6, 304(x31)
        fsd     f7, 312(x31)
        fsd     f8, 320(x31)
        fsd     f9, 328(x31)
        fsd     f10, 336(x31)
        fsd     f11, 344(x31)
        fsd     f12, 352(x31)
        fsd     f13, 360(x31)
        fsd     f14, 368(x31)
        fsd     f15, 376(x31)
        fsd     f16, 384(x31)
        fsd     f17, 392(x31)
        fsd     f18, 400(x31)
        fsd     f19, 408(x31)
        fsd     f20, 416(x31)
        fsd     f21, 424(x31)
        fsd     f22, 432(x31)
        fsd     f23, 440(x31)
        fsd     f24, 448(x31)
        fsd     f25, 456(x31)
        fsd     f26, 464(x31)
        fsd     f27, 472(x31)
        fsd     f28, 480(x31)
        fsd     f29, 488(x31)
        fsd     f30, 496(x31)
        fsd     f31, 504(x31)

        // store pc into trap frame
        csrr    t0, sepc
        sd      t0, 512(x31)

        // store satp into trap frame
        csrr    t0, satp
        sd      t0, 520(x31)

        // get trap frame
        mv      a0, x31

        // get status registers for trap handler
        csrr    a1, scause
        csrr    a2, stval
        csrr    a3, sstatus

        // swap to kernel pages
        ld      t0, 536(x31)
        csrw    satp, t0
        // sfence.vma
        
        // restore sscratch
        mv      t0, x31      
        csrrw   x31, sscratch, x31

        // enter Rust trap_handler
        ld      t0, 528(t0)
        jalr    t0

        // get sscratch
        csrr    x31, sscratch

        // swap to user pages
        ld      t0, 520(x31)
        csrw    satp, t0
        // sfence.vma
    
        // update return pc
        csrw    sepc, a0

        // load general purpose registers from trap frame
        ld      x1, 8(x31)
        ld      x2, 16(x31)
        ld      x3, 24(x31)
        ld      x4, 32(x31)
        ld      x5, 40(x31)
        ld      x6, 48(x31)
        ld      x7, 56(x31)
        ld      x8, 64(x31)
        ld      x9, 72(x31)
        ld      x10, 80(x31)
        ld      x11, 88(x31)
        ld      x12, 96(x31)
        ld      x13, 104(x31)
        ld      x14, 112(x31)
        ld      x15, 120(x31)
        ld      x16, 128(x31)
        ld      x17, 136(x31)
        ld      x18, 144(x31)
        ld      x19, 152(x31)
        ld      x20, 160(x31)
        ld      x21, 168(x31)
        ld      x22, 176(x31)
        ld      x23, 184(x31)
        ld      x24, 192(x31)
        ld      x25, 200(x31)
        ld      x26, 208(x31)
        ld      x27, 216(x31)
        ld      x28, 224(x31)
        ld      x29, 232(x31)
        ld      x30, 240(x31)

        // load fp registers from trap frame
        fld     f0, 256(x31)
        fld     f1, 264(x31)
        fld     f2, 272(x31)
        fld     f3, 280(x31)
        fld     f4, 288(x31)
        fld     f5, 296(x31)
        fld     f6, 304(x31)
        fld     f7, 312(x31)
        fld     f8, 320(x31)
        fld     f9, 328(x31)
        fld     f10, 336(x31)
        fld     f11, 344(x31)
        fld     f12, 352(x31)
        fld     f13, 360(x31)
        fld     f14, 368(x31)
        fld     f15, 376(x31)
        fld     f16, 384(x31)
        fld     f17, 392(x31)
        fld     f18, 400(x31)
        fld     f19, 408(x31)
        fld     f20, 416(x31)
        fld     f21, 424(x31)
        fld     f22, 432(x31)
        fld     f23, 440(x31)
        fld     f24, 448(x31)
        fld     f25, 456(x31)
        fld     f26, 464(x31)
        fld     f27, 472(x31)
        fld     f28, 480(x31)
        fld     f29, 488(x31)
        fld     f30, 496(x31)
        fld     f31, 504(x31)

        // load x31
        ld      x31, 248(x31)

        sret
    ", options(noreturn));
    
}



/// This function handles traps. The trap vector
/// jumps to this function after saving the trap
/// frame in order to be able to handle all supervisor
/// traps in Rust.
// #[link_section = ".trampoline.handler"]
#[no_mangle]
extern "C"
fn trap_handler(
    trap_frame: &mut TrapFrame,
    scause: u64, stval: u64, sstatus: u64,
) -> u64 {

    if log::log_enabled!(log::Level::Debug) {
        let mut msg = String::from("ENTERED TRAP HANDLER...\n");
        msg += &format!("\tstval:     {:#018x}\n", stval);
        msg += &format!("\tscause:    {:#018x}\n", scause);
        msg += &format!("\tsstatus:   {:#018x}\n", sstatus);
        log::debug!("{}", msg);
        //log::info!("trap:   {:#018x?}", trap_frame);
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
                unsafe { asm! {
                    "csrci  sip, 1 << 1",
                }}
            },
            5 => {
                // Supervisor timer interrupt.
                log::debug!("Supervisor timer interrupt.");
                sched(trap_frame);
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
                panic!("Load address misaligned at {:#010x}", stval);
            },
            5 => {
                // Load access fault.
                log::error!("Load access fault.");
                log::error!("Invalid access at {:#010x}", stval);
                trap_frame.pc += 4;
            },
            6 => {
                // Store/AMO address misaligned.
                panic!("Store/AMO address misaligned at {:#010x}", stval);
            },
            7 => {
                // Store/AMO access fault.
                log::error!("Store/AMO access fault.");
                log::error!("Invalid access at {:#010x}", stval);
                trap_frame.pc += 4;
            },
            8 => {
                // Environment call from U-mode.
                log::info!("Environment call from U-mode.");
                // ra = do_syscall(a7, a0-a6)
                trap_frame.gregs[1] = do_syscall(
                    trap_frame.gregs[17], &trap_frame.gregs[10..16]);
            },
            9 => {
                // Environment call from S-mode.
                log::info!("Environment call from S-mode.");
            },
            12 => {
                // Instruction page fault.
                log::info!("Instruction page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                trap_frame.pc += 4;
            },
            13 => {
                // Load page fault.
                log::info!("Load page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                trap_frame.pc += 4;
            },
            15 => {
                // Store/AMO page fault.
                log::info!("Store/AMO page fault.");
                log::info!("Invalid access at {:#010x}", stval);
                trap_frame.pc += 4;
            },
            _ => {
                panic!("Invalid scause: {:#018x}", scause);
            }
        }
    }

    trap_frame.pc
}