#![allow(dead_code, unused_variables)]

use bitflags::bitflags;
use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use alloc::string::String;

macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*));
    };
}

use crate::vmem::palloc;

bitflags! {
    pub struct PteFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
        const RW = Self::R.bits | Self::W.bits;
        const RX = Self::R.bits | Self::X.bits;
        const RWX = Self::R.bits | Self::W.bits | Self::X.bits;
    }
}

pub fn print_flags(flags: u8) -> String {
    const FLAGS: [char; 8] = ['V','R','W','X','U','G','A','D'];
    let mut out = ['-'; 8];
    for i in 0..8 {
        if flags & (1 << i) != 0 {
            out[7 - i] = FLAGS[i];
        }
    }
    String::from_iter(out)
}


#[bitfield(bits=64)]
#[derive(Clone, Copy)]
pub struct Pte {
    padding: B10,
    ppn: B44,
    rsw: B2,
    flags: B8,
}

impl Pte {

    pub fn from_u64(x: u64) -> Pte {
        Pte::new()
            .with_padding(0)
            .with_ppn((x >> 10) & 0xfff_ffff_ffff)
            .with_rsw(0)
            .with_flags(x as u8)
    }

    pub fn to_u64(pte: Pte) -> u64 {
        (pte.ppn() as u64) << 10 | (pte.flags() as u64)
    }
}

#[bitfield(bits=64)]
#[derive(Clone, Copy)]
pub struct VirtAddr {
    padding: B25,
    vpn2: B9,
    vpn1: B9,
    vpn0: B9,
    page_offset: B12,
}

impl VirtAddr {

    pub fn from_u64(x: u64) -> VirtAddr {
        VirtAddr::new()
            .with_padding(0)
            .with_vpn2(((x >> 30) as u16) & 0x1ff)
            .with_vpn1(((x >> 21) as u16) & 0x1ff)
            .with_vpn0(((x >> 12) as u16) & 0x1ff)
            .with_page_offset((x as u16) & 0xfff)
    }
}

#[bitfield(bits=64)]
#[derive(Clone, Copy)]
pub struct PhysAddr {
    padding: B8,
    ppn: B44,
    page_offset: B12,
}

impl PhysAddr {

    pub fn from_u64(x: u64) -> PhysAddr {
        PhysAddr::new()
            .with_padding(0)
            .with_ppn((x >> 12) & 0xfff_ffff_ffff)
            .with_page_offset((x as u16) & 0xfff)
    }
}

#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    entries: [Pte; 512],
}

pub unsafe fn map_page(root: *mut PageTable, va: VirtAddr, pa: PhysAddr, flags: PteFlags) {
    
    let vpn = [ va.vpn0(), va.vpn1(), va.vpn2() ];
    uart_print!("{:#05x},{:#05x},{:#05x}\n", vpn[2], vpn[1], vpn[0]);
    let mut pte;
    let mut pt = root;

    for lvl in (1..=2).rev() {
        pte = (*pt).entries[vpn[lvl] as usize];
        if (pte.flags() & PteFlags::V.bits) == 0 {
            let page = palloc::alloc() as *mut PageTable;
            if page.is_null() {
                panic!("Page Fault: No more pages can be allocated.");
            }
            pte = pte_set_page(page, PteFlags::V);
            (*pt).entries[vpn[lvl] as usize] = pte;
            uart_print!("{:#010x} + {:#05x} <= {:#010x} {:?}\n", 
                pt as usize, vpn[lvl] as usize, pte.ppn(), print_flags(pte.flags()));
        }
        pt = pte.ppn() as *mut PageTable;
    }
    pte = pte_set_page(pa.into_bytes()[0] as *mut PageTable, flags | PteFlags::V);
    
    (*pt).entries[vpn[0] as usize] = pte;
    uart_print!("{:#010x} + {:#05x} <= {:#010x} {:?}\n", 
        pt as usize, vpn[0] as usize, pte.ppn(), print_flags(pte.flags()));
}

fn pte_set_page(page: *mut PageTable, flags: PteFlags) -> Pte {
    let mut pte = Pte::new();
    pte.set_ppn(page as u64);
    pte.set_flags(flags.bits);
    pte
}


#[cfg(debug_assertions)]
pub unsafe fn print_pt(pt: *mut PageTable) {
    uart_print!("PT at {:#010x}...\n", pt as usize);
    for (i, entry) in (*pt).entries.iter().enumerate() {
        if entry.flags() & PteFlags::V.bits != 0 {
            let pa = entry.ppn();
            let flags = print_flags(entry.flags());
            uart_print!("\t{:#05x} => {:#010x} {:?}\n", i, pa, flags);
        }
    }
}

#[cfg(debug_assertions)]
pub unsafe fn print_pts_dfs(pt: *mut PageTable) {
    print_pt(pt);
    for entry in (*pt).entries {
        if entry.flags() & PteFlags::V.bits != 0 {
            uart_print!("HEYYOOO\n");
            print_pts_dfs(entry.ppn() as *mut PageTable);
        }
    }
}