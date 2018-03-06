use flip_flap::*;
use multi_gate::*;

pub struct Ram8 {
    registers: [Register; 8],
}

impl Ram8 {
    pub fn new() -> Ram8 {
        Ram8 {
            registers: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 3], load: bool) -> [bool; 16] {
        let sel = dmux8way(load, address);
        mux8way16(
            self.registers[0].register(a, sel[0]),
            self.registers[1].register(a, sel[1]),
            self.registers[2].register(a, sel[2]),
            self.registers[3].register(a, sel[3]),
            self.registers[4].register(a, sel[4]),
            self.registers[5].register(a, sel[5]),
            self.registers[6].register(a, sel[6]),
            self.registers[7].register(a, sel[7]),
            address,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use const_value::*;
    use test_util::*;

    #[test]
    fn test_ram8() {
        let mut sut = Ram8::new();

        for i in 0_i16..7_i16 {
            let t = i2b(i);
            let address = [t[0], t[1], t[2]];
            assert_eq!(ZERO, sut.ram(FULL, address, false));
            assert_eq!(ZERO, sut.ram(FULL, address, true));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, true));
            assert_eq!(ZERO, sut.ram(ZERO, address, false));
        }
    }
}
