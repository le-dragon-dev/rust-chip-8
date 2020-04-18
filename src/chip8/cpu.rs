//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// CPU methods
//************************************************************************

use std::thread::sleep;
use std::time::{SystemTime, Duration};

use crate::chip8::{Chip8, KeyInput};
use crate::chip8::constants::CHIP8_CPU_CLOCK_SPEED;
use crate::chip8::display::Display;

impl<Screen, Input> Chip8<Screen, Input> where Screen: Display, Input: KeyInput {
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