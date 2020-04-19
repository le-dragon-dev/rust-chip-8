//************************************************************************
// Rust CHIP-8 emulator, created by David Garcia
// Distributed under the MIT license
//************************************************************************

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Scancode, Keycode};

use crate::chip8::{Display, KeyInput};

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
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            data: [0; 2048],
        }
    }
}

impl Display for Screen {
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

    fn get_key(&mut self) -> u8 {
        loop {
            let event = self.event_pump.wait_event();
            match event {
                Event::KeyDown { scancode: Some(Scancode::Kp0), ..} => return 0x0,
                Event::KeyDown { scancode: Some(Scancode::Kp1), ..} => return 0x1,
                Event::KeyDown { scancode: Some(Scancode::Kp2), ..} => return 0x2,
                Event::KeyDown { scancode: Some(Scancode::Kp3), ..} => return 0x3,
                Event::KeyDown { scancode: Some(Scancode::Kp4), ..} => return 0x4,
                Event::KeyDown { scancode: Some(Scancode::Kp5), ..} => return 0x5,
                Event::KeyDown { scancode: Some(Scancode::Kp6), ..} => return 0x6,
                Event::KeyDown { scancode: Some(Scancode::Kp7), ..} => return 0x7,
                Event::KeyDown { scancode: Some(Scancode::Kp8), ..} => return 0x8,
                Event::KeyDown { scancode: Some(Scancode::Kp9), ..} => return 0x9,
                Event::KeyDown { scancode: Some(Scancode::A), ..}   => return 0xA,
                Event::KeyDown { scancode: Some(Scancode::B), ..}   => return 0xB,
                Event::KeyDown { scancode: Some(Scancode::C), ..}   => return 0xC,
                Event::KeyDown { scancode: Some(Scancode::D), ..}   => return 0xD,
                Event::KeyDown { scancode: Some(Scancode::E), ..}   => return 0xE,
                Event::KeyDown { scancode: Some(Scancode::F), ..}   => return 0xF,
                _ => continue
            }
        }
    }
}