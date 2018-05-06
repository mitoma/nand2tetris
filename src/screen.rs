use piston::window::WindowSettings;
use piston_window::*;
use image::*;

use ram::*;

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
                                    [255, 255, 255, 255]
                                } else {
                                    if counter % 2 == 1 {
                                        [255, 255, 255, 255]
                                    } else {
                                        [0, 0, 0, 255]
                                    }
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
            match key {
                Button::Keyboard(keyboard::Key::Up) => println!("Pressed keyboard key UP"),
                Button::Keyboard(keyboard::Key::Down) => println!("Pressed keyboard key UP"),
                _ => println!("Pressed keyboard key NONE"),
            }
        }
    }
}
