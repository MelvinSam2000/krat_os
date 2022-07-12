static mut PLIC_BASE: Option<*mut PlicRegisters> = None;

const CONTEXT_MAX: usize = 15872;
const SOURCES_MAX: usize = 1024;

#[repr(C, align(4096))]
struct PlicRegisters {
    // base + 0x000000
    priority: [u32; SOURCES_MAX],
    // base + 0x001000
    pending: [u32; SOURCES_MAX / 32],
    _padding1: [u8; 3968],
    // base + 0x002000
    enable: [[u32; SOURCES_MAX / 32]; CONTEXT_MAX],
    _padding2: [u8; 57344],
    // base + 0x200000
    tc: [PlicThresholdClaim; CONTEXT_MAX],
}

#[repr(C, align(4096))]
struct PlicThresholdClaim {
    threshold: u32,
    claim: u32,
    _padding: [u8; 4088],
}

pub fn init(plic_base: usize) {
    unsafe {
        PLIC_BASE = Some(plic_base as *mut PlicRegisters);
    }
    // configure UART interrupts for PLIC
    set_threshold(0, 1);
    enable(10, 1);
    set_priority(10, 1);

    log::debug!("PLIC initialized.");
}

pub fn claim(context: usize) -> Option<u32> {
    unsafe {
        let claim_num = (*PLIC_BASE.unwrap()).tc[context].claim;
        if claim_num == 0 {
            None
        } else {
            Some(claim_num)
        }
        // log::debug!("CLAIM ADDR: {:#010x}",
        //     (&(*PLIC_BASE.unwrap()).tc[context].claim as *const u32) as usize);
    }
}

pub fn complete(source: u32, context: usize) {
    unsafe {
        (*PLIC_BASE.unwrap()).tc[context].claim = source;
    }
}

pub fn set_threshold(threshold: u32, context: usize) {
    unsafe {
        (*PLIC_BASE.unwrap()).tc[context].threshold = threshold & 0b111;
        //     log::debug!("THRESHOLD ADDR: {:#010x}",
        //         (&(*PLIC_BASE.unwrap()).tc[context].threshold as *const u32) as usize);
    }
}

pub fn is_pending(source: usize) -> bool {
    unsafe {
        let out = (*PLIC_BASE.unwrap()).pending[source >> 5] & (1 << (source & 0x1f)) != 0;
        //     log::debug!("PENDING ADDR: {:#010x}",
        //         (&(*PLIC_BASE.unwrap()).pending[source >> 5] as *const u32) as usize);
        out
    }
}

pub fn enable(source: usize, context: usize) {
    unsafe {
        (*PLIC_BASE.unwrap()).enable[context][source >> 5] |= (1 << (source & 0x1f)) as u32;
        // log::debug!("ENABLE ADDR: {:#010x}",
        //     (&(*PLIC_BASE.unwrap()).enable[context][source >> 5] as *const u32) as usize);
    }
}

pub fn set_priority(source: usize, prio: u32) {
    unsafe {
        (*PLIC_BASE.unwrap()).priority[source] = prio;
        // log::debug!("PRIORITY ADDR: {:#010x}",
        //     (&(*PLIC_BASE.unwrap()).priority[source] as *const u32) as usize);
    }
}

// NOTE: Only for virt board
// fn plic_context_for(hart_id: usize) -> usize {
//     return 1 + 2 * hart_id;
// }
