use core::arch::asm;

/// Send a timer interrupt "usec" microseconds from now
pub fn timer_int(usec: usize) {

    let usec = usec*10;

    // Safety: Making an SBI systemcall for timer
    unsafe {
        asm! {
            // add time
            "csrr   t0, time",
            "mv     t1, {}",
            "add    t0, t0, t1",
            // call sbi sbi_set_time(time + usec)
            "li     a6, 0",
            "li     a7, 0x54494d45",
            "mv     a0, t0",
            "ecall",
            in(reg) usec
        }
    }
}