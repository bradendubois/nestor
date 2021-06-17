use crate::nestor::traits::{Power};

mod cartridge;
mod cpu;
mod io;
mod traits;
mod enums;

pub struct Nestor {
    cpu: cpu::CPU6502
}

impl Nestor {

    pub fn new() -> Nestor {

        let cartridge = cartridge::Cartridge::new();

        Nestor {
            cpu: cpu::CPU6502::new(cartridge)
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.power_up();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}