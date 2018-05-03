use basic_gate::*;
use multi_gate::*;
use ram::*;

use std::error::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

pub struct Screen {
    rams: [Ram4k; 2],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            rams: [Ram4k::new(), Ram4k::new()],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 13], load: bool) -> [bool; 16] {
        let upper = address[0];
        let lower = [
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
        ];
        let sel = dmux(load, upper);
        let result = mux16(
            self.rams[0].ram(a, lower, sel[0]),
            self.rams[1].ram(a, lower, sel[1]),
            upper,
        );

        // メモリの内容をファイルにダンプする
        // ここだけ普通のコード
        let path = Path::new("target/screen.txt");
        let display = path.display();

        let file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut f_writer = BufWriter::new(file);

        for rams4k in self.rams.iter() {
            for rams512 in rams4k.rams.iter() {
                for ram64 in rams512.rams.iter() {
                    for ram8 in ram64.rams.iter() {
                        for register in ram8.registers.iter() {
                            for bit in register.bits.iter() {
                                f_writer
                                    .write_fmt(format_args!(
                                        "{}",
                                        if bit.dff.pre_value { 1 } else { 0 }
                                    ))
                                    .expect("MOGE!");
                            }
                            f_writer.write_fmt(format_args!("\n")).expect("MOGE!");
                        }
                    }
                }
            }
        }
        //
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use const_value::*;
    use test_util::*;

    #[test]
    fn test_screen() {
        let mut sut = Screen::new();

        let t = i2b(0);
        let address = [
            t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8], t[9], t[10], t[11], t[12]
        ];
        sut.ram(FULL, address, true);
        sut.ram(FULL, address, true);
    }
}
