use linked_list_allocator::LockedHeap;

use crate::memlayout::*;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// This function is called to initialize kernel heap.
pub fn init() {
    unsafe {
        ALLOCATOR.lock().init(KHEAP_START, KHEAP_END - KHEAP_START);
    }
}

