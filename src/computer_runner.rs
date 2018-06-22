extern crate nand2tetlis;

use nand2tetlis::computer::*;
use std::env;
use std::thread;

fn main() {
    // main thread の stack サイズの都合で thread を新たに作っている
    // https://qiita.com/szktty/items/8a6e26f4b829d3689fce
    let builder = thread::Builder::new();
    let thread = builder.stack_size(10000000);
    let handle = thread
        .spawn(|| {
            let args: Vec<String> = env::args().collect();
            let program_path = &args[1];
            let mut computer = Computer::new();
            computer.load_rom(program_path.to_string());
            let mut counter = 0;
            while let Some(e) = computer.screen.window.next() {
                computer.cycle(false);
                computer.screen.draw(&e);
                computer.screen.key(&e);
                counter += 1;
                if counter > 50 {
//                    std::process::exit(0);
                }
            }
        })
        .unwrap();
    let _ = handle.join();
}
