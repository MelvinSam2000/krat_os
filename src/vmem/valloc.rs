#![allow(dead_code, unused_variables)]

use bitflags::bitflags;
use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::vmem::palloc;

bitflags! {
    pub struct PteFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[bitfield]
pub struct Pte {
    padding: B10,
    ppn2: B26,
    ppn1: B9,
    ppn0: B9,
    rsw: B2,
    flags: B8,
}

#[bitfield]
pub struct VirtAddr {
    padding: B25,
    vpn2: B9,
    vpn1: B9,
    vpn0: B9,
    page_offset: B12,
}

#[bitfield]
pub struct PhysAddr {
    padding: B8,
    ppn2: B26,
    ppn1: B9,
    ppn0: B9,
    page_offset: B12,
}


#[repr(C)]
pub struct PageTable {
    pages: [Pte; 512],
}

pub struct VMController {
    root_table: *mut PageTable,
}

impl VMController{

    pub fn new() -> Self {
        palloc::init();
        let root_table = unsafe { palloc::alloc() as *mut PageTable };
        Self {
            root_table,
        }
    }

    pub fn root(&mut self) -> *mut PageTable {
        self.root_table
    }

    pub fn map(table: &mut PageTable, vaddr: VirtAddr, paddr: PhysAddr, flags: PteFlags) {
        //let mut pte = Pte::new();
        //pte.set_flags(flags.bits);
        //pte.set_ppn0(paddr.ppn0());
        //pte.set_ppn1(paddr.ppn1());
        //pte.set_ppn2(paddr.ppn2());
        unimplemented!();
    }
}
