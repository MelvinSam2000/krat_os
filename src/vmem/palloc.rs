use alloc::vec::Vec;

extern "C" {
    static UMEMORY_START: usize;
    static UMEMORY_END: usize;
}

static mut ALLOC: PageAllocator = PageAllocator {
    cur: 0,
    total_allocated: 0,
    page_capacity: 0,
    allocated: Vec::new(),
};

struct PageAllocator {
    cur: usize,
    total_allocated: usize,
    page_capacity: usize,
    allocated: Vec<bool>
}

/// Initialize physical page allocator.
/// Creates an allocator that uses both static and heap memory,
/// but the pages themselves are stored in the user memory region. 
pub fn init() {
    unsafe {
        ALLOC.cur = UMEMORY_START;
        //let page_capacity = (UMEMORY_END - UMEMORY_START + 1) >> 12;
        let page_capacity = 300;
        ALLOC.page_capacity = page_capacity;
        ALLOC.allocated.resize(page_capacity, false);
    }
}

/// Allocate a page.
pub unsafe fn alloc() -> *mut u8 {

    if ALLOC.total_allocated == ALLOC.page_capacity {
        panic!("Out of page memory!");
    }

    while is_allocated(ALLOC.cur) {
        ALLOC.cur += 4096;
        if ALLOC.cur > UMEMORY_END {
            ALLOC.cur = UMEMORY_START;
        }
    }

    set_allocated(ALLOC.cur, true);
    ALLOC.cur as *mut u8
}

/// Deallocates a page.
pub unsafe fn dealloc(_page_ptr: *mut u8) {
    //unimplemented!();
}

unsafe fn is_allocated(page_ptr: usize) -> bool {
    ALLOC.allocated[(page_ptr - UMEMORY_START) >> 12]
}

unsafe fn set_allocated(page_ptr: usize, value: bool) {
    ALLOC.allocated[(page_ptr - UMEMORY_START) >> 12] = value
}