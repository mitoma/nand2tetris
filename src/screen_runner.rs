extern crate nand2tetlis;

use nand2tetlis::screen::*;

fn main() {
    let mut screen = Screen::new();
    while let Some(e) = screen.window.next() {
        screen.draw(&e);
        screen.key(&e);
    }
}
