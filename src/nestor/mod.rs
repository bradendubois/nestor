use crate::nestor::traits::MemoryMap;

mod cartridge;
mod cpu;
mod io;
mod traits;

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

    pub fn run(&mut self) {
        self.cartridge.read(0x4020);
        self.cpu.run();
    }
}