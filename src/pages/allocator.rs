use bitflags::bitflags;

use crate::pages::PageTable;

macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*));
    };
}

extern "C" {
    static UPAGES_START: usize;
    static UPAGES_END: usize;
}

const UPAGES_CAPACITY: usize = 1 << 14; // 64M / 4K = 16K = 2^14


bitflags! {
    #[derive(Default)]
    struct PageStatus: u8 {
        const TAKEN = 1 << 0;
    }
}

pub struct PhysicalPageAllocator {
    tables: *mut [PageTable; UPAGES_CAPACITY],
    status: [PageStatus; UPAGES_CAPACITY],
    cur: usize,
    total_allocated: usize,
}

impl PhysicalPageAllocator {

    pub fn new() -> Self {
        let tables = unsafe { UPAGES_START as *mut [PageTable; UPAGES_CAPACITY] };
        Self {
            tables,
            status: [Default::default(); UPAGES_CAPACITY],
            cur: 0,
            total_allocated: 0
        }
    }

    pub unsafe fn alloc(&mut self) -> Option<*mut PageTable> {
        if self.total_allocated == UPAGES_CAPACITY {
            return None;
        }
        while self.status[self.cur].contains(PageStatus::TAKEN) {
            self.cur = (self.cur + 1) % UPAGES_CAPACITY;
        }
        self.status[self.cur].insert(PageStatus::TAKEN);
        self.total_allocated += 1;
        Some(&mut (*self.tables)[self.cur] as *mut PageTable) 
    }

    pub unsafe fn dealloc(&mut self, page_table: *mut PageTable) -> Result<(), &'static str> {
        let i = (page_table as usize - UPAGES_START) >> 12;
        if !self.status[i].contains(PageStatus::TAKEN) {
            return Err("Deallocating invalid page.");
        }
        self.status[i].remove(PageStatus::TAKEN);
        self.total_allocated -= 1;
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn print_pages(&self) {
        unsafe {
            for i in 0..UPAGES_CAPACITY {
                let addr: *mut PageTable = &mut ((*self.tables)[i]);
                uart_print!("0x{:x} \n", addr as usize);
            }
            uart_print!("Number of pages: {}\n", UPAGES_CAPACITY);
            uart_print!("End ptr page: 0x{:x}\n", UPAGES_END);
        }
    }
}