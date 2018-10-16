extern crate num;
extern crate std;

use cpu::num::traits::PrimInt as PrimInt;

#[allow(dead_code)]
pub struct RegFile {
    a: i8,
    f: i8,
    b: i8,
    c: i8,
    d: i8,
    e: i8,
    h: i8,
    l: i8,
    pc: u16,
    sp : u16
}

#[allow(dead_code)]
impl RegFile<> {
    pub fn read_a(&self) -> i8 { return self.a; }

    pub fn read_f(&self) -> i8 { return self.f; }

    pub fn read_b(&self) -> i8 { return self.b; }

    pub fn read_c(&self) -> i8 { return self.c; }

    pub fn read_d(&self) -> i8 { return self.d; }
    
    pub fn read_e(&self) -> i8 { return self.e; }

    pub fn read_h(&self) -> i8 { return self.h; }

    pub fn read_l(&self) -> i8 { return self.l; }

    pub fn read_af(&self) -> i16 {
        return self.a as i16 | (self.f as i16).wrapping_shl(0xFF);
    }

    pub fn read_bc(&self) -> i16 {
        return self.b as i16 | (self.c as i16).wrapping_shl(0xFF);
    }

    pub fn read_de(&self) -> i16 {
        return self.d as i16 | (self.e as i16).wrapping_shl(0xFF);
    }

    pub fn read_hl(&self) -> i16 {
        return self.h as i16 | (self.l as i16).wrapping_shl(0xFF);
    }

    pub fn read_pc(&self) -> u16 {
        return self.pc;
    }

     pub fn read_sp(&self) -> u16 {
        return self.sp;
    }

    pub fn write_a(&mut self, value: i8) {
        self.a = value;
    } 

    pub fn write_f(&mut self, value: i8) {
        self.f = value;
    }

    pub fn write_b(&mut self, value: i8) {
        self.b = value;
    } 

    pub fn write_c(&mut self, value: i8) {
        self.c = value;
    } 

    pub fn write_d(&mut self, value: i8) {
        self.d = value;
    }

    pub fn write_e(&mut self, value: i8) {
        self.e = value;
    }

    pub fn write_h(&mut self, value: i8) {
        self.h = value;
    }

    pub fn write_l(&mut self, value: i8) {
        self.l = value;
    }

    pub fn write_af(&mut self, value: i16) {
        self.a = value as i8;
        self.f = (value >> 0x8) as i8;
    }

    pub fn write_bc(&mut self, value: i16) {
        self.b = value as i8;
        self.c = (value >> 0x8) as i8;
    }

    pub fn write_de(&mut self, value: i16) {
        self.d = value as i8;
        self.c = (value >> 0x8) as i8;
    }

    pub fn write_hl(&mut self, value: i16) {
        self.h = value as i8;
        self.l = (value >> 0x8) as i8;
    }

    pub fn write_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn write_sp(&mut self, value: u16) {
        self.sp = value;
    }
}

type Address = i16;

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
    return ((a & num::cast(0xF).unwrap()) + (b & num::cast(0xF).unwrap()) & num::cast(0x10).unwrap()) == num::cast(0x10).unwrap();
}

fn check_carry<T>(a: T, b: T) -> bool where T: PrimInt {
    return ((a & num::cast(0xFF).unwrap()) + (b & num::cast(0xFF).unwrap()) & num::cast(0x100).unwrap()) == num::cast(0x100).unwrap();
}

pub struct Cpu {
    pub registers : RegFile,
    pub mmu : Mmu,
    flags : u8
}

#[allow(dead_code)]
impl Cpu<> {
    // pub fn new(mut mmu: Mmu) -> Self {
    //    let mut myself = Cpu{ mmu: mmu };
    //    return myself;
    // }

    pub fn set_flag(&mut self, flags: CpuFlags) {
        self.flags |= match flags {
            CpuFlags::C => 0x10,
            CpuFlags::H => 0x20,
            CpuFlags::N => 0x40,
            CpuFlags::Z => 0x80,
        };
    }

    pub fn clear_flag(&mut self, flags: CpuFlags) {
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
                let pc = self.registers.read_pc();
                let data = self.mmu.read_word(pc as i16 + 1);
                self.registers.write_bc(data);
            }
            // LD (BC), A
            0x02 => {
                let address = self.registers.read_bc();
                self.mmu.write_byte(address, self.registers.read_a());
            }
            // INC BC
            0x03 => {
                let val = self.registers.read_bc() + 1;
                self.registers.write_bc(val);
            }
            // INC B
            0x04 => {
                self.clear_flag(CpuFlags::N);

                if !check_half_carry(self.registers.read_b(), 1) {
                    self.set_flag(CpuFlags::H);
                }

                let bval = self.registers.read_b();

                self.registers.write_b(bval + 1);
                let result = self.registers.read_b();

                if result == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // DEC B
            0x05 => {
                self.set_flag(CpuFlags::N);

                if !check_half_carry(self.registers.read_b(), -1) {
                    self.set_flag(CpuFlags::H);
                }

                let bval = self.registers.read_b();

                self.registers.write_b(bval - 1);
                let result = self.registers.read_b();

                if result == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            0x06 => {
                let pcval = self.registers.read_pc() as i16;
                self.registers.write_b(self.mmu.read_byte(pcval + 1));
            }
            0x07 => {
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);
                self.clear_flag(CpuFlags::Z);

                let mut value = self.registers.read_a();
                for _ in 0..8 {
                    let mib : bool = (value & (1 << 7)) == (1 << 7);
                    value <<= 1;
                    value |= mib as i8;

                    match mib {
                        false => self.clear_flag(CpuFlags::C),
                        true => self.set_flag(CpuFlags::C)
                    }
                }
            }
            // LD (a16), SP
            0x08 => {
                let data = self.mmu.read_word((self.registers.read_pc() + 1) as i16);
                self.mmu.write_word(data, self.registers.read_sp() as i16);
            }
            // ADD HL, BC
            0x09 => {
                if !check_half_carry(self.registers.read_hl(), self.registers.read_bc()) {
                    self.set_flag(CpuFlags::H);
                }

                if !check_carry(self.registers.read_hl(), self.registers.read_bc()) {
                    self.set_flag(CpuFlags::C);
                }

                
                let result = self.registers.read_hl() + self.registers.read_bc();

                self.registers.write_hl(result);

                self.clear_flag(CpuFlags::N);
            }
            // LD A, (BC)
            0x0A => {
                let address = self.registers.read_bc();
                let value = self.mmu.read_byte(address);
                self.registers.write_a(value);
            }
            // DEC BC
            0x0B => {
                let bcval = self.registers.read_bc();
                self.registers.write_bc(bcval - 1);
            }
            // INC C
            0x0C => {
                if !check_half_carry(self.registers.read_c(), 1) {
                    self.set_flag(CpuFlags::H);
                }
                
                self.clear_flag(CpuFlags::N);

                let result = self.registers.read_c() + 1;

                if result == 0 {
                    self.set_flag(CpuFlags::Z)
                }

                self.registers.write_c(result);
            }
            // DEC C
            0x0D => {
                if !check_half_carry(self.registers.read_c(), -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let result = self.registers.read_c() - 1;

                if result == 0 {
                    self.set_flag(CpuFlags::Z);
                }
                
                self.registers.write_c(result);
            }
            // LD C, d8
            0x0E => {
                let data = self.mmu.read_byte((self.registers.read_pc() + 1) as i16);
                self.registers.write_c(data);
            }

            0x0F => {
                self.clear_flag(CpuFlags::Z);
                self.clear_flag(CpuFlags::N);
                self.clear_flag(CpuFlags::H);

                let mut value = self.registers.read_a();

                for _ in 0..8 {
                    let lsb : u8 = value as u8 & 0x1;
                    value >>= 0x1;
                    value |= (lsb << 0x7) as i8;

                    if value == 0 {
                        self.clear_flag(CpuFlags::C);
                    } else {
                        self.set_flag(CpuFlags::C);
                    }
                }

                self.registers.write_a(value);
            }
            0x10 => {

            }
            0x11 => {
                let data = self.mmu.read_byte((self.registers.read_pc() + 1) as i16);
                self.registers.write_c(data);
            }
            0x12 => {
                let address = self.mmu.read_byte((self.registers.read_pc()) as i16) as i16;
                self.mmu.write_byte(address, self.registers.read_a());
            }
            0x13 => {
                let de = self.registers.read_de();
                self.registers.write_de(de + 1);
            }
            0x14 => {
                let d = self.registers.read_d();
                
                if !check_half_carry(d, 1) {
                    self.set_flag(CpuFlags::H);
                }

                if d + 1 == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_d(d + 1);
            }
            0x15 => {
                let d = self.registers.read_d();

                if !check_half_carry(d, -1) {
                    self.set_flag(CpuFlags::H);
                }

                if (d + 1 == 0) {
                    self.set_flag(CpuFlags::Z);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_d(d - 1);
            }
            0x16 => {
                let pcval = self.registers.read_pc() as i16;
                let data = self.mmu.read_byte(pcval + 1);
                self.registers.write_d(data);
            }
            // RLA
            0x17 => {

            }
            // JR r8
            0x18 => {
                let mut pcval = self.registers.read_pc() as i16;
                pcval += self.mmu.read_byte(pcval + 1) as i16;
                self.registers.write_pc(pcval as u16);
            }
            // ADD HL, DE
            0x19 => {
                let hl = self.registers.read_hl();
                let de = self.registers.read_de();

                self.clear_flag(CpuFlags::N);

                if !check_carry(hl, de) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(hl, de) {
                    self.set_flag(CpuFlags::H);
                }

                self.registers.write_hl(hl + de);
            }
            // LD A, (DE)
            0x1A => {
                let de = self.registers.read_de();
                let memdata = self.mmu.read_byte(de);
                self.registers.write_a(memdata);
            }
            // DEC DE
            0x1B => {
                let de = self.registers.read_de();
                self.registers.write_de(de - 1);
            }
            // INC E
            0x1C => {
                let e = self.registers.read_e();

                if !check_half_carry(e, 1) {
                    self.set_flag(CpuFlags::H);
                }

                if e + 1 == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_e(e + 1);
            }
            // DEC E
            0x1D => {
                let e = self.registers.read_e();

                if !check_half_carry(e, -1) {
                    self.set_flag(CpuFlags::H);
                }

                if e - 1 == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_e(e - 1);
            }
            // LD E, d8
            0x1E => {
                let pcval = self.registers.read_pc() as i16;
                let memdata = self.mmu.read_byte(pcval + 1);
                self.registers.write_e(memdata);
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
                let a = self.registers.read_a();
                let b = self.registers.read_b();

                if !check_carry(a, b) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(b + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, C
            0x81 => {
                let a = self.registers.read_a();
                let c = self.registers.read_c();

                if !check_carry(a, c) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(c + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, D
            0x82 => {
                let a = self.registers.read_a();
                let d = self.registers.read_d();

                if !check_carry(a, d) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(d + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, E
            0x83 => {
                let a = self.registers.read_a();
                let e = self.registers.read_e();

                if !check_carry(a, e) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(e + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, H
            0x84 => {
                let a = self.registers.read_a();
                let h = self.registers.read_h();

                if !check_carry(a, h) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(h + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, L
            0x85 => {
                let a = self.registers.read_a();
                let l = self.registers.read_l();

                if !check_carry(a, l) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(l + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, (HL)
            0x86 => {
                let f = self.registers.read_a();

                let address = self.registers.read_hl();
                let s = self.mmu.read_byte(address);

                if !check_carry(f, s) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(f, s) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(f + s);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADD A, A
            0x87 => {
                let a = self.registers.read_a();
                let a2 = self.registers.read_a();

                if !check_carry(a, a2) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a2) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                self.registers.write_a(a2 + a);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, B
            0x88 => {
                let a = self.registers.read_a();
                let b = self.registers.read_b();

                if !check_carry(a, b) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;                
                self.registers.write_a(a + b + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, C
            0x89 => {
                let a = self.registers.read_a();
                let c = self.registers.read_c();

                if !check_carry(a, c) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let carry = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + c + carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, D
            0x8A => {
                let a = self.registers.read_a();
                let d = self.registers.read_d();

                if !check_carry(a, d) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + d + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, E
            0x8B => {
                let a = self.registers.read_a();
                let e = self.registers.read_e();

                if !check_carry(a, e) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + e + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, H
            0x8C => {
                let a = self.registers.read_a();
                let h = self.registers.read_h();

                if !check_carry(a, h) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + h + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, L
            0x8D => {
                let a = self.registers.read_a();
                let l = self.registers.read_l();

                if !check_carry(a, l) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + l + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, (HL)
            0x8E => {
                let f = self.registers.read_a();

                let address = self.registers.read_hl();
                let s = self.mmu.read_byte(address);

                if !check_carry(f, s) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(f, s) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(f + s + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // ADC A, A
            0x8F => {
                let a = self.registers.read_a();
                let a2 = self.registers.read_a();

                if !check_carry(a, a2) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a2) {
                    self.set_flag(CpuFlags::H);
                }

                self.clear_flag(CpuFlags::N);

                let c = (self.get_flag(CpuFlags::C) != 0) as i8;
                self.registers.write_a(a + a2 + c);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB B
            0x90 => {
                let b = self.registers.read_b();

                if !check_carry(b, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(b, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_b(b - 1);

                if self.registers.read_b() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB C
            0x91 => {
                let c = self.registers.read_c();

                if !check_carry(c, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(c, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_c(c - 1);

                if self.registers.read_c() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB D
            0x92 => {
                let d = self.registers.read_d();

                if !check_carry(d, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(d, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_d(d - 1);

                if self.registers.read_d() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB E
            0x93 => {
                let e = self.registers.read_e();

                if !check_carry(e, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(e, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_e(e - 1);

                if self.registers.read_e() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB H
            0x94 => {
                let h = self.registers.read_h();

                if !check_carry(h, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(h, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_h(h - 1);

                if self.registers.read_h() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB L
            0x95 => {
                let l = self.registers.read_l();

                if !check_carry(l, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(l, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_l(l - 1);

                if self.registers.read_l() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB (HL)
            0x96 => {
                let address = self.registers.read_hl();
                let l = self.mmu.read_word(address);

                if !check_carry(l, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(l, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.mmu.write_word(address, l - 1);

                if l - 1 == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SUB A
            0x97 => {
                let a = self.registers.read_a();

                if !check_carry(a, -1) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, -1) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                self.registers.write_a(a - 1);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, B
            0x98 => {
                let a = self.registers.read_a();
                let b = self.registers.read_b();

                if !check_carry(a, b) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, b) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - b + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, C
            0x99 => {
                let a = self.registers.read_a();
                let c = self.registers.read_c();

                if !check_carry(a, c) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, c) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - c + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, D
            0x9A => {
                let a = self.registers.read_a();
                let d = self.registers.read_d();

                if !check_carry(a, d) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, d) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - d + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, E
            0x9B => {
                let a = self.registers.read_a();
                let e = self.registers.read_e();

                if !check_carry(a, e) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, e) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - e + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, H
            0x9C => {
                let a = self.registers.read_a();
                let h = self.registers.read_h();

                if !check_carry(a, h) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, h) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - h + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, L
            0x9D => {
                let a = self.registers.read_a();
                let l = self.registers.read_l();

                if !check_carry(a, l) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, l) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - l + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, (HL)
            0x9E => {
                let a = self.registers.read_a();

                let hl = self.registers.read_hl();
                let data = self.mmu.read_byte(hl);

                if !check_carry(a, data) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, data) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - data + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // SBC A, A
            0x9F => {
                let a = self.registers.read_a();

                if !check_carry(a, a) {
                    self.set_flag(CpuFlags::C);
                }

                if !check_half_carry(a, a) {
                    self.set_flag(CpuFlags::H);
                }

                self.set_flag(CpuFlags::N);

                let has_carry = (self.get_flag(CpuFlags::C) != 0) as i8;

                self.registers.write_a(a - a + has_carry);

                if self.registers.read_a() == 0 {
                    self.set_flag(CpuFlags::Z);
                }
            }
            // AND B
            0xA0 => {
                let a = self.registers.read_a();
                let b = self.registers.read_b();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND C
            0xA1 => {
                let a = self.registers.read_a();
                let b = self.registers.read_c();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND D
            0xA2 => {
                let a = self.registers.read_a();
                let b = self.registers.read_d();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND E
            0xA3 => {
                let a = self.registers.read_a();
                let b = self.registers.read_e();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND H
            0xA4 => {
                let a = self.registers.read_a();
                let b = self.registers.read_h();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND L
            0xA5 => {
                let a = self.registers.read_a();
                let b = self.registers.read_l();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND (HL)
            0xA6 => {
                let a = self.registers.read_a();
                let address = self.registers.read_hl();
                let b = self.mmu.read_byte(address);

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // AND A
            0xA7 => {
                let a = self.registers.read_a();
                let b = self.registers.read_a();

                self.clear_flag(CpuFlags::N);
                self.set_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::C);

                if a & b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a & b);
            }
            // XOR B
            0xA8 => {
                let a = self.registers.read_a();
                let b = self.registers.read_b();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR C
            0xA9 => {
                let a = self.registers.read_a();
                let b = self.registers.read_c();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR D
            0xAA => {
                let a = self.registers.read_a();
                let b = self.registers.read_d();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR E
            0xAB => {
                let a = self.registers.read_a();
                let b = self.registers.read_e();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR H
            0xAC => {
                let a = self.registers.read_a();
                let b = self.registers.read_h();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR L
            0xAD => {
                let a = self.registers.read_a();
                let b = self.registers.read_l();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR (HL)
            0xAE => {
                let a = self.registers.read_a();
                let address = self.registers.read_hl();
                let b = self.mmu.read_byte(address);
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ b);
            }
            // XOR A
            0xAF => {
                let a = self.registers.read_a();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a ^ a == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a ^ a);
            }
            // OR B
            0xB0 => {
                let a = self.registers.read_a();
                let b = self.registers.read_b();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR C
            0xB1 => {
                let a = self.registers.read_a();
                let b = self.registers.read_c();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR D
            0xB2 => {
                let a = self.registers.read_a();
                let b = self.registers.read_d();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR E
            0xB3 => {
                let a = self.registers.read_a();
                let b = self.registers.read_e();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR H
            0xB4 => {
                let a = self.registers.read_a();
                let b = self.registers.read_h();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR L
            0xB5 => {
                let a = self.registers.read_a();
                let b = self.registers.read_l();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR (HL)
            0xB6 => {
                let a = self.registers.read_a();
                let address = self.registers.read_hl();
                let b = self.mmu.read_byte(address);
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | b == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | b);
            }
            // OR A
            0xB7 => {
                let a = self.registers.read_a();
                
                self.clear_flag(CpuFlags::C);
                self.clear_flag(CpuFlags::H);
                self.clear_flag(CpuFlags::N);

                if a | a == 0 {
                    self.set_flag(CpuFlags::Z);
                }

                self.registers.write_a(a | a);
            }
            // CP B
            0xB8 => {
                
            }
            0xB9 => {

            }
            0xBA => {

            }
            0xBB => {

            }
            0xBC => {
                
            }
            0xBD => {

            }
            0xBE => {

            }
            0xBF => {

            }
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