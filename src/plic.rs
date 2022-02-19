static mut PLIC_BASE: usize = 0;

const REG_PRIORITY: usize =    0x00_0000;
const REG_PENDING: usize =     0x00_1000;
const REG_INT_ENABLE: usize =  0x00_2000;
const REG_THRESHOLD: usize =   0x20_0000;
const REG_CLAIM: usize =       0x20_0004;

pub fn init(plic_base: usize) {
    unsafe {
        PLIC_BASE = plic_base;
    }
    log::debug!("PLIC initialized.");
}

pub fn claim() -> Option<u32> {

    let claim_num = unsafe { *((PLIC_BASE + REG_CLAIM) as *const u32) };

    if claim_num == 0 {
        None
    } else {
        Some(claim_num)
    }
}

pub fn complete(id: u32) {
    unsafe {
        *((PLIC_BASE + REG_CLAIM) as *mut u32) = id as u32;
    }
}

pub fn set_threshold(threshold: u32) {
    unsafe {
        *((PLIC_BASE + REG_THRESHOLD) as *mut u32) = threshold & 0b111;
    }
}

pub fn is_pending(id: u32) -> bool {
    unsafe {
        *((PLIC_BASE + REG_PENDING) as *const u32) & (1 << id) != 0
    }
}

pub fn enable(id: u32) {
    unsafe {
        *((PLIC_BASE + REG_INT_ENABLE) as *mut u32) |= 1 << id;
    }
}

pub fn set_priority(id: u32, prio: u32) {
    unsafe {
        *((PLIC_BASE + REG_PRIORITY + (4*id as usize)) as *mut u32) = prio;
    }
}

// NOTE: Only for virt board
fn plic_context_for(hart_id: usize) -> usize {
    return 1 + 2 * hart_id;
}
