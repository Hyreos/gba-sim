extern crate num;
extern crate std;

use cpu::num::traits::PrimInt as PrimInt;

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

impl Cpu<> {
    // pub fn new(mut mmu: Mmu) -> Self {
    //    let mut myself = Cpu{ mmu: mmu };
    //    return myself;
    // }

    pub fn set_flag(&mut self, flags: CpuFlags) {
        match flags {
            CpuFlags::C => { self.flags |= 0x10u8; }
            CpuFlags::H => { self.flags |= 0x20u8; }
            CpuFlags::N => { self.flags |= 0x40u8; }
            CpuFlags::Z => { self.flags |= 0x80u8; }
        }
    }

    pub fn clear_flag(&mut self, flags: CpuFlags) {
         match flags {
            CpuFlags::C => { self.flags &= 0x10u8; }
            CpuFlags::H => { self.flags &= 0x20u8; }
            CpuFlags::N => { self.flags &= 0x40u8; }
            CpuFlags::Z => { self.flags &= 0x80u8; }
        }
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

            }
            0x12 => {

            }
            0x13 => {
                
            }
            0x14 => {

            }
            0x15 => {

            }
            0x16 => {

            }
            0x17 => {

            }
            0x18 => {
                
            }
            0x19 => {

            }
            0x1A => {

            }
            0x1B => {

            }
            0x1C => {
                
            }
            0x1D => {

            }
            0x1E => {

            }
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
            0x80 => {}
            0x81 => {

            }
            0x82 => {

            }
            0x83 => {
                
            }
            0x84 => {

            }
            0x85 => {

            }
            0x86 => {

            }
            0x87 => {

            }
            0x88 => {
                
            }
            0x89 => {

            }
            0x8A => {

            }
            0x8B => {

            }
            0x8C => {
                
            }
            0x8D => {

            }
            0x8E => {

            }
            0x8F => {

            }
            0x90 => {

            }
            0x91 => {

            }
            0x92 => {

            }
            0x93 => {
                
            }
            0x94 => {

            }
            0x95 => {

            }
            0x96 => {

            }
            0x97 => {

            }
            0x98 => {
                
            }
            0x99 => {

            }
            0x9A => {

            }
            0x9B => {

            }
            0x9C => {
                
            }
            0x9D => {

            }
            0x9E => {

            }
            0x9F => {

            }
            0xA0 => {

            }
            0xA1 => {

            }
            0xA2 => {

            }
            0xA3 => {
                
            }
            0xA4 => {

            }
            0xA5 => {

            }
            0xA6 => {

            }
            0xA7 => {

            }
            0xA8 => {
                
            }
            0xA9 => {

            }
            0xAA => {

            }
            0xAB => {

            }
            0xAC => {
                
            }
            0xAD => {

            }
            0xAE => {

            }
            0xAF => {

            }
            0xB0 => {

            }
            0xB1 => {

            }
            0xB2 => {

            }
            0xB3 => {
                
            }
            0xB4 => {

            }
            0xB5 => {

            }
            0xB6 => {

            }
            0xB7 => {

            }
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