extern crate piston;
extern crate piston_window;

use piston_window::*;
use piston::window::WindowSettings;

struct Screen {
    pub window: PistonWindow,
}

impl Screen {
    fn new() -> Screen {
        // Create an Glutin window.
        let window: PistonWindow = WindowSettings::new("hack screen", [512, 256])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Screen { window: window }
    }
}

fn main() {
    let mut screen = Screen::new();

    while let Some(e) = screen.window.next() {
        screen.window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],     // red
                [0.0, 0.0, 100.0, 100.0], // rectangle
                c.transform,
                g,
            );
        });
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
