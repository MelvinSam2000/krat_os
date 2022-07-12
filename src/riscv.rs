use core::arch::asm;

use riscv::register::satp::Mode;
use sbi::timer::set_timer;

/// Send a timer interrupt "usec" microseconds from now
pub fn timer_int(usec: usize) {
    let usec = usec * 10;
    let time = riscv::register::time::read().wrapping_add(usec);
    set_timer(time as u64).unwrap();
}

/// Set MMU
pub fn mmu_set(addr: usize) {
    // Safety: Setting SATP register and flushing TLB
    unsafe {
        riscv::register::satp::set(Mode::Sv39, 0, (addr as usize) >> 12);
        asm!("sfence.vma");
    }
}
