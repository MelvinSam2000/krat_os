#![allow(dead_code)]

use crate::vmem::palloc;
use crate::uart_print;
use crate::vmem::pte::*;
use crate::vmem::addr::*;

#[repr(C)]
pub struct PageTable {
    entries: [Pte; 512],
}

pub unsafe fn map_page(
    root: *mut PageTable, 
    va: VirtAddr, 
    pa: PhysAddr, 
    flags: PteFlags) {
    
    let vpn = [ va.vpn0(), va.vpn1(), va.vpn2() ];
    let mut pte = Pte::new();
    let mut pt = root;

    for lvl in (1..=2).rev() {
        pte = (*pt).entries[vpn[lvl] as usize];
        if !pte.flags().contains(PteFlags::V) {
            let page = palloc::alloc() as *mut PageTable;
            if page.is_null() {
                panic!("map_page: No more pages can be allocated.");
            }
            pte.set_ppn(page as u64 >> 12);
            pte.set_flags(PteFlags::V);
            (*pt).entries[vpn[lvl] as usize] = pte;
            //uart_print!("{:#010x} + {:#05x} <= {:#010x} {:?}\n", 
            //    pt as usize, vpn[lvl] as usize, pte.ppn(), print_flags(pte.flags()));
        }
        pt = (pte.ppn() << 12) as *mut PageTable;
    }

    if (*pt).entries[vpn[0] as usize].flags().contains(PteFlags::V) {
        panic!("map_page: Double mapping of page {:#010x} at {:#010x}\n", pt as usize, vpn[0]);
    }
    
    pte.set_ppn(pa.ppn());
    pte.set_flags(flags | PteFlags::V);
    (*pt).entries[vpn[0] as usize] = pte;
    //uart_print!("Mapped {:#010x} -> {:#010x}\n", va.bits, pa.ppn() << 12);
}

pub unsafe fn va_to_pa(root: *mut PageTable, va: VirtAddr) -> PhysAddr {

    let vpn = [ va.vpn0(), va.vpn1(), va.vpn2() ];
    let offset = va.page_offset();
    let mut pt = root;
    pt = (*pt).entries[vpn[2] as usize].ppn() as *mut PageTable;
    pt = (*pt).entries[vpn[1] as usize].ppn() as *mut PageTable;
    let pa = ((*pt).entries[vpn[0] as usize].ppn() << 12) | (offset as u64);
    PhysAddr::from_bits(pa)
}

pub unsafe fn map_many(
    root: *mut PageTable,
    va: VirtAddr,
    pa: PhysAddr,
    flags: PteFlags,
    num: u64,
) {
    let mut pa = pa;
    let mut va = va;
    for i in 0..num {
        pa.set_ppn(pa.ppn() + 1);
        va = VirtAddr::from_bits(va.bits + (1 << 12));
        //uart_print!("Iter {}: Mapping {:#010x} -> {:#010x}\n", i, va.bits, pa.ppn() << 12);
        map_page(root, va, pa, flags);
    }
}

pub unsafe fn id_map_range(
    root: *mut PageTable,
    va_i: VirtAddr,
    va_f: VirtAddr,
    pa: PhysAddr,
    flags: PteFlags,
) {
    let n = va_f.vpn0() - va_i.vpn0();
    map_many(root, va_i, pa, flags, n as u64);
}

#[cfg(debug_assertions)]
pub unsafe fn print_pt(pt: *mut PageTable) {
    uart_print!("PTEs at {:#010x}...\n", pt as usize);
    for (i, entry) in (*pt).entries.iter().enumerate() {
        if entry.flags().contains(PteFlags::V) {
            uart_print!("\t [{:03}] => PTE <{}>\n", i, entry);
        }
    }
}

#[cfg(debug_assertions)]
pub unsafe fn print_pts_dfs(pt: *mut PageTable, lvl: u8) {
    print_pt(pt);
    if lvl == 0 || lvl > 2 {
        return;
    }
    for entry in (*pt).entries.iter() {
        if entry.flags().contains(PteFlags::V) {
            print_pts_dfs((entry.ppn() << 12) as *mut PageTable , lvl - 1);
        }
    }
}