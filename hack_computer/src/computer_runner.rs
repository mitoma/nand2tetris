extern crate hack_computer;

use hack_computer::computer::*;
use minifb::Key;
use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "hack computer", long_about = None)]
struct Args {
    /// hack program path
    program_path: String,

    // max cycle count
    max_cycle_count: Option<u32>,

    // sleep ms for each cycle
    sleep_ms: Option<u64>,
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = Args::parse();
    let mut computer = Computer::default();

    computer.load_rom(args.program_path);
    let mut counter = 0;
    while computer.screen.window.is_open() && !computer.screen.window.is_key_down(Key::Escape) {
        computer.cycle(false);
        computer.screen.draw();
        computer.screen.key();
        counter += 1;
        if let Some(max_cycle_count) = args.max_cycle_count {
            if counter > max_cycle_count {
                std::process::exit(0)
            }
        }
        if let Some(sleep_ms) = args.sleep_ms {
            thread::sleep(Duration::from_millis(sleep_ms))
        }
    }
}
