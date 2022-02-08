use bitflags::bitflags;

macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*));
    };
}

extern "C" {
    static UMEMORY_START: usize;
    static UMEMORY_END: usize;
}

const TOTAL_PAGES_CAPACITY: usize = 1 << 14; // 64M / 4K = 16K = 2^14

#[repr(C)]
pub struct Page {
    bytes: [u8; 4096]
}

bitflags! {
    #[derive(Default)]
    struct PageStatus: u8 {
        const TAKEN = 1 << 0;
    }
}

pub struct PhysicalPageAllocator {
    pages: *mut [Page; TOTAL_PAGES_CAPACITY],
    status: [PageStatus; TOTAL_PAGES_CAPACITY],
    cur: usize,
    total_allocated: usize,
}

impl PhysicalPageAllocator {

    pub fn new() -> Self {
        unimplemented!();
    }

    pub unsafe fn alloc(&mut self) -> Option<*mut Page> {
        unimplemented!();
    }

    pub unsafe fn dealloc(&mut self, page_table: *mut Page) -> Result<(), &'static str> {
        unimplemented!();
    }

}