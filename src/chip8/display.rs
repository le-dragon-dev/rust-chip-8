//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Represents a screen
//************************************************************************

pub trait Display {
    fn clean(&mut self);
    fn draw(&mut self, pixels: [u8]);
}

