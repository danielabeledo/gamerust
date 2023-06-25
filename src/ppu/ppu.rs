use crate::mmu::bus::Bus;
use crate::ppu::oam_entry::OamEntry;

static LCDC: u16 = 0xFF40;
static STAT: u16 = 0xFF41;
static SCY: u16 = 0xFF42;
static SCX: u16 = 0xFF43;
static LY: u16 = 0xFF44;
static BGP: u16 = 0xFF47;
static WY: u16 = 0xFF4A;
static WX: u16 = 0xFF4B;
static IF: u16 = 0xFF0F;

pub struct Ppu {
    pub image: [u8; 160 * 144],
    pub bg: [u8; 256 * 256],
    pub bg2: [u8; 256 * 256],
    pub ready: bool,
    oam_entries: Vec<OamEntry>,
}

#[derive(Debug)]
enum VideoMode {
    HBLANK = 0,
    VBLANK = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

impl Ppu {
    pub fn new() -> Self {
        Self { image: [0; 160 * 144], bg: [0; 256 * 256], bg2: [0; 256 * 256], ready: false, oam_entries: Vec::with_capacity(10) }
    }

    pub fn tick(&mut self, bus: &mut Bus, steps: i32) -> i32 {
        if steps < 1 {
            return steps;
        };

        if !Ppu::lcdc_on(bus) {
            return 0;
        };


        let lyc_flag = bus.io_registers.ly == bus.io_registers.lyc;
        if lyc_flag {
            bus.io_registers.stat |= (1 << 2);
        } else {
            bus.io_registers.stat &= !(1 << 2);
        }
        if lyc_flag && bus.get_byte(STAT) >> 6 & 0x1 == 1 {
            bus.set_byte(IF, bus.get_byte(IF) | 0b10);
        }

        let remaining: i32 = match self.get_video_mode(bus) {
            VideoMode::OamSearch => {
                if steps < 20 { return steps; }
                self.do_oam_search(bus)
            }
            VideoMode::PixelTransfer => {
                if steps < 43 { return steps; }
                self.do_pixel_transfer(bus)
            }
            VideoMode::HBLANK => {
                if steps < 51 { return steps; }
                self.do_hblank(bus)
            }
            VideoMode::VBLANK => {
                if steps < 456 { return steps; }
                self.do_vblank(bus)
            }
        };

        remaining
    }

    fn write_image(&mut self, bus: &Bus) {
        self.ready = true;

        // write bg map here
        let bgp: u8 = bus.get_byte(BGP);
        let mut palette: [u8; 4] = [0; 4];
        for i in 0..4 {
            palette[i] = bgp >> i * 2 & 0b11;
        }

        let starting_bg_tilemap: u16 = 0x9800;
        let starting_bg_data: u16 = 0x8000;

        for i in 0..1024 {
            let tile_ptr: u8 = bus.get_byte(starting_bg_tilemap + i as u16);

            let mut tiles: [u8; 16] = [0; 16];
            for j in 0..16_u16 {
                let address = starting_bg_data + tile_ptr as u16 * 16 + j;
                tiles[j as usize] = bus.get_byte(address);
            }

            for (k, b) in tiles.chunks(2).enumerate() {
                for (l, v) in self.combine_byte(b[0], b[1], &palette).iter().enumerate() {
                    self.bg[(i % 32) * 8 + (k + (i / 32) * 8) * 256 + l] = v.clone();
                }
            }
        };
        for i in 0..1024 {
            let tile_ptr: u8 = bus.get_byte(starting_bg_tilemap + i as u16);

            let bgdata = if tile_ptr < 128 {
                0x9000_u16.wrapping_add(tile_ptr as u16 * 16)
            } else {
                0x8800_u16.wrapping_add((tile_ptr - 128) as u16 * 16)
            };

            let mut tiles: [u8; 16] = [0; 16];
            for j in 0..16_u16 {
                let address = bgdata + j;
                tiles[j as usize] = bus.get_byte(address);
            }

            for (k, b) in tiles.chunks(2).enumerate() {
                for (l, v) in self.combine_byte(b[0], b[1], &palette).iter().enumerate() {
                    self.bg2[(i % 32) * 8 + (k + (i / 32) * 8) * 256 + l] = v.clone();
                }
            }
        };
    }

    pub fn get_image(&mut self) -> &[u8] {
        self.ready = false;
        &self.image
    }

    fn do_oam_search(&mut self, bus: &mut Bus) -> i32 {
        // TODO find how many are in line

        let ly: u8 = self.get_line(bus);
        let scy: u8 = bus.get_byte(SCY);
        let lcdc: u8 = bus.get_byte(LCDC);
        let double_size = lcdc >> 2 & 0x1 == 1;

        self.oam_entries = OamEntry::get_oam_line(bus, ly.wrapping_add(scy), double_size);

        self.set_video_mode(VideoMode::PixelTransfer, bus);
        0
    }

    fn do_pixel_transfer(&mut self, bus: &mut Bus) -> i32 {
        self.writeline(bus);

        // mode0 interrupt enabled
        if bus.get_byte(STAT) >> 3 & 0x1 == 1 {
            bus.set_byte(IF, bus.get_byte(IF) | 0b10);
        }

        self.set_video_mode(VideoMode::HBLANK, bus);
        0
    }

    fn writeline(&mut self, bus: &mut Bus) {
        let ly: u8 = bus.get_byte(LY);
        let scx: u8 = bus.get_byte(SCX);
        let scy: u8 = bus.get_byte(SCY);
        let lcdc: u8 = bus.get_byte(LCDC);
        let wx: u8 = bus.get_byte(WX);
        let wy: u8 = bus.get_byte(WY);
        let bgp: u8 = bus.get_byte(BGP);
        let obj1: u8 = bus.get_byte(0xFF48);
        let obj2: u8 = bus.get_byte(0xFF49);

        let win_tile_map: bool = lcdc >> 6 & 0x1 == 1;
        let win_enabled: bool = lcdc >> 5 & 0x1 == 1;
        let tile_data: bool = lcdc >> 4 & 0x1 == 1;
        let bg_tile_map: bool = lcdc >> 3 & 0x1 == 1;
        let obj_size: bool = lcdc >> 2 & 0x1 == 1;
        let obj_enabled: bool = lcdc >> 1 & 0x1 == 1;
        let obj_priority: bool = lcdc & 0x1 == 1;

        let win_tile_map_address: u16 = if win_tile_map { 0x9C00 } else { 0x9800 };
        let bg_tile_map_address: u16 = if bg_tile_map { 0x9C00 } else { 0x9800 };

        let bgp_palette: [u8; 4] = {
            let mut palette: [u8; 4] = [0; 4];
            for i in 0..4 {
                palette[i] = bgp >> i * 2 & 0b11;
            }
            palette
        };
        let obj1_palette: [u8; 4] = {
            let mut palette: [u8; 4] = [0; 4];
            for i in 0..4 {
                palette[i] = obj1 >> i * 2 & 0b11;
            }
            palette
        };
        let obj2_palette: [u8; 4] = {
            let mut palette: [u8; 4] = [0; 4];
            for i in 0..4 {
                palette[i] = obj2 >> i * 2 & 0b11;
            }
            palette
        };

        let bg_line: [u8; 160] = {
            let y: u8 = ((ly.wrapping_sub(scy)) / 8);
            let y_offset = (ly.wrapping_sub(scy)) % 8;
            let tiles = self.get_tiles(bus, y, bg_tile_map_address);
            self.render_tiles(bus, tiles, y_offset, tile_data, bgp_palette)
        };

        let win_line: [u8; 160] = {
            if !win_enabled { [0; 160] } else {
                let y: u8 = ((ly.wrapping_sub(wy)) / 8);
                let y_offset = (ly.wrapping_sub(wy)) % 8;
                let tiles = self.get_tiles(bus, y, win_tile_map_address);
                self.render_tiles(bus, tiles, y_offset, tile_data, bgp_palette)
            }
        };

        let obj_line: [u8; 160] = {
            let mut res: [u8; 160] = [0; 160];
            if !obj_enabled { (res) } else {
                let y: u8 = ly.wrapping_sub(scy);
                for entry in &self.oam_entries {
                    let l: u8 = entry.ypos - 0x10 - ly;
                    if entry.obj16 {
                        let render = entry.render16;
                        for j in 0..8 {
                            let x = (entry.xpos - 0x08 + j) as usize;
                            let u8 = render[(l * 8 + j) as usize] as usize;
                            res[x] = if entry.palette {
                                obj2_palette[u8]
                            } else {
                                obj1_palette[u8]
                            };
                        }
                    }
                }
                res
            }
        };
        self.oam_entries.clear();

        let line: [u8; 160] = {
            let mut v: [u8; 160] = bg_line.clone();
            for i in 0..160 {
                if win_enabled && ly >= wy {
                    v[i] = win_line[i];
                } else if obj_line[i] != 0 {
                    v[i] = obj_line[i];
                }
            }

            v
        };


        for i in 0..160_u16 {
            let idx: u16 = (ly as u16 * 160) + i;
            self.image[idx as usize] = line[i as usize];
        }
    }

    fn get_tiles(&self, bus: &Bus, y: u8, map_address: u16) -> [u8; 20] {
        let wx: u8 = bus.get_byte(WX);
        let mut tiles: [u8; 20] = [0; 20];
        for i in 0..20_u16 {
            let off: u16 = if (wx as u16 / 8) + i > 32 { (wx as u16 / 8) + i - 32 } else { (wx as u16 / 8) + i };
            tiles[i as usize] = bus.get_byte(map_address.wrapping_add(y as u16 * 32).wrapping_add(off));
        };
        tiles
    }

    fn render_tiles(&self, bus: &Bus, tiles: [u8; 20], y_offset: u8, tile_data: bool, palette: [u8; 4]) -> [u8; 160] {
        // every tile is 16 bytes
        let mut pixels: [u8; 160] = [0; 160];
        for (i, b) in tiles.iter().enumerate() {
            let address = {
                if tile_data {
                    0x8000_u16.wrapping_add((*b as u16 * 16) as u16).wrapping_add((y_offset * 2) as u16)
                } else {
                    if *b <= 127 {
                        0x9000_u16.wrapping_add(*b as u16 * 16).wrapping_add((y_offset * 2) as u16)
                    } else {
                        0x8800_u16.wrapping_add((*b - 128) as u16 * 16).wrapping_add((y_offset * 2) as u16)
                    }
                }
            };
            let top_byte: u8 = bus.get_byte(address);
            let low_byte: u8 = bus.get_byte(address + 1u16);
            for j in (0..8).rev() {
                pixels[i * 8 + 7 - j] = palette[((top_byte >> j & 0x1) << 1 | (low_byte >> j & 0x1)) as usize];
            };
        };
        pixels
    }


    fn get_window_for_line(&self, bus: &Bus, line: u8) -> [u8; 160] {
        let lcdc: u8 = bus.get_byte(LCDC);
        let wy: u8 = bus.get_byte(WY);
        let wx: u8 = bus.get_byte(WX);

        let bg_tile_data: bool = lcdc >> 4 & 0x1 == 1;
        let win_tile_map_address: u16 = if lcdc >> 5 & 0x1 == 0x1 { 0x9C00 } else { 0x9800 };

        let bgp: u8 = bus.get_byte(BGP);
        let mut palette: [u8; 4] = [0; 4];
        for i in 0..4 {
            palette[i] = bgp >> i * 2 & 0b11;
        }


        let x: u16 = ((line.wrapping_sub(wy)) / 8) as u16;
        let mut tiles: [u8; 20] = [0; 20];
        for i in 0..20 {
            let off: u8 = if (wx / 8) + i > 32 { (wx / 8) + i - 32 } else { (wx / 8) + i };
            tiles[i as usize] = bus.get_byte(win_tile_map_address + (x * 32) + off as u16);
        };

        // every tile is 16 bytes
        let x_offset = (line.wrapping_sub(wy)) % 8;
        let mut pixels: [u8; 160] = [0; 160];
        for (i, b) in tiles.iter().enumerate() {
            let x = {
                if bg_tile_data {
                    0x8000_u16.wrapping_add((*b as u16 * 16) as u16).wrapping_add((x_offset * 2) as u16)
                } else {
                    if *b <= 127 {
                        0x9000_u16.wrapping_add(*b as u16 * 16).wrapping_add((x_offset * 2) as u16)
                    } else {
                        0x8800_u16.wrapping_add((*b - 128) as u16 * 16).wrapping_add((x_offset * 2) as u16)
                    }
                }
            };
            let result = self.combine_byte(bus.get_byte(x), bus.get_byte(x + 1u16), &palette);
            for j in 0..result.len() {
                pixels[i * 8 + j] = result[j];
            }
        };
        pixels
    }

    fn get_background_for_line(&self, bus: &Bus, line: u8) -> [u8; 160] {
        let lcdc: u8 = bus.get_byte(LCDC);
        let scx: u8 = bus.get_byte(SCX);
        let bgp: u8 = bus.get_byte(BGP);

        let bg_tile_data: bool = lcdc >> 4 & 0x1 == 1;
        let bg_tile_option: bool = lcdc >> 3 & 0x1 == 1;

        let mut palette: [u8; 4] = [0; 4];
        for i in 0..4 {
            palette[i] = bgp >> i * 2 & 0b11;
        }

        let x: u16 = (line / 8) as u16;

        let mut tiles: [u8; 20] = [0; 20];
        let starting_bg_tilemap: u16 = if bg_tile_option { 0x9C00 } else { 0x9800 };
        for i in 0..20 {
            let off: u8 = if (scx / 8) + i > 32 { (scx / 8) + i - 32 } else { (scx / 8) + i };
            tiles[i as usize] = bus.get_byte(starting_bg_tilemap + (x * 32) + off as u16);
        };

        // every tile is 16 bytes
        let x_offset = line % 8;
        let mut pixels: [u8; 160] = [0; 160];
        for (i, b) in tiles.iter().enumerate() {
            let x = {
                if bg_tile_data {
                    0x8000_u16.wrapping_add((*b as u16 * 16) as u16).wrapping_add((x_offset * 2) as u16)
                } else {
                    if *b <= 127 {
                        0x9000_u16.wrapping_add(*b as u16 * 16).wrapping_add((x_offset * 2) as u16)
                    } else {
                        0x8800_u16.wrapping_add((*b - 128) as u16 * 16).wrapping_add((x_offset * 2) as u16)
                    }
                }
            };
            let result = self.combine_byte(bus.get_byte(x), bus.get_byte(x + 1u16), &palette);
            for j in 0..result.len() {
                pixels[i * 8 + j] = result[j];
            }
        };
        pixels
    }

    fn get_sprites(&mut self, bus: &Bus, ly: u8) -> [u8; 160] {
        let lcdc = bus.get_byte(LCDC);
        if lcdc >> 1 & 0x1 == 0 {
            return [0; 160];
        }

        let obj1: u8 = bus.get_byte(0xFF48);
        let mut palette_1: [u8; 4] = [0; 4];
        for i in 0..4 {
            palette_1[i] = obj1 >> i * 2 & 0b11;
        }
        let obj2: u8 = bus.get_byte(0xFF49);
        let mut palette_2: [u8; 4] = [0; 4];
        for i in 0..4 {
            palette_2[i] = obj2 >> i * 2 & 0b11;
        }

        let mut res = [0; 160];
        for i in 0..160 {
            for entry in &self.oam_entries {
                if entry.xpos < i + 0x8 && entry.xpos >= i {
                    if entry.obj16 {
                        let render: [u8; 128] = entry.render16;

                        let l: u8 = ly - entry.ypos;

                        println!("L {:?} YPOS {:?} LY: {:?}", l, entry.ypos, ly);
                        for j in 0..8 {
                            let x = (entry.xpos - 8 + j) as usize;
                            let u8 = render[(l * 8 + j) as usize] as usize;
                            res[x] = if entry.palette {
                                palette_2[u8]
                            } else {
                                palette_1[u8]
                            };
                        }
                    } else {
                        let render: [u8; 64] = entry.render8;
                        let l: u8 = entry.ypos - ly;

                        for j in 0..8 {
                            let x = (entry.xpos - 8 + j) as usize;
                            let r = (l * 8 + j) as usize;
                            let u8 = render[r] as usize;
                            res[x] = if entry.palette {
                                palette_2[u8]
                            } else {
                                palette_1[u8]
                            };
                        }
                    }
                }
            }
        }
        self.oam_entries.clear();
        res
    }

    #[inline]
    fn combine_byte(&self, low: u8, top: u8, palette: &[u8; 4]) -> [u8; 8] {
        let mut result: [u8; 8] = [0; 8];
        for i in (0..8).rev() {
            result[7 - i] = palette[((top >> i & 0x1) << 1 | (low >> i & 0x1)) as usize];
        };
        result
    }

    fn do_vblank(&mut self, bus: &mut Bus) -> i32 {
        let line: u8 = self.get_line(bus);
        bus.set_byte(IF, bus.get_byte(IF) | 1);
        bus.io_registers.ly = line + 1;
        if line == 154 {
            bus.io_registers.ly = 0;
            self.set_video_mode(VideoMode::OamSearch, bus);
            self.write_image(bus);
        }
        0
    }

    fn do_hblank(&self, bus: &mut Bus) -> i32 {
        let line = self.get_line(bus);
        if line == 143 {
            // last drawing line
            // mode1 interrupt enabled
            if bus.get_byte(STAT) >> 4 & 0x1 == 1 {
                bus.set_byte(IF, bus.get_byte(IF) | 0b10);
            }
            bus.set_byte(IF, bus.get_byte(IF) | 0b1);
            self.set_video_mode(VideoMode::VBLANK, bus);
        } else {
            // next line
            // mode2 interrupt enabled
            if bus.get_byte(STAT) >> 5 & 0x1 == 1 {
                bus.set_byte(IF, bus.get_byte(IF) | 0b10);
            }
            self.set_video_mode(VideoMode::OamSearch, bus);
        }
        bus.io_registers.ly = line + 1;
        0
    }


    // move to BUS
    fn get_video_mode(&self, bus: &mut Bus) -> VideoMode {
        match bus.get_byte(STAT) & 0x3 {
            0 => VideoMode::HBLANK,
            1 => VideoMode::VBLANK,
            2 => VideoMode::OamSearch,
            3 => VideoMode::PixelTransfer,
            _ => panic!("")
        }
    }

    fn get_line(&self, bus: &Bus) -> u8 {
        bus.get_byte(LY)
    }

    fn set_video_mode(&self, video_mode: VideoMode, bus: &mut Bus) {
        bus.io_registers.stat = (bus.get_byte(STAT) & 0b11111100) | video_mode as u8;
    }

    pub fn lcdc_on(bus: &Bus) -> bool {
        bus.get_byte(LCDC) >> 7 & 0x1 == 1
    }
}

