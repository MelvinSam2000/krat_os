use linked_list_allocator::LockedHeap;

use alloc::alloc::Layout;
use alloc::alloc::GlobalAlloc;

extern "C" {
    static UMEMORY_START: usize;
    static UMEMORY_END: usize;
}

static PHYS_PAGE_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    unsafe {
        PHYS_PAGE_ALLOCATOR.lock().init(UMEMORY_START, UMEMORY_END - UMEMORY_START);
    }
}

pub unsafe fn alloc() -> *mut u8 {
    PHYS_PAGE_ALLOCATOR.alloc(Layout::from_size_align_unchecked(4096, 4096))
}

pub unsafe fn dealloc(page: *mut u8) {
    PHYS_PAGE_ALLOCATOR.dealloc(page, Layout::from_size_align_unchecked(4096, 4096));
}