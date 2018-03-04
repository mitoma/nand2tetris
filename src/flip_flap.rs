use basic_gate::*;

pub struct Dff {
    pub pre_value: bool,
}

impl Dff {
    pub fn new(initial_state: bool) -> Dff {
        Dff {
            pre_value: initial_state,
        }
    }

    pub fn dff(&mut self, a: bool) -> bool {
        let result = self.pre_value;
        self.pre_value = a;
        result
    }
}

pub struct Bit {
    pub dff: Dff,
}

impl Bit {
    pub fn new() -> Bit {
        Bit {
            dff: Dff::new(false),
        }
    }

    pub fn bit(&mut self, a: bool, load: bool) -> bool {
        let v = mux(self.dff.pre_value, a, load);
        self.dff.dff(v)
    }
}

pub struct Register {
    pub bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Register {
        Register {
            bits: [
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
            ],
        }
    }

    pub fn register(&mut self, a: [bool; 16], load: bool) -> [bool; 16] {
        [
            self.bits[0].bit(a[0], load),
            self.bits[1].bit(a[1], load),
            self.bits[2].bit(a[2], load),
            self.bits[3].bit(a[3], load),
            self.bits[4].bit(a[4], load),
            self.bits[5].bit(a[5], load),
            self.bits[6].bit(a[6], load),
            self.bits[7].bit(a[7], load),
            self.bits[8].bit(a[8], load),
            self.bits[9].bit(a[9], load),
            self.bits[10].bit(a[10], load),
            self.bits[11].bit(a[11], load),
            self.bits[12].bit(a[12], load),
            self.bits[13].bit(a[13], load),
            self.bits[14].bit(a[14], load),
            self.bits[15].bit(a[15], load),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util::*;

    #[test]
    fn test_dff() {
        let mut gate = Dff::new(false);
        assert_eq!(false, gate.dff(true));
        assert_eq!(true, gate.dff(false));
        assert_eq!(false, gate.dff(false));
        assert_eq!(false, gate.dff(true));
        assert_eq!(true, gate.dff(true));
        assert_eq!(true, gate.dff(true));
    }

    #[test]
    fn test_bit() {
        let mut bit = Bit::new();
        assert_eq!(false, bit.bit(true, false));
        assert_eq!(false, bit.bit(true, true)); // set true
        assert_eq!(true, bit.bit(false, false));
        assert_eq!(true, bit.bit(false, false));
        assert_eq!(true, bit.bit(false, true)); // set false
        assert_eq!(false, bit.bit(false, false));
        assert_eq!(false, bit.bit(false, false));
        assert_eq!(false, bit.bit(false, false));
    }

    #[test]
    fn test_register() {
        let mut register = Register::new();
        assert_eq!(
            i2b(0b_0000_0000_0000_0000_i16),
            register.register(i2b(0b_0000_0000_0000_0000_i16), false)
        );
        assert_eq!(
            i2b(0b_0000_0000_0000_0000_i16),
            register.register(i2b(0b_0101_0101_0101_0101_i16), true)
        ); // set mem
        assert_eq!(
            i2b(0b_0101_0101_0101_0101_i16),
            register.register(i2b(0b_0000_0000_0000_0000_i16), false)
        );
        assert_eq!(
            i2b(0b_0101_0101_0101_0101_i16),
            register.register(i2b(0b_0000_0000_0000_0000_i16), true)
        ); // set mem
        assert_eq!(
            i2b(0b_0000_0000_0000_0000_i16),
            register.register(i2b(0b_0000_0000_0000_0000_i16), false)
        );
    }

}
