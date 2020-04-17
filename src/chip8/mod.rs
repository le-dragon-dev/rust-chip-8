//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// CHIP-8 emulator
//************************************************************************

use std::time::SystemTime;

pub use crate::chip8::display::Display;
pub use crate::chip8::input::KeyInput;

use crate::chip8::constants::*;
use crate::chip8::timer::Timer;
use crate::chip8::types::Address;

mod constants;
mod cpu;
mod display;
mod input;
mod memory;
mod opcodes;
mod timer;
mod types;

// CHIP-8 structure
pub struct Chip8<Screen, Input> where Screen: Display, Input: KeyInput {
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
    screen: Screen,

    // Timers
    delay_timer: Timer,
    sound_timer: Timer,

    // Input
    key_input: Input
}

impl<Screen, Input> Chip8<Screen, Input> where Screen: Display, Input: KeyInput {
    // Initialize the emulator
    pub fn new(screen: Screen, key_input: Input) -> Self {
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
            screen,

            // Timers
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),

            // Input
            key_input
        }
    }
}