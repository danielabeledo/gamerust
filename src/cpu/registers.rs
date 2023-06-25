#[derive(Debug, Copy, Clone)]
// 8 bits registers
pub enum R {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug, Copy, Clone)]
// 16 bits registers
pub enum RR {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
// CPU register flags
pub enum F {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Copy, Clone)]
struct Register<T> {
    pub r: T,
}

impl<T> Register<T> where T: Clone + Copy {
    pub fn value(&self) -> T {
        self.r
    }
    pub fn set(&mut self, value: T) {
        self.r = value;
    }
}

pub struct Registers {
    b: Register<u8>,
    c: Register<u8>,
    d: Register<u8>,
    e: Register<u8>,
    h: Register<u8>,
    l: Register<u8>,
    a: Register<u8>,
    f: Register<u8>,
    sp: Register<u16>,
    pc: Register<u16>,
}

impl Registers {
    pub fn get_r8(&self, r: R) -> u8 {
        match r {
            R::A => &self.a,
            R::B => &self.b,
            R::C => &self.c,
            R::D => &self.d,
            R::E => &self.e,
            R::F => &self.f,
            R::H => &self.h,
            R::L => &self.l,
        }.value()
    }
    pub fn get_r16(&self, rr: RR) -> u16 {
        match rr {
            RR::AF => self.combine_rr(R::A, R::F),
            RR::BC => self.combine_rr(R::B, R::C),
            RR::DE => self.combine_rr(R::D, R::E),
            RR::HL => self.combine_rr(R::H, R::L),
            RR::SP => self.sp.value(),
            RR::PC => self.pc.value(),
        }
    }
    fn combine_rr(&self, r1: R, r2: R) -> u16 {
        return (self.get_r8(r1) as u16) << 8 | (self.get_r8(r2) as u16);
    }
    pub fn set_r8(&mut self, r: R, value: u8) {
        match r {
            R::A => self.a.set(value),
            R::B => self.b.set(value),
            R::C => self.c.set(value),
            R::D => self.d.set(value),
            R::E => self.e.set(value),
            R::F => self.f.set(value & 0xF0),
            R::H => self.h.set(value),
            R::L => self.l.set(value),
        }
    }
    pub fn set_r16(&mut self, rr: RR, value: u16) {
        match rr {
            RR::AF => {
                let (a, f) = Registers::split_rr(value);
                self.a.set(a);
                self.f.set(f & 0xF0);
            }
            RR::BC => {
                let (b, c) = Registers::split_rr(value);
                self.b.set(b);
                self.c.set(c);
            }
            RR::DE => {
                let (d, e) = Registers::split_rr(value);
                self.d.set(d);
                self.e.set(e);
            }
            RR::HL => {
                let (h, l) = Registers::split_rr(value);
                self.h.set(h);
                self.l.set(l);
            }
            RR::SP => self.sp.set(value),
            RR::PC => self.pc.set(value),
        }
    }
    fn split_rr(value: u16) -> (u8, u8) {
        ((value >> 8) as u8, (value & 0xFF) as u8)
    }
    pub fn get_pc_and_increase(&mut self) -> u16 {
        let pc: u16 = self.get_r16(RR::PC);
        self.set_r16(RR::PC, pc.wrapping_add(1));
        pc
    }
    pub fn get_hl_and_increase(&mut self) -> u16 {
        let hl: u16 = self.get_r16(RR::HL);
        self.set_r16(RR::HL, hl.wrapping_add(1));
        hl
    }
    pub fn get_hl_and_decrease(&mut self) -> u16 {
        let hl: u16 = self.get_r16(RR::HL);
        self.set_r16(RR::HL, hl.wrapping_sub(1));
        hl
    }

    pub fn get_sp_and_decrease(&mut self) -> u16 {
        let sp: u16 = self.get_r16(RR::SP);
        self.set_r16(RR::SP, sp.wrapping_sub(1));
        sp
    }
    pub fn get_sp_and_increase(&mut self) -> u16 {
        let sp: u16 = self.get_r16(RR::SP);
        self.set_r16(RR::SP, sp.wrapping_add(1));
        sp
    }

    pub fn decrease_and_get_sp(&mut self) -> u16 {
        let sp: u16 = self.get_r16(RR::SP).wrapping_sub(1);
        self.set_r16(RR::SP, sp);
        sp
    }
    pub fn increase_and_get_sp(&mut self) -> u16 {
        let sp: u16 = self.get_r16(RR::SP).wrapping_add(1);
        self.set_r16(RR::SP, sp);
        sp
    }

    pub fn set_flag(&mut self, f: F) {
        let value: u8 = self.get_r8(R::F) | 1 << f as u8;
        self.set_r8(R::F, value);
    }
    pub fn unset_flag(&mut self, f: F) {
        let value: u8 = self.get_r8(R::F) & !(1 << f as u8);
        self.set_r8(R::F, value);
    }
    pub fn unset_flags(&mut self) {
        self.set_r8(R::F, 0x00);
    }
    pub fn is_flag(&self, f: F) -> bool {
        ((self.get_r8(R::F) >> f as u8) & 0x1) == 1
    }
    pub fn flag(&mut self, f: F, set: bool) {
        if set { self.set_flag(f) } else { self.unset_flag(f) }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: Register { r: 0 },
            b: Register { r: 0 },
            c: Register { r: 0 },
            d: Register { r: 0 },
            e: Register { r: 0 },
            f: Register { r: 0 },
            h: Register { r: 0 },
            l: Register { r: 0 },
            sp: Register { r: 0 },
            pc: Register { r: 0 },
        }
    }
}