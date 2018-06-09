use counter::*;
use flip_flap::*;

pub struct Cpu {
    a_register: Register,
    d_register: Register,
    pc: Counter,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a_register: Register::new(),
            d_register: Register::new(),
            pc: Counter::new(),
        }
    }

    pub fn cycle(
        &mut self,
        in_memory: [bool; 16],
        instruction: [bool; 16],
        reset: bool,
    ) -> ([bool; 16], bool, [bool; 15], [bool; 15]) {
        let is_c = instruction[0];
        let comp = [
            instruction[3],
            instruction[4],
            instruction[5],
            instruction[6],
            instruction[7],
            instruction[8],
            instruction[9],
        ];
        let dest = [instruction[10], instruction[11], instruction[12]];
        let jump = [instruction[13], instruction[14], instruction[15]];

        // A命令時のvalue。これはいらないかも
        let value = [
            instruction[1],
            instruction[2],
            instruction[3],
            instruction[4],
            instruction[5],
            instruction[6],
            instruction[7],
            instruction[8],
            instruction[9],
            instruction[10],
            instruction[11],
            instruction[12],
            instruction[13],
            instruction[14],
            instruction[15],
        ];

        //    pub fn count(&mut self, a: [bool; 16], inc: bool, load: bool, reset: bool) -> [bool; 16] {
        //pc.count(/* TODO : next_value */[false; 16], /* TODO : load */ ,reset)

        // 暫定の返り値
        ([false; 16], false, [false; 15], [false; 15])
    }
}
