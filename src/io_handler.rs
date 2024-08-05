use crate::cpu::CPU;
use minifb::{Key, Window, WindowOptions};

// fn draw(index: u8) {
//    handle_input(&cpu, &window)
//    if cpu.draw_flag {
//        cpu.draw_flag = false;
//    }
//}

fn handle_input(chip8: &mut CPU, window: &Window) {
    chip8.keys.iter_mut().for_each(|k| *k = false);

    for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
        if let Some(index) = get_key_index(key) {
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

