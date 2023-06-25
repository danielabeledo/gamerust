use crate::cpu::opcode::Opcode;
use crate::cpu::registers::{F, Registers, RR};
use crate::mmu::bus::Bus;
use crate::mmu::memory::Memory;
use crate::ppu::ppu::Ppu;

static IE: u16 = 0xFFFF;
static IF: u16 = 0xFF0F;
static DMA: u16 = 0xFF46;

pub struct Cpu {
    pub registers: Registers,
    pub ime: bool,
    pub ime_delay: bool,
    pub waiting_for_interrupt: bool,
    tima_ctr: u16,
    tma_flag: bool,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Default::default(), ime: false, ime_delay: false, tma_flag: false, waiting_for_interrupt: false,  tima_ctr: 0 , halted: false}
    }

    pub fn tick(&mut self, bus: &mut Bus, steps: i32) -> i32 {
        if steps < 1 {
            return steps;
        }

        // TIMERS
        self.update_timer_registers(bus);

        // INTERRUPTS
        if self.ime_delay {
            //self.print_registers();
            self.ime_delay = false;
            self.ime = true;
        }

        if self.interrupt_requested(bus) {
            self.waiting_for_interrupt = true;
            self.halted = false;
        }

        if self.ime && self.waiting_for_interrupt {
            let interrupt_type = self.get_interrupt_type(bus);
            self.waiting_for_interrupt = false;
            if interrupt_type != InterruptType::None {
                self.ime = false;
                println!("Interrupt type executing: {:?}", interrupt_type);
                self.unset_interrupt_flag(bus, &interrupt_type);
                Opcode::new_interrupt(interrupt_type).execute(self, bus);
                return steps - 5;
            }
        }

        if self.halted {
            // println!("{:?}", bus.get_byte(IE) & bus.get_byte(IF));
            if bus.get_byte(IE) & bus.get_byte(IF) > 0 {
                println!("halted removed");
                self.halted = false;
            }
            return 0;
        }

        // DMA
        if bus.io_registers.dma_triggered {
            println!("DMA triggered");
            self.execute_dma(bus);
            bus.io_registers.dma_triggered = false;
            return steps - 40;
        }

        // OPCODE
        let instruction: Opcode = self.get_instruction(bus);
        let cycles = instruction.execute(self, bus);
        return steps.wrapping_sub(cycles as i32);
    }

    fn update_timer_registers(&mut self, bus: &mut Bus) {

        bus.io_registers.div = bus.io_registers.div.wrapping_add(1);

        // timer disabled
        if (bus.io_registers.tac >> 2 & 1) == 0 {
            return;
        }
        let clock = match bus.io_registers.tac & 0b11 {
            0 => 16,
            1 => 1,
            2 => 4,
            3 => 8,
            _ => panic!("unreacheable")
        };
        if self.tma_flag {
            self.tma_flag = false;
            bus.set_byte(IF, bus.get_byte(IF) | 0b0100);
            bus.io_registers.tima = bus.io_registers.tma;
        } else {
            self.tima_ctr = self.tima_ctr.wrapping_add(1);
        }

        if self.tima_ctr >= clock {
            self.tima_ctr -= clock;
            if bus.io_registers.tima == 0xFF {
                bus.io_registers.tima = 0;
                self.tma_flag = true;
            } else {
                bus.io_registers.tima = bus.io_registers.tima.wrapping_add(1);
            }
        }
    }

    fn execute_dma(&self, bus: &mut Bus) {
        let starting_point = (bus.io_registers.get_byte(DMA) as u16) << 8;
        for i in 0..0x100 {
            bus.set_byte(0xFE00 + i, bus.get_byte(starting_point + i));
        }
    }

    #[inline]
    fn unset_interrupt_flag(&self, bus: &mut Bus, flag: &InterruptType) {
        bus.set_byte(IF, bus.get_byte(IF) & !(1 << *flag as usize));
    }

    #[inline]
    fn interrupt_requested(&self, bus: &Bus) -> bool {
        self.ime && bus.get_byte(IE) & bus.get_byte(IF) > 0
    }

    fn get_interrupt_type(&self, bus: &Bus) -> InterruptType {
        let interrupts: u8 = bus.get_byte(IE) & bus.get_byte(IF);
        for i in 0..5 {
            if interrupts >> i & 0x1 == 1 {
                return InterruptType::from(i);
            }
        };
        InterruptType::None
    }

    #[inline]
    fn get_instruction(&mut self, bus: &mut Bus) -> Opcode {
        let opcode_id: u8 = bus.get_byte(self.registers.get_pc_and_increase());
        if opcode_id == 0xCB {
            return Opcode::fetch_cb(bus.get_byte(self.registers.get_pc_and_increase()));
        }
        Opcode::fetch(opcode_id)
    }

    pub fn print_registers(&self) {
        println!("Registers:\tAF: 0x{:04X?}\tBC: 0x{:04X?}\tDE: 0x{:04X?}\tHL: 0x{:04X?}",
                 self.registers.get_r16(RR::AF), self.registers.get_r16(RR::BC),
                 self.registers.get_r16(RR::DE), self.registers.get_r16(RR::HL));
        println!("\t\t\tSP: 0x{:04X?}\tPC: 0x{:04X?}", self.registers.get_r16(RR::SP),
                 self.registers.get_r16(RR::PC));
        println!("Flags:\t\tZ: {:?} N: {:?} C: {:?} H: {:?}", self.registers.is_flag(F::Z),
                 self.registers.is_flag(F::N), self.registers.is_flag(F::C),
                 self.registers.is_flag(F::H));
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InterruptType {
    VBLANK,
    LCD,
    TIMER,
    SERIAL,
    JOYPAD,
    None
}

impl InterruptType {
    pub fn from(u: usize) -> InterruptType {
        match u {
            0 => InterruptType::VBLANK,
            1 => InterruptType::LCD,
            2 => InterruptType::TIMER,
            3 => InterruptType::SERIAL,
            4 => InterruptType::JOYPAD,
            _ => panic!("invalid interrupt type")
        }
    }
}