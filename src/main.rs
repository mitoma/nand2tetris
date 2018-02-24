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

fn dmux(a: bool, sel: bool) -> [bool; 2] {
    [and(not(sel), a), and(sel, a)]
}

// multi gate
fn not16(ass: [bool; 16]) -> [bool; 16] {
    [
        not(ass[0]),
        not(ass[1]),
        not(ass[2]),
        not(ass[3]),
        not(ass[4]),
        not(ass[5]),
        not(ass[6]),
        not(ass[7]),
        not(ass[8]),
        not(ass[9]),
        not(ass[10]),
        not(ass[11]),
        not(ass[12]),
        not(ass[13]),
        not(ass[14]),
        not(ass[15]),
    ]
}

fn and16(ass: [bool; 16], bss: [bool; 16]) -> [bool; 16] {
    [
        and(ass[0], bss[0]),
        and(ass[1], bss[1]),
        and(ass[2], bss[2]),
        and(ass[3], bss[3]),
        and(ass[4], bss[4]),
        and(ass[5], bss[5]),
        and(ass[6], bss[6]),
        and(ass[7], bss[7]),
        and(ass[8], bss[8]),
        and(ass[9], bss[9]),
        and(ass[10], bss[10]),
        and(ass[11], bss[11]),
        and(ass[12], bss[12]),
        and(ass[13], bss[13]),
        and(ass[14], bss[14]),
        and(ass[15], bss[15]),
    ]
}

fn or16(ass: [bool; 16], bss: [bool; 16]) -> [bool; 16] {
    [
        or(ass[0], bss[0]),
        or(ass[1], bss[1]),
        or(ass[2], bss[2]),
        or(ass[3], bss[3]),
        or(ass[4], bss[4]),
        or(ass[5], bss[5]),
        or(ass[6], bss[6]),
        or(ass[7], bss[7]),
        or(ass[8], bss[8]),
        or(ass[9], bss[9]),
        or(ass[10], bss[10]),
        or(ass[11], bss[11]),
        or(ass[12], bss[12]),
        or(ass[13], bss[13]),
        or(ass[14], bss[14]),
        or(ass[15], bss[15]),
    ]
}

fn mux16(ass: [bool; 16], bss: [bool; 16], sel: bool) -> [bool; 16] {
    [
        mux(ass[0], bss[0], sel),
        mux(ass[1], bss[1], sel),
        mux(ass[2], bss[2], sel),
        mux(ass[3], bss[3], sel),
        mux(ass[4], bss[4], sel),
        mux(ass[5], bss[5], sel),
        mux(ass[6], bss[6], sel),
        mux(ass[7], bss[7], sel),
        mux(ass[8], bss[8], sel),
        mux(ass[9], bss[9], sel),
        mux(ass[10], bss[10], sel),
        mux(ass[11], bss[11], sel),
        mux(ass[12], bss[12], sel),
        mux(ass[13], bss[13], sel),
        mux(ass[14], bss[14], sel),
        mux(ass[15], bss[15], sel),
    ]
}

fn or8way(a: [bool; 8]) -> bool {
    or(
        or(or(a[0], a[1]), or(a[2], a[3])),
        or(or(a[4], a[5]), or(a[6], a[7])),
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
    [
        mux4(ass[0], bss[0], css[0], dss[0], sels),
        mux4(ass[1], bss[1], css[1], dss[1], sels),
        mux4(ass[2], bss[2], css[2], dss[2], sels),
        mux4(ass[3], bss[3], css[3], dss[3], sels),
        mux4(ass[4], bss[4], css[4], dss[4], sels),
        mux4(ass[5], bss[5], css[5], dss[5], sels),
        mux4(ass[6], bss[6], css[6], dss[6], sels),
        mux4(ass[7], bss[7], css[7], dss[7], sels),
        mux4(ass[8], bss[8], css[8], dss[8], sels),
        mux4(ass[9], bss[9], css[9], dss[9], sels),
        mux4(ass[10], bss[10], css[10], dss[10], sels),
        mux4(ass[11], bss[11], css[11], dss[11], sels),
        mux4(ass[12], bss[12], css[12], dss[12], sels),
        mux4(ass[13], bss[13], css[13], dss[13], sels),
        mux4(ass[14], bss[14], css[14], dss[14], sels),
        mux4(ass[15], bss[15], css[15], dss[15], sels),
    ]
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
    [
        mux8(
            ass[0],
            bss[0],
            css[0],
            dss[0],
            ess[0],
            fss[0],
            gss[0],
            hss[0],
            sels,
        ),
        mux8(
            ass[1],
            bss[1],
            css[1],
            dss[1],
            ess[1],
            fss[1],
            gss[1],
            hss[1],
            sels,
        ),
        mux8(
            ass[2],
            bss[2],
            css[2],
            dss[2],
            ess[2],
            fss[2],
            gss[2],
            hss[2],
            sels,
        ),
        mux8(
            ass[3],
            bss[3],
            css[3],
            dss[3],
            ess[3],
            fss[3],
            gss[3],
            hss[3],
            sels,
        ),
        mux8(
            ass[4],
            bss[4],
            css[4],
            dss[4],
            ess[4],
            fss[4],
            gss[4],
            hss[4],
            sels,
        ),
        mux8(
            ass[5],
            bss[5],
            css[5],
            dss[5],
            ess[5],
            fss[5],
            gss[5],
            hss[5],
            sels,
        ),
        mux8(
            ass[6],
            bss[6],
            css[6],
            dss[6],
            ess[6],
            fss[6],
            gss[6],
            hss[6],
            sels,
        ),
        mux8(
            ass[7],
            bss[7],
            css[7],
            dss[7],
            ess[7],
            fss[7],
            gss[7],
            hss[7],
            sels,
        ),
        mux8(
            ass[8],
            bss[8],
            css[8],
            dss[8],
            ess[8],
            fss[8],
            gss[8],
            hss[8],
            sels,
        ),
        mux8(
            ass[9],
            bss[9],
            css[9],
            dss[9],
            ess[9],
            fss[9],
            gss[9],
            hss[9],
            sels,
        ),
        mux8(
            ass[10],
            bss[10],
            css[10],
            dss[10],
            ess[10],
            fss[10],
            gss[10],
            hss[10],
            sels,
        ),
        mux8(
            ass[11],
            bss[11],
            css[11],
            dss[11],
            ess[11],
            fss[11],
            gss[11],
            hss[11],
            sels,
        ),
        mux8(
            ass[12],
            bss[12],
            css[12],
            dss[12],
            ess[12],
            fss[12],
            gss[12],
            hss[12],
            sels,
        ),
        mux8(
            ass[13],
            bss[13],
            css[13],
            dss[13],
            ess[13],
            fss[13],
            gss[13],
            hss[13],
            sels,
        ),
        mux8(
            ass[14],
            bss[14],
            css[14],
            dss[14],
            ess[14],
            fss[14],
            gss[14],
            hss[14],
            sels,
        ),
        mux8(
            ass[15],
            bss[15],
            css[15],
            dss[15],
            ess[15],
            fss[15],
            gss[15],
            hss[15],
            sels,
        ),
    ]
}

fn dmux4way(a: bool, sels: [bool; 2]) -> [bool; 4] {
    let r1 = dmux(a, sels[0]);
    [
        and(not(sels[1]), r1[0]),
        and(sels[1], r1[0]),
        and(not(sels[1]), r1[1]),
        and(sels[1], r1[1]),
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
        assert_eq!([false, false], dmux(false, false));
        assert_eq!([false, false], dmux(false, true));
        assert_eq!([true, false], dmux(true, false));
        assert_eq!([false, true], dmux(true, true));
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
