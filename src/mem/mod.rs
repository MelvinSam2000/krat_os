use crate::mem::pte::*;
use crate::mem::virt::*;
use crate::memlayout::*;
use crate::riscv::mmu_init;

/// Initialize virtual memory.
/// 1. Initialize physical page allocator.
/// 2. Create kernel root page table.
/// 3. Create page mmapings for kernel memory regions and mmio devices.
/// 4. Store kernel page table in satp and turn on MMU
pub fn init() {
    unsafe {
        // initialize kernel root page table
        phys::init();
        let kern_pt = phys::alloc() as *mut PageTable;
        if kern_pt.is_null() {
            panic!("Unable to allocate kernel root page table.");
        }

        // Number of 4KB pages required per section
        let text_pages = ((TEXT_END - TEXT_START) >> 12) + 1;
        let rodata_pages = ((RODATA_END - RODATA_START) >> 12) + 1;
        let data_bss_pages = ((BSS_END - DATA_START) >> 12) + 1;
        let kheap_pages = ((KHEAP_END - KHEAP_START) >> 12) + 1;
        let kstack_pages = ((KSTACK_END - KSTACK_START) >> 12) + 1;

        // Virtual memory address offsets
        let kern_vaddr_base = !0usize - (KSTACK_END - TEXT_START);
        let text_vaddr_base = kern_vaddr_base;
        let rodata_vaddr_base = text_vaddr_base + (text_pages << 12);
        let data_bss_vaddr_base = rodata_vaddr_base + (rodata_pages << 12);
        let kheap_vaddr_base = data_bss_vaddr_base + (data_bss_pages << 12);
        let kstack_vaddr_base = kheap_vaddr_base + (kheap_pages << 12);

        // map kernel text
        map_many(
            kern_pt,
            text_vaddr_base.into(),
            TEXT_START.into(),
            PteFlags::GRX,
            text_pages,
        );

        // map kernel rodata
        map_many(
            kern_pt,
            rodata_vaddr_base.into(),
            RODATA_START.into(),
            PteFlags::GR,
            rodata_pages,
        );

        // map kernel rw data (data and bss)
        map_many(
            kern_pt,
            data_bss_vaddr_base.into(),
            DATA_START.into(),
            PteFlags::GRW,
            data_bss_pages,
        );

        // map kernel heap
        map_many(
            kern_pt,
            kheap_vaddr_base.into(),
            KHEAP_START.into(),
            PteFlags::GRW,
            kheap_pages,
        );

        // map kernel stack
        map_many(
            kern_pt,
            kstack_vaddr_base.into(),
            KSTACK_START.into(),
            PteFlags::GRW,
            kstack_pages,
        );

        // map UART registers
        map_page(
            kern_pt,
            UART_BASE_ADDR.into(),
            UART_BASE_ADDR.into(),
            PteFlags::RW,
        );

        // map PLIC registers
        map_many(
            kern_pt,
            PLIC_BASE_ADDR.into(),
            PLIC_BASE_ADDR.into(),
            PteFlags::RW,
            3,
        );
        map_page(
            kern_pt,
            (PLIC_BASE_ADDR + 0x20_1000).into(),
            (PLIC_BASE_ADDR + 0x20_1000).into(),
            PteFlags::RW,
        );

        print_pts_dfs(kern_pt, 2);

        // turn on MMU
        mmu_init(kern_pt as usize);
        log::debug!("Virtual memory initialized.");
    }
}

pub mod addr;
pub mod phys;
pub mod pte;
pub mod virt;
