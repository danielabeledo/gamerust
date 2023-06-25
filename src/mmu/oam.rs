use crate::mmu::memory::Memory;

pub struct OAM {
    oam: [u8; 0xA0],
}

impl Default for OAM {
    fn default() -> Self {
        OAM {
            oam: [0; 0xA0]
        }
    }
}

impl Memory for OAM {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0xFEA0 && address >= 0xFE00 {
            self.oam[(address - 0xFE00) as usize]
        } else {
            panic!("Unaccessible memory.")
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0xFEA0 && address >= 0xFE00 {
            self.oam[(address - 0xFE00) as usize] = value;
        } else {
            panic!("Unaccessible memory.")
        }
    }
}