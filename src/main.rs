//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//************************************************************************

mod chip8;
mod display_input;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::display_input::{Screen, Input};

const WIDTH : u32 = 640;
const HEIGHT: u32 = 320;
const TITLE : &'static str = "Rust CHIP-8 emulator";

fn init_sdl() -> (sdl2::Sdl, sdl2::video::Window) {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    (context, window)
}

fn main() -> Result<(), String> {
    // Get the rom file from the args
    let args: Vec<String> = std::env::args().collect();
    let has_path_arg = args.len() >= 2;

    // If there is no path, return an error
    if !has_path_arg {
        return Err(format!("Missing the path to the CHIP-8 ROM file!"));
    }

    // Init SDL
    let (sdl_context, window) = init_sdl();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Get the rom file path
    let rom_file = args.get(1).cloned().unwrap();

    // Prepare the emulator
    let mut chip8 = chip8::Chip8::new(Screen::new(), Input::new(&mut event_pump));

    // Load the rom file
    chip8.load_rom_file(&rom_file)?;

    // Init chip-8
    chip8.init()?;

    // Main loop
    loop {
        // Run a step from chip-8
        chip8.step()?;

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        // Draw pixels
        canvas.set_draw_color(Color::RGB(255,255,255));
        for y in 0..32 {
            for x in 0..64 {
                let pixel = chip8.screen.data[(x + y * 64) as usize];

                if pixel != 0 {
                    canvas.fill_rect(Rect::new(x*10, y*10, 10, 10))?;
                }
            }
        }

        // Check if we have to quit
        if chip8.key_input.check_quit() {
            break;
        }

        canvas.present();
    }

    Ok(())
}

