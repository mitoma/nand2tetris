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
        let is_c_command = instruction[15]; // C命令なら true
        let use_memory = instruction[12];
        let alu_y_value = mux16(a_register_current_value, in_memory, use_memory);

        let alu_result = alu(
            d_register_current_value,
            alu_y_value,
            instruction[11],
            instruction[10],
            instruction[9],
            instruction[8],
            instruction[7],
            instruction[6],
        );

        let dest = [
            and(is_c_command, instruction[5]),
            and(is_c_command, instruction[4]),
            and(is_c_command, instruction[3]),
        ];
        let jump = [
            and(is_c_command, instruction[2]),
            and(is_c_command, instruction[1]),
            and(is_c_command, instruction[0]),
        ];

        let a_register_store_value = mux16(instruction, alu_result.0, is_c_command);

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
                and(jump[1], alu_out_is_zero),
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
    use const_value::*;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use test_util::*;

    #[test]
    fn test_cpu() {
        let mut cpu = Cpu::new();

        let f = fs::File::open("test/CPU.cmp").unwrap();
        let reader = BufReader::new(f);

        let mut counter = 0;
        for line in reader.lines().skip(1) {
            counter = counter + 1;
            let l = line.unwrap();
            let tokens = l.split("|")
                .map(|str| str.trim())
                .filter(|str| !str.is_empty())
                .collect::<Vec<&str>>();

            println!("tokens={:?}", tokens);

            // input
            let time = tokens[0];
            if !time.ends_with("+") {
                continue;
            }

            //let in_memory = u16::from_str_radix(tokens[1], 2).unwrap();
            let in_memory = tokens[1].parse::<i16>().unwrap();
            let instruction = u16::from_str_radix(tokens[2], 2).unwrap();
            let reset = u16::from_str_radix(tokens[3], 2).unwrap() == 1;

            // output
            let out_memory = if tokens[4].starts_with("*") {
                (false, 0_i16)
            } else {
                (true, tokens[4].parse::<i16>().unwrap())
            };
            let write_memory = u16::from_str_radix(tokens[5], 2).unwrap() == 1;
            let address = tokens[6].parse::<i16>().unwrap();
            let pc = tokens[7].parse::<i16>().unwrap();
            let d_register = tokens[8].parse::<i16>().unwrap();

            let result = cpu.cycle(i2b(in_memory), u2b(instruction), reset);

            if out_memory.0 {
                assert_eq!(i2b(out_memory.1), result.out_memory);
            }
            assert_eq!(write_memory, result.write_memory);
            assert_eq!(i2b15(address), result.address_memory);
            assert_eq!(i2b15(pc), result.pc);
            assert_eq!(i2b(d_register), cpu.d_register.register(ZERO, false));
        }
    }
}
