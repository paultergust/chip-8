struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
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
                        0 => { self.ld(x, self.registers[y as usize])},
                        1 => { self.or_xy(x, y) },
                        2 => { self.and_xy(x, y) },
                        3 => { self.xor_xy(x, y) },
                        4 => { self.add_xy(x, y); },
                        5 => { self.sub_xy(x, y); },
                        _ => { todo!("Opcode: {:04x}", opcode); },
                    }
                },
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

    // (3xkk) | (5xy) skip instruction if x equals y or kk
    fn se(&mut self, x: u8, kk: u8) {
        if x == kk {
            self.pc += 2;
        }
    }

    // (4xkk) skip if x NOT equal to kk
    fn sne(&mut self, x: u8, kk: u8) {
        if x != kk {
            self.pc += 2;
        }
    }
    
    // (6xkk) loads kk into register x
    fn ld(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    // (7xkk) Adds kk to value stored in register x
    fn add(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] += kk;
    }

    // (8xy1) bitwise x or y and store in x
    fn or_xy(&mut self, x:u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];

        self.registers[x as usize] = _x | _x;
    }

    // (8xy2) bitwise x and y and store in x
    fn and_xy(&mut self, x:u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];

        self.registers[x as usize] = _x & _x;
    }

    // (8xy3) bitwise x xor y and store in x
    fn xor_xy(&mut self, x:u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];

        self.registers[x as usize] = _x ^ _x;
    }

    // (8xy4) add y to x and store in x with overflow
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    // (8xy5) sub y from x and store in x and set VF to 0 if underflow
    fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xf] = 0;
        } else {
            self.registers[0xf] = 1;
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
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 0x1000],
        pc: 0,
        stack: [0;16],
        stack_pointer: 0,
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
