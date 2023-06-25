use crate::mmu::bus::Bus;

pub struct OamEntry {
    pub ypos: u8,
    pub xpos: u8,
    pub priority: bool,
    pub palette: bool,
    pub obj16: bool,
    pub render8: [u8; 64],
    pub render16: [u8; 128],
}

impl OamEntry {
    pub fn new() -> Self {
        Self {
            ypos: 0,
            xpos: 0,

            priority: false,
            palette: false,
            obj16: false,
            render8: [0; 64],
            render16: [0; 128],
        }
    }

    pub fn get_oam_line(bus: &Bus, ly: u8, obj16: bool) -> Vec<OamEntry> {
        let mut ret = Vec::with_capacity(10);
        let mut ctr = 0;
        if obj16 {
            for i in 0..20 {
                if ctr >= 10 { break; }
                let oam = {
                    let mut result = [0; 8];
                    for j in 0..8 {
                        result[j as usize] = bus.get_byte(0xFE00 + (i * 4) + j);
                    };
                    result
                };

                let ypos: u8 = oam[0];
                let xpos: u8 = oam[1];

                if ypos == 0 || ypos >= 160 {
                    continue;
                }

                if ly <= ypos - 0x10 || ly > ypos + 0x10 {
                    println!("{:?} {:?}", ypos, ly);
                    continue;
                }

                if xpos == 0 || xpos >= 168 {
                    continue;
                }

                let mut oam_entry = OamEntry::new();
                oam_entry.xpos = xpos;
                oam_entry.ypos = ypos;
                oam_entry.obj16 = true;
                ctr += 1;
                for i in 0..8 {
                    let low = bus.get_byte(0x8000_u16.wrapping_add(oam[2] as u16 * 16).wrapping_add(i * 2));
                    let top = bus.get_byte(0x8000_u16.wrapping_add(oam[2] as u16 * 16).wrapping_add(i * 2)).wrapping_add(1);
                    for j in (0..8).rev() {
                        oam_entry.render16[((7 - j) + 8 * i) as usize] = ((top >> j & 0x1) << 1 | (low >> j & 0x1));
                    };
                    let low = bus.get_byte(0x8000_u16.wrapping_add(oam[6] as u16 * 16).wrapping_add(i * 2));
                    let top = bus.get_byte(0x8000_u16.wrapping_add(oam[6] as u16 * 16).wrapping_add(i * 2)).wrapping_add(1);
                    for j in (0..8).rev() {
                        oam_entry.render16[((7 - j) + 8 * i + 64) as usize] = ((top >> j & 0x1) << 1 | (low >> j & 0x1));
                    };
                };

                ret.push(oam_entry);
            }
        } else {
            for i in 0..40 {
                if ctr >= 10 { break; }
                let oam = {
                    let mut result = [0; 4];
                    for j in 0..4 {
                        result[j as usize] = bus.get_byte(0xFE00 + (i * 4) + j);
                    };
                    result
                };

                let ypos: u8 = oam[0];
                let xpos: u8 = oam[1];
                if ypos == 0 || ypos >= 160 || ly + 0x10 > ypos || ly + 0x18 <= ypos {
                    continue;
                }

                if xpos == 0 || xpos >= 168 {
                    continue;
                }

                let mut oam_entry = OamEntry::new();
                oam_entry.xpos = xpos;
                oam_entry.ypos = ypos;
                oam_entry.obj16 = false;
                ctr += 1;
                for i in 0..8 {
                    let low = bus.get_byte(0x8000_u16.wrapping_add(oam[2] as u16 * 16).wrapping_add(i * 2));
                    let top = bus.get_byte(0x8000_u16.wrapping_add(oam[2] as u16 * 16).wrapping_add(i * 2)).wrapping_add(1);
                    for j in (0..8).rev() {
                        oam_entry.render8[((7 - j) + 8 * i) as usize] = ((top >> j & 0x1) << 1 | (low >> j & 0x1));
                    };
                };

                ret.push(oam_entry);
            }
        }
        ret
    }
}