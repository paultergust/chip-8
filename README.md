# CPU Emulator

## Introduction

[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) is an interpreted programming language developed in the mid-1970s to simplify game development on the COSMAC VIP and other early microcomputers.

## Architecture
* Memory: 4KB (4096 bytes), with the first 512 bytes (0x000 to 0x1FF) reserved for the interpreter.
* Registers: 16 general-purpose 8-bit registers (V0 to VF) and a 16-bit register (I) for memory addresses.
* Stack: Used to store return addresses when subroutines are called, typically up to 16 levels deep.
* Timers: Two 8-bit timers, a delay timer, and a sound timer, which decrement at 60Hz.
* Display: 64x32 monochrome pixel display, updated through draw instructions.
* Input: 16-key hexadecimal keypad (0-9, A-F).
* Instruction Set: CHIP-8 has 35 opcodes, each 2 bytes long. These instructions cover basic operations such as arithmetic, control flow, drawing graphics, and interacting with the keypad.

## Emulator Features

* Complete Instruction Set Implementation: Supports all 35 CHIP-8 opcodes, ensuring compatibility with a wide range of CHIP-8 programs.
* Graphics Rendering: Accurate 64x32 pixel display output using modern graphics libraries.
* Keyboard Input Handling: Emulates the 16-key hexadecimal keypad using standard keyboard mappings.
* Timers: Implements delay and sound timers that decrement at 60Hz for proper timing and sound effects.

## Running the emulator

### Dependencies
You just need Rust with its toolchain installed. Follow [these instructions](https://www.rust-lang.org/tools/install) if you need.

### Clone the repository 

``` sh
git clone https://github.com/paultergust/chip-8
cd chip-8
```
### Build and run

```sh
cargo run
```

### Controls

```mathematica
1 2 3 4
Q W E R
A S D F
Z X C V
```
