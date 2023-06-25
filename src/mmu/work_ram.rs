use crate::mmu::memory::Memory;

pub struct WorkRam {
    bank0: [u8; 0x1000],
    bank1: [u8; 0x1000],
}

impl Default for WorkRam {
    fn default() -> Self {
        WorkRam {
            bank0: [0; 0x1000],
            bank1: [0; 0x1000],
        }
    }
}

impl Memory for WorkRam {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0xD000 && address >= 0xC000 {
            self.bank0[(address - 0xC000) as usize]
        } else if address < 0xE000 && address >= 0xD000 {
            self.bank1[(address - 0xD000) as usize]
        } else {
            panic!("Unaccessible memory.")
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0xD000 && address >= 0xC000 {
            self.bank0[(address - 0xC000) as usize] = value;
        } else if address < 0xE000 && address >= 0xD000 {
            self.bank1[(address - 0xD000) as usize] = value;
        } else {
            panic!("Unaccessible memory.")
        }
    }
}