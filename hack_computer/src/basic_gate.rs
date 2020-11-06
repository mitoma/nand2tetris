pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(a: bool, sel: bool) -> [bool; 2] {
    [and(not(sel), a), and(sel, a)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_test() {
        assert_eq!(true, nand(false, false));
        assert_eq!(true, nand(true, false));
        assert_eq!(true, nand(false, true));
        assert_eq!(false, nand(true, true));
    }

    #[test]
    fn not_test() {
        assert_eq!(true, not(false));
        assert_eq!(false, not(true));
    }

    #[test]
    fn and_test() {
        assert_eq!(false, and(false, false));
        assert_eq!(false, and(true, false));
        assert_eq!(false, and(false, true));
        assert_eq!(true, and(true, true));
    }

    #[test]
    fn or_test() {
        assert_eq!(false, or(false, false));
        assert_eq!(true, or(true, false));
        assert_eq!(true, or(false, true));
        assert_eq!(true, or(true, true));
    }

    #[test]
    fn xor_test() {
        assert_eq!(false, xor(false, false));
        assert_eq!(true, xor(true, false));
        assert_eq!(true, xor(false, true));
        assert_eq!(false, xor(true, true));
    }

    #[test]
    fn mux_test() {
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
    fn dmux_test() {
        assert_eq!([false, false], dmux(false, false));
        assert_eq!([false, false], dmux(false, true));
        assert_eq!([true, false], dmux(true, false));
        assert_eq!([false, true], dmux(true, true));
    }
}
