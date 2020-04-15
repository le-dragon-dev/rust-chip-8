//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//************************************************************************

use std::time::{SystemTime, Duration};
use std::thread::sleep;

type OpCode   = u16;
type Address  = u16;
type Register = u8;

const CHIP8_REGISTER_COUNT : usize = 16;   // Nb of registers
const CHIP8_CPU_CLOCK_SPEED: u16   = 500;  // Hz
const CHIP8_MEMORY_SIZE    : usize = 4096; // bytes

pub struct Chip8 {
    // CPU
    registers: [u8; CHIP8_REGISTER_COUNT],
    addr_register: u16,
    last_instruction_time: Option<SystemTime>,

    // Memory
    data: [u8; CHIP8_MEMORY_SIZE]
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            registers    : [0; CHIP8_REGISTER_COUNT],
            addr_register: 0,
            data         : [0; CHIP8_MEMORY_SIZE],
            last_instruction_time: None,
        }
    }
}

// ------- CPU -------
impl Chip8 {
    // Emulate clock speed, should be call after each instruction
    fn emulate_cpu_speed(&mut self) {
        // Get the current system time
        let time_now = SystemTime::now();

        // If there is an instruction before, simulate latency
        if self.last_instruction_time.is_some() {
            let duration = self.last_instruction_time.unwrap().elapsed().unwrap();

            // We have to sleep
            if duration < Duration::from_micros(1_000_000 / CHIP8_CPU_CLOCK_SPEED as u64) {
                sleep(duration - Duration::from_micros(1_000_000 / CHIP8_CPU_CLOCK_SPEED as u64));
            }
        }

        // Set the new last instruction time
        self.last_instruction_time = Some(time_now);
    }
}

// ------- OPCODES -------
impl Chip8 {
    // 0NNN
    fn call_rca1802_program(&self) {
        unimplemented!()
    }

    // 00E0
    fn clear_screen(&self) {
        todo!("Clear screen")
    }

    // 00EE
    fn return_from_subroutine(&mut self) {
        todo!("Return from subroutine")
    }

    // 1NNN
    fn goto(&mut self, op_code: OpCode) {
        todo!("Goto address")
    }

    // 2NNN
    fn call_subroutine(&mut self, op_code: OpCode) {
        todo!("Call a subroutine")
    }

    // 3XNN
    fn if_eq_const_skip(&mut self, op_code: OpCode) {
        todo!("Jump if Vx == NN")
    }

    // 4XNN
    fn if_neq_const_skip(&mut self, op_code: OpCode) {
        todo!("Jump if Vx != NN")
    }

    // 5XY0
    fn if_eq_reg_skip(&mut self) {
        todo!("Jump if Vx == Vy")
    }

    // 6XNN
    fn set_reg(&mut self, op_code: OpCode) {
        todo!("Vx = NN")
    }

    // 7XNN
    fn add_const_to_reg(&mut self, op_code: OpCode) {
        todo!("Vx += NN")
    }

    // 8XY0
    fn copy_reg(&mut self, op_code: OpCode) {
        todo!("Vx = Vy")
    }

    // 8XY1
    fn or_reg(&mut self, op_code: OpCode) {
        todo!("Vx |= Vy")
    }

    // 8XY2
    fn and_reg(&mut self, op_code: OpCode) {
        todo!("Vx &= Vy")
    }

    // 8XY3
    fn xor_reg(&mut self, op_code: OpCode) {
        todo!("Vx ^= Vy")
    }

    // 8XY4
    fn add_reg_to_reg(&mut self, op_code: OpCode) {
        todo!("Vx += Vy")
    }

    // 8XY5
    fn sub_reg1_to_reg0(&mut self, op_code: OpCode) {
        todo!("Vx -= Vy")
    }

    // 8XY6
    fn shift_right_reg(&mut self, op_code: OpCode) {
        todo!("Vx >>= 1")
    }

    // 8XY7
    fn sub_reg0_to_reg1(&mut self, op_code: OpCode) {
        todo!("Vx = Vy - Vx")
    }

    // 8XYE
    fn shift_left_reg(&mut self, op_code: OpCode) {
        todo!("Vx <<= 1")
    }

    // 9XY0
    fn if_neq_reg_skip(&mut self) {
        todo!("Jump if Vx != Vy")
    }

    // ANNN
    fn set_addr(&mut self, op_code: OpCode) {
        todo!("Set address")
    }

    // BNNN
    fn jump_to_addr(&mut self, op_code: OpCode) {
        todo!("PC = NNN + V0")
    }

    // CXNN
    fn rand(&mut self, op_code: OpCode) {
        todo!("V0 = rand() & NN")
    }

    // DXYN
    fn draw(&mut self, op_code: OpCode) {
        unimplemented!()
    }

    // EX9E
    fn if_eq_key_skip(&mut self, op_code: OpCode) {
        todo!("if key() == Vx")
    }

    // EXA1
    fn if_neq_key_skip(&mut self, op_code: OpCode) {
        todo!("if key() != Vx")
    }

    // FX07
    fn get_delay_timer_value(&mut self, op_code: OpCode) {
        todo!("Vx = get_delay()")
    }

    // FX0A
    fn get_key_value(&mut self, op_code: OpCode) {
        todo!("Vx = get_key()")
    }

    // FX15
    fn set_delay_timer(&mut self, op_code: OpCode) {
        todo!("set_delay_timer(Vx)")
    }

    // FX18
    fn set_sound_timer(&mut self, op_code: OpCode) {
        todo!("set_sound_timer(Vx)")
    }

    // FX1E
    fn add_ref_to_addr(&mut self, op_code: OpCode) {
        todo!("I += Vx")
    }

    // FX29
    fn set_sprite_to_addr(&mut self, op_code: OpCode) {
        todo!("I = sprite_addr[Vx]")
    }

    // FX33
    fn set_bcd(&mut self, op_code: OpCode) {
        unimplemented!()
    }

    // FX55
    fn reg_dump(&mut self, op_code: OpCode) {
        todo!("Store V0 to VX (include) in memory starting at I")
    }

    // FX65
    fn reg_load(&mut self, op_code: OpCode) {
        todo!("Fill V0 to VX (include) registers from memory starting at I")
    }
}

// ------- Utils -------
fn get_addr_from_opcode(opcode: OpCode) -> Address {
    opcode & 0x0FFF
}

fn get_reg_from_opcode(opcode: OpCode) -> Register {
    (opcode >> 8 & 0x000F) as Register
}

fn get_reg_and_value_from_opcode(opcode: OpCode) -> (Register, u8) {
    (get_reg_from_opcode(opcode), (opcode & 0x00FF) as u8)
}

fn get_reg_and_reg_from_opcode(opcode: OpCode) -> (Register, Register) {
    (get_reg_from_opcode(opcode), (opcode >> 4 & 0x000F) as Register)
}

fn get_reg_and_reg_and_value_from_opcode(opcode: OpCode) -> (Register, Register, u8) {
    (get_reg_from_opcode(opcode), (opcode >> 4 & 0x000F) as Register, (opcode & 0x000F) as u8)
}