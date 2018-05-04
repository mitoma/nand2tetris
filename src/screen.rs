use piston::window::WindowSettings;
use piston_window::*;

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
        let ram = &self.ram;
        self.window.draw_2d(e, |c, g| {
            // clear window
            clear([0.0, 0.0, 0.0, 1.0], g);

            // draw memory
            let mut counter: u32 = 0;
            for rams4k in ram.rams[0..1].iter() {
                for rams512 in rams4k.rams.iter() {
                    for ram64 in rams512.rams.iter() {
                        for ram8 in ram64.rams.iter() {
                            for register in ram8.registers.iter() {
                                for bit in register.bits.iter() {
                                    let color = if bit.dff.pre_value {
                                        [1.0, 1.0, 1.0, 1.0]
                                    } else {
                                        [0.0, 0.0, 0.0, 1.0]
                                    };
                                    let x = counter % 512;
                                    let y = counter / 512;
                                    rectangle(
                                        color,                                                    // color
                                        [x as f64, y as f64, (x as f64 + 1.0), (y as f64 + 1.0)], // dot
                                        c.transform,
                                        g,
                                    );
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
                println!("{}", counter);
            }
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
