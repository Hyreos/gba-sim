extern crate num;
extern crate std;

use cpu::num::traits::PrimInt as PrimInt;

type Address = u16;

mod registers;

pub struct Mmu;

impl Mmu<> {
    pub fn read_byte(&self, address: Address) -> i8 {
        unimplemented!();
    }

    pub fn read_word(&self, address: Address) -> i16 {
        unimplemented!();
    }

    pub fn write_byte(&self, address: Address, value: i8) {
        unimplemented!();
    }

    pub fn write_word(&self, address: Address, value: i16) {
        unimplemented!();
    }
}

pub enum CpuFlags {
    C,
    H,
    N,
    Z
}

fn check_half_carry<T>(a: T, b: T) -> bool where T: PrimInt {
    let nibble_mask = T::from(0xF).unwrap();
    let half_mask = T::from(0x10).unwrap();
    return ((a & nibble_mask) + (b & nibble_mask) & half_mask) == half_mask;
}

fn check_carry<T>(a: T, b: T) -> bool where T: PrimInt {
    let byte_mask = T::from(0xFF).unwrap();
    let carry_mask = T::from(0x100).unwrap();
    
    return (((a & byte_mask) + (b & byte_mask)) & carry_mask) == carry_mask;
}

pub struct Cpu {
    pub registers : registers::File,
    pub mmu : Mmu,
    flags : u8
}

#[allow(dead_code)]
impl Cpu<> {
    // pub fn new(mut mmu: Mmu) -> Self {
    //    let mut myself = Cpu{ mmu: mmu };
    //    return myself;
    // }

    pub fn toggle_flag(&mut self, flags: CpuFlags) {
        self.flags |= match flags {
            CpuFlags::C => 0x10,
            CpuFlags::H => 0x20,
            CpuFlags::N => 0x40,
            CpuFlags::Z => 0x80,
        };
    }

    pub fn untoggle_flag(&mut self, flags: CpuFlags) {
         self.flags &= match flags {
            CpuFlags::C => 0x10,
            CpuFlags::H => 0x20,
            CpuFlags::N => 0x40,
            CpuFlags::Z => 0x80,
        };
    }

    pub fn get_flag(&self, flags: CpuFlags) -> i8 {
        (match flags {
            CpuFlags::C => self.flags & 0x10,
            CpuFlags::H => self.flags & 0x20,
            CpuFlags::N => self.flags & 0x40,
            CpuFlags::Z => self.flags & 0x80,
        }) as i8
    }

    pub fn opexec(&mut self, op_code: u8) {
        match op_code {
            // NOP
            0x00 => {}
            // LD BC, d16
            0x01 => {
                let pc = self.registers.pc.read();
                let data = self.mmu.read_word(pc + 1);
                self.registers.bc.write(data as i16);
            }
            // LD (BC), A
            0x02 => {
                let address = self.registers.bc.read() as u16;
                self.mmu.write_byte(address, self.registers.af.read_lo());
            }
            // INC BC
            0x03 => {
                let val = self.registers.bc.read() + 1;
                self.registers.bc.write(val);
            }
            // INC B
            0x04 => {
                self.untoggle_flag(CpuFlags::N);

                if !check_half_carry(self.registers.bc.read_lo(), 1) {
                    self.toggle_flag(CpuFlags::H);
                }

                let bval = self.registers.bc.read_lo();

                self.registers.bc.write_lo(bval + 1);
                let result = self.registers.bc.read_lo();

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // DEC B
            0x05 => {
                self.toggle_flag(CpuFlags::N);

                if !check_half_carry(self.registers.bc.read_lo(), -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                let bval = self.registers.bc.read_lo();

                self.registers.bc.write_lo(bval - 1);
                let result = self.registers.bc.read_lo();

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            0x06 => {
                let pcval = self.registers.pc.read() as i16;
                self.registers.bc.write_lo(self.mmu.read_byte(pcval as u16 + 1));
            }
            0x07 => {
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);
                self.untoggle_flag(CpuFlags::Z);

                let mut value = self.registers.af.read_lo();
                for _ in 0..8 {
                    let mib : bool = (value & (1 << 7)) == (1 << 7);
                    value <<= 1;
                    value |= mib as i8;

                    match mib {
                        false => self.untoggle_flag(CpuFlags::C),
                        true => self.toggle_flag(CpuFlags::C)
                    }
                }
            }
            // LD (a16), SP
            0x08 => {
                let data = self.mmu.read_word(self.registers.pc.read() as u16 + 1) as u16;
                self.mmu.write_word(data, self.registers.sp.read() as i16);
            }
            // ADD HL, BC
            0x09 => {
                if !check_half_carry(self.registers.hl.read(), self.registers.bc.read()) {
                    self.toggle_flag(CpuFlags::H);
                }

                if !check_carry(self.registers.hl.read(), self.registers.bc.read()) {
                    self.toggle_flag(CpuFlags::C);
                }

                
                let result = self.registers.hl.read() + self.registers.bc.read();

                self.registers.hl.write(result);

                self.untoggle_flag(CpuFlags::N);
            }
            // LD A, (BC)
            0x0A => {
                let address = self.registers.bc.read() as u16;
                let value = self.mmu.read_byte(address);
                self.registers.af.write_lo(value);
            }
            // DEC BC
            0x0B => {
                let bcval = self.registers.bc.read();
                self.registers.bc.write(bcval - 1);
            }
            // INC C
            0x0C => {
                if !check_half_carry(self.registers.bc.read_hi(), 1) {
                    self.toggle_flag(CpuFlags::H);
                }
                
                self.untoggle_flag(CpuFlags::N);

                let result = self.registers.bc.read_hi() + 1;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z)
                }

                self.registers.bc.write_hi(result);
            }
            // DEC C
            0x0D => {
                if !check_half_carry(self.registers.bc.read_hi(), -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let result = self.registers.bc.read_hi() - 1;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
                
                self.registers.bc.write_hi(result);
            }
            // LD C, d8
            0x0E => {
                let data = self.mmu.read_byte(self.registers.pc.read() as u16 + 1);
                self.registers.bc.write_hi(data);
            }
            0x0F => {
                self.untoggle_flag(CpuFlags::Z);
                self.untoggle_flag(CpuFlags::N);
                self.untoggle_flag(CpuFlags::H);

                let mut value = self.registers.af.read_lo();

                for _ in 0..8 {
                    let lsb : u8 = value as u8 & 0x1;
                    value >>= 0x1;
                    value |= (lsb << 0x7) as i8;

                    if value == 0 {
                        self.untoggle_flag(CpuFlags::C);
                    } else {
                        self.toggle_flag(CpuFlags::C);
                    }
                }

                self.registers.af.write_lo(value);
            }
            0x10 => {

            }
            0x11 => {
                let data = self.mmu.read_byte(self.registers.pc.read() as u16 + 1);
                self.registers.bc.write_hi(data);
            }
            0x12 => {
                let address = self.mmu.read_byte(self.registers.pc.read()) as u16;
                self.mmu.write_byte(address, self.registers.af.read_lo());
            }
            0x13 => {
                let de = self.registers.de.read();
                self.registers.de.write(de + 1);
            }
            0x14 => {
                let d = self.registers.de.read_lo();
                
                if !check_half_carry(d, 1) {
                    self.toggle_flag(CpuFlags::H);
                }

                if d + 1 == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.de.write_lo(d + 1);
            }
            0x15 => {
                let d = self.registers.de.read_lo();

                if !check_half_carry(d, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                if d + 1 == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.de.write_lo(d - 1);
            }
            0x16 => {
                let pcval = self.registers.pc.read() as i16;
                let data = self.mmu.read_byte(pcval as u16 + 1);
                self.registers.de.write_lo(data);
            }
            // RLA
            0x17 => {

            }
            // JR r8
            0x18 => {
                let mut pcval = self.registers.pc.read() as i16;
                pcval += self.mmu.read_byte(pcval as u16 + 1) as i16;
                self.registers.pc.write(pcval);
            }
            // ADD HL, DE
            0x19 => {
                let hl = self.registers.hl.read();
                let de = self.registers.de.read();

                self.untoggle_flag(CpuFlags::N);

                if !check_carry(hl, de) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(hl, de) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.registers.hl.write(hl + de);
            }
            // LD A, (DE)
            0x1A => {
                let de = self.registers.de.read() as Address;
                let memdata = self.mmu.read_byte(de);
                self.registers.af.write_lo(memdata);
            }
            // DEC DE
            0x1B => {
                let de = self.registers.de.read();
                self.registers.de.write(de - 1);
            }
            // INC E
            0x1C => {
                let e = self.registers.de.read_hi();

                if !check_half_carry(e, 1) {
                    self.toggle_flag(CpuFlags::H);
                }

                if e + 1 == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.de.write_hi(e + 1);
            }
            // DEC E
            0x1D => {
                let e = self.registers.de.read_hi();

                if !check_half_carry(e, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                if e - 1 == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.de.write_hi(e - 1);
            }
            // LD E, d8
            0x1E => {
                let pcval = self.registers.pc.read() as Address;
                let memdata = self.mmu.read_byte(pcval + 1);
                self.registers.de.write_hi(memdata);
            }
            // RRA
            0x1F => {
                
            }
            0x20 => {

            }
            0x21 => {

            }
            0x22 => {

            }
            0x23 => {
                
            }
            0x24 => {

            }
            0x25 => {

            }
            0x26 => {

            }
            0x27 => {

            }
            0x28 => {
                
            }
            0x29 => {

            }
            0x2A => {

            }
            0x2B => {

            }
            0x2C => {
                
            }
            0x2D => {

            }
            0x2E => {

            }
            0x2F => {

            }
            0x30 => {

            }
            0x31 => {

            }
            0x32 => {

            }
            0x33 => {
                
            }
            0x34 => {

            }
            0x35 => {

            }
            0x36 => {

            }
            0x37 => {

            }
            0x38 => {
                
            }
            0x39 => {

            }
            0x3A => {

            }
            0x3B => {

            }
            0x3C => {
                
            }
            0x3D => {

            }
            0x3E => {

            }
            0x3F => {

            }
            0x40 => {

            }
            0x41 => {

            }
            0x42 => {

            }
            0x43 => {
                
            }
            0x44 => {

            }
            0x45 => {

            }
            0x46 => {

            }
            0x47 => {

            }
            0x48 => {
                
            }
            0x49 => {

            }
            0x4A => {

            }
            0x4B => {

            }
            0x4C => {
                
            }
            0x4D => {

            }
            0x4E => {

            }
            0x4F => {

            }
            0x50 => {}
            0x51 => {

            }
            0x52 => {

            }
            0x53 => {
                
            }
            0x54 => {

            }
            0x55 => {

            }
            0x56 => {

            }
            0x57 => {

            }
            0x58 => {
                
            }
            0x59 => {

            }
            0x5A => {

            }
            0x5B => {

            }
            0x5C => {
                
            }
            0x5D => {

            }
            0x5E => {

            }
            0x5F => {

            }
            0x60 => {

            }
            0x61 => {

            }
            0x62 => {

            }
            0x63 => {
                
            }
            0x64 => {

            }
            0x65 => {

            }
            0x66 => {

            }
            0x67 => {

            }
            0x68 => {
                
            }
            0x69 => {

            }
            0x6A => {

            }
            0x6B => {

            }
            0x6C => {
                
            }
            0x6D => {

            }
            0x6E => {

            }
            0x6F => {

            }
            0x70 => {}
            0x71 => {

            }
            0x72 => {

            }
            0x73 => {
                
            }
            0x74 => {

            }
            0x75 => {

            }
            0x76 => {

            }
            0x77 => {

            }
            0x78 => {
                
            }
            0x79 => {

            }
            0x7A => {

            }
            0x7B => {

            }
            0x7C => {
                
            }
            0x7D => {

            }
            0x7E => {

            }
            0x7F => {

            }
            // ADD A, B
            0x80 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();

                if !check_carry(a, b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(b + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, C
            0x81 => {
                let a = self.registers.af.read_lo();
                let c = self.registers.bc.read_hi();

                if !check_carry(a, c) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(c + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, D
            0x82 => {
                let a = self.registers.af.read_lo();
                let d = self.registers.de.read_lo();

                if !check_carry(a, d) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(d + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, E
            0x83 => {
                let a = self.registers.af.read_lo();
                let e = self.registers.de.read_hi();

                if !check_carry(a, e) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(e + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, H
            0x84 => {
                let a = self.registers.af.read_lo();
                let h = self.registers.hl.read_lo();

                if !check_carry(a, h) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(h + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, L
            0x85 => {
                let a = self.registers.af.read_lo();
                let l = self.registers.hl.read_lo();

                if !check_carry(a, l) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(l + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, (HL)
            0x86 => {
                let f = self.registers.af.read_lo();

                let address = self.registers.hl.read() as Address; 
                let s = self.mmu.read_byte(address);

                if !check_carry(f, s) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(f, s) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(f + s);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADD A, A
            0x87 => {
                let a = self.registers.af.read_lo();
                let a2 = self.registers.af.read_lo();

                if !check_carry(a, a2) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a2) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                self.registers.af.write_lo(a2 + a);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, B
            0x88 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();

                if !check_carry(a, b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;                
                self.registers.af.write_lo(a + b + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, C
            0x89 => {
                let a = self.registers.af.read_lo();
                let c = self.registers.bc.read_hi();

                if !check_carry(a, c) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let carry = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + c + carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, D
            0x8A => {
                let a = self.registers.af.read_lo();
                let d = self.registers.de.read_lo();

                if !check_carry(a, d) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + d + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, E
            0x8B => {
                let a = self.registers.af.read_lo();
                let e = self.registers.de.read_hi();

                if !check_carry(a, e) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + e + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, H
            0x8C => {
                let a = self.registers.af.read_lo();
                let h = self.registers.hl.read_lo();

                if !check_carry(a, h) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + h + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, L
            0x8D => {
                let a = self.registers.af.read_lo();
                let l = self.registers.hl.read_hi();

                if !check_carry(a, l) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + l + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, (HL)
            0x8E => {
                let f = self.registers.af.read_lo();

                let address = self.registers.hl.read() as Address;
                let s = self.mmu.read_byte(address);

                if !check_carry(f, s) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(f, s) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(f + s + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // ADC A, A
            0x8F => {
                let a = self.registers.af.read_lo();
                let a2 = self.registers.af.read_lo();

                if !check_carry(a, a2) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a2) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.untoggle_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.af.write_lo(a + a2 + c);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB B
            0x90 => {
                let b = self.registers.bc.read_lo();

                if !check_carry(b, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(b, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.bc.write_lo(b - 1);

                if self.registers.bc.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB C
            0x91 => {
                let c = self.registers.bc.read_hi();

                if !check_carry(c, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(c, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.bc.write_hi(c - 1);

                if self.registers.bc.read_hi() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB D
            0x92 => {
                let d = self.registers.de.read_lo();

                if !check_carry(d, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(d, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.de.write_lo(d - 1);

                if self.registers.de.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB E
            0x93 => {
                let e = self.registers.de.read_hi();

                if !check_carry(e, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(e, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.de.write_hi(e - 1);

                if self.registers.de.read_hi() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB H
            0x94 => {
                let h = self.registers.hl.read_lo();

                if !check_carry(h, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(h, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.hl.write_lo(h - 1);

                if self.registers.hl.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB L
            0x95 => {
                let l = self.registers.hl.read_hi();

                if !check_carry(l, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(l, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.hl.write_hi(l - 1);

                if self.registers.hl.read_hi() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB (HL)
            0x96 => {
                let address = self.registers.hl.read() as Address;
                let l = self.mmu.read_word(address);

                if !check_carry(l, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(l, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.mmu.write_word(address, l - 1);

                if l - 1 == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SUB A
            0x97 => {
                let a = self.registers.af.read_lo();

                if !check_carry(a, -1) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, -1) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                self.registers.af.write_lo(a - 1);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, B
            0x98 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();

                if !check_carry(a, b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - b + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, C
            0x99 => {
                let a = self.registers.af.read_lo();
                let c = self.registers.bc.read_hi();

                if !check_carry(a, c) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - c + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, D
            0x9A => {
                let a = self.registers.af.read_lo();
                let d = self.registers.de.read_lo();

                if !check_carry(a, d) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - d + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, E
            0x9B => {
                let a = self.registers.af.read_lo();
                let e = self.registers.de.read_hi();

                if !check_carry(a, e) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - e + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, H
            0x9C => {
                let a = self.registers.af.read_lo();
                let h = self.registers.hl.read_lo();

                if !check_carry(a, h) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - h + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, L
            0x9D => {
                let a = self.registers.af.read_lo();
                let l = self.registers.hl.read_hi();

                if !check_carry(a, l) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - l + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, (HL)
            0x9E => {
                let a = self.registers.af.read_lo();

                let hl = self.registers.hl.read();
                let data = self.mmu.read_byte(hl as Address);

                if !check_carry(a, data) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, data) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - data + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // SBC A, A
            0x9F => {
                let a = self.registers.af.read_lo();

                if !check_carry(a, a) {
                    self.toggle_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a) {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.af.write_lo(a - a + has_carry);

                if self.registers.af.read_lo() == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }
            }
            // AND B
            0xA0 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND C
            0xA1 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_hi();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND D
            0xA2 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_lo();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND E
            0xA3 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_hi();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND H
            0xA4 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_lo();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND L
            0xA5 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_hi();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND (HL)
            0xA6 => {
                let a = self.registers.af.read_lo();
                let address = self.registers.hl.read() as Address;
                let b = self.mmu.read_byte(address);

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // AND A
            0xA7 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.af.read_lo();

                self.untoggle_flag(CpuFlags::N);
                self.toggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::C);

                if a & b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a & b);
            }
            // XOR B
            0xA8 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR C
            0xA9 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR D
            0xAA => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR E
            0xAB => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR H
            0xAC => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR L
            0xAD => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR (HL)
            0xAE => {
                let a = self.registers.af.read_lo();
                let address = self.registers.hl.read();
                let b = self.mmu.read_byte(address as Address);
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ b);
            }
            // XOR A
            0xAF => {
                let a = self.registers.af.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a ^ a == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a ^ a);
            }
            // OR B
            0xB0 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR C
            0xB1 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR D
            0xB2 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR E
            0xB3 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR H
            0xB4 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR L
            0xB5 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_hi();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR (HL)
            0xB6 => {
                let a = self.registers.af.read_lo();
                let address = self.registers.hl.read() as Address;
                let b = self.mmu.read_byte(address);
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | b == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | b);
            }
            // OR A
            0xB7 => {
                let a = self.registers.af.read_lo();
                
                self.untoggle_flag(CpuFlags::C);
                self.untoggle_flag(CpuFlags::H);
                self.untoggle_flag(CpuFlags::N);

                if a | a == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                self.registers.af.write_lo(a | a);
            }
            // CP B
            0xB8 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_lo();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP C
            0xB9 => {
                let a = self.registers.af.read_lo();
                let b = self.registers.bc.read_hi();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP D
            0xBA => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_lo();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP E
            0xBB => {
                let a = self.registers.af.read_lo();
                let b = self.registers.de.read_hi();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP H
            0xBC => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_lo();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP L
            0xBD => {
                let a = self.registers.af.read_lo();
                let b = self.registers.hl.read_hi();

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP (HL)
            0xBE => {
                let a = self.registers.af.read_lo();
                let address = self.registers.hl.read() as Address;
                let b = self.mmu.read_byte(address);

                let result = a - b;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -b) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < b {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // CP A
            0xBF => {
                let a = self.registers.af.read_lo();

                let result = a - a;

                if result == 0 {
                    self.toggle_flag(CpuFlags::Z);
                }

                if !check_carry(a, -a) {
                    self.toggle_flag(CpuFlags::C);
                }

                if a < a {
                    self.toggle_flag(CpuFlags::H);
                }

                self.toggle_flag(CpuFlags::N);
            }
            // RET NZ
            0xC0 => {
            }
            0xC1 => {

            }
            0xC2 => {

            }
            0xC3 => {
                
            }
            0xC4 => {

            }
            0xC5 => {

            }
            0xC6 => {

            }
            0xC7 => {

            }
            0xC8 => {
                
            }
            0xC9 => {

            }
            0xCA => {

            }
            0xCB => {

            }
            0xCC => {
                
            }
            0xCD => {

            }
            0xCE => {

            }
            0xCF => {

            }
            0xD0 => {

            }
            0xD1 => {

            }
            0xD2 => {

            }
            0xD3 => {
                
            }
            0xD4 => {

            }
            0xD5 => {

            }
            0xD6 => {

            }
            0xD7 => {

            }
            0xD8 => {
                
            }
            0xD9 => {

            }
            0xDA => {

            }
            0xDB => {

            }
            0xDC => {
                
            }
            0xDD => {

            }
            0xDE => {

            }
            0xDF => {

            }
            0xE0 => {}
            0xE1 => {

            }
            0xE2 => {

            }
            0xE3 => {
                
            }
            0xE4 => {

            }
            0xE5 => {

            }
            0xE6 => {

            }
            0xE7 => {

            }
            0xE8 => {
                
            }
            0xE9 => {

            }
            0xEA => {

            }
            0xEB => {

            }
            0xEC => {
                
            }
            0xED => {

            }
            0xEE => {

            }
            0xEF => {

            }
            0xF0 => {}
            0xF1 => {

            }
            0xF2 => {

            }
            0xF3 => {
                
            }
            0xF4 => {

            }
            0xF5 => {

            }
            0xF6 => {

            }
            0xF7 => {

            }
            0xF8 => {
                
            }
            0xF9 => {

            }
            0xFA => {

            }
            0xFB => {

            }
            0xFC => {
                
            }
            0xFD => {

            }
            0xFE => {

            }
            0xFF => {

            }
            _ => {
                panic!("Invalid instruction");
            }
        }
    }
}
