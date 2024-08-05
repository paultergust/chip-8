mod cpu;
mod frames;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory();

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
