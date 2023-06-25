use crate::cartridge::cartridge::Cartridge;
use crate::mmu::memory::Memory;

pub struct RomOnly {
    bank: [u8; 0x8000],
}

impl Cartridge for RomOnly {}

impl RomOnly {
    pub fn load(v: Vec<u8>) -> Box<Self> {
        let mut bank: [u8; 0x8000] = [0; 0x8000];
        bank.copy_from_slice(&v);
        Box::new(RomOnly { bank })
    }
}

impl Memory for RomOnly {
    fn get_byte(&self, address: u16) -> u8 {
        self.bank[address as usize]
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        println!("trying to write 0x{:X?} to address 0x{:X?}", value, address);
    }
}