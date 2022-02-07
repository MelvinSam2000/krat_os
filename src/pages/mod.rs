#![allow(dead_code, unused_variables)]
use bitflags::bitflags;

bitflags! {
    struct PageTableEntry: u64 {
        const V = 0 << 1;
        const R = 1 << 1;
        const W = 2 << 1;
        const X = 3 << 1;
        const U = 4 << 1;
        const G = 5 << 1;
        const A = 6 << 1;
        const D = 7 << 1;
        const PPN0 = 0x1ff << 10;
        const PPN1 = 0x1ff << 19;
        const PPN2 = 0x3ff_ffff << 28;
    }
}

const PAGE_TABLE_CAPACITY: usize = 1 << 9;

#[repr(C)]
pub struct PageTable {
    pages: [PageTableEntry; PAGE_TABLE_CAPACITY],
}


pub fn virt_to_phys(virt: usize) -> usize {

    // Get VPN and offset
    let vpn = [
        (virt >> 12) & 0x1ff,
        (virt >> 21) & 0x1ff,
        (virt >> 30) & 0x1ff,
    ];
    let page_offset = virt & 0xfff;

    // Get PPN 0 from csr satp
    let mut ppn = [0; 3];
    ppn[0] = riscv::register::satp::read().ppn() << 12 ;



    unimplemented!();
}

pub mod allocator;