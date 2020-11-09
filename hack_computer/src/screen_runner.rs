extern crate hack_computer;

use hack_computer::ram::*;
use hack_computer::screen::*;
use hack_computer::test_util::*;

use std::thread;

fn main() {
    // main thread の stack サイズの都合で thread を新たに作っている
    // https://qiita.com/szktty/items/8a6e26f4b829d3689fce
    let builder = thread::Builder::new();
    let thread = builder.stack_size(10000000);
    let handle = thread
        .spawn(|| {
            let ram = Ram16kHiSpeed::default();

            // ram = draw_data(ram);

            let mut screen = Screen::new(ram);
            while let Some(e) = screen.window.next() {
                screen.draw(&e);
                screen.key(&e);
            }
        })
        .unwrap();
    let _ = handle.join();
}

// 画面へのデータ出力のテスト
#[allow(dead_code, clippy::identity_op)]
#[rustfmt::skip]
fn draw_data(mut ram: Ram16kHiSpeed) -> Ram16kHiSpeed {
    for i in 0..16 {
        // draw A
        ram.ram(
            u2b(0b_1111_0001_1000_1111_u16),
            u2b14(0b_0000_0000_0000_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0000_0010_0100_0000_u16),
            u2b14(0b_0000_0000_0010_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0000_0100_0010_0000_u16),
            u2b14(0b_0000_0000_0100_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0000_1000_0001_0000_u16),
            u2b14(0b_0000_0000_0110_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0001_0000_0000_1000_u16),
            u2b14(0b_0000_0000_1000_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0010_0000_0000_0100_u16),
            u2b14(0b_0000_0000_1010_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0000_1100_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0000_1110_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0111_1111_1111_1110_u16),
            u2b14(0b_0000_0001_0000_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_0010_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_0100_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_0110_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_1000_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_1010_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_1100_0000_u16 + i),
            true,
        );
        ram.ram(
            u2b(0b_0100_0000_0000_0010_u16),
            u2b14(0b_0000_0001_1110_0000_u16 + i),
            true,
        );
    }
    ram
}
