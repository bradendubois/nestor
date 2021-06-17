mod instructions;
mod registers;

use super::enums::OperandMode;
use super::io::IO;
use super::traits::{MemoryMap, Power};

use registers::Registers;
use crate::nestor::cartridge::Cartridge;


#[allow(dead_code)]
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
}


/// ALU Operations
impl CPU6502 {

    pub fn adc(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn and(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn asl(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn bcc(&mut self) -> u8 {
        0
    }

    pub fn bcs(&mut self) -> u8 {
        0
    }

    pub fn beq(&mut self) -> u8 {
        0
    }

    pub fn bit(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn bmi(&mut self) -> u8 {
        0
    }

    pub fn bne(&mut self) -> u8 {
        0
    }

    pub fn bpl(&mut self) -> u8 {
        0
    }

    pub fn brk(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn bvc(&mut self) -> u8 {
        0
    }

    pub fn bvs(&mut self) -> u8 {
        0
    }

    pub fn clc(&mut self) -> u8 {
        0
    }

    pub fn cld(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn cli(&mut self) -> u8 {
        0
    }

    pub fn clv(&mut self) -> u8 {
        0
    }

    pub fn cmp(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn cpx(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn cpy(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn dec(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn dex(&mut self) -> u8 {
        0
    }

    pub fn dey(&mut self) -> u8 {
        0
    }

    pub fn eor(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn inc(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn inx(&mut self) -> u8 {
        0
    }

    pub fn iny(&mut self) -> u8 {
        0
    }

    pub fn jmp(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn jsr(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn lda(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn ldx(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn ldy(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn lsr(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn nop(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn ora(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn pha(&mut self) -> u8 {
        0
    }

    pub fn php(&mut self) -> u8 {
        0
    }

    pub fn pla(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn plp(&mut self) -> u8 {
        0
    }

    pub fn rol(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn ror(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn rti(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn rts(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn sbc(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn sec(&mut self) -> u8 {
        0
    }

    pub fn sed(&mut self) -> u8 {
        0
    }

    pub fn sei(&mut self) -> u8 {
        0
    }

    pub fn sta(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn stx(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn sty(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    pub fn tax(&mut self) -> u8 {
        0
    }

    pub fn tay(&mut self) -> u8 {
        0
    }

    pub fn tsx(&mut self) -> u8 {
        0
    }

    pub fn txa(&mut self) -> u8 {
        0
    }

    pub fn txs(&mut self) -> u8 {
        0
    }

    pub fn tya(&mut self) -> u8 {
        0
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