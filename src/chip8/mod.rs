//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//
// CHIP-8 emulator
//************************************************************************

use std::time::SystemTime;

use crate::chip8::constants::*;
use crate::chip8::types::Address;

pub mod constants;
pub mod cpu;
pub mod memory;
pub mod opcodes;
pub mod types;

// CHIP-8 structure
pub struct Chip8 {
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
    stack_ptr: usize
}

impl Chip8 {
    // Initialize the emulator
    pub fn new() -> Self {
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
            stack_ptr: 0
        }
    }
}