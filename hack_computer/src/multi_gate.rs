use basic_gate::*;

pub fn not16(a: [bool; 16]) -> [bool; 16] {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

pub fn or8way(a: [bool; 8]) -> bool {
    or(
        or(or(a[0], a[1]), or(a[2], a[3])),
        or(or(a[4], a[5]), or(a[6], a[7])),
    )
}

pub fn mux4(a: bool, b: bool, c: bool, d: bool, sel: [bool; 2]) -> bool {
    mux(mux(a, b, sel[1]), mux(c, d, sel[1]), sel[0])
}

#[allow(clippy::many_single_char_names)]
pub fn mux8(
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    sel: [bool; 3],
) -> bool {
    mux(
        mux(mux(a, b, sel[2]), mux(c, d, sel[2]), sel[1]),
        mux(mux(e, f, sel[2]), mux(g, h, sel[2]), sel[1]),
        sel[0],
    )
}

pub fn mux4way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    sel: [bool; 2],
) -> [bool; 16] {
    [
        mux4(a[0], b[0], c[0], d[0], sel),
        mux4(a[1], b[1], c[1], d[1], sel),
        mux4(a[2], b[2], c[2], d[2], sel),
        mux4(a[3], b[3], c[3], d[3], sel),
        mux4(a[4], b[4], c[4], d[4], sel),
        mux4(a[5], b[5], c[5], d[5], sel),
        mux4(a[6], b[6], c[6], d[6], sel),
        mux4(a[7], b[7], c[7], d[7], sel),
        mux4(a[8], b[8], c[8], d[8], sel),
        mux4(a[9], b[9], c[9], d[9], sel),
        mux4(a[10], b[10], c[10], d[10], sel),
        mux4(a[11], b[11], c[11], d[11], sel),
        mux4(a[12], b[12], c[12], d[12], sel),
        mux4(a[13], b[13], c[13], d[13], sel),
        mux4(a[14], b[14], c[14], d[14], sel),
        mux4(a[15], b[15], c[15], d[15], sel),
    ]
}

#[allow(clippy::many_single_char_names)]
pub fn mux8way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16],
    f: [bool; 16],
    g: [bool; 16],
    h: [bool; 16],
    sel: [bool; 3],
) -> [bool; 16] {
    [
        mux8(a[0], b[0], c[0], d[0], e[0], f[0], g[0], h[0], sel),
        mux8(a[1], b[1], c[1], d[1], e[1], f[1], g[1], h[1], sel),
        mux8(a[2], b[2], c[2], d[2], e[2], f[2], g[2], h[2], sel),
        mux8(a[3], b[3], c[3], d[3], e[3], f[3], g[3], h[3], sel),
        mux8(a[4], b[4], c[4], d[4], e[4], f[4], g[4], h[4], sel),
        mux8(a[5], b[5], c[5], d[5], e[5], f[5], g[5], h[5], sel),
        mux8(a[6], b[6], c[6], d[6], e[6], f[6], g[6], h[6], sel),
        mux8(a[7], b[7], c[7], d[7], e[7], f[7], g[7], h[7], sel),
        mux8(a[8], b[8], c[8], d[8], e[8], f[8], g[8], h[8], sel),
        mux8(a[9], b[9], c[9], d[9], e[9], f[9], g[9], h[9], sel),
        mux8(a[10], b[10], c[10], d[10], e[10], f[10], g[10], h[10], sel),
        mux8(a[11], b[11], c[11], d[11], e[11], f[11], g[11], h[11], sel),
        mux8(a[12], b[12], c[12], d[12], e[12], f[12], g[12], h[12], sel),
        mux8(a[13], b[13], c[13], d[13], e[13], f[13], g[13], h[13], sel),
        mux8(a[14], b[14], c[14], d[14], e[14], f[14], g[14], h[14], sel),
        mux8(a[15], b[15], c[15], d[15], e[15], f[15], g[15], h[15], sel),
    ]
}

pub fn dmux4way(a: bool, sel: [bool; 2]) -> [bool; 4] {
    [
        and(and(a, not(sel[0])), not(sel[1])),
        and(and(a, not(sel[0])), sel[1]),
        and(and(a, sel[0]), not(sel[1])),
        and(and(a, sel[0]), sel[1]),
    ]
}

pub fn dmux8way(a: bool, sel: [bool; 3]) -> [bool; 8] {
    [
        and(and(and(a, not(sel[0])), not(sel[1])), not(sel[2])),
        and(and(and(a, not(sel[0])), not(sel[1])), sel[2]),
        and(and(and(a, not(sel[0])), sel[1]), not(sel[2])),
        and(and(and(a, not(sel[0])), sel[1]), sel[2]),
        and(and(and(a, sel[0]), not(sel[1])), not(sel[2])),
        and(and(and(a, sel[0]), not(sel[1])), sel[2]),
        and(and(and(a, sel[0]), sel[1]), not(sel[2])),
        and(and(and(a, sel[0]), sel[1]), sel[2]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmux4way_test() {
        assert_eq!(true, mux4(true, false, false, false, [false, false]));
        assert_eq!(false, mux4(false, false, false, false, [false, false]));
        assert_eq!(true, mux4(false, true, false, false, [false, true]));
        assert_eq!(false, mux4(false, false, false, false, [false, true]));
        assert_eq!(true, mux4(false, false, true, false, [true, false]));
        assert_eq!(false, mux4(false, false, false, false, [true, false]));
        assert_eq!(true, mux4(false, false, false, true, [true, true]));
        assert_eq!(false, mux4(true, false, false, false, [true, true]));
    }

    #[test]
    fn dmux8way_test() {
        assert_eq!(
            [true, false, false, false, false, false, false, false],
            dmux8way(true, [false, false, false])
        );
        assert_eq!(
            [false, true, false, false, false, false, false, false],
            dmux8way(true, [false, false, true])
        );
        assert_eq!(
            [false, false, true, false, false, false, false, false],
            dmux8way(true, [false, true, false])
        );
        assert_eq!(
            [false, false, false, true, false, false, false, false],
            dmux8way(true, [false, true, true])
        );
        assert_eq!(
            [false, false, false, false, true, false, false, false],
            dmux8way(true, [true, false, false])
        );
        assert_eq!(
            [false, false, false, false, false, true, false, false],
            dmux8way(true, [true, false, true])
        );
        assert_eq!(
            [false, false, false, false, false, false, true, false],
            dmux8way(true, [true, true, false])
        );
        assert_eq!(
            [false, false, false, false, false, false, false, true],
            dmux8way(true, [true, true, true])
        );
    }
}
