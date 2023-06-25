use crate::cartridge::cartridge::Cartridge;
use crate::mmu::memory::Memory;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    size: u32,
    ramg: bool,
    mode: bool,
    bank1_reg: u8,
    bank2_reg: u8,
}

impl Cartridge for MBC1 {}

impl MBC1 {
    pub fn load(v: Vec<u8>) -> Box<Self> {
        let size: u32 = 0x8000 << v.get(0x148).unwrap();
        let ram_size: u32 = match v.get(0x149).unwrap() {
            0 => 0,
            1 => 2 * 1024,
            2 => 8 * 1024,
            3 => 32 * 1024,
            4 => 128 * 1024,
            5 => 64 * 1024,
            _ => panic!("RAM size not implemented for {:?}", v.get(0x149).unwrap())
        };
        println!("Size of the cartridge ROM {:?} bytes RAM {:?}", size, ram_size);
        Box::new(MBC1 {
            rom: v,
            ram: Vec::with_capacity(ram_size as usize),
            size: size,
            bank1_reg: 1,
            bank2_reg: 0,
            ramg: false,
            mode: false,
        })
    }
}

impl Memory for MBC1 {
    fn get_byte(&self, address: u16) -> u8 {
        if address < 0x4000 {
            if self.mode {
                let reg: usize = (((self.bank2_reg as u32) << 18) | address as u32) as usize;
                return *self.rom.get((reg as u32 % self.size) as usize).unwrap();
            } else {
                return *self.rom.get(address as usize).unwrap();
            }
        } else if address >= 0x4000 && address < 0x8000 {
            let reg: usize = (((self.bank2_reg as u32) << 18) | ((self.bank1_reg as u32) << 14) | (address - 0x4000) as u32) as usize;
            return *self.rom.get((reg as u32 % self.size) as usize).unwrap();
        } else if address >= 0xA000 && address < 0xC000 {
            if !self.ramg || self.ram.len() == 0 {
                return 0x00;
            }
            let reg: usize = (address - 0xA000) as usize;
            if !self.mode {
                return *self.ram.get(reg).unwrap();
            } else {
                return *self.ram.get((((self.bank2_reg as u32) << 12) | reg as u32) as usize).unwrap();
            }
        }
        return 0;
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.ramg = value & 0b1111 == 0b1010;
        } else if address >= 0x2000 && address < 0x4000 {
            let reg: u8 = value & 0b11111;
            if reg % 0x20 == 0 {
                self.bank1_reg = reg + 1;
            } else {
                self.bank1_reg = reg;
            }
        } else if address >= 0x4000 && address < 0x6000 {
            self.bank2_reg = value & 0b11;
        } else if address >= 0x6000 && address < 0x8000 {
            self.mode = value & 0b1 == 1;
        } else if address >= 0xA000 && address < 0xC000 {
            if self.ramg && self.ram.len() != 0 {
                let reg: u16 = if !self.mode {
                    address & 0b1111_1111_1111
                } else {
                    (self.bank2_reg as u16) << 12 | address & 0b1111_1111_1111
                };
                self.ram[reg as usize] = value;
            }
        }
    }
}