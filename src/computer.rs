use cpu::{Cpu, CpuResult};
use ram::{Ram16kHiSpeed, Ram32kHiSpeed};
use screen::Screen;

pub struct Computer {
    pub rom: Ram32kHiSpeed,
    pub ram: Ram16kHiSpeed,
    pub screen: Screen,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            rom: Ram32kHiSpeed::new(),
            ram: Ram16kHiSpeed::new(),
            screen: Screen::new(Ram16kHiSpeed::new()),
        }
    }

    pub fn cycle(&mut self, reset: bool) {}
}
