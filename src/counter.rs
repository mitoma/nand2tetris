use adder::*;
use const_value::*;
use flip_flap::*;
use multi_gate::*;

pub struct Counter {
    register: Register,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            register: Register::new(),
        }
    }

    pub fn count(&mut self, a: [bool; 16], inc: bool, load: bool, reset: bool) -> [bool; 16] {
        // feedbackループの都合でregisterを二回呼び出している
        let current_value = self.register.register(ZERO, false);
        let next_value = inc16(current_value);
        let calced_value = mux8way16(
            current_value, // f,f,f
            next_value,    // t,f,f
            a,             // f, t, f
            a,             // t, t, f
            ZERO,          // f, f, t
            ZERO,          // f, f, t
            ZERO,          // f, f, t
            ZERO,          // f, f, t
            [reset, load, inc],
        );
        self.register.register(calced_value, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util::*;

    #[test]
    fn test_counter() {
        let mut sut = Counter::new();
        let in_value = u2b(527);

        sut.count(u2b(47), false, true, false);

        assert_eq!(u2b(47), sut.count(in_value, false, false, false));
        assert_eq!(u2b(47), sut.count(in_value, false, false, true));
        assert_eq!(u2b(0), sut.count(in_value, false, false, false));
        assert_eq!(u2b(0), sut.count(in_value, true, false, false));
        assert_eq!(u2b(1), sut.count(in_value, true, false, false));
        assert_eq!(u2b(2), sut.count(in_value, true, false, false));
        assert_eq!(u2b(3), sut.count(in_value, true, false, false));
        assert_eq!(u2b(4), sut.count(in_value, true, true, false));
        assert_eq!(u2b(527), sut.count(in_value, true, false, false));
        assert_eq!(u2b(528), sut.count(in_value, true, false, false));
        assert_eq!(u2b(529), sut.count(in_value, true, false, false));
        assert_eq!(u2b(530), sut.count(in_value, false, false, false));
        assert_eq!(u2b(530), sut.count(in_value, false, false, false));
    }
}
