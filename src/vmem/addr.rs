#[derive(Clone, Copy)]
#[repr(C)]
pub struct VirtAddr {
    pub bits: u64
}

impl VirtAddr {

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn vpn0(&self) -> u64 {
        (self.bits >> 12) & 0x1ff
    }

    pub fn vpn1(&self) -> u64 {
        (self.bits >> 21) & 0x1ff
    }

    pub fn vpn2(&self) -> u64 {
        (self.bits >> 30) & 0x1ff
    }

    pub fn vpn(&self) -> [usize; 3] {
        [self.vpn0() as usize, self.vpn1() as usize, self.vpn2() as usize]
    }

    pub fn page_offset(&self) -> u64 {
        self.bits & 0xfff
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysAddr {
    pub bits: u64
}

impl PhysAddr {

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn ppn(&self) -> u64 {
        (self.bits >> 12) & 0xfff_ffff_ffff
    }

    pub fn set_ppn(&mut self, ppn: u64) {
        self.bits &= !(0xfff_ffff_ffff << 12);
        self.bits |= ppn << 12;
    }
}