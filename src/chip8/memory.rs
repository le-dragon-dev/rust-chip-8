//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Memory methods
//************************************************************************

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::path::Path;

use crate::chip8::{Chip8, KeyInput};
use crate::chip8::constants::{CHIP8_MAX_EXECUTABLE_SIZE, CHIP8_MEMORY_START};
use crate::chip8::display::Display;

impl<Screen, Input> Chip8<Screen, Input> where Screen: Display, Input: KeyInput {
    // Load the fontset in memory
    pub fn load_fontset(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        self.memory[0x0050..0x00A0].copy_from_slice(&font[..]);
    }

    // Try to load the executable in memory
    pub fn load_rom_file(&mut self, path: &String) -> Result<(), String> {
        let path = Path::new(path);

        // The path does not exist
        if !path.exists() {
            return Err(format!("Error load: The path {} does not exist!", path.to_str().unwrap()));
        }

        // The path is not a file
        if !path.is_file() {
            return Err(format!("Error load: {} is not a file!", path.to_str().unwrap()));
        }

        // Open the file and get the size
        let mut file = File::open(path)
            .map_err(|_| format!("Impossible to load the file {}", path.to_str().unwrap()))?;

        let file_size = file.seek(SeekFrom::End(0))
            .map_err(|_| format!("Impossible to read the file {}", path.to_str().unwrap()))?;

        // If the file size is over 3kb, return an error
        if file_size > CHIP8_MAX_EXECUTABLE_SIZE as u64 {
            return Err(format!("The file {} is too big! ({} bytes / {} allowed bytes)",
                               path.to_str().unwrap(), file_size, CHIP8_MAX_EXECUTABLE_SIZE));
        }

        // Return at the start of the file
        file.seek(SeekFrom::Start(0))
            .map_err(|_| format!("Impossible to read the file {}", path.to_str().unwrap()))?;

        // Copy the file into memory
        file.read_exact(&mut self.memory[CHIP8_MEMORY_START as usize..(CHIP8_MEMORY_START as usize + file_size as usize)])
            .map_err(|_| format!("Impossible to copy the executable into memory"))?;

        Ok(())
    }
}