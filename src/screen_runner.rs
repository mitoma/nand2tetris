extern crate nand2tetlis;

use nand2tetlis::ram::*;
use nand2tetlis::screen::*;
use nand2tetlis::test_util::*;

use std::thread;

fn main() {
    // main thread の stack サイズの都合で thread を新たに作っている
    // https://qiita.com/szktty/items/8a6e26f4b829d3689fce
    let builder = thread::Builder::new();
    let thread = builder.stack_size(10000000);
    let handle = thread
        .spawn(|| {
            let mut ram = Ram16k::new();

            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_0000_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_0000_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_1000_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_1000_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_0100_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_0100_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_1100_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_1100_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_0010_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_0010_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_1010_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_1010_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_0110_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_0110_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0000_1110_0000_u16),
                true,
            );
            ram.ram(
                u2b(0b_0101_0101_0101_0101_u16),
                u2b14(0b_0000_0001_1110_0000_u16),
                true,
            );
            //ram load
            /*
            for i in 0..64 {
                let val = i2b(i);
                let add = [
                    val[13], val[12], val[11], val[10], val[9], val[8], val[7], val[6], val[5],
                    val[4], val[3], val[2], val[1], val[0],
                ];
                ram.ram(val, add, true);
            }
            */

            let mut screen = Screen::new(ram);
            while let Some(e) = screen.window.next() {
                screen.draw(&e);
                screen.key(&e);
            }
        })
        .unwrap();
    let _ = handle.join();
}
