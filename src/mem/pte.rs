use core::fmt;
use core::str;

use bitflags::bitflags;

use crate::mem::PageTable;

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
        const GR = Self::G.bits | Self::R.bits;
        const GRW = Self::G.bits | Self::R.bits | Self::W.bits;
        const GRX = Self::G.bits | Self::R.bits | Self::X.bits;
        const GRWX = Self::G.bits | Self::R.bits | Self::W.bits | Self::X.bits;
    }
}

impl From<PteFlags> for [u8; 8] {
    fn from(other: PteFlags) -> Self {
        const FLAGS: &[u8; 8] = b"VRWXUGAD";
        let mut out = *b"--------";
        for i in 0..8 {
            if other.bits & (1 << i) != 0 {
                out[7 - i] = FLAGS[i];
            }
        }
        out
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Pte(pub u64);

impl Pte {
    pub fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub fn ppn(&self) -> u64 {
        (self.0 >> 10) & 0xfff_ffff_ffff
    }

    pub fn flags(&self) -> PteFlags {
        PteFlags::from_bits(self.0 as u8).unwrap()
    }

    pub fn set_ppn(&mut self, ppn: u64) {
        self.0 &= !(0xfff_ffff_ffff << 10);
        self.0 |= ppn << 10;
    }

    pub fn set_flags(&mut self, flags: PteFlags) {
        let flags = flags.bits() as u64;
        self.0 &= !0xff;
        self.0 |= flags;
    }

    pub fn clear_flags(&mut self) {
        self.0 &= !0xff;
    }

    pub fn pt(&self) -> *mut PageTable {
        (self.ppn() << 12) as *mut PageTable
    }

    pub fn is_valid(&self) -> bool {
        self.flags().contains(PteFlags::V)
    }
}

impl From<u64> for Pte {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl fmt::Display for Pte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPN: {:#018x}, FLAGS: {}", self.ppn(), self.flags())
    }
}

impl fmt::Display for PteFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&<[u8; 8]>::from(*self)).unwrap())
    }
}
