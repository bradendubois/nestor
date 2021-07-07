use crate::nestor::traits::{Power};
use std::time::Duration;

mod cartridge;
mod cpu;
mod io;
mod traits;
mod enums;
mod apu;


#[allow(dead_code)]
const MASTER_CLOCK: f64 = 236.25 / 11.0;

#[allow(dead_code)]
const MASTER_TO_60: f64 = MASTER_CLOCK / 60.0;


pub struct Nestor {
    cpu: cpu::CPU6502,
    master: u64
}

impl Nestor {

    pub fn new(rom_path: &str) -> Nestor {

        let cartridge = cartridge::Cartridge::new(rom_path);

        Nestor {
            master: 0,
            cpu: cpu::CPU6502::new(cartridge, false, None, true)
        }
    }

    pub fn run(&mut self) {

        'core: loop {

            // Use CPU as a 'base' by which to advance the clock
            let cpu_cycles = self.cpu.cycle();

            // Master Clock : CPU Clock = 12:1
            let cycles = cpu_cycles * 12;

            self.master += cycles;

            // Master Clock : PPU Clock = 4:1
            // self.ppu.tick(cycles / 4);
            //
            // if self.master as f64 > MASTER_TO_60 {
            //     self.master %= MASTER_TO_60 as u64;
            //     self.apu.tick();
            // }
        }
    }
}


impl Power for Nestor {

    fn power_up(&mut self) {
        self.cpu.power_up();
    }

    fn reset(&mut self) {
        self.cpu.reset();
    }

}