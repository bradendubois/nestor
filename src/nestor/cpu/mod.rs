mod instructions;
mod registers;

use crate::nestor::traits::{MemoryMap, Power};

use super::cartridge::Cartridge;
use super::io::IO;

use registers::Registers;
use std::process::exit;


pub struct CPU6502 {
    registers: Registers,
    io: IO,

    pub execution_trace: Option<Vec<String>>,
    exit_code: Option<u16>,
}


impl CPU6502 {

    /// Constructor for a new CPU
    pub fn new(cartridge: Cartridge, trace: bool, exit_code: Option<u16>) -> CPU6502 {
        CPU6502 {
            registers: Registers::new(),
            io: IO::new(cartridge),
            exit_code,
            execution_trace: if trace || !exit_code.is_none() { Some(Vec::new()) } else { None }
        }
    }

    /// Run the CPU in an infinite loop until stopped
    pub fn run(&mut self) {
        'core: loop {

            self.trace_store(format!("{:04X}", self.registers.pc));

            if self.exit_code.is_some() && self.exit_code.unwrap() == self.registers.pc {
                break 'core
            }

            let opcode = self.byte();

            self.call(opcode);

            self.trace_store(format!("A:{:02X}", self.registers.a));
            self.trace_store(format!("X:{:02X}", self.registers.x));
            self.trace_store(format!("Y:{:02X}", self.registers.y));
            self.trace_store(format!("P:{:02X}", self.registers.p));
            self.trace_store(format!("SP:{:02X}", self.registers.s));
        }
    }

    fn trace_store(&mut self, message: String) {
        if let Some(trace) = self.execution_trace.as_mut() {
            trace.push(message);
        }
    }

    /**************************/
    /*     Program Control    */
    /**************************/

    /// Read one byte at the current PC value (incrementing the PC)
    pub fn byte(&mut self) -> u8 {
        let result = self.io.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.trace_store(format!("{:02X} ", result));
        result
    }

    /// Read one word (2 bytes) at the current PC value and adjust PC accordingly
    pub fn word(&mut self) -> u16 {
        (self.byte() as u16) | ((self.byte() as u16) << 8)
    }

    /**************************/
    /*       Stack Data       */
    /**************************/

    /// Push a byte onto the stack
    fn push(&mut self, value: u8) {
        self.io.write(self.registers.s as u16 + 0x0100, value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Pull (pop) a byte from the stack
    fn pull(&mut self) -> u8 {
        self.registers.s = self.registers.s.wrapping_add(1);
        self.io.read(self.registers.s as u16 + 0x0100)
    }

    /// Push a word (2 bytes) onto the stack
    fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    /// Pull (pop) a word (2 bytes) from the stack
    fn pull_word(&mut self) -> u16 {
        (self.pull() as u16) | ((self.pull() as u16) << 8)
    }

    /**************************/
    /*           ALU          */
    /**************************/

    /// ALU - Add (to accumulator) with Carry
    /// Flags: N Z C - - - - V
    fn alu_adc(&mut self, value: u8) {
        let (r1, overflow1) = self.registers.a.overflowing_add(value);
        let (r2, overflow2) = r1.overflowing_add(if self.registers.carry() { 0x01 } else { 0x00 });

        self.registers.set_negative(r2 >= 0x80);
        self.registers.set_overflow((self.registers.a ^ r2) & (value ^ r2) & 0x80 != 0);
        self.registers.set_zero(r2 == 0);
        self.registers.set_carry(overflow1 | overflow2);
        self.registers.a = r2;
    }

    /// ALU - AND with accumulator
    /// Flags: N Z - - - - - -
    fn alu_and(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a & value == 0);
    }

    /// ASL - Shift Left
    /// Flags: N Z C - - - - -
    fn alu_asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x80 != 0);
        result
    }

    /// ASL - Test Bits with Accumulator (Transfer bits 7, 6 into A, Z = Operand & A)
    /// Flags: N Z - - - - - V
    fn alu_bit(&mut self, value: u8) {
        self.registers.set_negative(value & 0x80 != 0);
        self.registers.set_overflow(value & 0x40 != 0);
        self.registers.set_zero(self.registers.a & value == 0);
    }

    /// CMP - Compare
    /// Flags: N Z C - - - - -
    fn alu_cmp(&mut self, src: u8, value: u8) {
        self.registers.set_negative(src.wrapping_sub(value) >= 0x80);
        self.registers.set_zero(src == value);
        self.registers.set_carry(src >= value);
    }

    /// DEC - Decrement a value
    /// Flags - N Z - - - - - -
    fn alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        result
    }

    /// ALU XOR (EOR) - Exclusive OR (XOR) with accumulator
    /// Flags: N Z - - - - - -
    fn alu_xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a == 0);
    }

    /// INC - Increment value (with wrapping)
    /// Flags: N Z - - - - - -
    fn alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        result
    }

    /// LSR - Shift Right
    /// Flags: N Z C - - - - -
    fn alu_lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    /// ORA - OR with accumulator
    /// Flags: N Z - - - - - -
    fn alu_ora(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.set_negative(self.registers.a >= 0x80);
        self.registers.set_zero(self.registers.a == 0);
    }

    /// ROL - Rotate Left (with C rotated in)
    /// Flags: N Z C - - - - -
    fn alu_rol(&mut self, value: u8) -> u8 {
        let result = value << 1 | if self.registers.carry() { 0x01 } else { 0x00 };
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x80 != 0);
        result
    }

    /// ROR - Rotate Right (with C rotated in)
    /// Flags: N Z C - - - - -
    fn alu_ror(&mut self, value: u8) -> u8 {
        let result = value >> 1 | if self.registers.carry() { 0x80 } else { 0x00 };
        self.registers.set_negative(result >= 0x80);
        self.registers.set_zero(result == 0);
        self.registers.set_carry(value & 0x01 != 0);
        result
    }

    /// SBC - Subtract with Carry
    /// Flags: N Z C - - - - V
    fn alu_sbc(&mut self, value: u8) {
        self.alu_adc(!value);
    }

    /**************************/
    /*    Addressing Modes    */
    /**************************/

    /// Accumulator register
    fn accumulator(&mut self) -> u8 {
        self.registers.a
    }

    /// Immediate byte, BB
    fn immediate(&mut self) -> u8 {
        self.byte()
    }

    /// Relative to PC, BB given, address is PC + (signed BB)
    fn relative(&mut self, bb: i8) -> (u16, bool) {
        let result = ((self.registers.pc as u32 as i32) + bb as i32) as u16;
        (result, (self.registers.pc & 0xFF00) != (result & 0xFF00))
    }

    /// Zero-Page, only LL given, interpreted as 00LL
    fn zero_page(&mut self) -> (u16, u8) {
        let address = self.byte() as u16;
        (address, self.io.read(address))
    }

    /// Zero-Page, given as LL, incremented by X without carry
    fn zero_page_x(&mut self) -> (u16, u8) {
        let address = self.byte().wrapping_add(self.registers.x) as u16;
        (address, self.io.read(address))
    }

    /// Zero-Page, given as LL, incremented by Y without carry
    fn zero_page_y(&mut self) -> (u16, u8) {
        let address = self.byte().wrapping_add(self.registers.y) as u16;
        (address, self.io.read(address))
    }

    /// Address HHLL given in little-endian as LLHH
    fn absolute(&mut self) -> (u16, u8) {
        let address = self.word();
        (address, self.io.read(address))
    }

    /// Address incremented by X, with carry
    fn absolute_x(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word.wrapping_add(self.registers.x as u16);
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    /// Address incremented by Y, with carry
    fn absolute_y(&mut self) -> (u16, u8, bool) {
        let word = self.word();
        let address = word.wrapping_add(self.registers.y as u16);
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    /// Word at (HHLL)
    fn indirect(&mut self) -> (u16, u8) {
        let word = self.word();
        let address = self.wrap_with_carry_bug(word);
        (address, self.io.read(address))
    }

    /// Word at Zero-Page, (LL+X, LL+X+1) without carry
    fn x_indirect(&mut self) -> (u16, u8) {
        let byte = self.byte().wrapping_add(self.registers.x);
        let address = self.wrap_with_carry_bug(byte as u16);
        (address, self.io.read(address))
    }

    /// Word at Zero-Page, (LL, LL+1) with carry
    fn indirect_y(&mut self) -> (u16, u8, bool) {
        let byte = self.byte();
        let word = self.wrap_with_carry_bug(byte as u16);
        let address = word.wrapping_add(self.registers.y as u16);
        (address, self.io.read(address), (word & 0xFF00) != (address & 0xFF00))
    }

    /**************************/

    /// Helper function for interpretation of words without carry - wrap on same page
    /// and do not 'correctly' cross pages: this affects the Indirect, X,Indirect, and
    /// Indirect,Y addressing modes
    fn wrap_with_carry_bug(&mut self, address: u16) -> u16 {
        let lower = self.io.read(address);
        let upper = self.io.read((address & 0xFF00) | ((address + 1) & 0x00FF));
        ((upper as u16) << 8) | (lower as u16)
    }
}


impl Power for CPU6502 {

    /// POWER - Initializes system state to normal values
    fn power_up(&mut self) {

        self.registers.power_up();

        for address in 0x4000..=0x4013 {
            self.io.write(address, 0x00);
        }

        self.io.write(0x4015, 0x00);
        self.io.write(0x4017, 0x00);
    }


    /// RESET - Adjusts the CPU (and registers) if a system reset occurs
    fn reset(&mut self) {

        self.registers.reset();

        self.io.write(0x4017, 0x00);
    }
}


#[cfg(test)]
mod test {

    use crate::nestor::cpu::CPU6502;
    use crate::nestor::cartridge::Cartridge;

    #[test]
    fn nestest() {

        let cartridge = Cartridge::new("./roms/testing/cpu/nestest/nestest.nes");
        let mut reference_file: Vec<String> = std::fs::read_to_string("./roms/testing/cpu/nestest/nestest-expected.txt").unwrap().split(' ').flat_map(str::parse::<String>).collect::<Vec<_>>();
        reference_file.reverse();

        let mut cpu = CPU6502::new(cartridge, true, Some(0xC66E));

        cpu.registers.pc = 0x0C000;
        cpu.registers.p = 0x24;
        cpu.run();

        let x = cpu.execution_trace.as_mut().unwrap();
        x.reverse();

        assert!(x.len() > 8000);

        while x.len() > 0 {
            let current = x.pop();
            let expect = reference_file.pop();

            assert_eq!(current.unwrap().trim(), expect.unwrap().trim());
        }
    }
}
