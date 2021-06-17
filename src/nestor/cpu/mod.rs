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

    pub fn test(&mut self) {
        self.registers.pc = 0x0c000;
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let mut s = String::new();

            print!("program counter {:#06X} ; ", self.registers.pc);
            let opcode = self.byte();
            println!("fetched {:#04X}; A:{:#02X} X:{:#02X} Y:{:#02X}", opcode, self.registers.a,  self.registers.x, self.registers.y);

            self.call(opcode);

            std::io::stdin().read_line(&mut s);
            println!("program counter {:#06X}                A:{:#02X} X:{:#02X} Y:{:#02X}", self.registers.pc, self.registers.a, self.registers.x, self.registers.y);

            std::io::stdin().read_line(&mut s);
        }
    }

    #[allow(dead_code)]
    pub fn byte(&mut self) -> u8 {
        println!("         PC AT: {:#06X}", self.registers.pc);
        let result = self.io.read(self.registers.pc);
        println!("BYTE: {:#04X}", result);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        println!("PC ADJUSTED TO: {:#06X}", self.registers.pc);
        result
    }

    #[allow(dead_code)]
    pub fn word(&mut self) -> u16 {
        let lower = self.byte();
        let higher = self.byte();
        let address = ((higher as u16) << 8) | (lower as u16);
        println!("lower: {:#06X}, higher: {:#06X}, formed: {:#06X}", lower, higher, address);
        address
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

    fn accumulator(&mut self) -> u8 {
        self.registers.a
    }

    fn absolute(&mut self) -> (u16, u8) {
        let address = self.word();
        (address, self.io.read(address))
    }

    fn absolute_x(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word + self.registers.x as u16;
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    fn absolute_y(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word + self.registers.y as u16;
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    fn immediate(&mut self) -> u8 {
        self.byte()
    }

    fn indirect(&mut self) -> (u16, u8) {
        let word = self.word();
        let lower = self.io.read(word);
        let upper = self.io.read((word & 0xFF00) | ((word + 1) & 0x00FF));
        let address = ((upper as u16) << 8) | (lower as u16);
        (address, self.io.read(address))
    }

    fn x_indirect(&mut self) -> (u16, u8) {
        let byte = self.byte();
        let address = self.io.read(byte.wrapping_add(self.registers.x) as u16) as u16;
        (address, self.io.read(address))
    }

    fn indirect_y(&mut self) -> (u16, u8, bool) {
        let byte = self.byte();
        let lower = self.io.read(byte as u16);
        let higher = self.io.read(byte as u16 + 1);
        let word = ((higher as u16) << 8) | (lower as u16);
        let address = word + self.registers.y as u16;
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    fn relative(&mut self, bb: i8) -> (u16, bool) {
        let result = ((self.registers.pc as u32 as i32) + bb as i32) as u16;
        (result, (self.registers.pc & 0xFF00) != (result & 0xFF00))
    }

    fn zero_page(&mut self) -> (u16, u8) {
        let address = self.byte() as u16;
        (address, self.io.read(address))
    }

    fn zero_page_x(&mut self) -> (u16, u8) {
        let address = self.byte().wrapping_add(self.registers.x) as u16;
        (address, self.io.read(address))
    }

    fn zero_page_y(&mut self) -> (u16, u8) {
        let address = self.byte().wrapping_add(self.registers.y) as u16;
        (address, self.io.read(address))
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
