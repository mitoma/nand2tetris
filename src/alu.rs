use adder::*;
use basic_gate::*;
use const_value;
use multi_gate::*;

pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    let x = mux4way16(
        x,
        not16(x),
        const_value::ZERO,
        /* undefined value*/ const_value::FULL,
        [zx, nx],
    );
    let y = mux4way16(
        y,
        not16(y),
        const_value::ZERO,
        /* undefined value*/ const_value::FULL,
        [zy, ny],
    );
    println!(
        "alu [zx,nx,zy,ny]:{:?} x:{:?}, y:{:?}",
        [zx, nx, zy, ny],
        x,
        y
    );

    let result = mux4way16(
        and16(x, y),
        add16(x, y),
        not16(and16(x, y)),
        not16(add16(x, y)),
        [no, f],
    );

    let ng_result = not16(result);
    (
        result,
        and(
            and(
                and(
                    and(ng_result[0], ng_result[1]),
                    and(ng_result[2], ng_result[3]),
                ),
                and(
                    and(ng_result[4], ng_result[5]),
                    and(ng_result[6], ng_result[7]),
                ),
            ),
            and(
                and(
                    and(ng_result[8], ng_result[9]),
                    and(ng_result[10], ng_result[11]),
                ),
                and(
                    and(ng_result[12], ng_result[13]),
                    and(ng_result[14], ng_result[15]),
                ),
            ),
        ),
        result[15],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use test_util::*;

    #[test]
    fn alu_test() {
        let f = fs::File::open("test/ALU.cmp").unwrap();
        let reader = BufReader::new(f);

        let mut counter = 0;
        for line in reader.lines().skip(1) {
            counter = counter + 1;
            let l = line.unwrap();
            let tokens = l.split("|")
                .map(|str| str.trim())
                .filter(|str| !str.is_empty())
                .collect::<Vec<&str>>();

            // input
            let x = u16::from_str_radix(tokens[0], 2).unwrap();
            let y = u16::from_str_radix(tokens[1], 2).unwrap();
            let zx = u16::from_str_radix(tokens[2], 2).unwrap() == 1;
            let nx = u16::from_str_radix(tokens[3], 2).unwrap() == 1;
            let zy = u16::from_str_radix(tokens[4], 2).unwrap() == 1;
            let ny = u16::from_str_radix(tokens[5], 2).unwrap() == 1;
            let f = u16::from_str_radix(tokens[6], 2).unwrap() == 1;
            let no = u16::from_str_radix(tokens[7], 2).unwrap() == 1;
            // output
            let out = u16::from_str_radix(tokens[8], 2).unwrap();
            let zr = u16::from_str_radix(tokens[9], 2).unwrap() == 1;
            let ng = u16::from_str_radix(tokens[10], 2).unwrap() == 1;

            let result = alu(u2b(x), u2b(y), zx, nx, zy, ny, f, no);

            assert_eq!(u2b(out), result.0);
            assert_eq!(zr, result.1);
            assert_eq!(ng, result.2);
        }
    }

    #[test]
    fn alu_nostat_test() {
        let f = fs::File::open("test/ALU-nostat.cmp").unwrap();
        let reader = BufReader::new(f);

        let mut counter = 0;
        for line in reader.lines().skip(1) {
            counter = counter + 1;
            let l = line.unwrap();
            let tokens = l.split("|")
                .map(|str| str.trim())
                .filter(|str| !str.is_empty())
                .collect::<Vec<&str>>();

            // input
            let x = u16::from_str_radix(tokens[0], 2).unwrap();
            let y = u16::from_str_radix(tokens[1], 2).unwrap();
            let zx = u16::from_str_radix(tokens[2], 2).unwrap() == 1;
            let nx = u16::from_str_radix(tokens[3], 2).unwrap() == 1;
            let zy = u16::from_str_radix(tokens[4], 2).unwrap() == 1;
            let ny = u16::from_str_radix(tokens[5], 2).unwrap() == 1;
            let f = u16::from_str_radix(tokens[6], 2).unwrap() == 1;
            let no = u16::from_str_radix(tokens[7], 2).unwrap() == 1;
            // output
            let out = u16::from_str_radix(tokens[8], 2).unwrap();

            let result = alu(u2b(x), u2b(y), zx, nx, zy, ny, f, no);

            assert_eq!(u2b(out), result.0);
        }
    }
}
