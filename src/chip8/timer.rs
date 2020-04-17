//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Timer
//************************************************************************

use std::time::SystemTime;

use crate::chip8::constants::CHIP8_TIMER_CLOCK_SPEED;

pub struct Timer {
    value: u8,
    last_update_time: Option<SystemTime>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            value: 0,
            last_update_time: None
        }
    }

    pub fn start(&mut self, value: u8 ) {
        self.value = value;
        self.last_update_time = Some(SystemTime::now());
    }

    pub fn update(&mut self) {
        const UPDATE_DURATION: u16 = 1000 / CHIP8_TIMER_CLOCK_SPEED;

        // if the time is running, update
        if self.last_update_time.is_some() && self.value != 0 {
            let duration = self.last_update_time.unwrap().elapsed();

            if duration.unwrap().as_millis() as u16 > UPDATE_DURATION {
                self.value -= 1;
                self.last_update_time = Some(SystemTime::now());
            }
        }

        // if we're to zero, stop the timer
        if self.value == 0 {
            self.last_update_time = None;
        }
    }

    pub fn get_delay(&self) -> u8 {
        self.value
    }
}