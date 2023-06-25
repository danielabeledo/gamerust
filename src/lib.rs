pub mod cpu {
    pub mod cpu;
    pub mod registers;
    mod opcode;
}

pub mod cartridge {
    pub mod cartridge;
    pub mod rom_only;
    pub mod mbc1;
}

pub mod ppu {
    pub mod ppu;
    pub mod oam_entry;
}

pub mod mmu {
    pub mod bus;
    pub mod memory;
    pub mod joypad;
    mod bios;
    mod interrupt;
    mod vram;
    mod ext_ram;
    mod work_ram;
    mod io_registers;
    mod oam;
    mod hram;
}

pub mod gameboy;