use crate::memlayout::*;
use crate::mem::addr::*;
use crate::mem::pte::*;
use crate::mem::virt::*;
use crate::riscv::mmu_set;

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

        // map kernel text
        map_range(
            kern_pt,
            VirtAddr::from(TEXT_START),
            PhysAddr::from(TEXT_START),
            PhysAddr::from(TEXT_END),
            PteFlags::RX,
        );

        // map kernel rodata
        map_range(
            kern_pt,
            VirtAddr::from(RODATA_START),
            PhysAddr::from(RODATA_START),
            PhysAddr::from(RODATA_END),
            PteFlags::R,
        );

        // map kernel rw data (data and bss)
        map_range(
            kern_pt,
            VirtAddr::from(DATA_START),
            PhysAddr::from(DATA_START),
            PhysAddr::from(BSS_END),
            PteFlags::RW,
        );

        // map trampoline text
        map_range(
            kern_pt,
            VirtAddr::from(TRAMP_VADDR),
            PhysAddr::from(TRAMP_VECTOR),
            PhysAddr::from(TRAMP_FRAME - 1),
            PteFlags::RX | PteFlags::G,
        );

        // map trampoline data
        map_page(
            kern_pt,
            VirtAddr::from(TRAMP_VADDR + (TRAMP_FRAME - TRAMP_VECTOR)),
            PhysAddr::from(TRAMP_FRAME),
            PteFlags::RW | PteFlags::G,
        );

        // map kernel heap
        map_range(
            kern_pt,
            VirtAddr::from(KHEAP_START),
            PhysAddr::from(KHEAP_START),
            PhysAddr::from(KHEAP_END),
            PteFlags::RW,
        );

        // map kernel stack
        map_range(
            kern_pt,
            VirtAddr::from(KSTACK_START),
            PhysAddr::from(KSTACK_START),
            PhysAddr::from(KSTACK_END),
            PteFlags::RW,
        );

        // map UART registers
        map_page(
            kern_pt,
            VirtAddr::from(UART_BASE_ADDR),
            PhysAddr::from(UART_BASE_ADDR),
            PteFlags::RW,
        );

        // map PLIC registers
        map_many(
            kern_pt,
            VirtAddr::from(PLIC_BASE_ADDR),
            PhysAddr::from(PLIC_BASE_ADDR),
            PteFlags::RW,
            3,
        );
        map_page(
            kern_pt,
            VirtAddr::from(PLIC_BASE_ADDR + 0x20_1000),
            PhysAddr::from(PLIC_BASE_ADDR + 0x20_1000),
            PteFlags::RW,
        );

        print_pts_dfs(kern_pt, 2);

        // turn on MMU
        mmu_set(kern_pt as usize);
        log::debug!("Virtual memory initialized.");
    }
}

pub mod addr;
pub mod phys;
pub mod pte;
pub mod virt;
