//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//
// This file take care of the CHIP-8 cpu
//************************************************************************

use std::time::{SystemTime, Duration};
use std::thread::sleep;

const CHIP8_CPU_CLOCK_SPEED: u16 = 500; // Hz

struct CPU {
    registers: [u8; 16],
    addr_register: u16,
    last_instruction_time: Option<SystemTime>
}

impl CPU {
    // Create a cpu
    pub fn new() -> Self{
        CPU {
            registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            addr_register: 0,
            last_instruction_time: None
        }
    }

    // Emulate clock speed, should be call after each instruction
    pub fn emulate_cpu_speed(&mut self){
        // Get the current system time
        let time_now = SystemTime::now();

        // If there is an instruction before, simulate latency
        if self.last_instruction_time.is_some() {
            let duration = self.last_instruction_time.unwrap().elapsed().unwrap();

            // We have to sleep
            if duration < Duration::from_micros(1_000_000 / CHIP8_CPU_CLOCK_SPEED as u64) {
                sleep(duration - Duration::from_micros(1_000_000 / CHIP8_CPU_CLOCK_SPEED as u64));
            }
        }

        // Set the new last instruction time
        self.last_instruction_time = Some(time_now);
    }
}