use crate::mm::addr::*;
use crate::mm::phys;
use crate::mm::pte::*;
use crate::uart_print;

#[repr(C)]
pub struct PageTable {
    entries: [Pte; 512],
}

/// Creates a 3 level mapping of va to pa, using
/// the page table entries from the root provided.
pub unsafe fn map_page(root: *mut PageTable, va: VirtAddr, pa: PhysAddr, flags: PteFlags) {
    let vpn = va.vpn();
    let mut pte = Pte::default();
    let mut pt = root;

    for lvl in (1..=2).rev() {
        pte = (*pt).entries[vpn[lvl]];
        if !pte.is_valid() {
            let page = phys::alloc() as *mut PageTable;
            if page.is_null() {
                panic!("map_page: No more pages can be allocated.");
            }
            pte.set_ppn(page as u64 >> 12);
            pte.set_flags(PteFlags::V);
            (*pt).entries[vpn[lvl]] = pte;
        }
        pt = pte.pt();
    }

    if (*pt).entries[vpn[0]].is_valid() {
        panic!(
            "map_page: Double mapping of page {:#010x} at {:#010x}\n",
            pt as usize, vpn[0]
        );
    }

    pte.set_ppn(pa.ppn());
    pte.set_flags(flags | PteFlags::V);
    (*pt).entries[vpn[0]] = pte;
}

/// Unmap a virtual address from the page table.
/// Assume the virtual address is indeed present, otherwise panic.
pub unsafe fn unmap_page(root: *mut PageTable, va: VirtAddr) {
    let vpn = va.vpn();
    let mut pt = root;
    pt = (*pt).entries[vpn[2]].pt();
    pt = (*pt).entries[vpn[1]].pt();
    let pte = &mut (*pt).entries[vpn[0]];
    if !pte.is_valid() {
        panic!("Attempting to unmap an invalid va");
    }
    let pt = pte.pt();
    phys::dealloc(pt as *mut u8);
    pte.set_ppn(0);
    pte.clear_flags();
}

/// Translate a va to pa given a page table.
/// Currently it is assumed that the mapping is present,
/// so no faulting/panic is implemented.
pub unsafe fn va_to_pa(root: *mut PageTable, va: VirtAddr) -> PhysAddr {
    let vpn = va.vpn();
    let offset = va.page_offset();
    let mut pt = root;
    pt = (*pt).entries[vpn[2]].pt();
    pt = (*pt).entries[vpn[1]].pt();
    let pa = ((*pt).entries[vpn[0]].ppn() << 12) | (offset as u64);
    PhysAddr::from(pa)
}

/// Map "n" page tables with n being the
/// number of pages.
pub unsafe fn map_many(
    root: *mut PageTable,
    mut va: VirtAddr,
    mut pa: PhysAddr,
    flags: PteFlags,
    num: u64,
) {
    for _ in 0..num {
        map_page(root, va, pa, flags);
        pa.set_ppn(pa.ppn() + 1);
        va = VirtAddr::from(va.0 + 0x1000);
    }
}

/// Map many pages using memory ranges instead
/// of a number of pages. It is more helpful
/// when only memory regions are known.
pub unsafe fn map_range(
    root: *mut PageTable,
    va: VirtAddr,
    pa_i: PhysAddr,
    pa_f: PhysAddr,
    flags: PteFlags,
) {
    let n = pa_f.ppn() - pa_i.ppn() + 1;
    map_many(root, va, pa_i, flags, n as u64);
}

/// Prints all entries of a given page table.
pub fn print_pt(pt: *mut PageTable) {
    uart_print!("PTEs at {:#010x}...\n", pt as usize);
    unsafe { &*pt }
        .entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.is_valid())
        .for_each(|(i, entry)| log::debug!("\t [{:03}] => PTE <{}>\n", i, entry));
}

/// Prints all page tables and their ptes recursively using dfs.
pub fn print_pts_dfs(pt: *mut PageTable, lvl: u8) {
    print_pt(pt);
    if lvl == 0 || lvl > 2 {
        return;
    }
    unsafe { &*pt }
        .entries
        .iter()
        .filter(|entry| entry.is_valid())
        .for_each(|entry| print_pts_dfs((entry.ppn() << 12) as *mut PageTable, lvl - 1));
}
