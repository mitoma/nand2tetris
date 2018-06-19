use basic_gate::*;
use const_value::*;
use cpu::{Cpu, CpuResult};
use multi_gate::*;
use ram::{Ram16kHiSpeed, Ram32kHiSpeed};
use screen::Screen;

pub struct Computer {
    pub rom: Ram32kHiSpeed,
    pub ram: Ram16kHiSpeed,
    pub screen: Screen,
    pub cpu: Cpu,
    pub pre_cpu_result: CpuResult,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            rom: Ram32kHiSpeed::new(),
            ram: Ram16kHiSpeed::new(),
            screen: Screen::new(Ram16kHiSpeed::new()),
            cpu: Cpu::new(),
            pre_cpu_result: CpuResult {
                out_memory: [false; 16],
                write_memory: false,
                address_memory: [false; 15],
                pc: [false; 15],
            },
        }
    }

    pub fn cycle(&mut self, reset: bool) {
        let add15 = self.pre_cpu_result.address_memory;
        let add14 = [
            add15[0], add15[1], add15[2], add15[3], add15[4], add15[5], add15[6], add15[7],
            add15[8], add15[9], add15[10], add15[11], add15[12], add15[13],
        ];

        let ram_data = self.ram
            .ram(self.pre_cpu_result.out_memory, add14, not(add15[14]));
        let io_data = self.screen
            .ram
            .ram(self.pre_cpu_result.out_memory, add14, add15[14]);

        let in_memory = mux16(ram_data, io_data, add15[14]);

        let instruction = self.rom.ram(ZERO, self.pre_cpu_result.pc, false);
        self.pre_cpu_result = self.cpu.cycle(in_memory, instruction, reset);
    }
}
