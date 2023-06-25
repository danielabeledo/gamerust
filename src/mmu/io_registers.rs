use crate::mmu::memory::Memory;
use crate::mmu::joypad::Joypad;

pub struct IORegisters {
    // interrupt request
    interrupt_flag: u8,
    // input
    pub p1: Joypad,
    // serial
    sb: u8,
    sc: u8,
    // timer
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    //sound
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,
    nr50: u8,
    nr51: u8,
    nr52: u8,
    // Wave Pattern RAM
    wave_ram: [u8; 16],
    // LCD controller
    lcdc: u8,
    pub stat: u8,
    scy: u8,
    scx: u8,
    pub ly: u8,
    pub lyc: u8,
    dma: u8,
    pub dma_triggered: bool,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    bootrom: u8,
    pub boot: bool,
}

impl Default for IORegisters {
    fn default() -> Self {
        IORegisters {
            interrupt_flag: 0,
            p1: Joypad::new(),
            sb: 0,
            sc: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            nr50: 0,
            nr51: 0,
            nr52: 0,
            wave_ram: [0; 16],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            dma_triggered: false,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            bootrom: 0,
            boot: true,
        }
    }
}

impl Memory for IORegisters {
    fn get_byte(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.p1.get_byte(address),
            0xFF01 => self.sb,
            0xFF02 => self.sc & 0b10000001 | 0b01111110,
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac & 0b00000111 | 0b11111000,
            0xFF0F => self.interrupt_flag & 0b00011111 | 0b11100000,
            0xFF10 => self.nr10 & 0b01111111 | 0b10000000,
            0xFF11 => self.nr11,
            0xFF12 => self.nr12,
            0xFF13 => self.nr13,
            0xFF14 => self.nr14,
            0xFF16 => self.nr21,
            0xFF17 => self.nr22,
            0xFF18 => self.nr23,
            0xFF19 => self.nr24,
            0xFF1A => self.nr30 & 0b1000_0000 | 0b0111_1111,
            0xFF1B => self.nr31,
            0xFF1C => self.nr32 & 0b0110_0000 | 0b1001_1111,
            0xFF1D => self.nr33,
            0xFF1E => self.nr34,
            0xFF20 => self.nr41 & 0b0011_1111 | 0b1100_0000,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44 & 0b1100_0000 | 0b0011_1111,
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.nr52 & 0b1000_1111 | 0b0111_0000,
            0xFF30 => self.wave_ram[0],
            0xFF31 => self.wave_ram[1],
            0xFF32 => self.wave_ram[2],
            0xFF33 => self.wave_ram[3],
            0xFF34 => self.wave_ram[4],
            0xFF35 => self.wave_ram[5],
            0xFF36 => self.wave_ram[6],
            0xFF37 => self.wave_ram[7],
            0xFF38 => self.wave_ram[8],
            0xFF39 => self.wave_ram[9],
            0xFF3A => self.wave_ram[10],
            0xFF3B => self.wave_ram[11],
            0xFF3C => self.wave_ram[12],
            0xFF3D => self.wave_ram[13],
            0xFF3E => self.wave_ram[14],
            0xFF3F => self.wave_ram[15],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat & 0b01111111 | 0b10000000,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF50 => 0xFF,
            _ => 0xFF
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.p1.set_byte(address, value),
            0xFF01 => self.sb = value,
            0xFF02 => self.sc = value,
            0xFF04 => self.div = 0x00,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            0xFF0F => self.interrupt_flag = value,
            0xFF10 => self.nr10 = value,
            0xFF11 => self.nr11 = value,
            0xFF12 => self.nr12 = value,
            0xFF13 => self.nr13 = value,
            0xFF14 => self.nr14 = value,
            0xFF16 => self.nr21 = value,
            0xFF17 => self.nr22 = value,
            0xFF18 => self.nr23 = value,
            0xFF19 => self.nr24 = value,
            0xFF1A => self.nr30 = value,
            0xFF1B => self.nr31 = value,
            0xFF1C => self.nr32 = value,
            0xFF1D => self.nr33 = value,
            0xFF1E => self.nr34 = value,
            0xFF20 => self.nr41 = value,
            0xFF21 => self.nr42 = value,
            0xFF22 => self.nr43 = value,
            0xFF23 => self.nr44 = value,
            0xFF24 => self.nr50 = value,
            0xFF25 => self.nr51 = value,
            0xFF26 => self.nr52 = value,
            0xFF30 => self.wave_ram[0] = value,
            0xFF31 => self.wave_ram[1] = value,
            0xFF32 => self.wave_ram[2] = value,
            0xFF33 => self.wave_ram[3] = value,
            0xFF34 => self.wave_ram[4] = value,
            0xFF35 => self.wave_ram[5] = value,
            0xFF36 => self.wave_ram[6] = value,
            0xFF37 => self.wave_ram[7] = value,
            0xFF38 => self.wave_ram[8] = value,
            0xFF39 => self.wave_ram[9] = value,
            0xFF3A => self.wave_ram[10] = value,
            0xFF3B => self.wave_ram[11] = value,
            0xFF3C => self.wave_ram[12] = value,
            0xFF3D => self.wave_ram[13] = value,
            0xFF3E => self.wave_ram[14] = value,
            0xFF3F => self.wave_ram[15] = value,
            0xFF40 => {
                self.lcdc = value;
                println!("LCDC {:08b}", value);
                if (value & (1 << 7)) == 0 {
                    println!("Disabling LCDC");
                    self.ly = 0;
                    self.stat &= 0b1111_1000;
                    println!("STAT after disable {:08b}", self.stat);
                } else {
                    println!("Enabling LCDC");
                }
            }
            0xFF41 => self.stat = (value & 0b1111_1000) | (self.stat & 0b111),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {
                println!("trying to write LY");
            }
            0xFF45 => self.lyc = value,
            0xFF46 => {
                self.dma = value;
                self.dma_triggered = true
            }
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF50 => {
                if self.boot && value == 0x1 { self.boot = false; }
            }
            _ => println!("trying to write in {:04X?}", address)
        };
    }
}