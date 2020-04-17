//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Input
//************************************************************************

pub trait KeyInput {
    fn is_key_pressed(&self, key: u8) -> bool;
    fn get_key(&self)                 -> u8;
}