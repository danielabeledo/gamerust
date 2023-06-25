use crate::cartridge::mbc1::MBC1;
use crate::cartridge::rom_only::RomOnly;
use crate::mmu::memory::Memory;

pub enum CartridgeType {
    RomOnly,
    MBC1,
}

pub trait Cartridge: Memory {}

impl CartridgeType {
    pub fn load(&self, v: Vec<u8>) -> Box<dyn Cartridge> {
        match *self {
            CartridgeType::RomOnly => RomOnly::load(v),
            CartridgeType::MBC1 => MBC1::load(v),
        }
    }

    pub fn get_cartridge_type(b: &u8) -> CartridgeType {
        match *b {
            0 => CartridgeType::RomOnly,
            1 => CartridgeType::MBC1,
            2 => CartridgeType::MBC1,
            3 => CartridgeType::MBC1,
            _ => panic!("value not implemented: {:x?}", b)
        }
    }
}
