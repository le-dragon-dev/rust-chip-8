//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT licence
//************************************************************************

use std::time::{SystemTime, Duration};
use std::thread::sleep;

type OpCode   = u16;
type Address  = u16;
type Register = usize;

const CHIP8_REGISTER_COUNT : usize = 16;   // Nb of registers
const CHIP8_REGISTER_VF    : usize = 0xF;  // Index
const CHIP8_CPU_CLOCK_SPEED: u16   = 500;  // Hz
const CHIP8_MEMORY_SIZE    : usize = 4096; // bytes

pub struct Chip8 {
    // CPU
    registers            : [u8; CHIP8_REGISTER_COUNT],
    addr_register        : u16,
    program_counter      : u16,
    last_instruction_time: Option<SystemTime>,

    // Memory
    memory: [u8; CHIP8_MEMORY_SIZE]
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            registers      : [0; CHIP8_REGISTER_COUNT],
            addr_register  : 0,
            program_counter: 0,
            memory: [0; CHIP8_MEMORY_SIZE],
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
    fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            0x00E0          => { self.clear_screen(); }
            0x00EE          => { self.return_from_subroutine(); }
            0x0000..=0x0FFF => { self.call_rca1802_program(opcode); }
            0x1000..=0x1FFF => { self.goto(opcode); }
            0x2000..=0x2FFF => { self.call_subroutine(opcode); }
            0x3000..=0x3FFF => { self.if_eq_const_skip(opcode); }
            0x4000..=0x4FFF => { self.if_neq_const_skip(opcode); }
            0x5000..=0x5FF0 => { self.if_eq_reg_skip(opcode); }
            0x6000..=0x6FFF => { self.set_reg(opcode); }
            0x7000..=0x7FFF => { self.add_const_to_reg(opcode); }

            0x8000..=0x8FFF if opcode & 0x000F == 0x0 => { self.copy_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x1 => { self.or_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x2 => { self.and_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x3 => { self.xor_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x4 => { self.and_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x5 => { self.sub_reg1_to_reg0(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x6 => { self.shift_right_reg(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0x7 => { self.sub_reg0_to_reg1(opcode); }
            0x8000..=0x8FFF if opcode & 0x000F == 0xE => { self.shift_left_reg(opcode); }

            0x9000..=0x9FF0 => { self.if_neq_reg_skip(opcode); }
            0xA000..=0xAFFF => { self.set_addr(opcode); }
            0xB000..=0xBFFF => { self.jump_to_addr(opcode); }
            0xC000..=0xCFFF => { self.rand(opcode); }
            0xD000..=0xDFFF => { self.draw(opcode); }

            0xE09E..=0xEF9E if opcode & 0x00FF == 0x9E => { self.if_eq_key_skip(opcode); }
            0xE0A1..=0xEFA1 if opcode & 0x00FF == 0xA1 => { self.if_neq_key_skip(opcode); }

            0xF007..=0xFF07 if opcode & 0x00FF == 0x07 => { self.get_delay_timer_value(opcode); }
            0xF00A..=0xFF0A if opcode & 0x00FF == 0x0A => { self.get_key_value(opcode); }
            0xF015..=0xFF15 if opcode & 0x00FF == 0x15 => { self.set_delay_timer(opcode); }
            0xF018..=0xFF18 if opcode & 0x00FF == 0x18 => { self.set_sound_timer(opcode); }
            0xF01E..=0xFF1E if opcode & 0x00FF == 0x1E => { self.add_reg_to_addr(opcode); }
            0xF029..=0xFF29 if opcode & 0x00FF == 0x29 => { self.set_sprite_to_addr(opcode); }
            0xF033..=0xFF33 if opcode & 0x00FF == 0x33 => { self.set_bcd(opcode); }
            0xF055..=0xFF55 if opcode & 0x00FF == 0x55 => { self.reg_dump(opcode); }
            0xF065..=0xFF65 if opcode & 0x00FF == 0x65 => { self.reg_load(opcode); }

            _ => { panic!("Unknowed OPCODE!"); }
        }

        // Emulate CPU speed
        self.emulate_cpu_speed();
    }

    // 0NNN
    fn call_rca1802_program(&self, _op_code: OpCode) {
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
    fn if_eq_reg_skip(&mut self, op_code: OpCode) {
        todo!("Jump if Vx == Vy")
    }

    // 6XNN
    fn set_reg(&mut self, op_code: OpCode) {
        let (register, value) = get_reg_and_value_from_opcode(op_code);
        self.registers[register] = value;
    }

    // 7XNN
    fn add_const_to_reg(&mut self, op_code: OpCode) {
        let (register, value) = get_reg_and_value_from_opcode(op_code);
        self.registers[register] += value;
    }

    // 8XY0
    fn copy_reg(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] = self.registers[register_2];
    }

    // 8XY1
    fn or_reg(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] |= self.registers[register_2];
    }

    // 8XY2
    fn and_reg(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] &= self.registers[register_2];
    }

    // 8XY3
    fn xor_reg(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] ^= self.registers[register_2];
    }

    // 8XY4
    fn add_reg_to_reg(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] += self.registers[register_2];

        todo!("VF is set to 1 when there's a carry, otherwise 0")
    }

    // 8XY5
    fn sub_reg1_to_reg0(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] -= self.registers[register_2];

        todo!("VF is set to 0 when there's a borrow, otherwise 1")
    }

    // 8XY6
    fn shift_right_reg(&mut self, op_code: OpCode) {
        let register = get_reg_from_opcode(op_code);

        // Store the less significant bit in VF
        self.registers[CHIP8_REGISTER_VF] = self.registers[register] & 0x01;

        // Shift right
        self.registers[register] >>= 1;
    }

    // 8XY7
    fn sub_reg0_to_reg1(&mut self, op_code: OpCode) {
        let (register_1, register_2) = get_reg_and_reg_from_opcode(op_code);
        self.registers[register_1] = self.registers[register_2] - self.registers[register_1];

        todo!("VF is set to 0 when there's a borrow, otherwise 1")
    }

    // 8XYE
    fn shift_left_reg(&mut self, op_code: OpCode) {
        let register = get_reg_from_opcode(op_code);

        // Store the most significant bit in VF
        self.registers[CHIP8_REGISTER_VF] = self.registers[register] & 0x80;

        // Shift right
        self.registers[register] <<= 1;
    }

    // 9XY0
    fn if_neq_reg_skip(&mut self, op_code: OpCode) {
        todo!("Jump if Vx != Vy")
    }

    // ANNN
    fn set_addr(&mut self, op_code: OpCode) {
        self.addr_register = get_addr_from_opcode(op_code);
    }

    // BNNN
    fn jump_to_addr(&mut self, op_code: OpCode) {
        self.program_counter = self.registers[0] as u16 + get_addr_from_opcode(op_code);
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
    fn add_reg_to_addr(&mut self, op_code: OpCode) {
        let register = get_reg_from_opcode(op_code);
        let old_addr_value = self.addr_register;

        // Add Vx to I
        self.addr_register += self.registers[register] as u16;

        // VF set to 1 if overflow, otherwise 0
        self.registers[CHIP8_REGISTER_VF] = (self.addr_register < old_addr_value) as u8;
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
        let register = get_reg_from_opcode(op_code);
        for x in 0 ..= register {
            self.memory[self.addr_register as usize + x] = self.registers[x];
        }
    }

    // FX65
    fn reg_load(&mut self, op_code: OpCode) {
        let register = get_reg_from_opcode(op_code);
        for x in 0 ..= register {
            self.registers[x] = self.memory[self.addr_register as usize + x];
        }
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