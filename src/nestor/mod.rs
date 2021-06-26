use crate::nestor::traits::{Power};

mod cartridge;
mod cpu;
mod io;
mod traits;
mod enums;
mod apu;

pub struct Nestor {
    cpu: cpu::CPU6502
}

impl Nestor {

    pub fn new(rom_path: &str) -> Nestor {

        let cartridge = cartridge::Cartridge::new(rom_path);

        Nestor {
            cpu: cpu::CPU6502::new(cartridge, false, None)
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.power_up();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}
