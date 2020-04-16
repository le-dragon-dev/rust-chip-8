//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// CHIP-8 emulator
//************************************************************************

use std::time::SystemTime;

use crate::chip8::constants::*;
use crate::chip8::display::Display;
use crate::chip8::types::Address;

mod constants;
mod cpu;
mod display;
mod memory;
mod opcodes;
mod types;

// CHIP-8 structure
pub struct Chip8<Screen> where Screen: Display {
    // CPU
    registers            : [u8; CHIP8_REGISTER_COUNT],
    addr_register        : Address,
    program_counter      : Address,
    clock_speed          : u16,
    last_instruction_time: Option<SystemTime>,

    // Memory
    memory: [u8; CHIP8_MEMORY_SIZE],

    // Stack
    stack    : [Address; CHIP8_STACK_COUNT],
    stack_ptr: usize,

    // Screen
    gfx   : [u8; CHIP8_PIXEL_COUNT],
    screen: Screen
}

impl<Screen> Chip8<Screen> where Screen: Display {
    // Initialize the emulator
    pub fn new(screen: Screen) -> Self {
        Chip8 {
            // CPU
            registers      : [0; CHIP8_REGISTER_COUNT],
            addr_register  : 0,
            program_counter: 0,
            clock_speed    : CHIP8_CPU_CLOCK_SPEED,
            last_instruction_time: None,

            // Memory
            memory: [0; CHIP8_MEMORY_SIZE],

            // Stack
            stack    : [0; CHIP8_STACK_COUNT],
            stack_ptr: 0,

            // Screen
            gfx: [0; CHIP8_PIXEL_COUNT],
            screen
        }
    }
}