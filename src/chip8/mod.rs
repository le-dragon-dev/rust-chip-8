//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// CHIP-8 emulator
//************************************************************************

use std::time::{SystemTime, Duration};

pub use crate::chip8::display::Display;
pub use crate::chip8::input::KeyInput;

use crate::chip8::constants::*;
use crate::chip8::timer::Timer;
use crate::chip8::types::{Address, OpCode};
use std::thread::sleep;

mod constants;
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

    // Main loop
    pub fn run(&mut self) -> Result<(), String> {
        // Load the fontset
        self.load_fontset();

        // Check if the program is loaded
        if self.memory[0x0200] == 0 {
            return Err(format!("No rom loaded!"));
        }

        // Set the PC at 0x200
        self.program_counter = 0x0200;

        // Loop
        loop {
            // Get the opcode
            let op_code: OpCode = ((self.memory[self.program_counter as usize] as OpCode) << 8) +
                                  (self.memory[self.program_counter as usize + 1] as OpCode);

            // Execute the opcode
            self.execute_opcode(op_code);

            // Emulate CPU speed
            self.emulate_cpu_speed();

            // Update timers
            self.delay_timer.update();
            self.sound_timer.update();
        }
    }

    // Emulate clock speed, should be call after each instruction
    fn emulate_cpu_speed(&mut self) {
        // Get the current system time
        let time_now = SystemTime::now();

        // If there is an instruction before, simulate latency
        if self.last_instruction_time.is_some() {
            let duration = self.last_instruction_time.unwrap().elapsed().unwrap();

            // We have to sleep
            if duration < Duration::from_micros(1_000_000 / self.clock_speed as u64) {
                sleep(Duration::from_micros(1_000_000 / self.clock_speed as u64) - duration);
            }
        }

        // Set the new last instruction time
        self.last_instruction_time = Some(time_now);
    }
}