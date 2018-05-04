extern crate nand2tetlis;

use nand2tetlis::screen::*;
use nand2tetlis::ram::*;

use std::thread;

fn main() {
    // main thread の stack サイズの都合で thread を新たに作っている
    // https://qiita.com/szktty/items/8a6e26f4b829d3689fce
    let builder = thread::Builder::new();
    let thread = builder.stack_size(10000000);
    let handle = thread
        .spawn(|| {
            let ram = Ram16k::new();
            let mut screen = Screen::new(ram);
            while let Some(e) = screen.window.next() {
                screen.draw(&e);
                screen.key(&e);
            }
        })
        .unwrap();
    let _ = handle.join();
}
