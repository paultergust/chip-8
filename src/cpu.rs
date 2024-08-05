use rand::Rng;

const SCREEN_HEIGHT: u8 = 32;
const SCREEN_WIDTH: u8 = 64;
const BUFFER_SIZE: usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize;

pub struct CPU {
    pub registers: [u8; 16],
    pub keys: [bool; 16],
    pc: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
    index_register: u16,
    gfx: [u8; BUFFER_SIZE],
    draw_flag: bool,
    dt: u8, // delaytime
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 0x1000],
            pc: 0,
            stack: [0;16],
            stack_pointer: 0,
            index_register: 0,
            gfx: [0; BUFFER_SIZE],
            draw_flag: false,
            keys: [false; 16],
            dt: 0,
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
        loop {
            let opcode = self.read_opcode();
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
                0x00E0 => { /* clear screen */ }
                0x00EE => { self.ret(); }
                0x1000..=0x1FFF => { self.jmp(addr); },
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FFF => { self.se(x, y); },
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
                0xF015..=0xFF15 => { self.set_dt(x); },
                _ => { todo!("Opcode: {:04x}", opcode); },
            }
        }
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
        self.pc = addr as usize;
    }

    // (3xkk) | (5xy) skip instruction if vx equals vy or kk
    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.pc += 2;
        }
    }

    // (4xkk) skip if vx NOT equal to kk
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
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
        self.index_register = addr;
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
        let vx = self.get_register(x);
        let vy = self.get_register(y);

        // Wrap coordinates around the screen dimensions
        let vx = vx % SCREEN_WIDTH;
        let vy = vy % SCREEN_HEIGHT;

        // Set VF to 0 before drawing
        self.registers[0xF] = 0;

        for byte in 0..height {
            let y_coord = (vy + byte) % SCREEN_HEIGHT;
            let sprite_line = self.memory[(self.index_register + byte as u16) as usize];

            for bit in 0..8 {
                let x_coord = (vx + bit) % SCREEN_WIDTH;
                let pixel = (sprite_line >> (7 - bit)) & 1;

                let idx = y_coord * SCREEN_WIDTH + x_coord;
                let screen_pixel = self.gfx[idx as usize];

                // Collision detection
                if screen_pixel == 1 && pixel == 1 {
                    self.registers[0xF] = 1;
                }

                // XOR operation to draw the pixel
                self.gfx[idx as usize] ^= pixel;
            }
        }

        // Set the draw flag to true to update the display
        self.draw_flag = true;
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
        self.registers[vx as usize] = self.dt;
    }

    // Fx15 loads the value of Vx into dt
    fn set_dt(&mut self, vx: u8) {
        self.dt = self.get_register(vx);
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

    pub fn memory(self) -> [u8; 0x1000] {
        self.memory
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

