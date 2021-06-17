mod instructions;
mod registers;

use crate::nestor::traits::{MemoryMap, Power};

use super::cartridge::Cartridge;
use super::io::IO;

use registers::Registers;


pub struct CPU6502 {
    registers: Registers,
    io: IO
}


impl CPU6502 {

    pub fn new(cartridge: Cartridge) -> CPU6502 {
        CPU6502 {
            registers: Registers::new(),
            io: IO::new(cartridge)
        }
    }

    pub fn run(&mut self) {
        let x = 0;
        self.call(x);
    }

    #[allow(dead_code)]
    pub fn byte(&mut self) -> u8 {
        0
    }

    #[allow(dead_code)]
    pub fn word(&mut self) -> u16 {
        0
    }


    pub fn relative(&mut self, bb: i8) -> (u16, bool) {
        let result = ((self.registers.pc as u32 as i32) + bb as i32) as u16;
        (result, (self.registers.pc & 0xFF00) != (result & 0xFF00))
    }
}


impl Power for CPU6502 {

    fn power_up(&mut self) {

        self.registers.power_up();

        for address in 0x4000..=0x4013 {
            self.io.write(address, 0x00);
        }

        self.io.write(0x4015, 0x00);
        self.io.write(0x4017, 0x00);

        self.run();
    }

    fn reset(&mut self) {

        self.registers.reset();

        self.io.write(0x4017, 0x00);
    }
}