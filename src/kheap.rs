use linked_list_allocator::LockedHeap;
use lazy_static::lazy_static;

extern "C" {
    static KHEAP_START: usize;
    static KHEAP_END: usize;
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    unsafe {
        ALLOCATOR.lock().init(KHEAP_START, KHEAP_END - KHEAP_START);
    }
}
