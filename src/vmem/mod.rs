use valloc::*;

use crate::memlayout::*;

macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*));
    };
}

pub fn init() {
    unsafe {

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
            VirtAddr::from_bytes(DATA_START.to_be_bytes()), 
            PhysAddr::from_bytes(DATA_START.to_be_bytes()), 
            PteFlags::RW);

        // map UART registers
        map_page(kern_pt, 
            VirtAddr::from_bytes(UART_BASE_ADDR.to_be_bytes()), 
            PhysAddr::from_bytes(UART_BASE_ADDR.to_be_bytes()), 
            PteFlags::RW);

        // map kernel stack
        map_page(kern_pt, 
            VirtAddr::from_bytes(UART_BASE_ADDR.to_be_bytes()), 
            PhysAddr::from_bytes(UART_BASE_ADDR.to_be_bytes()), 
            PteFlags::RW);
            
        
        print_pts_dfs(kern_pt);
    }
    
}

pub mod palloc;
pub mod valloc;