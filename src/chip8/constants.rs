//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//
// Constants
//************************************************************************

use crate::chip8::types::{Address, Register};

pub const CHIP8_REGISTER_COUNT     : usize    = 16;    // Nb of registers
pub const CHIP8_REGISTER_VF        : Register = 0xF;   // Index
pub const CHIP8_CPU_CLOCK_SPEED    : u16      = 500;   // Hz
pub const CHIP8_PROGRAM_COUNTER_INC: u16      = 2;     // Bytes
pub const CHIP8_MEMORY_START       : Address  = 0x200; // Address
pub const CHIP8_MEMORY_SIZE        : usize    = 4096;  // Bytes
pub const CHIP8_STACK_COUNT        : usize    = 16;    // Nb of stacks
pub const CHIP8_MAX_EXECUTABLE_SIZE: u16      = 3072;  // Bytes