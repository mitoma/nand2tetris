extern crate nand2tetlis;

use nand2tetlis::computer::*;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    // main thread の stack サイズの都合で thread を新たに作っている
    // https://qiita.com/szktty/items/8a6e26f4b829d3689fce
    let builder = thread::Builder::new();
    let thread = builder.stack_size(10000000);
    let handle = thread
        .spawn(|| {
            let program_path = env::args().nth(1);
            let max_cycle_count = env::args()
                .nth(2)
                .ok_or("max_cycle_count is none.".to_owned())
                .and_then(|v| {
                    v.parse::<u32>()
                        .map_err(|_err| "please input number".to_owned())
                });
            let sleep_ms = env::args()
                .nth(3)
                .ok_or("sleep_ms is none.".to_owned())
                .and_then(|v| {
                    v.parse::<u64>()
                        .map_err(|_err| "please input number".to_owned())
                });
            let mut computer = Computer::new();
            match program_path {
                Some(path) => computer.load_rom(path),
                None => {
                    println!(
                        "{}",
                        "hack バイナリのパスを指定してください"
                    );
                    std::process::exit(0)
                }
            }
            let mut counter = 0;
            while let Some(e) = computer.screen.window.next() {
                computer.cycle(false);
                computer.screen.draw(&e);
                computer.screen.key(&e);
                counter += 1;
                match max_cycle_count {
                    Ok(value) if counter > value => std::process::exit(0),
                    _ => {}
                }
                match sleep_ms {
                    Ok(value) => thread::sleep(Duration::from_millis(value)),
                    _ => {}
                }
            }
        })
        .unwrap();
    let _ = handle.join();
}
