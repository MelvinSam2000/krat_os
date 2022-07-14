#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub fn vpn0(&self) -> u64 {
        (self.0 >> 12) & 0x1ff
    }

    pub fn vpn1(&self) -> u64 {
        (self.0 >> 21) & 0x1ff
    }

    pub fn vpn2(&self) -> u64 {
        (self.0 >> 30) & 0x1ff
    }

    pub fn vpn(&self) -> [usize; 3] {
        [
            self.vpn0() as usize,
            self.vpn1() as usize,
            self.vpn2() as usize,
        ]
    }

    pub fn page_offset(&self) -> u64 {
        self.0 & 0xfff
    }
}

impl From<u64> for VirtAddr {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl From<usize> for VirtAddr {
    fn from(bits: usize) -> Self {
        Self(bits as u64)
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    pub fn ppn(&self) -> u64 {
        (self.0 >> 12) & 0xfff_ffff_ffff
    }

    pub fn set_ppn(&mut self, ppn: u64) {
        self.0 &= !(0xfff_ffff_ffff << 12);
        self.0 |= ppn << 12;
    }
}

impl From<u64> for PhysAddr {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl From<usize> for PhysAddr {
    fn from(bits: usize) -> Self {
        Self(bits as u64)
    }
}