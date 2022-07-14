use core::arch::asm;

use riscv::register::satp::Mode;
use sbi::timer::set_timer;

/// Send a timer interrupt "usec" microseconds from now
pub fn timer_int(usec: usize) {
    let time = riscv::register::time::read().wrapping_add(usec);
    set_timer(time as u64).unwrap();
}

/// Set MMU and immediately update PC register to match the same virtual address
pub fn mmu_init(addr: usize) -> ! {
    // Safety: Setting SATP register and flushing TLB
    unsafe {
        asm! {
            "la     t0, kmain_end",
        }
        riscv::register::satp::set(Mode::Sv39, 0, (addr as usize) >> 12);
        asm! {
            "sfence.vma",
            "jalr   t0"
        }
    }
    loop {}
}
