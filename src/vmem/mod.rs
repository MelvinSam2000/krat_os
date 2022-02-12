use valloc::*;

use crate::memlayout::*;

pub fn init() {
    unsafe {
        // initialize kernel root page table
        palloc::init();
        let kern_pt = palloc::alloc() as *mut PageTable;
        if kern_pt.is_null() {
            panic!("Unable to allocate kernel root page table.");
        }

        // map kernel text and rodata
        map_page(kern_pt, 
            VirtAddr::from_u64(TEXT_START as u64), 
            PhysAddr::from_u64(TEXT_START as u64), 
            PteFlags::RX);

        // map kernel rw data (data and bss)
        map_page(kern_pt, 
            VirtAddr::from_u64(DATA_START as u64), 
            PhysAddr::from_u64(DATA_START as u64), 
            PteFlags::RW);

        // map UART registers
        map_page(kern_pt, 
            VirtAddr::from_u64(UART_BASE_ADDR as u64), 
            PhysAddr::from_u64(UART_BASE_ADDR as u64), 
            PteFlags::RW);

        // map kernel stack
        map_page(kern_pt, 
            VirtAddr::from_u64(KSTACK_START as u64), 
            PhysAddr::from_u64(KSTACK_START as u64), 
            PteFlags::RW);

        // map kernel heap
        map_page(kern_pt, 
            VirtAddr::from_u64(KHEAP_START as u64), 
            PhysAddr::from_u64(KHEAP_START as u64), 
            PteFlags::RW);
        
        print_pts_dfs(kern_pt, 2);
    }
    
}

pub mod palloc;
pub mod valloc;