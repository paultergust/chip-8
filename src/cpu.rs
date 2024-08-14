use rand::Rng;
use minifb::{Key, Window, WindowOptions};

use crate::io_handler::IOHandler;

const SCREEN_HEIGHT: u8 = 32;
const SCREEN_WIDTH: u8 = 64;
const BUFFER_SIZE: usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize;
const SCALE: usize = 10;
const FPS: usize = 120;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct CPU {
    pub registers: [u8; 16],
    pub keys: [bool; 16],
    pub window: Window,
    pc: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
    memory_pointer: u16,
    gfx: [u8; BUFFER_SIZE],
    delay_timer: u8, // delaytime
    sound_timer: u8, // sound timer
}

impl CPU {
    pub fn new() -> CPU {
        let window = Window::new(
            "CHIP-8",
            SCREEN_WIDTH as usize * SCALE,
            SCREEN_HEIGHT as usize * SCALE,
            WindowOptions::default(),
        ).unwrap();
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 0x1000],
            pc: 0x200,
            stack: [0;16],
            stack_pointer: 0,
            memory_pointer: 0,
            gfx: [0; BUFFER_SIZE],
            keys: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
            window,
        };
        cpu.load_fontset();
        cpu
    }

    fn load_fontset(&mut self) {
        let begin = 0x50;
        for i in 0..FONTSET.len() {
            self.memory[begin + i] = FONTSET[i];
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn get_register(&self, location: u8) -> u8{
        self.registers[location as usize]
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let opcode = self.read_opcode();
            println!("{:4X}", opcode);
            self.pc += 2;

            // let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let kk = (opcode & 0x0FF) as u8;
            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            match opcode {
                0x0000 => { return; }
                0x00E0 => { self.clear_screen(); }
                0x00EE => { self.ret(); }
                0x1000..=0x1FFF => { self.jmp(addr); },
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FF0 => { self.sev(x, y); },
                0x6000..=0x6FFF => { self.ld(x, kk); },
                0x7000..=0x7FFF => { self.add(x, kk); },
                0x8000..=0x8FFF => {
                    match op_minor {
                        0x0 => { self.ld(x, self.get_register(y))},
                        0x1 => { self.or_xy(x, y) },
                        0x2 => { self.and_xy(x, y) },
                        0x3 => { self.xor_xy(x, y) },
                        0x4 => { self.add_xy(x, y); },
                        0x5 => { self.sub_xy(x, y); },
                        0x6 => { self.shift_right(x); },
                        0x7 => { self.sub_y_from_x(x, y); },
                        0xE => { self.shift_left(x); },
                        _ => { todo!("Opcode: {:04x}", opcode); },
                    }
                },
                0x9000..=0x9FFF => { self.sne(x, y); },
                0xA000..=0xAFFF => { self.set_index(addr); },
                0xB000..=0xBFFF => { self.jmp_plus_register(addr); },
                0xC000..=0xCFFF => { self.rand(x, kk); },
                0xD000..=0xDFFF => { self.draw_sprite(x, y, d); },
                0xE09E..=0xEF9E => { self.skp(x); },
                0xE0A1..=0xEFA1 => { self.sknp(x); },
                0xF007..=0xFF07 => { self.load_td(x); },
                0xF00A..=0xFF0A => { self.await_keypress(x); },
                0xF015..=0xFF15 => { self.set_dt(x); },
                0xF018..=0xFF18 => { self.set_st(x); },
                0xF01E..=0xFF1E => { self.add_vx_to_index(x); },
                0xF029..=0xFF29 => { self.index_digit(x); },
                0xF033..=0xFF33 => { self.bcd_to_i(x); },
                0xF055..=0xFF55 => { self.load_registers(x); },
                0xF065..=0xFF65 => { self.read_registers(x); },
                _ => { todo!("Opcode: {:04x}", opcode); },
            }
            IOHandler::handle_input(self);
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[self.pc + i] = *byte;
        }
    }

    // 00E0 CLS clear screen
    fn clear_screen(&mut self) {
        self.gfx.iter_mut().for_each(|pixel| *pixel = 0);
    }

    // (1nnn) jump to addr nnn
    fn jmp(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    // (2nnn) call function at addr nnn
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.pc as u16;
        self.stack_pointer += 1;
        stack[sp + 1] = addr;
        self.jmp(addr);
    }

    // (3xkk) | skip instruction if vx equals kk
    fn se(&mut self, vx: u8, kk: u8) {
        if self.get_register(vx) == kk {
            self.pc += 2;
        }
    }

    // (4xkk) skip if vx NOT equal to kk
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.pc += 2;
        }
    }

    // (5xy) skip instruction if vx equals vy
    fn sev(&mut self, vx: u8, kk: u8) {
        if self.get_register(vx) == self.get_register(kk) {
            self.pc += 2;
        }
    }
    
    // (6xkk) loads kk into register vx
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    // (7xkk) Adds kk to value stored in register vx
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    // (8xy1) bitwise vx or vy and store in vx
    fn or_xy(&mut self, vx: u8, vy: u8) {
        let x = self.get_register(vx);
        let y = self.get_register(vy);

        self.registers[vx as usize] = x | y;
    }

    // (8xy2) bitwise vx and vy and store in vx
    fn and_xy(&mut self, vx: u8, vy: u8) {
        let x = self.get_register(vx);
        let y = self.get_register(vy);

        self.registers[vx as usize] = x & y;
    }

    // (8xy3) bitwise vx xor vy and store in vx
    fn xor_xy(&mut self, vx: u8, vy: u8) {
        let x = self.get_register(vx);
        let y = self.get_register(vy);

        self.registers[vx as usize] = x ^ y;
    }

    // (8xy4) add vy to vx and store in vx with overflow
    fn add_xy(&mut self, vx: u8, vy: u8) {
        let arg1 = self.get_register(vx);
        let arg2 = self.get_register(vy);

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[vx as usize] = val;

        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    // (8xy5) sub y from vx and store in vx and set VF to 0 if underflow
    fn sub_xy(&mut self, vx: u8, vy: u8) {
        let arg1 = self.get_register(vx);
        let arg2 = self.get_register(vy);

        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[vx as usize] = val;

        if overflow {
            self.registers[0xf] = 0;
        } else {
            self.registers[0xf] = 1;
        }
    }

    // (8xy6) bitshift VX to the right and store previous least significant bit in VF
    fn shift_right(&mut self, vx: u8) {
        let x = self.get_register(vx);
        let previous_lsb = x & 0b00000001;

        self.registers[vx as usize] = x >> 1;
        self.registers[0xf] = previous_lsb;
    }

    // (8xy7) bitshift VX to the right and store previous least significant bit in VF
    fn sub_y_from_x(&mut self, vx: u8, vy: u8) {
        let x = self.get_register(vx);
        let y = self.get_register(vy);
        let (val, overflow) = y.overflowing_sub(x);
        self.registers[vx as usize] = val;

        if overflow {
            self.registers[0xf] = 0;
        } else {
            self.registers[0xf] = 1;
        }
    }

    // (8xyE) bitshift VX to the left and store previous most significant bit in VF
    fn shift_left(&mut self, vx: u8) {
        let x = self.get_register(vx);
        let previous_msb = x & 0b10000000;

        self.registers[vx as usize] = x << 1;
        self.registers[0xf] = previous_msb;
    }

    // (Annn) set index register to addr nnn
    fn set_index(&mut self, addr: u16) {
        self.memory_pointer = addr;
    }

    // (Bnnn) jmp pc to v0 + addr
    fn jmp_plus_register(&mut self, addr: u16) {
        self.pc = ((self.registers[0] as u16) + addr) as usize;
    }

    // (Cxnn) bitwise AND between x and nn
    fn rand(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = rand_u8() & kk;
    }

    // (Dxyn) Draw a N sized sprite (I) to point (vx, vy)
    fn draw_sprite(&mut self, x: u8, y: u8, height: u8) {
        let h = SCREEN_HEIGHT as usize;
        let w = SCREEN_WIDTH as usize;
        // Retrieve the coordinates from the registers
        let vx = self.get_register(x) as usize;
        let vy = self.get_register(y) as usize;

        // Wrap coordinates around the screen dimensions
        let vx = vx % w;
        let vy = vy % h;

        // Set VF to 0 before drawing
        self.registers[0xF] = 0;

        for byte in 0..height {
            // Calculate the y coordinate for the current row of the sprite
            let y_coord = (vy + byte as usize) % h;

            // Retrieve the current line of the sprite
            let sprite_line = self.memory[(self.memory_pointer + byte as u16) as usize];

            for bit in 0..8 {
                // Calculate the x coordinate for the current pixel
                let x_coord = (vx + bit as usize) % w;
                let pixel = (sprite_line >> (7 - bit)) & 1;

                // Determine the index in the framebuffer
                let idx = y_coord * w + x_coord;
                let screen_pixel = self.gfx[idx as usize];

                // Collision detection
                if screen_pixel == 1 && pixel == 1 {
                    self.registers[0xF] = 1; // Set VF to 1 if there is a collision
                }

                // XOR operation to draw the pixel (flip the pixel)
                self.gfx[idx as usize] ^= pixel;
            }
        }
        let buffer = IOHandler::draw(&self.gfx);
        self.window.update_with_buffer(&buffer, SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()).unwrap();
    }

    // Ex9E skip next instruction if key is pressed
    fn skp(&mut self, vx: u8) {
        let x = self.get_register(vx);
        if self.keys[x as usize] {
            self.pc += 2;
        }
    }

    // ExA1 skip next instruction if key is NOT pressed
    fn sknp(&mut self, vx: u8) {
        let x = self.get_register(vx);
        if !self.keys[x as usize] {
            self.pc += 2;
        }
    }

    // Fx07 loads the dt value into Vx
    fn load_td(&mut self, vx: u8) {
        self.registers[vx as usize] = self.delay_timer;
    }

    // Fx0A wait for keypress
    fn await_keypress(&mut self, vx: u8) {
        // check each key
        for i in 0..self.keys.len() {
            if self.keys[i] {
                self.registers[vx as usize] = i as u8;
                return;
            }
        }
        // if none is pressed, jmp to previous instruction (recursion)
        self.pc -= 2;
    }

    // Fx15 loads the value of Vx into dt
    fn set_dt(&mut self, vx: u8) {
        self.delay_timer = self.get_register(vx);
    }

    // Fx18 loads the value of Vx into st
    fn set_st(&mut self, vx: u8) {
        self.sound_timer = self.get_register(vx);
    }

    // Fx1E add I to Vx and store in I
    fn add_vx_to_index(&mut self, vx: u8) {
        self.memory_pointer += self.get_register(vx) as u16;
    }

    // Fx29 set I to adress of digit sprit at Vx
    fn index_digit(&mut self, vx: u8) {
        self.memory_pointer = (self.memory[vx as usize] as u16) * 5 + 0x50;
    }

    // Fx33 Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn bcd_to_i(&mut self, vx: u8) {
        let value = self.get_register(vx);
        self.memory[self.memory_pointer as usize] = value / 100;
        self.memory[self.memory_pointer as usize + 1] = (value / 10) % 10;
        self.memory[self.memory_pointer as usize + 2] = value % 10;
    }

    // Fx55 read V0 to Vx and store into I..I+x
    fn load_registers(&mut self, vx:u8) {
        let index_start:usize = self.memory_pointer.into();
        for i in 0..vx {
            self.memory[index_start + i as usize] = self.get_register(i);
        }
    }

    // Fx65 Store into V0 to Vx values from I..I+x
    fn read_registers(&mut self, vx:u8) {
        let index_start:usize = self.memory_pointer.into();
        for i in 0..vx {
            self.registers[i as usize] = self.memory[index_start + i as usize];
        }
    }

    // (0000) returns and decrements stack pointer
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.pc = call_addr as usize;
    }


    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }
}

fn rand_u8() -> u8 {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    r
}

