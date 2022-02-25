use riscv::register::satp::Mode;

use core::arch::asm;

use crate::memlayout::*;
use crate::vmem::pte::*;
use crate::vmem::addr::*;
use crate::vmem::virt::*;

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

        // map kernel text and rodata
        map_range(kern_pt, 
            VirtAddr::from(TEXT_START),
            VirtAddr::from(RODATA_END),
            PhysAddr::from(TEXT_START), 
            PteFlags::RX);

        // map kernel rw data (data and bss)
        map_page(kern_pt, 
            VirtAddr::from(DATA_START), 
            PhysAddr::from(DATA_START), 
            PteFlags::RW);

        // map kernel heap
        map_range(kern_pt, 
            VirtAddr::from_bits(KHEAP_START as u64),
            VirtAddr::from_bits(KHEAP_END as u64),
            PhysAddr::from_bits(KHEAP_START as u64), 
            PteFlags::RW);

        // map kernel stack
        map_range(kern_pt, 
            VirtAddr::from(KSTACK_START),
            VirtAddr::from(KSTACK_END),
            PhysAddr::from(KSTACK_START), 
            PteFlags::RW);

        // map trampoline
        map_many(kern_pt, 
            VirtAddr::from_bits(TRAMPOLINE_VADDR as u64), 
            PhysAddr::from_bits(TRAMPOLINE_START as u64), 
            PteFlags::RWX | PteFlags::G, 4);

        // map UART registers
        map_page(kern_pt, 
            VirtAddr::from(UART_BASE_ADDR), 
            PhysAddr::from(UART_BASE_ADDR), 
            PteFlags::RW);

        // map PLIC registers
        map_many(kern_pt,
            VirtAddr::from(PLIC_BASE_ADDR), 
            PhysAddr::from(PLIC_BASE_ADDR), 
            PteFlags::RW, 3);
        map_page(kern_pt, 
            VirtAddr::from(PLIC_BASE_ADDR + 0x20_1000), 
            PhysAddr::from(PLIC_BASE_ADDR + 0x20_1000), 
            PteFlags::RW);

        // turn on MMU
        riscv::register::satp::set(Mode::Sv39, 0, (kern_pt as usize) >> 12);
        
        // flush TLB
        asm!("sfence.vma");

        log::debug!("Virtual memory initialized.");
    }
}

pub mod phys;
pub mod virt;
pub mod pte;
pub mod addr;