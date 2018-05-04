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

    fn draw(&mut self, e: &Event) {
        self.window.draw_2d(e, |c, g| {
            // clear window
            clear([0.0, 0.0, 0.0, 1.0], g);
            //
            rectangle(
                [1.0, 0.0, 0.0, 1.0],     // color
                [0.0, 0.0, 100.0, 100.0], // dot
                c.transform,
                g,
            );
        });
    }

    fn key(&mut self, e: &Event) {
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

fn main() {
    let mut screen = Screen::new();
    while let Some(e) = screen.window.next() {
        screen.draw(&e);
        screen.key(&e);
    }
}
