extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use piston_window::*;
use piston::window::WindowSettings;
use opengl_graphics::OpenGL;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("hack screen", [512, 256])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = image::ImageBuffer::new(512, 256);
    let mut texture: G2dTexture =
        Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            for x in 0..512 {
                for y in 0..256 {
                    canvas.put_pixel(x, y, image::Rgba([255, 0, 255, 255]))
                }
            }
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }

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
