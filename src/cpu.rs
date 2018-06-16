use alu::*;
use basic_gate::*;
use const_value::*;
use counter::*;
use flip_flap::*;
use multi_gate::*;

pub struct Cpu {
    a_register: Register,
    d_register: Register,
    pc: Counter,
}

pub struct CpuResult {
    out_memory: [bool; 16],     // outM
    write_memory: bool,         // writeM
    address_memory: [bool; 15], // addressM
    pc: [bool; 15],             // pc
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
    ) -> CpuResult {
        // 配線の都合で現在値を先に読みだしておく
        let a_register_current_value = self.a_register.register(ZERO, false);
        let d_register_current_value = self.d_register.register(ZERO, false);

        let is_c = instruction[0]; // C命令なら true

        let use_memory = instruction[3]; //

        let alu_y_value = mux16(a_register_current_value, in_memory, use_memory);

        let alu_result = alu(
            d_register_current_value,
            alu_y_value,
            instruction[4],
            instruction[5],
            instruction[6],
            instruction[7],
            instruction[8],
            instruction[9],
        );

        let dest = [instruction[10], instruction[11], instruction[12]];
        let jump = [instruction[13], instruction[14], instruction[15]];

        let a_register_store_value = mux16(alu_result.0, instruction, is_c);
        self.a_register.register(
            a_register_store_value,
            or(
                /* A命令の時 */ not(is_c),
                /* C命令の時のフラグ */ dest[0],
            ),
        );
        self.d_register.register(alu_result.0, dest[1]);

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

        // TODO ここが山だ
        // pub fn count(&mut self, a: [bool; 16], inc: bool, load: bool, reset: bool) -> [bool; 16] {
        // pc.count(a_register_current_value, /* TODO : load */ ,reset)

        let mut address_memory = [false; 15];
        address_memory.copy_from_slice(&a_register_current_value[0..15]);
        // 暫定の返り値
        CpuResult {
            out_memory: alu_result.0,
            write_memory: dest[2],
            address_memory: address_memory,
            pc: [false; 15],
        }
    }
}
