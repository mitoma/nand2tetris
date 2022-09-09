extern crate hack_computer;

use hack_computer::ram::*;
use hack_computer::screen::*;
use hack_computer::test_util::*;

fn main() {
    let mut ram = Ram16kHiSpeed::default();

    ram = draw_data(ram);

    let mut screen = Screen::new(ram);
    while screen.window.is_open() {
        screen.draw();
        //screen.key();
    }
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
