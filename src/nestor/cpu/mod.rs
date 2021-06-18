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
        self.registers.pc = 0x0C000;
        self.registers.p = 0x24;
        self.run();
    }

    pub fn run(&mut self) {
        let mut clock = 0;
        loop {
            let _s = String::new();

            print!("{:04X} ", self.registers.pc);
            let opcode = self.byte();

            self.call(opcode);
            clock+=1;
            println!("A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", self.registers.a, self.registers.x, self.registers.y, self.registers.p, self.registers.s);
            if clock == 9000 { std::process::exit(0) };
            // std::io::stdin().read_line(&mut s);
        }
    }

    pub fn byte(&mut self) -> u8 {
        let result = self.io.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        print!("{:02X} ", result);
        result
    }

    pub fn word(&mut self) -> u16 {
        (self.byte() as u16) | ((self.byte() as u16) << 8)
    }

    fn push(&mut self, value: u8) {
        self.io.write(self.registers.s as u16 + 0x0100, value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    fn pull(&mut self) -> u8 {
        self.registers.s = self.registers.s.wrapping_add(1);
        self.io.read(self.registers.s as u16 + 0x0100)
    }

    fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    fn pull_word(&mut self) -> u16 {
        (self.pull() as u16) | ((self.pull() as u16) << 8)
    }

    /*********** ALU **********/

    fn alu_adc(&mut self, value: u8) {
        let (r1, overflow1) = self.registers.a.overflowing_add(value);
        let (r2, overflow2) = r1.overflowing_add(if self.registers.carry() { 0x01 } else { 0x00 });

        self.registers.set_negative(r2 >= 0x80);
        self.registers.set_overflow((self.registers.a ^ r2) & (value ^ r2) & 0x80 != 0);
        self.registers.set_zero(r2 == 0);
        self.registers.set_carry(overflow1 | overflow2);
        self.registers.a = r2;
    }

    fn alu_and(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a & value == 0);
    }

    fn alu_asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.registers.set_negative(result >= 0x80);
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
        self.registers.set_negative(src.wrapping_sub(value) >= 0x80);
        self.registers.set_zero(src == value);
        self.registers.set_carry(src >= value);
    }

    fn alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        result
    }

    fn alu_xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a == 0);
    }

    fn alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        result
    }

    fn alu_lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    fn alu_ora(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a == 0);
    }

    fn alu_rol(&mut self, value: u8) -> u8 {
        let result = value << 1 | if self.registers.carry() { 0x01 } else { 0x00 };
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x80 != 0);
        result
    }

    fn alu_ror(&mut self, value: u8) -> u8 {
        let result = value >> 1 | if self.registers.carry() { 0x80 } else { 0x00 };
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    fn alu_sbc(&mut self, value: u8) {
        self.alu_adc(!value);
    }


    /*********** Addressing Modes **********/

    fn wrap_with_carry_bug(&mut self, address: u16) -> u16 {
        let lower = self.io.read(address);
        let upper = self.io.read((address & 0xFF00) | ((address + 1) & 0x00FF));
        ((upper as u16) << 8) | (lower as u16)
    }

    fn accumulator(&mut self) -> u8 {
        self.registers.a
    }

    fn absolute(&mut self) -> (u16, u8) {
        let address = self.word();
        (address, self.io.read(address))
    }

    fn absolute_x(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word.wrapping_add(self.registers.x as u16);
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    fn absolute_y(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word.wrapping_add(self.registers.y as u16);
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    fn immediate(&mut self) -> u8 {
        self.byte()
    }

    fn indirect(&mut self) -> (u16, u8) {
        let word = self.word();
        let address = self.wrap_with_carry_bug(word);
        (address, self.io.read(address))
    }

    fn x_indirect(&mut self) -> (u16, u8) {
        let byte = self.byte().wrapping_add(self.registers.x);
        let address = self.wrap_with_carry_bug(byte as u16);
        (address, self.io.read(address))
    }

    fn indirect_y(&mut self) -> (u16, u8, bool) {
        let byte = self.byte();
        let word = self.wrap_with_carry_bug(byte as u16);
        let address = word.wrapping_add(self.registers.y as u16);
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
