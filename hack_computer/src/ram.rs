use flip_flap::*;
use multi_gate::*;
use test_util::*;

pub struct Ram8 {
    pub registers: [Register; 8],
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

pub struct Ram64 {
    pub rams: [Ram8; 8],
}

impl Ram64 {
    pub fn new() -> Ram64 {
        Ram64 {
            rams: [
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
            ],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 6], load: bool) -> [bool; 16] {
        let upper = [address[0], address[1], address[2]];
        let lower = [address[3], address[4], address[5]];

        let sel = dmux8way(load, upper);
        mux8way16(
            self.rams[0].ram(a, lower, sel[0]),
            self.rams[1].ram(a, lower, sel[1]),
            self.rams[2].ram(a, lower, sel[2]),
            self.rams[3].ram(a, lower, sel[3]),
            self.rams[4].ram(a, lower, sel[4]),
            self.rams[5].ram(a, lower, sel[5]),
            self.rams[6].ram(a, lower, sel[6]),
            self.rams[7].ram(a, lower, sel[7]),
            upper,
        )
    }
}

pub struct Ram512 {
    pub rams: [Ram64; 8],
}

impl Ram512 {
    pub fn new() -> Ram512 {
        Ram512 {
            rams: [
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
            ],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 9], load: bool) -> [bool; 16] {
        let upper = [address[0], address[1], address[2]];
        let lower = [
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];

        let sel = dmux8way(load, upper);
        mux8way16(
            self.rams[0].ram(a, lower, sel[0]),
            self.rams[1].ram(a, lower, sel[1]),
            self.rams[2].ram(a, lower, sel[2]),
            self.rams[3].ram(a, lower, sel[3]),
            self.rams[4].ram(a, lower, sel[4]),
            self.rams[5].ram(a, lower, sel[5]),
            self.rams[6].ram(a, lower, sel[6]),
            self.rams[7].ram(a, lower, sel[7]),
            upper,
        )
    }
}

pub struct Ram4k {
    pub rams: [Ram512; 8],
}

impl Ram4k {
    pub fn new() -> Ram4k {
        Ram4k {
            rams: [
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
            ],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 12], load: bool) -> [bool; 16] {
        let upper = [address[0], address[1], address[2]];
        let lower = [
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];

        let sel = dmux8way(load, upper);
        mux8way16(
            self.rams[0].ram(a, lower, sel[0]),
            self.rams[1].ram(a, lower, sel[1]),
            self.rams[2].ram(a, lower, sel[2]),
            self.rams[3].ram(a, lower, sel[3]),
            self.rams[4].ram(a, lower, sel[4]),
            self.rams[5].ram(a, lower, sel[5]),
            self.rams[6].ram(a, lower, sel[6]),
            self.rams[7].ram(a, lower, sel[7]),
            upper,
        )
    }
}

// debug 用の native 実装
pub struct Ram16kHiSpeed {
    pub rams: [u16; 1024 * 16],
}

impl Ram16kHiSpeed {
    pub fn new() -> Ram16kHiSpeed {
        Ram16kHiSpeed {
            rams: [0; 1024 * 16],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 14], load: bool) -> [bool; 16] {
        let address = b142u(address) as usize;
        let result = u2b(self.rams[address]);
        if load {
            let a_u16 = b2u(a);
            self.rams[address] = a_u16;
        }
        result
    }
}

pub struct Ram16k {
    pub rams: [Ram4k; 4],
}

impl Ram16k {
    pub fn new() -> Ram16k {
        Ram16k {
            rams: [Ram4k::new(), Ram4k::new(), Ram4k::new(), Ram4k::new()],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 14], load: bool) -> [bool; 16] {
        let upper = [address[0], address[1]];
        let lower = [
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
            address[13],
        ];

        let sel = dmux4way(load, upper);
        mux4way16(
            self.rams[0].ram(a, lower, sel[0]),
            self.rams[1].ram(a, lower, sel[1]),
            self.rams[2].ram(a, lower, sel[2]),
            self.rams[3].ram(a, lower, sel[3]),
            upper,
        )
    }
}

// debug 用の native 実装
pub struct Ram32kHiSpeed {
    pub rams: [u16; 1024 * 32],
}

impl Ram32kHiSpeed {
    pub fn new() -> Ram32kHiSpeed {
        Ram32kHiSpeed {
            rams: [0; 1024 * 32],
        }
    }

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 15], load: bool) -> [bool; 16] {
        let address = b152u(address) as usize;
        let result = u2b(self.rams[address]);
        if load {
            let a_u16 = b2u(a);
            self.rams[address] = a_u16;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use const_value::*;

    #[test]
    fn test_ram8() {
        let mut sut = Ram8::new();

        for i in 0_u16..7_u16 {
            let t = u2b(i);
            let address = [t[0], t[1], t[2]];
            assert_eq!(ZERO, sut.ram(FULL, address, false));
            assert_eq!(ZERO, sut.ram(FULL, address, true));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, true));
            assert_eq!(ZERO, sut.ram(ZERO, address, false));
        }
    }

    #[test]
    #[ignore]
    fn test_ram16k() {
        let mut sut = Ram16k::new();

        for i in 0_u16..128_u16 {
            let t = u2b(i);
            let address = [
                t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8], t[9], t[10], t[11], t[12],
                t[13],
            ];
            assert_eq!(ZERO, sut.ram(FULL, address, false));
            assert_eq!(ZERO, sut.ram(FULL, address, true));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, true));
            assert_eq!(ZERO, sut.ram(ZERO, address, false));
        }
    }

    #[test]
    fn test_ram16k_hi_speed() {
        let mut sut = Ram16kHiSpeed::new();

        for i in 0_u16..128_u16 {
            let t = u2b(i);
            let address = [
                t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8], t[9], t[10], t[11], t[12],
                t[13],
            ];
            assert_eq!(ZERO, sut.ram(FULL, address, false));
            assert_eq!(ZERO, sut.ram(FULL, address, true));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, false));
            assert_eq!(FULL, sut.ram(ZERO, address, true));
            assert_eq!(ZERO, sut.ram(ZERO, address, false));
        }
    }

}
