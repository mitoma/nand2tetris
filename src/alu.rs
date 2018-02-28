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
        not16(add16(x, y)),
        [no, f],
    );

    let ng_result = not16(result);
    return (
        result,
        result[15],
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
    );
}

#[cfg(test)]
mod tests {
    use test_util::*;
    use super::*;

    #[test]
    fn and_add_test() {
        assert_eq!(
            (i2b(0b_0000_0000_0000_0001_i16), false, false),
            alu(
                i2b(0b0000_0000_0000_0011i16),
                i2b(0b0000_0000_0000_0001i16),
                false,
                false,
                false,
                false,
                false,
                false
            )
        );
        assert_eq!(
            (i2b(0b0000_0000_0000_0100i16), false, false),
            alu(
                i2b(0b0000_0000_0000_0011i16),
                i2b(0b0000_0000_0000_0001i16),
                false,
                false,
                false,
                false,
                true,
                false
            )
        );
    }

    #[test]
    fn out_test() {
        assert_eq!(
            (i2b(0b_1111_1111_1111_1011_i16), true, false),
            alu(
                i2b(0b_0000_0000_0000_0011_i16),
                i2b(0b_0000_0000_0000_0001_i16),
                false,
                false,
                false,
                false,
                true,
                true
            )
        );
    }

    #[test]
    fn zx_zy_test() {
        assert_eq!(
            (i2b(0b_1010_1010_1010_1010_i16), true, false),
            alu(
                i2b(0b_0101_0101_0101_0101_i16),
                i2b(0b_1010_1010_1010_1010_i16),
                false,
                true,
                false,
                false,
                true,
                false
            )
        );
        assert_eq!(
            (i2b(0b_0101_0101_0101_0101_i16), false, false),
            alu(
                i2b(0b_0101_0101_0101_0101_i16),
                i2b(0b_1010_1010_1010_1010_i16),
                false,
                false,
                false,
                true,
                true,
                false
            )
        );
    }
}
