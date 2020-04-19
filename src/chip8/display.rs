//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Represents a screen
//************************************************************************

use crate::chip8::constants::CHIP8_PIXEL_COUNT;

pub trait Display {
    fn draw(&mut self, pixels: [u8; CHIP8_PIXEL_COUNT]);
}

