fn main() {
    println!("Hello, world!");
}

// basic gate
fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn not(a: bool) -> bool {
    nand(a, a)
}

fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

fn dmux(a: bool, sel: bool) -> (bool, bool) {
    (and(not(sel), a), and(sel, a))
}

// multi gate
fn not16(ass: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = not(ass[i]);
    }
    result
}

fn and16(ass: [bool; 16], bss: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = and(ass[i], bss[i]);
    }
    result
}

fn or16(ass: [bool; 16], bss: [bool; 16]) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = or(ass[i], bss[i]);
    }
    result
}

fn mux16(ass: [bool; 16], bss: [bool; 16], sel: bool) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = mux(ass[i], bss[i], sel);
    }
    result
}

fn or8way(a: [bool; 8]) -> bool {
    or(
        a[0],
        or(a[1], or(a[2], or(a[3], or(a[4], or(a[5], or(a[6], a[7])))))),
    )
}

fn mux4(a: bool, b: bool, c: bool, d: bool, sels: [bool; 2]) -> bool {
    mux(mux(a, b, sels[1]), mux(c, d, sels[1]), sels[0])
}

fn mux8(
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    sels: [bool; 3],
) -> bool {
    mux(
        mux(mux(a, b, sels[2]), mux(c, d, sels[2]), sels[1]),
        mux(mux(e, f, sels[2]), mux(g, h, sels[2]), sels[1]),
        sels[0],
    )
}

fn mux4way16(
    ass: [bool; 16],
    bss: [bool; 16],
    css: [bool; 16],
    dss: [bool; 16],
    sels: [bool; 2],
) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = mux4(ass[i], bss[i], css[i], dss[i], sels)
    }
    result
}

fn mux8way16(
    ass: [bool; 16],
    bss: [bool; 16],
    css: [bool; 16],
    dss: [bool; 16],
    ess: [bool; 16],
    fss: [bool; 16],
    gss: [bool; 16],
    hss: [bool; 16],
    sels: [bool; 3],
) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..ass.len() {
        result[i] = mux8(
            ass[i],
            bss[i],
            css[i],
            dss[i],
            ess[i],
            fss[i],
            gss[i],
            hss[i],
            sels,
        )
    }
    result
}

fn dmux4way(a: bool, sels: [bool; 2]) -> [bool; 4] {
    let mut result = [false; 4];
    let r1 = dmux(a, sels[0]);
    [
        and(not(sels[1]), r1.0),
        and(sels[1], r1.0),
        and(not(sels[1]), r1.1),
        and(sels[1], r1.1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(true, nand(false, false));
        assert_eq!(true, nand(true, false));
        assert_eq!(true, nand(false, true));
        assert_eq!(false, nand(true, true));
    }

    #[test]
    fn test_not() {
        assert_eq!(true, not(false));
        assert_eq!(false, not(true));
    }

    #[test]
    fn test_and() {
        assert_eq!(false, and(false, false));
        assert_eq!(false, and(true, false));
        assert_eq!(false, and(false, true));
        assert_eq!(true, and(true, true));
    }

    #[test]
    fn test_or() {
        assert_eq!(false, or(false, false));
        assert_eq!(true, or(true, false));
        assert_eq!(true, or(false, true));
        assert_eq!(true, or(true, true));
    }

    #[test]
    fn test_xor() {
        assert_eq!(false, xor(false, false));
        assert_eq!(true, xor(true, false));
        assert_eq!(true, xor(false, true));
        assert_eq!(false, xor(true, true));
    }

    #[test]
    fn test_mux() {
        assert_eq!(false, mux(false, false, false));
        assert_eq!(false, mux(false, true, false));
        assert_eq!(true, mux(true, false, false));
        assert_eq!(true, mux(true, true, false));
        assert_eq!(false, mux(false, false, true));
        assert_eq!(true, mux(false, true, true));
        assert_eq!(false, mux(true, false, true));
        assert_eq!(true, mux(true, true, true));
    }

    #[test]
    fn test_dmux() {
        assert_eq!((false, false), dmux(false, false));
        assert_eq!((false, false), dmux(false, true));
        assert_eq!((true, false), dmux(true, false));
        assert_eq!((false, true), dmux(true, true));
    }

    #[test]
    fn test_mux4() {
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
    fn test_dmux4way() {
        assert_eq!([true, false, false, false], dmux4way(true, [false, false]));
        assert_eq!([false, true, false, false], dmux4way(true, [false, true]));
        assert_eq!([false, false, true, false], dmux4way(true, [true, false]));
        assert_eq!([false, false, false, true], dmux4way(true, [true, true]));
    }

}
