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

    #[allow(dead_code)]
    fn push(&mut self, _value: u8) {

    }

    #[allow(dead_code)]
    fn pull(&mut self) -> u8 {
        0
    }

    #[allow(dead_code)]
    fn push_word(&mut self, _value: u16) {

    }

    #[allow(dead_code)]
    fn pull_word(&mut self) -> u16 {
        0
    }

    /*********** ALU **********/

    fn alu_adc(&mut self, value: u8) {
        let (r1, overflow1) = self.registers.a.overflowing_add(value);
        let (r2, overflow2) = r1.overflowing_add(if self.registers.carry() { 0x01 } else { 0x00 });

        self.registers.set_negative(false);
        self.registers.set_overflow((self.registers.a ^ r2) & (value ^ r2) & 0x80 != 0);
        self.registers.set_zero(r2 == 0);
        self.registers.set_carry(overflow1 | overflow2);
        self.registers.a = r2;
    }

    fn alu_and(&mut self, value: u8) {
        self.registers.set_negative(false);
        self.registers.set_zero(self.registers.a & value == 0);
    }

    fn alu_asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x80 != 0);
        result
    }

    fn alu_bit(&mut self, value: u8) {
        self.registers.set_negative(value & 0x80 != 0);
        self.registers.set_overflow(value & 0x40 != 0);
        self.registers.set_zero(self.registers.a & value == 0);
    }

    fn alu_cmp(&mut self, src: u8, value: u8) {
        self.registers.set_negative(src >= 0x80);
        self.registers.set_zero(src == value);
        self.registers.set_carry(src >= value);
    }

    fn alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_negative(true);
        self.registers.set_zero(result == 0);
        result
    }

    fn alu_xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
    }

    fn alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
        result
    }

    fn alu_lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    fn alu_ora(&mut self, value: u8) {
        self.registers.set_negative(false);
        self.registers.set_zero(self.registers.a | value == 0);
    }

    fn alu_rol(&mut self, value: u8) -> u8 {
        let result = value << 1 | if self.registers.carry() { 0x01 } else { 0x00 };
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x80 != 0);
        result
    }

    fn alu_ror(&mut self, value: u8) -> u8 {
        let result = value >> 1 | if self.registers.carry() { 0x80 } else { 0x00 };
        self.registers.set_negative(false);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    fn alu_sbc(&mut self, value: u8) {
        let (r1, overflow1) = self.registers.a.overflowing_sub(value);
        let (r2, overflow2) = r1.overflowing_sub(if self.registers.carry() { 0x01 } else { 0x00 });

        self.registers.set_negative(true);
        self.registers.set_overflow((self.registers.a ^ r2) & (value ^ r2) & 0x80 != 0);
        self.registers.set_zero(r2 == 0);
        self.registers.set_carry(overflow1 | overflow2);
        self.registers.a = r2;
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

    fn immediate(&mut self) -> u8 {
        self.byte()
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