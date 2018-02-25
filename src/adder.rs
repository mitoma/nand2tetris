use basic_gate::*;

/// half_adder
///
/// return [carry, sum]
///
/// # Example
/// ```
/// use nand2tetlis::adder::*;
///
/// assert_eq!([false, false], half_adder(false, false));
/// assert_eq!([false, true], half_adder(false, true));
/// assert_eq!([false, true], half_adder(true, false));
/// assert_eq!([true, false], half_adder(true, true));
/// ```
pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [and(a, b), xor(a, b)]
}

/// full_adder
///
/// return [carry, sum]
///
/// # Example
/// ```
/// use nand2tetlis::adder::*;
///
/// assert_eq!([false, false], full_adder(false, false, false));
/// assert_eq!([false, true], full_adder(false, false, true));
/// assert_eq!([false, true], full_adder(false, true, false));
/// assert_eq!([true, false], full_adder(false, true, true));
/// assert_eq!([false, true], full_adder(true, false, false));
/// assert_eq!([true, false], full_adder(true, false, true));
/// assert_eq!([true, false], full_adder(true, true, false));
/// assert_eq!([true, true], full_adder(true, true, true));
/// ```
pub fn full_adder(a: bool, b: bool, carry: bool) -> [bool; 2] {
    let tmp1 = half_adder(a, b);
    let tmp2 = half_adder(tmp1[1], carry);
    [or(tmp1[0], tmp2[0]), tmp2[1]]
}

/// add16
///
/// # Example
/// ```
/// use nand2tetlis::adder::*;
/// use nand2tetlis::test_util::*;
///
/// for i in -1000..1000 {
///     for j in -1000..1000 {
///         assert_eq!(i + j,
///                    bool_array_to_i16(
///                      add16(i16_to_bool_array(i),
///                            i16_to_bool_array(j))));
///     }
/// }
/// ```
pub fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let tmp1 = half_adder(a[0], b[0]);
    let tmp2 = full_adder(a[1], b[1], tmp1[0]);
    let tmp3 = full_adder(a[2], b[2], tmp2[0]);
    let tmp4 = full_adder(a[3], b[3], tmp3[0]);
    let tmp5 = full_adder(a[4], b[4], tmp4[0]);
    let tmp6 = full_adder(a[5], b[5], tmp5[0]);
    let tmp7 = full_adder(a[6], b[6], tmp6[0]);
    let tmp8 = full_adder(a[7], b[7], tmp7[0]);
    let tmp9 = full_adder(a[8], b[8], tmp8[0]);
    let tmp10 = full_adder(a[9], b[9], tmp9[0]);
    let tmp11 = full_adder(a[10], b[10], tmp10[0]);
    let tmp12 = full_adder(a[11], b[11], tmp11[0]);
    let tmp13 = full_adder(a[12], b[12], tmp12[0]);
    let tmp14 = full_adder(a[13], b[13], tmp13[0]);
    let tmp15 = full_adder(a[14], b[14], tmp14[0]);
    let tmp16 = full_adder(a[15], b[15], tmp15[0]);
    [
        tmp1[1], tmp2[1], tmp3[1], tmp4[1], tmp5[1], tmp6[1], tmp7[1], tmp8[1], tmp9[1], tmp10[1],
        tmp11[1], tmp12[1], tmp13[1], tmp14[1], tmp15[1], tmp16[1],
    ]
}

/// inc16
///
/// # Example
/// ```
/// use nand2tetlis::adder::*;
/// use nand2tetlis::test_util::*;
///
/// for i in -1000..1000 {
///     assert_eq!(i + 1,
///        bool_array_to_i16(
///          inc16(i16_to_bool_array(i))));
/// }
/// ```
pub fn inc16(a: [bool; 16]) -> [bool; 16] {
    add16(
        a,
        [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ],
    )
}
