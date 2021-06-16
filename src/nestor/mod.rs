use crate::nestor::traits::{MemoryMap, Power};

mod cartridge;
mod cpu;
mod io;
mod traits;
mod enums;

pub struct Nestor {
    cartridge: cartridge::Cartridge,
    cpu: cpu::CPU6502
}

impl Nestor {

    pub fn new() -> Nestor {
        Nestor {
            cartridge: cartridge::Cartridge::new(),
            cpu: cpu::CPU6502::new()
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.power_up();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}