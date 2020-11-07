use basic_gate::*;
use const_value::*;
use cpu::{Cpu, CpuResult};
use multi_gate::*;
use ram::{Ram16kHiSpeed, Ram32kHiSpeed};
use screen::Screen;
use std::fs;
use std::io::{BufRead, BufReader};
use test_util::*;

pub struct Computer {
    pub rom: Ram32kHiSpeed,
    pub ram: Ram16kHiSpeed,
    pub screen: Screen,
    pub cpu: Cpu,
    pub pre_cpu_result: CpuResult,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            rom: Ram32kHiSpeed::default(),
            ram: Ram16kHiSpeed::default(),
            screen: Screen::new(Ram16kHiSpeed::default()),
            cpu: Cpu::default(),
            pre_cpu_result: CpuResult {
                out_memory: [false; 16],
                write_memory: false,
                address_memory: [false; 15],
                pc: [false; 15],
            },
        }
    }
}

impl Computer {
    pub fn load_rom(&mut self, program_path: String) {
        let f = fs::File::open(program_path).unwrap();
        let reader = BufReader::new(f);

        //let mut counter = 0;
        for (i, line) in reader.lines().enumerate() {
            let word = u16::from_str_radix(line.unwrap().trim(), 2).unwrap();
            println!("line:{:06}, word:{:016b}, ({})", i, word, word);
            self.rom.ram(u2b(word), u2b15(i as u16), true);
        }
    }

    pub fn cycle(&mut self, reset: bool) {
        let add15 = self.cpu.a_register.register(ZERO, false);
        let add14 = [
            add15[0], add15[1], add15[2], add15[3], add15[4], add15[5], add15[6], add15[7],
            add15[8], add15[9], add15[10], add15[11], add15[12], add15[13],
        ];

        let ram_data = self.ram.ram(
            self.pre_cpu_result.out_memory,
            add14,
            and(not(add15[14]), self.pre_cpu_result.write_memory),
        );
        let io_data = self.screen.ram.ram(
            self.pre_cpu_result.out_memory,
            add14,
            and(add15[14], self.pre_cpu_result.write_memory),
        );

        if and(not(add15[14]), self.pre_cpu_result.write_memory) {
            println!("                                                                            write memory!");
        }
        if and(add15[14], self.pre_cpu_result.write_memory) {
            println!("                                                                            write screen!");
        }

        let in_memory = mux16(ram_data, io_data, add15[14]);
        let instruction = self.rom.ram(ZERO, self.pre_cpu_result.pc, false);

        let a_register_current_value = self.cpu.a_register.register(ZERO, false);
        let d_register_current_value = self.cpu.d_register.register(ZERO, false);

        println!();
        println!("--------------------------------------------------------");
        println!("in_memory:1---2---3---4---, instruction:CxxAC-----D--J--",);
        println!(
            "address_memory:{:016b}",
            b152u(self.pre_cpu_result.address_memory),
        );
        println!(
            "in_memory:{:016b}, instruction:{:016b}, pc:{:04?}\nd_register:{:016b}, a_register:{:016b}",
            b2u(in_memory),
            b2u(instruction),
            b152u(self.pre_cpu_result.pc) ,
            b2u(d_register_current_value),
            b2u(a_register_current_value),
        );
        println!("execute");
        self.pre_cpu_result = self.cpu.cycle(in_memory, instruction, reset);

        let a_register_post_value = self.cpu.a_register.register(ZERO, false);
        let d_register_post_value = self.cpu.d_register.register(ZERO, false);
        println!(
            "d_register:{:016b}, a_register:{:016b}",
            b2u(d_register_post_value),
            b2u(a_register_post_value),
        );
        println!(
            "out_memory:{:016b}, address_memory:{:016b}",
            b2u(self.pre_cpu_result.out_memory),
            b152u(self.pre_cpu_result.address_memory),
        );
    }
}
