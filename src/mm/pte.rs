use bitflags::bitflags;
use alloc::string::String;

use core::fmt;

use crate::mm::PageTable;

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

impl PteFlags {

    pub fn to_string(&self) -> String {
        const FLAGS: [char; 8] = ['V','R','W','X','U','G','A','D'];
        let mut out = ['-'; 8];
        for i in 0..8 {
            if self.bits & (1 << i) != 0 {
                out[7 - i] = FLAGS[i];
            }
        }
        String::from_iter(out)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pte {
    pub bits: u64
}

impl Pte {

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn ppn(&self) -> u64 {
        (self.bits >> 10) & 0xfff_ffff_ffff
    }

    pub fn flags(&self) -> PteFlags {
        PteFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn set_ppn(&mut self, ppn: u64) {
        self.bits &= !(0xfff_ffff_ffff << 10);
        self.bits |= ppn << 10;
    }

    pub fn set_flags(&mut self, flags: PteFlags) {
        let flags = flags.bits() as u64;
        self.bits &= !0xff;
        self.bits |= flags;
    }

    pub fn clear_flags(&mut self) {
        self.bits &= !0xff;
    }

    pub fn pt(&self) -> *mut PageTable {
        (self.ppn() << 12) as *mut PageTable
    }

    pub fn is_valid(&self) -> bool {
        self.flags().contains(PteFlags::V)
    }
}

impl fmt::Display for Pte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPN: {:#010x}, FLAGS: {:?}", self.ppn(), self.flags().to_string())
    }
}

