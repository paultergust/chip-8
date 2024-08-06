use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;

mod cpu;
mod io_handler;
use crate::cpu::CPU;

#[derive(Parser)]
#[command(name = "chip8")]
#[command(about = "A simple CHIP-8 emulator", long_about = None)]
struct Cli {
    filepath: String,
}

fn main() {
    let args = Cli::parse();
    let filepath = args.filepath;
    let mut cpu = CPU::new();
    let mut program = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(hex_str) = line {
                // Parse each line as a 16-bit hexadecimal number
                if let Ok(opcode) = u16::from_str_radix(&hex_str.trim(), 16) {
                    program.push((opcode >> 8) as u8);
                    program.push((opcode & 0xFF) as u8);
                }
            }
        }
    }
    cpu.load_program(program);
    cpu.run();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
