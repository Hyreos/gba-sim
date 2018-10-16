pub struct SP {
    val : i16
}

impl SP<> {
    pub fn pop(&mut self, size: i16) -> super::Address {
        self.val += size;
        self.val as super::Address
    }

    pub fn push(&mut self, size: i16) -> super::Address {
        self.val -= size;
        self.val as super::Address
    }

    pub fn read(&mut self) -> super::Address {
        self.val as super::Address
    }
}

pub struct PC {
    val : i16
}

impl PC<> {
    pub fn jr(&mut self, offset: i16) {
        self.val += offset;
    }

    pub fn jmp(&mut self, address: super::Address) {
        self.val = address as i16;
    }

    pub fn write(&mut self, value: i16) {
        self.val = value;
    }

    pub fn read(&mut self) -> super::Address {
        self.val as super::Address
    }
}

pub struct AF {
    lo : i8,
    hi : i8
}

impl AF<> {
    pub fn write(&mut self, value: i16) -> &mut Self {
        self.lo = (value & 0xFF) as i8;
        self.hi = (value >> 0x8) as i8;
        self
    }

    pub fn write_lo(&mut self, value: i8) -> &mut Self {
        self.lo = value;
        self
    }

    pub fn write_hi(&mut self, value: i8) -> &mut Self {
        self.hi = value;
        self
    }

    pub fn read(&mut self) -> i16 {
        (self.hi as i16).wrapping_shl(0x8) | (self.lo as i16)
    }

    pub fn read_lo(&mut self) -> i8 {
        self.lo
    }

    pub fn read_hi(&mut self) -> i8 {
        self.hi
    }
}

pub struct BC {
    lo : i8,
    hi : i8
}

impl BC<> {
    pub fn write(&mut self, value: i16) -> &mut Self {
        self.lo = (value & 0xFF) as i8;
        self.hi = (value >> 0x8) as i8;
        self
    }

    pub fn write_lo(&mut self, value: i8) -> &mut Self {
        self.lo = value;
        self
    }

    pub fn write_hi(&mut self, value: i8) -> &mut Self {
        self.hi = value;
        self
    }

    pub fn read(&mut self) -> i16 {
        (self.hi as i16).wrapping_shl(0x8) | (self.lo as i16)
    }

    pub fn read_lo(&mut self) -> i8 {
        self.lo
    }

    pub fn read_hi(&mut self) -> i8 {
        self.hi
    }
}

pub struct DE {
    lo: i8,
    hi: i8
}

impl DE<> {
    pub fn write(&mut self, value: i16) -> &mut Self {
        self.lo = (value & 0xFF) as i8;
        self.hi = (value >> 0x8) as i8;
        self
    }

    pub fn write_lo(&mut self, value: i8) -> &mut Self {
        self.lo = value;
        self
    }

    pub fn write_hi(&mut self, value: i8) -> &mut Self {
        self.hi = value;
        self
    }

    pub fn read(&mut self) -> i16 {
        (self.hi as i16).wrapping_shl(0x8) | (self.lo as i16)
    }

    pub fn read_lo(&mut self) -> i8 {
        self.lo
    }

    pub fn read_hi(&mut self) -> i8 {
        self.hi
    }
}

pub struct HL {
    lo: i8,
    hi: i8
}

impl HL<> {
    pub fn write(&mut self, value: i16) -> &mut Self {
        self.lo = (value & 0xFF) as i8;
        self.hi = (value >> 0x8) as i8;
        self
    }

    pub fn write_lo(&mut self, value: i8) -> &mut Self {
        self.lo = value;
        self
    }

    pub fn write_hi(&mut self, value: i8) -> &mut Self {
        self.hi = value;
        self
    }

    pub fn read(&mut self) -> i16 {
        (self.hi as i16).wrapping_shl(0x8) | (self.lo as i16)
    }

    pub fn read_lo(&mut self) -> i8 {
        self.lo
    }

    pub fn read_hi(&mut self) -> i8 {
        self.hi
    }
}

#[allow(dead_code)]
pub struct File {
    pub af: AF,
    pub bc: BC,
    pub de: DE,
    pub hl: HL,
    pub pc: PC,
    pub sp: SP
}
