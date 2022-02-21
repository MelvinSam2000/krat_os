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
            VirtAddr::from_bits(TEXT_START as u64),
            VirtAddr::from_bits(RODATA_END as u64),
            PhysAddr::from_bits(TEXT_START as u64), 
            PteFlags::RX);

        // map kernel rw data (data and bss)
        map_page(kern_pt, 
            VirtAddr::from_bits(DATA_START as u64), 
            PhysAddr::from_bits(DATA_START as u64), 
            PteFlags::RW);

        // map kernel stack
        map_range(kern_pt, 
            VirtAddr::from_bits(KSTACK_START as u64),
            VirtAddr::from_bits(KSTACK_END as u64),
            PhysAddr::from_bits(KSTACK_START as u64), 
            PteFlags::RW);

        // map kernel heap
        map_range(kern_pt, 
            VirtAddr::from_bits(KHEAP_START as u64),
            VirtAddr::from_bits(KHEAP_END as u64),
            PhysAddr::from_bits(KHEAP_START as u64), 
            PteFlags::RW);

        // map UART registers
        map_page(kern_pt, 
            VirtAddr::from_bits(UART_BASE_ADDR as u64), 
            PhysAddr::from_bits(UART_BASE_ADDR as u64), 
            PteFlags::RW);

        // map PLIC registers
        map_many(kern_pt,
            VirtAddr::from_bits(PLIC_BASE_ADDR as u64), 
            PhysAddr::from_bits(PLIC_BASE_ADDR as u64), 
            PteFlags::RW, 3);
        map_page(kern_pt, 
            VirtAddr::from_bits((PLIC_BASE_ADDR + 0x20_1000) as u64), 
            PhysAddr::from_bits((PLIC_BASE_ADDR + 0x20_1000) as u64), 
            PteFlags::RW);

        // turn on MMU
        riscv::register::satp::set(Mode::Sv39, 0, (kern_pt as usize) >> 12);
        
        // flush TLBs
        asm!("sfence.vma");

        log::debug!("Virtual memory initialized.");
    }
}

pub mod phys;
pub mod virt;
pub mod pte;
pub mod addr;