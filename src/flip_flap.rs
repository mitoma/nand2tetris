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
        return result;
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
        return self.dff.dff(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut register = Bit::new();
        assert_eq!(false, register.bit(true, false));
        assert_eq!(false, register.bit(true, true)); // set true
        assert_eq!(true, register.bit(false, false));
        assert_eq!(true, register.bit(false, false));
        assert_eq!(true, register.bit(false, true)); // set false
        assert_eq!(false, register.bit(false, false));
        assert_eq!(false, register.bit(false, false));
        assert_eq!(false, register.bit(false, false));
    }

}
