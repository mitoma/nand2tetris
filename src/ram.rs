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

    pub fn ram(&mut self, a: [bool; 16], address: [bool; 16], load: bool) -> [bool; 16] {
        let dmux_sel = [address[0], address[1], address[2]];
        let sel = dmux8way(load, dmux_sel);
        mux8way16(
            self.registers[0].register(a, sel[0]),
            self.registers[1].register(a, sel[1]),
            self.registers[2].register(a, sel[2]),
            self.registers[3].register(a, sel[3]),
            self.registers[4].register(a, sel[4]),
            self.registers[5].register(a, sel[5]),
            self.registers[6].register(a, sel[6]),
            self.registers[7].register(a, sel[7]),
            dmux_sel,
        )
    }
}
