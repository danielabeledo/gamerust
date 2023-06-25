use crate::mmu::memory::Memory;

// $8000 - $9FFF -- Video RAM (8192 Bytes)
// $8000 - $9800 -- Character RAM (6144 Bytes)
// $9800 - $9FFF -- BG display (2048 Bytes)
// $9800 - $9BFF -- BG display MAP 1 (1024 Bytes)
// $9C00 - $9FFF -- BG display MAP 2 (1024 Bytes)
pub struct VRam {
    bank0: [u8; 0x2000],
}

impl Default for VRam {
    fn default() -> Self {
        VRam {
            bank0: [0; 0x2000]
        }
    }
}

impl Memory for VRam {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0xA000 && address >= 0x8000 {
            self.bank0[(address - 0x8000) as usize]
        } else {
            panic!("Unaccessible memory.")
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0xA000 && address >= 0x8000 {
            self.bank0[(address - 0x8000) as usize] = value;
        } else {
            panic!("Unaccessible memory.")
        }
    }
}