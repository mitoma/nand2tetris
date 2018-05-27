pub fn i16_to_bool_array(a: i16) -> [bool; 16] {
    [
        (a & (1 << 0)) != 0,
        (a & (1 << 1)) != 0,
        (a & (1 << 2)) != 0,
        (a & (1 << 3)) != 0,
        (a & (1 << 4)) != 0,
        (a & (1 << 5)) != 0,
        (a & (1 << 6)) != 0,
        (a & (1 << 7)) != 0,
        (a & (1 << 8)) != 0,
        (a & (1 << 9)) != 0,
        (a & (1 << 10)) != 0,
        (a & (1 << 11)) != 0,
        (a & (1 << 12)) != 0,
        (a & (1 << 13)) != 0,
        (a & (1 << 14)) != 0,
        (a & (1 << 15)) != 0,
    ]
}

pub fn i2b(a: i16) -> [bool; 16] {
    i16_to_bool_array(a)
}

pub fn u16_to_bool_array(a: u16) -> [bool; 16] {
    [
        (a & (1 << 0)) != 0,
        (a & (1 << 1)) != 0,
        (a & (1 << 2)) != 0,
        (a & (1 << 3)) != 0,
        (a & (1 << 4)) != 0,
        (a & (1 << 5)) != 0,
        (a & (1 << 6)) != 0,
        (a & (1 << 7)) != 0,
        (a & (1 << 8)) != 0,
        (a & (1 << 9)) != 0,
        (a & (1 << 10)) != 0,
        (a & (1 << 11)) != 0,
        (a & (1 << 12)) != 0,
        (a & (1 << 13)) != 0,
        (a & (1 << 14)) != 0,
        (a & (1 << 15)) != 0,
    ]
}

pub fn u2b(a: u16) -> [bool; 16] {
    u16_to_bool_array(a)
}

pub fn u2b14(a: u16) -> [bool; 14] {
    let mut target: [bool; 14] = [false; 14];
    target.copy_from_slice(&u2b(a)[0..14]);
    target
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn bool_array_to_i16(a: [bool; 16]) -> i16 {
      (if a[0]  { 1 << 0  } else { 0 })
    + (if a[1]  { 1 << 1  } else { 0 })
    + (if a[2]  { 1 << 2  } else { 0 })
    + (if a[3]  { 1 << 3  } else { 0 })
    + (if a[4]  { 1 << 4  } else { 0 })
    + (if a[5]  { 1 << 5  } else { 0 })
    + (if a[6]  { 1 << 6  } else { 0 })
    + (if a[7]  { 1 << 7  } else { 0 })
    + (if a[8]  { 1 << 8  } else { 0 })
    + (if a[9]  { 1 << 9  } else { 0 })
    + (if a[10] { 1 << 10 } else { 0 })
    + (if a[11] { 1 << 11 } else { 0 })
    + (if a[12] { 1 << 12 } else { 0 })
    + (if a[13] { 1 << 13 } else { 0 })
    + (if a[14] { 1 << 14 } else { 0 })
    + (if a[15] { 1 << 15 } else { 0 })
}

pub fn b2i(a: [bool; 16]) -> i16 {
    bool_array_to_i16(a)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn bool_array_to_u16(a: [bool; 16]) -> u16 {
      (if a[0]  { 1 << 0  } else { 0 })
    + (if a[1]  { 1 << 1  } else { 0 })
    + (if a[2]  { 1 << 2  } else { 0 })
    + (if a[3]  { 1 << 3  } else { 0 })
    + (if a[4]  { 1 << 4  } else { 0 })
    + (if a[5]  { 1 << 5  } else { 0 })
    + (if a[6]  { 1 << 6  } else { 0 })
    + (if a[7]  { 1 << 7  } else { 0 })
    + (if a[8]  { 1 << 8  } else { 0 })
    + (if a[9]  { 1 << 9  } else { 0 })
    + (if a[10] { 1 << 10 } else { 0 })
    + (if a[11] { 1 << 11 } else { 0 })
    + (if a[12] { 1 << 12 } else { 0 })
    + (if a[13] { 1 << 13 } else { 0 })
    + (if a[14] { 1 << 14 } else { 0 })
    + (if a[15] { 1 << 15 } else { 0 })
}

pub fn b2u(a: [bool; 16]) -> u16 {
    bool_array_to_u16(a)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn b142u(a: [bool; 14]) -> u16 {
      (if a[0]  { 1 << 0  } else { 0 })
    + (if a[1]  { 1 << 1  } else { 0 })
    + (if a[2]  { 1 << 2  } else { 0 })
    + (if a[3]  { 1 << 3  } else { 0 })
    + (if a[4]  { 1 << 4  } else { 0 })
    + (if a[5]  { 1 << 5  } else { 0 })
    + (if a[6]  { 1 << 6  } else { 0 })
    + (if a[7]  { 1 << 7  } else { 0 })
    + (if a[8]  { 1 << 8  } else { 0 })
    + (if a[9]  { 1 << 9  } else { 0 })
    + (if a[10] { 1 << 10 } else { 0 })
    + (if a[11] { 1 << 11 } else { 0 })
    + (if a[12] { 1 << 12 } else { 0 })
    + (if a[13] { 1 << 13 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i16_to_bool_array_test() {
        assert_eq!(
            [
                true, true, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            i16_to_bool_array(3)
        );
        assert_eq!(
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            i16_to_bool_array(0)
        );
        assert_eq!(
            [
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true,
            ],
            i16_to_bool_array(-1)
        );
        assert_eq!(
            [
                false, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true,
            ],
            i16_to_bool_array(-2)
        );
        assert_eq!(
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            i16_to_bool_array(i16::min_value())
        );
        assert_eq!(
            [
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, false,
            ],
            i16_to_bool_array(i16::max_value())
        );
    }

    #[test]
    fn bool_array_to_i16_test() {
        assert_eq!(
            0,
            bool_array_to_i16([
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ])
        );
        assert_eq!(
            1,
            bool_array_to_i16([
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ])
        );
        assert_eq!(
            -1,
            bool_array_to_i16([
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true,
            ])
        );
        assert_eq!(
            -2,
            bool_array_to_i16([
                false, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true,
            ])
        );
    }
}
