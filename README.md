# CHIP-8 Emulator

## Introduction

[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) is a simple, interpreted, programming language which was first used on some do-it-yourself computer systems in the late 1970s and early 1980s. The COSMAC VIP, DREAM 6800, and ETI 660 computers are a few examples. These computers typically were designed to use a television as a display, had between 1 and 4K of RAM, and used a 16-key hexadecimal keypad for input. The interpreter took up only 512 bytes of memory, and programs, which were entered into the computer in hexadecimal, were even smaller.

This version, written in Rust, is my own implementation of the interpreter. It does not reside on the first 512 bytes of the chip's memory, and is intended to run in modern computers and to develop simple games and programs.

## Architecture
* Memory: 4KB (4096 bytes)
* Registers: 16 general-purpose 8-bit registers (V0 to VF) and a 12-bit register (I) for memory addresses.
* Timers: Two 8-bit timers, a delay timer, and a sound timer, which decrement at 60Hz.
* Display: 64x32 monochrome pixel display, updated through draw instructions.
* Input: 16-key keypad
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

### How to write code for it

Simply write each opcode (with instruction and data) as a 16-bit hexcode line-by-line. Example:

```
A000
A050
6000
6000
D015
1202
```

Currently, it supports neither assembly sintax nor comments on "src code". But these will be implemented in the near future.

### Shout-out

[This page](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) by Thomas Greene helped me a lot in developing this.
