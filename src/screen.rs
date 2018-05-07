use piston::window::WindowSettings;
use piston_window::*;
use image::*;

use ram::*;
use test_util::*;

pub struct Screen {
    pub ram: Ram16k,
    pub window: PistonWindow,
}

impl Screen {
    pub fn new(ram: Ram16k) -> Screen {
        // Create an Glutin window.
        let window: PistonWindow = WindowSettings::new("hack screen", [512, 256])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Screen {
            ram: ram,
            window: window,
        }
    }

    pub fn draw(&mut self, e: &Event) {
        let ram = &mut self.ram;

        let mut canvas = ImageBuffer::new(512, 256);
        // draw memory
        let mut counter: u32 = 0;
        for rams4k in ram.rams[0..2].iter() {
            for rams512 in rams4k.rams.iter() {
                for ram64 in rams512.rams.iter() {
                    for ram8 in ram64.rams.iter() {
                        for register in ram8.registers.iter() {
                            for bit in register.bits.iter() {
                                let color = if bit.dff.pre_value {
                                    [0, 255, 0, 255]
                                } else {
                                    [0, 0, 0, 255]
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
        let texture: G2dTexture =
            Texture::from_image(&mut self.window.factory, &canvas, &TextureSettings::new())
                .unwrap();
        self.window.draw_2d(e, |c, g| {
            image(&texture, c.transform, g);
        });
    }

    pub fn key(&mut self, e: &Event) {
        // http://docs.piston.rs/mush/piston/input/keyboard/enum.Key.html
        if let Some(key) = e.press_args() {
            let key_bits = i2b(self.key_to_code(key));
            println!("keycoard {}", self.key_to_code(key));

            let ram = &mut self.ram;
            ram.ram(
                key_bits,
                [
                    false, false, false, false, false, true, false, false, false, false, false,
                    false, false, false,
                ],
                true,
            );
        }
    }

    fn key_to_code(&mut self, key: Button) -> i16 {
        println!("to : {:?}", key);
        match key {
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
}
