use crate::cpu::CPU;
use minifb::Key;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct IOHandler;
impl IOHandler {
    pub fn draw(gfx: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT]) -> Vec<u32> {
        let mut buffer = vec![0; gfx.len()];
        for (i, &pixel) in gfx.iter().enumerate() {
            let color = if pixel == 1 { 0xFFFFFF } else { 0x000000 };
            buffer[i] = color;
        }
        buffer
    }

fn draw_sprite(buffer: &mut Vec<u32>, x: usize, y: usize, sprite: &[u8]) {
    for (row, &byte) in sprite.iter().enumerate() {
        for bit in 0..8 {
            if (byte & (0x80 >> bit)) != 0 {
                let pixel_x = (x + bit) % SCREEN_WIDTH;
                let pixel_y = (y + row) % SCREEN_HEIGHT;
                let index = pixel_y * SCREEN_WIDTH + pixel_x;
                buffer[index] = 0xFFFFFF; // White pixel
            }
        }
    }
}

fn draw_sprite(buffer: &mut Vec<u32>, x: usize, y: usize, sprite: &[u8]) {
    for (row, &byte) in sprite.iter().enumerate() {
        for bit in 0..8 {
            if (byte & (0x80 >> bit)) != 0 {
                let pixel_x = (x + bit) % SCREEN_WIDTH;
                let pixel_y = (y + row) % SCREEN_HEIGHT;
                let index = pixel_y * SCREEN_WIDTH + pixel_x;
                buffer[index] = 0xFFFFFF; // White pixel
            }
        }
    }
}

fn handle_input(chip8: &mut CPU, window: &Window) {
    chip8.keys.iter_mut().for_each(|k| *k = false);

        for key in chip8.window.get_keys_pressed(minifb::KeyRepeat::No) {
            if let Some(index) = IOHandler::get_key_index(key) {
                chip8.set_key(index, true);
            }
        }
    }

    fn get_key_index(key: Key) -> Option<usize> {
        match key {
            Key::X => Some(0x0), Key::Key1 => Some(0x1), Key::Key2 => Some(0x2), Key::Key3 => Some(0x3),
            Key::Q => Some(0x4), Key::W => Some(0x5), Key::E => Some(0x6), Key::A => Some(0x7),
            Key::S => Some(0x8), Key::D => Some(0x9), Key::Z => Some(0xA), Key::C => Some(0xB),
            Key::Key4 => Some(0xC), Key::R => Some(0xD), Key::F => Some(0xE), Key::V => Some(0xF),
            _ => None,
        }
    }
}
