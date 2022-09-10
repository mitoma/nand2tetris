use crate::ram::*;
use crate::test_util::*;
use image::*;
use minifb::Key;
use minifb::{Window, WindowOptions};

pub struct Screen {
    pub ram: Ram16kHiSpeed,
    pub window: Window,
    screen_changed: bool,
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
            self.window.update();
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
                let color = if *v {
                    [0, 100, 200, 100]
                } else {
                    [0, 0, 10, 0]
                };
                let x = counter % 512;
                let y = counter / 512;
                canvas.put_pixel(x, y, Rgba(color));
                counter += 1;
            }
        }

        let buffer: Vec<u32> = canvas
            .chunks(4)
            .map(|v| {
                let result = ((v[0] as u32) << 24)
                    | ((v[1] as u32) << 16)
                    | (v[2] as u32) << 8
                    | v[3] as u32;
                result
            })
            .collect();

        self.window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
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

    pub fn key(&mut self) {
        let current_keys = self.window.get_keys();
        let keycode = if current_keys.is_empty() {
            0
        } else {
            let on_shift = current_keys.iter().any(|key| Self::is_shift(key));
            let key = current_keys.iter().find(|key| !Self::is_shift(key));
            key.map(|key| Self::key_to_code(key, on_shift)).unwrap_or(0)
        };
        self.ram.rams[0x2000] = keycode;
    }

    fn is_shift(key: &Key) -> bool {
        match key {
            Key::LeftShift | Key::RightShift => true,
            _ => false,
        }
    }

    fn key_to_code(key: &Key, on_shift: bool) -> u16 {
        let shift_value = if on_shift { 32 } else { 0 };

        match key {
            // num
            Key::Key0 => 48,
            Key::Key1 => 49,
            Key::Key2 => 50,
            Key::Key3 => 51,
            Key::Key4 => 52,
            Key::Key5 => 53,
            Key::Key6 => 54,
            Key::Key7 => 55,
            Key::Key8 => 56,
            Key::Key9 => 57,

            // alpha
            Key::A => 65 + shift_value,
            Key::B => 66 + shift_value,
            Key::C => 67 + shift_value,
            Key::D => 68 + shift_value,
            Key::E => 69 + shift_value,
            Key::F => 70 + shift_value,
            Key::G => 71 + shift_value,
            Key::H => 72 + shift_value,
            Key::I => 73 + shift_value,
            Key::J => 74 + shift_value,
            Key::K => 75 + shift_value,
            Key::L => 76 + shift_value,
            Key::M => 77 + shift_value,
            Key::N => 78 + shift_value,
            Key::O => 79 + shift_value,
            Key::P => 80 + shift_value,
            Key::Q => 81 + shift_value,
            Key::R => 82 + shift_value,
            Key::S => 83 + shift_value,
            Key::T => 84 + shift_value,
            Key::U => 85 + shift_value,
            Key::V => 86 + shift_value,
            Key::W => 87 + shift_value,
            Key::X => 88 + shift_value,
            Key::Y => 89 + shift_value,
            Key::Z => 90 + shift_value,

            // other key
            Key::Space => 32,
            Key::Enter => 128,
            Key::Backspace => 129,
            Key::Left => 130,
            Key::Up => 131,
            Key::Right => 132,
            Key::Down => 133,
            Key::Home => 134,
            Key::End => 135,
            Key::PageUp => 136,
            Key::PageDown => 137,
            Key::Insert => 138,
            Key::Delete => 139,
            Key::Escape => 140,
            Key::F1 => 141,
            Key::F2 => 142,
            Key::F3 => 143,
            Key::F4 => 144,
            Key::F5 => 145,
            Key::F6 => 146,
            Key::F7 => 147,
            Key::F8 => 148,
            Key::F9 => 149,
            Key::F10 => 150,
            Key::F11 => 151,
            Key::F12 => 152,
            _ => 0,
        }
    }
}
