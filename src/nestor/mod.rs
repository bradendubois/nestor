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

    pub fn new(rom_path: &str) -> Nestor {

        let cartridge = cartridge::Cartridge::new(rom_path);

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

    pub fn test(&mut self) {
        self.cpu.test();
    }
}

#[cfg(test)]
mod test {

    use crate::nestor::Nestor;

    const NES_TEST: &str = "./roms/testing/nestest.nes";

    #[test]
    fn nes_test() {

        let mut nestor = Nestor::new(NES_TEST);

        nestor.test();
    }
}