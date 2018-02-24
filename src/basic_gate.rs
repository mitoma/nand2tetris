/// nand gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(true, nand(false, false));
/// assert_eq!(true, nand(true, false));
/// assert_eq!(true, nand(false, true));
/// assert_eq!(false, nand(true, true));
/// ```
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

/// not gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(true, not(false));
/// assert_eq!(false, not(true));
/// ```
pub fn not(a: bool) -> bool {
    nand(a, a)
}

/// and gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(false, and(false, false));
/// assert_eq!(false, and(true, false));
/// assert_eq!(false, and(false, true));
/// assert_eq!(true, and(true, true));
/// ```
pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

/// or gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(false, or(false, false));
/// assert_eq!(true, or(true, false));
/// assert_eq!(true, or(false, true));
/// assert_eq!(true, or(true, true));
/// ```
pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

/// xor gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(false, xor(false, false));
/// assert_eq!(true, xor(true, false));
/// assert_eq!(true, xor(false, true));
/// assert_eq!(false, xor(true, true));
/// ```
pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

/// mux gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!(false, mux(false, false, false));
/// assert_eq!(false, mux(false, true, false));
/// assert_eq!(true, mux(true, false, false));
/// assert_eq!(true, mux(true, true, false));
/// assert_eq!(false, mux(false, false, true));
/// assert_eq!(true, mux(false, true, true));
/// assert_eq!(false, mux(true, false, true));
/// assert_eq!(true, mux(true, true, true));
/// ```
pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

/// dmux gate
///
/// # Example
/// ```
/// use nand2tetlis::basic_gate::*;
///
/// assert_eq!([false, false], dmux(false, false));
/// assert_eq!([false, false], dmux(false, true));
/// assert_eq!([true, false], dmux(true, false));
/// assert_eq!([false, true], dmux(true, true));
/// ```
pub fn dmux(a: bool, sel: bool) -> [bool; 2] {
    [and(not(sel), a), and(sel, a)]
}
