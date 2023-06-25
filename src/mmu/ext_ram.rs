use crate::mmu::memory::Memory;

pub struct ExtRam {
    bank0: [u8; 0x2000],
}

impl Default for ExtRam {
    fn default() -> Self {
        ExtRam {
            bank0: [0; 0x2000]
        }
    }
}

impl Memory for ExtRam {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0xC000 && address >= 0xA000 {
            self.bank0[(address - 0xA000) as usize]
        } else {
            panic!("Unaccessible memory.")
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0xC000 && address >= 0xA000 {
            self.bank0[(address - 0xA000) as usize] = value;
        } else {
            panic!("Unaccessible memory.")
        }
    }
}