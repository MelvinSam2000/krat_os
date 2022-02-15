use valloc::*;

use riscv::register::satp::Mode;

use crate::memlayout::*;
use crate::uart_print;
use core::arch::asm;
use crate::vmem::pte::*;
use crate::vmem::addr::*;

pub fn init() {
    unsafe {

        // initialize kernel root page table
        palloc::init();
        let kern_pt = palloc::alloc() as *mut PageTable;
        if kern_pt.is_null() {
            panic!("Unable to allocate kernel root page table.");
        }

        // map kernel text and rodata
        id_map_range(kern_pt, 
            VirtAddr::from_bits(TEXT_START as u64),
            VirtAddr::from_bits(RODATA_END as u64),
            PhysAddr::from_bits(TEXT_START as u64), 
            PteFlags::RX);

        // map kernel rw data (data and bss)
        map_page(kern_pt, 
            VirtAddr::from_bits(DATA_START as u64), 
            PhysAddr::from_bits(DATA_START as u64), 
            PteFlags::RW);

        // map UART registers
        map_page(kern_pt, 
            VirtAddr::from_bits(UART_BASE_ADDR as u64), 
            PhysAddr::from_bits(UART_BASE_ADDR as u64), 
            PteFlags::RW);

        // map kernel stack
        map_page(kern_pt, 
            VirtAddr::from_bits(KSTACK_START as u64), 
            PhysAddr::from_bits(KSTACK_START as u64), 
            PteFlags::RW);

        // map kernel heap
        map_page(kern_pt, 
            VirtAddr::from_bits(KHEAP_START as u64), 
            PhysAddr::from_bits(KHEAP_START as u64),
            PteFlags::RW);

        // map for no reason
        map_page(kern_pt, 
            VirtAddr::from_bits(0x80300000), 
            PhysAddr::from_bits(0x80420000),
            PteFlags::RWX);
        
        print_pts_dfs(kern_pt, 2);

        uart_print!("{:#018x}\n", *(UMEMORY_START as *const u64));

        let va = VirtAddr::from_bits(0x80300dea);
        uart_print!("{:#010x} -> {:#010x}\n", 
            va.bits, va_to_pa(kern_pt, va).bits);

        // turn on MMU
        let in_satp: u64 = (8 << 60) | ((kern_pt as u64) >> 12);
        let addr: u64 = 0x80214008;
        let pte1 = *(addr as *const Pte);
        let pte2 = Pte::from_bits(*(addr as *const u64));
        uart_print!("RAW (u64): {:#010x}\n", *(addr as *const u64));
        uart_print!("PTE1 (Pte): {}\n", pte1);
        uart_print!("PTE2 (u64): {}\n", pte2);
        uart_print!("PTE1 (raw): {:#010x}\n", pte1.bits);
        uart_print!("PTE2 (raw): {:#010x}\n", pte2.bits);
        // uart_print!("PTE1 (raw): {:#010x}\n", u64::from(pte1));
        // uart_print!("PTE2 (raw): {:#010x}\n", Pte::to_bits(pte2));
        asm!("csrw satp, {}", in(reg) in_satp);
        asm!("sfence.vma");
        //riscv::register::satp::set(Mode::Sv39, 0, kern_pt as usize);
    }
    
}

pub mod palloc;
pub mod valloc;
pub mod pte;
pub mod addr;