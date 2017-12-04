fn main() {
    println!("Hello, world!");
}

fn bool_nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn bool_not(a: bool) -> bool {
    bool_nand(a, a)
}

fn bool_and(a: bool, b: bool) -> bool {
    bool_not(bool_nand(a, b))
}

fn bool_or(a: bool, b: bool) -> bool {
    bool_nand(bool_not(a), bool_not(b))
}

fn bool_xor(a: bool, b: bool) -> bool {
    bool_nand(bool_nand(a, bool_nand(a, b)), bool_nand(b, bool_nand(a, b)))
}

fn bool_mux(a: bool, b: bool, sel: bool) -> bool {
    bool_or(bool_and(a, bool_not(sel)), bool_and(b, sel))
}

fn bool_dmux(a: bool, sel: bool) -> (bool, bool) {
    (bool_and(bool_not(sel), a), bool_and(sel, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_nand() {
        assert_eq!(true, bool_nand(false, false));
        assert_eq!(true, bool_nand(true, false));
        assert_eq!(true, bool_nand(false, true));
        assert_eq!(false, bool_nand(true, true));
    }

    #[test]
    fn test_bool_not() {
        assert_eq!(true, bool_not(false));
        assert_eq!(false, bool_not(true));
    }

    #[test]
    fn test_bool_and() {
        assert_eq!(false, bool_and(false, false));
        assert_eq!(false, bool_and(true, false));
        assert_eq!(false, bool_and(false, true));
        assert_eq!(true, bool_and(true, true));
    }

    #[test]
    fn test_bool_or() {
        assert_eq!(false, bool_or(false, false));
        assert_eq!(true, bool_or(true, false));
        assert_eq!(true, bool_or(false, true));
        assert_eq!(true, bool_or(true, true));
    }

    #[test]
    fn test_bool_xor() {
        assert_eq!(false, bool_xor(false, false));
        assert_eq!(true, bool_xor(true, false));
        assert_eq!(true, bool_xor(false, true));
        assert_eq!(false, bool_xor(true, true));
    }

    #[test]
    fn test_bool_mux() {
        assert_eq!(false, bool_mux(false, false, false));
        assert_eq!(false, bool_mux(false, true, false));
        assert_eq!(true, bool_mux(true, false, false));
        assert_eq!(true, bool_mux(true, true, false));
        assert_eq!(false, bool_mux(false, false, true));
        assert_eq!(true, bool_mux(false, true, true));
        assert_eq!(false, bool_mux(true, false, true));
        assert_eq!(true, bool_mux(true, true, true));
    }

    #[test]
    fn test_bool_dmux() {
        assert_eq!((false, false), bool_dmux(false, false));
        assert_eq!((false, false), bool_dmux(false, true));
        assert_eq!((true, false), bool_dmux(true, false));
        assert_eq!((false, true), bool_dmux(true, true));
    }
}
