use std::borrow::BorrowMut;

use crate::cartridge::cartridge::Cartridge;
use crate::mmu::bios::Bios;
use crate::mmu::ext_ram::ExtRam;
use crate::mmu::hram::HRam;
use crate::mmu::interrupt::Interrupt;
use crate::mmu::io_registers::IORegisters;
use crate::mmu::memory::Memory;
use crate::mmu::oam::OAM;
use crate::mmu::vram::VRam;
use crate::mmu::work_ram::WorkRam;

pub struct Bus {
    // 0x0000 - 0x00FF - 256B BIOS
    pub bios: Bios,
    // 0x0000 - 0x7FFF - 32kb ROM
    pub cartridge: Box<dyn Cartridge>,
    // 0x8000 - 0x9FFF - 8kb VRAM
    pub vram: VRam,
    // 0xA000 - 0xBFFF - 8kb SRAM
    pub ext_ram: ExtRam,
    // 0xC000 - 0xCFFF - 4kb WRAM 0
    // 0xD000 - 0xDFFF - 4kb WRAM 1
    // 0xE000 - 0xFDFF - 7.5KB ECHO RAM
    pub work_ram: WorkRam,
    // 0xFE00 - 0xFE9F - 160B OAM
    pub oam: OAM,
    // 0xFEA0 - 0xFEFF - 96B unusable

    // 0xFF00 - 0xFF7F - 128B I/O registers
    pub io_registers: IORegisters,
    // 0xFF80 - 0xFFFE - 127B HRAM
    pub hram: HRam,
    //          0xFFFF - Interrupts
    pub interrupts: Interrupt,
}

impl Bus {
    pub fn new(rom: Box<dyn Cartridge>) -> Self {
        Bus {
            bios: Default::default(),
            cartridge: rom,
            vram: Default::default(),
            ext_ram: Default::default(),
            work_ram: Default::default(),
            oam: Default::default(),
            io_registers: Default::default(),
            hram: Default::default(),
            interrupts: Default::default(),
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        if address < 0x8000 {
            if self.boot_rom() && address < 0x100 {
                return self.bios.get_byte(address);
            }
            return (*self.cartridge).get_byte(address);
        } else if address < 0xA000 && address >= 0x8000 {
            self.vram.get_byte(address)
        } else if address < 0xC000 && address >= 0xA000 {
            self.ext_ram.get_byte(address)
        } else if address < 0xE000 && address >= 0xC000 {
            self.work_ram.get_byte(address)
        } else if address < 0xFDFF && address >= 0xE000 {
            self.work_ram.get_byte(address - 0x2000)
        } else if address < 0xFEA0 && address >= 0xFE00 {
            self.oam.get_byte(address)
        } else if address < 0xFEFF && address >= 0xFEA0 {
            return 0xFF;
        } else if address < 0xFF80 && address >= 0xFF00 {
            self.io_registers.get_byte(address)
        } else if address < 0xFFFF && address >= 0xFF80 {
            self.hram.get_byte(address)
        } else {
            self.interrupts.get_byte(address)
        }
    }

    pub fn set_byte(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            if self.boot_rom() && address < 0x100 {
                self.bios.set_byte(address, value)
            }
            (*self.cartridge).borrow_mut().set_byte(address, value)
        } else if address < 0xA000 && address >= 0x8000 {
            self.vram.set_byte(address, value)
        } else if address < 0xC000 && address >= 0xA000 {
            self.ext_ram.set_byte(address, value)
        } else if address < 0xE000 && address >= 0xC000 {
            self.work_ram.set_byte(address, value)
        } else if address < 0xFDFF && address >= 0xE000 {
            self.work_ram.set_byte(address - 0x2000, value)
        } else if address < 0xFEA0 && address >= 0xFE00 {
            self.oam.set_byte(address, value)
        } else if address < 0xFEFF && address >= 0xFEA0 {
            // do nothing
        } else if address < 0xFF80 && address >= 0xFF00 {
            self.io_registers.set_byte(address, value)
        } else if address < 0xFFFF && address >= 0xFF80 {
            self.hram.set_byte(address, value)
        } else {
            self.interrupts.set_byte(address, value);
        }
    }

    pub fn boot_rom(&self) -> bool {
        self.io_registers.boot
    }
}

