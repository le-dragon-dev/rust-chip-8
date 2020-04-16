//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//
// Memory methods
//************************************************************************

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::path::Path;

use crate::chip8::Chip8;
use crate::chip8::constants::{CHIP8_MAX_EXECUTABLE_SIZE, CHIP8_MEMORY_START};

impl Chip8 {
    // Load the fontset in memory
    pub fn load_fontset() {
        todo!("Add the fontset");
    }

    // Try to load the executable in memory
    pub fn load_executable(&mut self, path: &'static str) -> Result<(), String> {
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

        // Copy the file into memory
        file.read(self.memory[CHIP8_MEMORY_START as usize..=CHIP8_MAX_EXECUTABLE_SIZE as usize+CHIP8_MEMORY_START as usize].as_mut())
            .map_err(|e| format!("Impossible to copy the executable into memory"))?;

        Ok(())
    }
}