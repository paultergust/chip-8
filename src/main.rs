use rand::Rng;

struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
    index_register: u16,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn get_register(&self, location: u8) -> u8{
        self.registers[location as usize]
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            // let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            // let d = ((opcode & 0x000F) >> 0) as u8;

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

    // (0000) returns and decrements stack pointer
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.pc = call_addr as usize;
    }
}

fn rand_u8() -> u8 {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    r
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 0x1000],
        pc: 0,
        stack: [0;16],
        stack_pointer: 0,
        index_register: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("{}", cpu.registers[0]);
}
