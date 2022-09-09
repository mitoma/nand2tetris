use image::*;
use minifb::{Window, WindowOptions};

use ram::*;
use test_util::*;

pub struct Screen {
    pub ram: Ram16kHiSpeed,
    pub window: Window,
    screen_changed: bool,
    on_shift: bool,
    current_keycode: u16,
}

const WIDTH: usize = 512;
const HEIGHT: usize = 256;

impl Screen {
    pub fn new(ram: Ram16kHiSpeed) -> Screen {
        let window = Window::new("Hack Screen", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        Screen {
            ram,
            window,
            screen_changed: false,
            on_shift: false,
            current_keycode: 0,
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 14], load: bool) -> [bool; 16] {
        if !self.screen_changed && load {
            self.screen_changed = true;
        }
        self.ram.ram(a, address, load)
    }

    pub fn draw(&mut self) {
        if !self.screen_changed {
            return;
        }
        self.screen_changed = false;

        let ram = &mut self.ram;
        let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 256);

        // for debug
        let mut counter: u32 = 0;
        for i in 0..(1024 * 8) {
            let value = ram.ram(u2b(0), u2b14(i), false);
            for v in value.iter() {
                let color = if *v { [0, 255, 0, 255] } else { [0, 0, 0, 255] };
                let x = counter % 512;
                let y = counter / 512;
                canvas.put_pixel(x, y, Rgba(color));
                counter += 1;
            }
        }

        let buffer: Vec<u32> = canvas
            .chunks(4)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();

        self.window
            .update_with_buffer(buffer.as_slice(), WIDTH, HEIGHT)
            .unwrap();

        // for naive
        /*
        {
            let mut counter: u32 = 0;
            for rams4k in ram.rams[0..2].iter() {
                for rams512 in rams4k.rams.iter() {
                    for ram64 in rams512.rams.iter() {
                        for ram8 in ram64.rams.iter() {
                            for register in ram8.registers.iter() {
                                for bit in register.bits.iter() {
                                    let zero_pos = if counter % 8 == 0 { 128 } else { 0 };
                                    let four_pos = if counter % 8 == 4 { 128 } else { 0 };

                                    let color = if bit.dff.pre_value {
                                        [zero_pos, 255, four_pos, 255]
                                    } else {
                                        [zero_pos, 0, four_pos, 255]
                                    };
                                    let x = counter % 512;
                                    let y = counter / 512;
                                    canvas.put_pixel(x, y, Rgba(color));
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        */
    }
    /*
    pub fn key(&mut self, e: &Event) {
        // http://docs.piston.rs/mush/piston/input/keyboard/enum.Key.html
        if let Some(key) = e.press_args() {
            if self.is_shift(key) {
                self.on_shift = true;
            } else {
                self.current_keycode = self.key_to_code(key);
            }
        }
        if let Some(key) = e.release_args() {
            if self.is_shift(key) {
                self.on_shift = false;
            } else {
                self.current_keycode = 0;
            }
        }
        self.ram.rams[0x2000] = self.current_keycode;
    }

    fn is_shift(&mut self, key: Button) -> bool {
        match key {
            Button::Keyboard(keyboard::Key::LShift) | Button::Keyboard(keyboard::Key::RShift) => {
                true
            }
            _ => false,
        }
    }

    fn key_to_code(&mut self, key: Button) -> u16 {
        let shift_value = if self.on_shift { 32 } else { 0 };

        match key {
            // num
            Button::Keyboard(keyboard::Key::D0) => 48,
            Button::Keyboard(keyboard::Key::D1) => 49,
            Button::Keyboard(keyboard::Key::D2) => 50,
            Button::Keyboard(keyboard::Key::D3) => 51,
            Button::Keyboard(keyboard::Key::D4) => 52,
            Button::Keyboard(keyboard::Key::D5) => 53,
            Button::Keyboard(keyboard::Key::D6) => 54,
            Button::Keyboard(keyboard::Key::D7) => 55,
            Button::Keyboard(keyboard::Key::D8) => 56,
            Button::Keyboard(keyboard::Key::D9) => 57,

            // alpha
            Button::Keyboard(keyboard::Key::A) => 65 + shift_value,
            Button::Keyboard(keyboard::Key::B) => 66 + shift_value,
            Button::Keyboard(keyboard::Key::C) => 67 + shift_value,
            Button::Keyboard(keyboard::Key::D) => 68 + shift_value,
            Button::Keyboard(keyboard::Key::E) => 69 + shift_value,
            Button::Keyboard(keyboard::Key::F) => 70 + shift_value,
            Button::Keyboard(keyboard::Key::G) => 71 + shift_value,
            Button::Keyboard(keyboard::Key::H) => 72 + shift_value,
            Button::Keyboard(keyboard::Key::I) => 73 + shift_value,
            Button::Keyboard(keyboard::Key::J) => 74 + shift_value,
            Button::Keyboard(keyboard::Key::K) => 75 + shift_value,
            Button::Keyboard(keyboard::Key::L) => 76 + shift_value,
            Button::Keyboard(keyboard::Key::M) => 77 + shift_value,
            Button::Keyboard(keyboard::Key::N) => 78 + shift_value,
            Button::Keyboard(keyboard::Key::O) => 79 + shift_value,
            Button::Keyboard(keyboard::Key::P) => 80 + shift_value,
            Button::Keyboard(keyboard::Key::Q) => 81 + shift_value,
            Button::Keyboard(keyboard::Key::R) => 82 + shift_value,
            Button::Keyboard(keyboard::Key::S) => 83 + shift_value,
            Button::Keyboard(keyboard::Key::T) => 84 + shift_value,
            Button::Keyboard(keyboard::Key::U) => 85 + shift_value,
            Button::Keyboard(keyboard::Key::V) => 86 + shift_value,
            Button::Keyboard(keyboard::Key::W) => 87 + shift_value,
            Button::Keyboard(keyboard::Key::X) => 88 + shift_value,
            Button::Keyboard(keyboard::Key::Y) => 89 + shift_value,
            Button::Keyboard(keyboard::Key::Z) => 90 + shift_value,

            // other key
            Button::Keyboard(keyboard::Key::Space) => 32,
            Button::Keyboard(keyboard::Key::Return) => 128,
            Button::Keyboard(keyboard::Key::Backspace) => 129,
            Button::Keyboard(keyboard::Key::Left) => 130,
            Button::Keyboard(keyboard::Key::Up) => 131,
            Button::Keyboard(keyboard::Key::Right) => 132,
            Button::Keyboard(keyboard::Key::Down) => 133,
            Button::Keyboard(keyboard::Key::Home) => 134,
            Button::Keyboard(keyboard::Key::End) => 135,
            Button::Keyboard(keyboard::Key::PageUp) => 136,
            Button::Keyboard(keyboard::Key::PageDown) => 137,
            Button::Keyboard(keyboard::Key::Insert) => 138,
            Button::Keyboard(keyboard::Key::Delete) => 139,
            Button::Keyboard(keyboard::Key::Escape) => 140,
            Button::Keyboard(keyboard::Key::F1) => 141,
            Button::Keyboard(keyboard::Key::F2) => 142,
            Button::Keyboard(keyboard::Key::F3) => 143,
            Button::Keyboard(keyboard::Key::F4) => 144,
            Button::Keyboard(keyboard::Key::F5) => 145,
            Button::Keyboard(keyboard::Key::F6) => 146,
            Button::Keyboard(keyboard::Key::F7) => 147,
            Button::Keyboard(keyboard::Key::F8) => 148,
            Button::Keyboard(keyboard::Key::F9) => 149,
            Button::Keyboard(keyboard::Key::F10) => 150,
            Button::Keyboard(keyboard::Key::F11) => 151,
            Button::Keyboard(keyboard::Key::F12) => 152,
            _ => 0,
        }
    }
     */
}
