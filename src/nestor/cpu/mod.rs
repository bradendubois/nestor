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

    fn push(&mut self, _value: u8) {

    }

    fn pull(&mut self) -> u8 {
        0
    }

    /*********** Addressing Modes **********/

    #[allow(dead_code)]
    fn accumulator(&mut self) -> u8 {
        self.registers.a
    }

    fn absolute(&mut self) -> u16 {
        self.word()
    }

    fn absolute_x(&mut self) -> (u16, bool) {
        let word = self.word();
        let result = word + self.registers.x as u16;
        (result, (word & 0xFF00) != (result & 0xFF00))
    }

    fn absolute_y(&mut self) -> (u16, bool) {
        let word = self.word();
        let result = word + self.registers.y as u16;
        (result, (word & 0xFF00) != (result & 0xFF00))
    }

    fn immediate(&mut self) -> u16 {
        self.byte() as u16
    }

    fn indirect(&mut self) -> u16 {
        let word = self.word();
        let lower = self.io.read(word);
        let upper = self.io.read((word & 0xFF00) | ((word + 1) & 0x00FF));
        ((upper as u16) << 8) | (lower as u16)
    }

    fn x_indirect(&mut self) -> u16 {
        let byte = self.byte();
        self.io.read(byte.wrapping_add(self.registers.x) as u16) as u16
    }

    fn indirect_y(&mut self) -> (u16, bool) {
        let byte = self.byte();
        let word = self.io.read_word(byte as u16);
        let result = word + self.registers.y as u16;
        (result, (word & 0xFF00) != (result & 0xFF00))
    }

    fn relative(&mut self, bb: i8) -> (u16, bool) {
        let result = ((self.registers.pc as u32 as i32) + bb as i32) as u16;
        (result, (self.registers.pc & 0xFF00) != (result & 0xFF00))
    }

    fn zero_page(&mut self) -> u16 {
        self.byte() as u16
    }

    fn zero_page_x(&mut self) -> u16 {
        self.byte().wrapping_add(self.registers.x) as u16
    }

    fn zero_page_y(&mut self) -> u16 {
        self.byte().wrapping_add(self.registers.y) as u16
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