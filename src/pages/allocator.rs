use crate::pages::PageTable;
use crate::uart;

extern "C" {
    static UPAGES_START: usize;
    static UPAGES_END: usize;
}

const UPAGES_CAPACITY: usize = 1 << 14; // 64M / 4K = 16K = 2^14

pub struct PhysicalPageAllocator {
    tables: *mut [PageTable; UPAGES_CAPACITY],
}

impl PhysicalPageAllocator {

    pub fn new() -> Self {
        let tables = unsafe { UPAGES_START as *mut [PageTable; UPAGES_CAPACITY] };
        Self {
            tables,
        }
    }

    pub unsafe fn alloc(&mut self) -> Option<*mut PageTable> {
        todo!();
    }

    pub unsafe fn dealloc(&mut self, page_table: *mut PageTable) {
        todo!();
    }

    #[cfg(debug_assertions)]
    pub fn print_pages(&self) {
        unsafe {
            for i in 0..UPAGES_CAPACITY {
                let addr: *mut PageTable = &mut ((*self.tables)[i]);
                uart::write_fmt(format_args!("0x{:x} \n", addr as usize));
            }
            uart::write_fmt(format_args!("Number of pages: {}\n", UPAGES_CAPACITY));
            uart::write_fmt(format_args!("End ptr page: 0x{:x}\n", UPAGES_END));
        }
    }
}