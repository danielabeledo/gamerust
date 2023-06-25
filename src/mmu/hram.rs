use crate::mmu::memory::Memory;

pub struct HRam {
    mem: [u8; 127],
}

impl Default for HRam {
    fn default() -> Self {
        HRam {
            mem: [0; 127]
        }
    }
}

impl Memory for HRam {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0xFFFF && address >= 0xFF80 {
            self.mem[(address - 0xFF80) as usize]
        } else {
            panic!("Unaccessible memory.")
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0xFFFF && address >= 0xFF80 {
            self.mem[(address - 0xFF80) as usize] = value;
        } else {
            panic!("Unaccessible memory.")
        }
    }
}