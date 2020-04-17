Rust CHIP-8 emulator documentation
==================================

### CPU
- **Registers**:
    - CHIP-8 has 16 8-bit data registers names V**0** to V**F**.
    - The address register named **I**, is a 16 bits wide and is used with several **opcodes** that involve
    memory operations.

- **Opcode tables**:

    The CHIP-8 has 35 opcodes, two bytes long and stored in **big-endian** (https://en.wikipedia.org/wiki/CHIP-8#Opcode_table)
    - **NNN**: address
    - **NN**: 8-bit constant
    - **N**: 4-bit constant
    - **X** and **Y**: 4-bit register identifier
    - **PC**: Program Counter
    - **I**: 16bit register (For memory address) (Similar to void pointer)
    - **VN**: One of the 16 available variables. **N** may be _0x0_ to _0xF_.
    
    |Opcode| Type |    Code Eq        |Explanation|
    |------|------|----------------   |-------------------------------------|
    | 0NNN |Call  |                   |Calls RCA 1802 program at address NNN|
    | 00E0 |Disp  |clear_screen()     |Clear the screen|
    | 00EE |Flow  |return;            |Returns from a subroutine|
    | 1NNN |Flow  |goto NNN;          |Jumps to address NNN|
    | 2NNN |Flow  |*(0xNNN)();        |Calls subroutine at NNN|
    | 3XNN |Cond  |if Vx==NN          |Skips the next instruction if VX equals NN|
    | 4XNN |Cond  |if Vx!=NN          |Skips the next instruction if VX doesn't equals NN|
    | 5XY0 |Cond  |if Vx==Vy          |Skips the next instruction if VX equals VY|
    | 6XNN |Const |Vx = NN            |Sets VX to NN|
    | 7XNN |Const |Vx += NN           |Adds NN to VX|
    | 8XY0 |Assign|Vx = Vy            |Sets VX to the value of VY|
    | 8XY1 |BitOp |Vx &#124;= Vy      |Sets VX to VX **or** VY|
    | 8XY2 |BitOp |Vx &= Vy           |Sets VX to VX **and** VY|
    | 8XY3 |BitOp |Vx ^= Vy           |Sets VX to VX **xor** VY|
    | 8XY4 |Math  |Vx += Vy           |Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't|
    | 8XY5 |Math  |Vx -= Vy           |Subs VY to VX. VF is set to 0 when there's a borrow, and to 1 when there isn't|
    | 8XY6 |BitOp |Vx >>= 1           |Stores the least significant bit of VX in VF and then shifts VX to the right by 1|
    | 8XY7 |Math  |Vx = Vy - Vx       |Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't|
    | 8XYE |BitOp |Vx <<= 1           |Stores the most significant bit of VX in VF and then shifts VX to the left by 1|
    | 9XY0 |Cond  |if Vx!=Vy          |Skips the next instruction if VX doesn't equals VY|
    | ANNN |Mem   |I = NNN            |Sets I to address NNN|
    | BNNN |Flow  |PC = V0+NNN        |Jumps to the address NNN + V0|
    | CXNN |Rand  |Vx = Rand()&NN     |Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.|
    | DXYN |Disp  |draw(Vx,Vy,N)      |Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen|
    | EX9E |KeyOp |if key() == Vx     |Skips the next instruction if the key stored in VX is pressed|
    | EXA1 |KeyOp |if key() != Vx     |Skips the next instruction if the key stored in VX isn't pressed|
    | FX07 |Timer |Vx = get_delay()   |Sets VX to the value of the delay timer|
    | FX0A |KeyOp |Vx = get_key()     |A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)|
    | FX15 |Timer |delay_timer(Vx)    |Sets the delay timer to VX|
    | FX18 |Sound |sound_timer(Vx)    |Sets the sound timer to VX|
    | FX1E |Mem   |I += Vx            |Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't|
    | FX29 |Mem   |I = sprite_addr[Vx]|Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font|
    | FX33 |BCD   |set_BCD(Vx); *(I+0)=BCD(3); *(I+1)=BCD(2); *(I+2)=BCD(1); |Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)|        
    | FX55 |Mem   |reg_dump(Vx, &I)   |Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified|
    | FX65 |Mem   |reg_load(Vx, &I)   |Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified|

### Memory
CHIP-8 uses **4096 bytes** of memory to run:
- The first **512 bytes (0x200)** are occupied by the CHIP-8 interpreter itself (In modern implementations, 
the interpreter is running outside the 4k memory space, and there is no need to avoid this memory, it is
common to store font data).
- The uppermost **256 bytes (0xF00-0xFFF)** are reserved for display refresh.
- The **96 bytes below (0xEA0-0xEFF)** were reserved for the call stack, internal use and other variables.

### Stack
The stack is only used to store return addresses when subroutines are called.
In our emulator, we will use 16 levels (16-bit each).

### Display
The display is 64 x 32 pixels (monochrome).

### Timers
Two timers count down at 60 hertz, until they reach 0.
- Delay timer: Used to timing the events of games.
- Sound timer: Does a sound until the timer value is 0.

### Rom loader
Will load a ".ch8" rom file in memory

### State save / load
Will save all the status: Memory, registers, stack, display and timers.

