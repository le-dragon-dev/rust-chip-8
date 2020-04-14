//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//
// This file take care of the CHIP-8 memory
//************************************************************************

// CHIP-8 memory
const CHIP8_MEMORY_SIZE: usize = 4096usize; // bytes

pub struct Memory {
    data: [u8; CHIP8_MEMORY_SIZE] // 4k
}

impl Memory {
    // Create a CHIP-8 memory (all byte to 0)
    pub fn new(initial_data: [u8; CHIP8_MEMORY_SIZE]) -> Self {
        Memory {
            data: [0; CHIP8_MEMORY_SIZE]
        }
    }

    // Create a CHIP-8 memory with data
    pub fn new_from_data(initial_data: [u8; CHIP8_MEMORY_SIZE]) -> Self {
        Memory {
            data: initial_data
        }
    }

    // Read a byte from memory
    pub fn read_byte(&self, index: usize) -> u8 {
        debug_assert!(index < CHIP8_MEMORY_SIZE,
                      format!("Read byte: Memory index out of bounds! ({}/{})", index, CHIP8_MEMORY_SIZE));
        self.data[index]
    }

    // Write a byte in memory
    pub fn write_byte(&mut self, index: usize, value: u8) {
        debug_assert!(index < CHIP8_MEMORY_SIZE,
                      format!("Write byte: Memory index out of bounds! ({}/{})", index, CHIP8_MEMORY_SIZE));
        self.data[index] = value;
    }
}