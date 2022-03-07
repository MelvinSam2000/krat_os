use alloc::string::String;
use alloc::format;

use crate::memlayout::UMEMORY_START;
use crate::memlayout::UMEMORY_END;

#[repr(C, align(4096))]
struct FreeListNode {
    next: Option<*mut FreeListNode>,
    _padding: [u8; 4088],
}

struct PageAllocator {
    total_allocated: usize,
    page_capacity: usize,
    free_list_head: Option<*mut FreeListNode>,
}

static mut ALLOC: PageAllocator = PageAllocator {
    total_allocated: 0,
    page_capacity: 0,
    free_list_head: None,
};

/// Initialize physical page allocator.
pub fn init() {
    unsafe {
        // Create free list
        let umem_start = UMEMORY_START >> 12;
        let umem_end = (UMEMORY_END >> 12) - 1;
        let mut tmp = None;
        for i in (umem_start..umem_end).rev() {
            let page_ptr = (i << 12) as *mut FreeListNode;
            (*page_ptr).next = tmp;
            tmp = Some(page_ptr);
            ALLOC.page_capacity += 1;
        }
        ALLOC.free_list_head = tmp;
        // print_free_list();
    }
}

/// Allocate a page.
pub unsafe fn alloc() -> *mut u8 {
    if let Some(head) = ALLOC.free_list_head {
        ALLOC.free_list_head = (*head).next;
        ALLOC.total_allocated += 1;
        // clear page
        clear_page(((head as usize) >> 12) << 12);
        return head as *mut u8;

    } else {
        panic!("Cannot allocated more pages.");
    }
}

/// Deallocate a page.
pub unsafe fn dealloc(page_ptr: *mut u8) {
    let page_ptr = page_ptr as *mut FreeListNode;
    if let Some(head) = ALLOC.free_list_head {
        (*head).next = Some(head);
    }
    ALLOC.total_allocated -= 1;
    ALLOC.free_list_head = Some(page_ptr);
}

// Fill page with zero
unsafe fn clear_page(ptr: usize) {
    for i in 0..512 {
        *((ptr + 8*i) as *mut u64) = 0;
    }
}

#[cfg(debug_assertions)]
pub unsafe fn print_free_list() {
    let mut tmp = ALLOC.free_list_head;
    let mut out = String::from("");
    let iter = 30;
    let mut i = 0;
    while tmp.is_some() && i < iter {
        out += &format!("{:#010x} -> ", tmp.unwrap() as usize);
        tmp = (*tmp.unwrap()).next;
        i += 1;
    }
    out += "...";
    log::debug!("Free List: {:?}", out);
}
