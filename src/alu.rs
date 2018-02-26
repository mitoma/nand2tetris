use const_value;
use adder::*;
use basic_gate::*;
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
        const_value::ZERO,
        not16(x),
        /* undefined value*/ const_value::FULL,
        [zx, nx],
    );
    let y = mux4way16(
        y,
        const_value::ZERO,
        not16(y),
        /* undefined value*/ const_value::FULL,
        [zy, ny],
    );

    let result = mux4way16(
        and16(x, y),
        add16(x, y),
        not16(and16(x, y)),
        not16(and16(x, y)),
        [f, no],
    );

    return (
        result,
        and(result[0], true),
        nand(
            nand(
                nand(nand(result[0], result[1]), nand(result[2], result[3])),
                nand(nand(result[4], result[5]), nand(result[6], result[7])),
            ),
            nand(
                nand(nand(result[8], result[9]), nand(result[10], result[11])),
                nand(nand(result[12], result[13]), nand(result[14], result[15])),
            ),
        ),
    );
}
