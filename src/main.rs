use pixel_engine::traits::ScreenTrait;

use gamerust::gameboy::Gameboy;
use pixel_engine::vector2::Vu2d;
use pixel_engine::inputs::Keycodes;
use pixel_engine::Color;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut gb: Gameboy = Gameboy::load_rom(args);

    let game = pixel_engine::EngineWrapper::new("gamerust".to_owned(), (800, 300, 4));

    game.run(move |game: &mut pixel_engine::Engine| {
        game.clear([0, 64, 255].into());

        handle_input(game, &mut gb);

        while !gb.ppu.ready {
            gb.tick();
        }

        if gb.bus.get_byte(0xFF40) >> 7 & 1 == 1 {
            for (i, c) in gb.ppu.get_image().iter().enumerate() {
                game.draw(Vu2d::from(((i % 160 + 5) as u32, (i / 160 + 5) as u32)), get_color(*c));
            }
        } else {
            for (i, _c) in gb.ppu.get_image().iter().enumerate() {
                game.draw(Vu2d::from(((i % 160 + 5) as u32, (i / 160 + 5) as u32)), [0, 0, 0].into());
            }
        }

        for (i, c) in gb.ppu.bg.iter().enumerate() {
            game.draw(Vu2d::from(((i % 256 + 180) as u32, (i / 256 + 5) as u32)), get_color(*c));
        }

        for (i, c) in gb.ppu.bg2.iter().enumerate() {
            game.draw(Vu2d::from(((i % 256 + 450) as u32, (i / 256 + 5) as u32)), get_color(*c));
        }


        Ok(true)
    });
}

fn get_color(i: u8) -> Color {
    match i {
        0 => [155, 188, 15].into(),
        1 => [132, 172, 15].into(),
        2 => [48, 98, 48].into(),
        3 => [15, 56, 15].into(),
        _ => panic!("as")
    }
}

#[inline]
fn handle_input(game: &mut pixel_engine::Engine, gb: &mut Gameboy) {
    if game.get_key(Keycodes::A).released {
        gb.bus.io_registers.p1.a = false;
        println!("[A RELEASED]")
    }
    if game.get_key(Keycodes::A).pressed {
        gb.bus.io_registers.p1.a = true;
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        println!("[A HELD]")
    }
    if game.get_key(Keycodes::S).released {
        gb.bus.io_registers.p1.b = false;
        println!("[B RELEASED]")
    }
    if game.get_key(Keycodes::S).pressed {
        gb.bus.io_registers.p1.b = true;
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        println!("[B HELD]")
    }
    if game.get_key(Keycodes::LShift).released {
        gb.bus.io_registers.p1.select = false;
        println!("[SELECT RELEASED]")
    }
    if game.get_key(Keycodes::LShift).pressed {
        gb.bus.io_registers.p1.select = true;
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        println!("[SELECT HELD]")
    }
    if game.get_key(Keycodes::LControl).released {
        gb.bus.io_registers.p1.start = false;
        println!("[START RELEASED]")
    }
    if game.get_key(Keycodes::LControl).pressed {
        gb.bus.io_registers.p1.start = true;
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        println!("[START HELD]")
    }
    if game.get_key(Keycodes::Up).released {
        gb.bus.io_registers.p1.up = false;
        println!("[UP RELEASED]")
    }
    if game.get_key(Keycodes::Up).pressed {
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        gb.bus.io_registers.p1.up = true;
        println!("[UP HELD]")
    }
    if game.get_key(Keycodes::Left).released {
        gb.bus.io_registers.p1.left = false;
        println!("[LEFT RELEASED]")
    }
    if game.get_key(Keycodes::Left).pressed {
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        gb.bus.io_registers.p1.left = true;
        println!("[LEFT HELD]")
    }
    if game.get_key(Keycodes::Right).released {
        gb.bus.io_registers.p1.right = false;
        println!("[RIGHT RELEASED]")
    }
    if game.get_key(Keycodes::Right).pressed {
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        gb.bus.io_registers.p1.right = true;
        println!("[RIGHT HELD]")
    }
    if game.get_key(Keycodes::Down).released {
        gb.bus.io_registers.p1.down = false;
        println!("[DOWN RELEASED]")
    }
    if game.get_key(Keycodes::Down).pressed {
        gb.bus.set_byte(0xFF0F, gb.bus.get_byte(0xFF0F) | 0b10000);
        gb.bus.io_registers.p1.down = true;
        println!("[DOWN HELD]")
    }
}
