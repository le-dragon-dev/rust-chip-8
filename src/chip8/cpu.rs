//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//
// CPU methods
//************************************************************************

use std::thread::sleep;
use std::time::{SystemTime, Duration};

use crate::chip8::Chip8;
use crate::chip8::constants::CHIP8_CPU_CLOCK_SPEED;

impl Chip8 {
    // Emulate clock speed, should be call after each instruction
    pub(crate) fn emulate_cpu_speed(&mut self) {
        // Get the current system time
        let time_now = SystemTime::now();

        // If there is an instruction before, simulate latency
        if self.last_instruction_time.is_some() {
            let duration = self.last_instruction_time.unwrap().elapsed().unwrap();

            // We have to sleep
            if duration < Duration::from_micros(1_000_000 / self.clock_speed as u64) {
                sleep(duration - Duration::from_micros(1_000_000 / self.clock_speed as u64));
            }
        }

        // Set the new last instruction time
        self.last_instruction_time = Some(time_now);
    }

    // Change the cpu clock speed (0 to default speed)
    pub fn change_cpu_clock_speed(&mut self, clock_speed_hz: u16) {
        if clock_speed_hz == 0 {
            self.clock_speed = CHIP8_CPU_CLOCK_SPEED;
        }
        else {
            self.clock_speed = clock_speed_hz;
        }
    }
}