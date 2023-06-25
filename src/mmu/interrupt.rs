use crate::mmu::memory::Memory;

pub struct Interrupt {
    value: u8,
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Default for Interrupt {
    fn default() -> Self {
        Self {
            value: 0,
            vblank: false,
            lcd: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }
}

impl Memory for Interrupt {
    fn get_byte(&self, _: u16) -> u8 {
        self.value
    }

    fn set_byte(&mut self, _: u16, value: u8) {
        self.value = value;
        self.vblank = value & 0x1 == 1;
        self.lcd = value & 0x2 == 1;
        self.timer = value & 0x4 == 1;
        self.serial = value & 0x8 == 1;
        self.joypad = value & 0x10 == 1;
    }
}