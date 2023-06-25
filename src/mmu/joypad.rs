use crate::mmu::memory::Memory;

pub struct Joypad {
    value: u8,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Memory for Joypad {
    fn get_byte(&self, _: u16) -> u8 {
        let mut res: u8 = 0xFF;
        if (self.value >> 5) & 0x1 == 0 {
            if self.a { res &= !(1 << 0) }
            if self.b { res &= !(1 << 1) }
            if self.select { res &= !(1 << 2) }
            if self.start { res &= !(1 << 3) }
            res &= !(1 << 5)
        } else if (self.value >> 4) & 0x1 == 0 {
            if self.right { res &= !(1 << 0) }
            if self.left { res &= !(1 << 1) }
            if self.up { res &= !(1 << 2) }
            if self.down { res &= !(1 << 3) }
            res &= !(1 << 4)
        }
        res
    }

    fn set_byte(&mut self, _: u16, value: u8) {
        self.value = value;
    }
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            value: 0xFF,
            a: false,
            b: false,
            select: false,
            start: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

}