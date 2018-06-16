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

        let is_c_command = instruction[0]; // C命令なら true
        let use_memory = instruction[3];
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

        let a_register_store_value = mux16(alu_result.0, instruction, is_c_command);
        self.a_register.register(
            a_register_store_value,
            or(
                /* A命令の時 */ not(is_c_command),
                /* C命令の時のフラグ */ dest[0],
            ),
        );
        self.d_register.register(alu_result.0, dest[1]);

        let alu_out_is_zero = alu_result.1;
        let alu_out_is_negative = alu_result.2;
        let alu_out_is_positive = and(not(alu_out_is_zero), not(alu_out_is_negative));

        let load = or(
            or(
                and(jump[0], alu_out_is_negative),
                and(jump[0], alu_out_is_zero),
            ),
            and(jump[2], alu_out_is_positive),
        );
        let pc_result = self.pc.count(a_register_current_value, true, load, reset);

        CpuResult {
            out_memory: alu_result.0,
            write_memory: dest[2],
            address_memory: [
                a_register_current_value[0],
                a_register_current_value[1],
                a_register_current_value[2],
                a_register_current_value[3],
                a_register_current_value[4],
                a_register_current_value[5],
                a_register_current_value[6],
                a_register_current_value[7],
                a_register_current_value[8],
                a_register_current_value[9],
                a_register_current_value[10],
                a_register_current_value[11],
                a_register_current_value[12],
                a_register_current_value[13],
                a_register_current_value[14],
            ],
            pc: [
                pc_result[0],
                pc_result[1],
                pc_result[2],
                pc_result[3],
                pc_result[4],
                pc_result[5],
                pc_result[6],
                pc_result[7],
                pc_result[8],
                pc_result[9],
                pc_result[10],
                pc_result[11],
                pc_result[12],
                pc_result[13],
                pc_result[14],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util::*;

    //#[test]
    fn cpu_test() {
        let mut cpu = Cpu::new();
        let result = cpu.cycle(u2b(0b_0), u2b(0b_0011000000111001), false);
        assert_eq!(result.write_memory, false);
        assert_eq!(result.pc, u2b15(0b_0000_0000_0000_0001));
    }
}
