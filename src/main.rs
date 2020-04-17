//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//************************************************************************

mod chip8;

// Todo: This part if just for testing the display, should be removed!
struct BasicTerminalDisplay;
impl chip8::Display for BasicTerminalDisplay {
    fn clean(&mut self) {
        std::process::Command::new("clear").spawn().unwrap();
    }

    fn draw(&mut self, pixels: [u8; 2048]) {
        const PIXEL_EMPTY: char = ' ';
        const PIXEL_FULL : char = '█';

        let mut screen = String::new();
        for y in 0..64 {
            // Jump line
            screen.push('\n');

            // Draw new line
            for x in 0..32 {
                let index = x + y * 64;
                if pixels[index] == 0 {
                    screen.push(PIXEL_EMPTY);
                }
                else {
                    screen.push(PIXEL_FULL);
                }
            }
        }

        // Print screen
        println!("{}", screen);
    }
}

struct FakeInput;
impl chip8::KeyInput for FakeInput {
    fn is_key_pressed(&self, key: u8) -> bool {
        false
    }

    fn get_key(&self) -> u8 {
        0
    }
}
// Todo: This part if just for testing the display, should be removed!


fn main() -> Result<(), String> {
    // Get the rom file from the args
    let args: Vec<String> = std::env::args().collect();
    let has_path_arg = args.len() < 2;

    if !has_path_arg {
        return Err(format!("Missing an argument: Path to the CHIP-8 ROM file!"));
    }

    // Get the rom file path
    let rom_file = args.get(1).cloned().unwrap();

    // Prepare the emulator
    let mut chip8 = chip8::Chip8::new(BasicTerminalDisplay{}, FakeInput{});

    // Load the rom file
    chip8.load_rom_file(&rom_file)?;

    Ok(())
}

