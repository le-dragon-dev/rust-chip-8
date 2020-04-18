//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//************************************************************************

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;

use crate::chip8::{Display, KeyInput};
use sdl2::keyboard::{Scancode, Keycode};
use sdl2::EventPump;
use sdl2::video::{WindowContext, Window};
use sdl2::event::Event;

const SCANCODES: [Scancode; 16] = [
    Scancode::Kp0,
    Scancode::Kp1,
    Scancode::Kp2,
    Scancode::Kp3,
    Scancode::Kp4,
    Scancode::Kp5,
    Scancode::Kp6,
    Scancode::Kp7,
    Scancode::Kp8,
    Scancode::Kp9,
    Scancode::A,
    Scancode::B,
    Scancode::C,
    Scancode::D,
    Scancode::E,
    Scancode::F];

//-------------------------- DISPLAY --------------------------
pub struct Screen {
    pub data: [u8; 2048],
    pub ask_for_clean: bool
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            data: [0; 2048],
            ask_for_clean: false
        }
    }
}

impl Display for Screen {
    fn clean(&mut self) {
        self.ask_for_clean = true;
    }

    fn draw(&mut self, pixels: [u8; 2048]) {
        self.data = pixels.clone();
    }
}

//-------------------------- INPUT --------------------------
pub struct Input<'a> {
    event_pump: &'a mut EventPump
}

impl<'a> Input<'a> {
    pub fn new(event_pump: &'a mut EventPump) -> Self {
        Input {
            event_pump
        }
    }

    pub fn check_quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                _ => {}
            }
        }

        false
    }
}

impl KeyInput for Input<'_> {
    fn is_key_pressed(&self, key: u8) -> bool {
        self.event_pump.keyboard_state().is_scancode_pressed(SCANCODES[key as usize])
    }

    fn get_key(&self) -> u8 {
        let mut keyboard_state = self.event_pump.keyboard_state();
        let mut scan_codes = keyboard_state.pressed_scancodes();
        loop {
            let valid_key = scan_codes.filter_map(|e| if SCANCODES.contains(&e) {Some(e)} else {None}).collect::<Vec<Scancode>>();
            if !valid_key.is_empty() {
                return SCANCODES.iter().position(|p| p == valid_key.first().unwrap()).unwrap() as u8;
            }
            scan_codes = keyboard_state.pressed_scancodes();
        }
    }
}