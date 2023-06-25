use std::borrow::BorrowMut;

use crate::cartridge::cartridge::CartridgeType;
use crate::cpu::cpu::Cpu;
use crate::mmu::bus::Bus;
use crate::ppu::ppu::Ppu;
use crate::cpu::registers::RR;

pub struct Gameboy {
    pub bus: Bus,
    pub cpu: Cpu,
    pub ppu: Ppu,

    pub clock_cpu: i32,
    pub clock_ppu: i32,

    debug: bool,
}

impl Gameboy {
    pub fn print(&self) {
        self.cpu.print_registers();
        println!("LY: {:?} SCY: {:?} SCX: {:?}", self.bus.get_byte(0xFF44),
                 self.bus.get_byte(0xFF42), self.bus.get_byte(0xFF43));
        println!("LCDC: 0x{:X?} STAT: 0x{:X?} IE: {:X?} IF: {:X?} IME: {:?}", self.bus.get_byte(0xFF40),
                 self.bus.get_byte(0xFF41), self.bus.get_byte(0xFFFF),
                 self.bus.get_byte(0xFF0F), self.cpu.ime);
    }

    pub fn tick(&mut self) {
        self.clock_cpu = self.cpu.tick(self.bus.borrow_mut(), self.clock_cpu + 1);
        self.clock_ppu = self.ppu.tick(self.bus.borrow_mut(), self.clock_ppu + 1);
    }

    pub fn load_rom(args: Vec<String>) -> Gameboy {
        let rom_data: Vec<u8> = std::fs::read(args.get(1)
            .unwrap_or(&String::from("roms/Battletoads (Japan).gb")))
            .expect("file not found");

        let cartridge_type: CartridgeType = CartridgeType::get_cartridge_type(rom_data.get(0x147).unwrap());

        let cartridge = cartridge_type.load(rom_data);

        let bus: Bus = Bus::new(cartridge);
        let cpu: Cpu = Cpu::new();
        let ppu: Ppu = Ppu::new();

        Gameboy { bus, cpu, ppu, clock_cpu: 0, clock_ppu: 0, debug: false }
    }
}