use crate::nestor::cpu::CPU6502;

use crate::nestor::enums::OperandMode::*;
use crate::nestor::enums::OperandMode;

impl CPU6502 {


    pub fn call(&mut self, opcode: u8) -> u8 {

        #[allow(unreachable_patterns)]
        match opcode {
            0x00 => self.brk(Implied),
            0x01 => self.ora(IndirectX),
            0x02 => self.jam(),
            0x03 => self.slo(IndirectX),
            0x04 => self.nop(ZeroPage),
            0x05 => self.ora(ZeroPage),
            0x06 => self.asl(ZeroPage),
            0x07 => self.slo(ZeroPage),
            0x08 => self.php(),
            0x09 => self.ora(Immediate),
            0x0A => self.asl(Accumulator),
            0x0B => self.anc(Immediate),
            0x0C => self.nop(Absolute),
            0x0D => self.ora(Absolute),
            0x0E => self.asl(Absolute),
            0x0F => self.slo(Absolute),

            0x10 => self.bpl(),
            0x11 => self.ora(IndirectY),
            0x12 => self.jam(),
            0x13 => self.slo(IndirectY),
            0x14 => self.nop(ZeroPageX),
            0x15 => self.ora(ZeroPageX),
            0x16 => self.asl(ZeroPageX),
            0x17 => self.slo(ZeroPageX),
            0x18 => self.clc(),
            0x19 => self.ora(AbsoluteY),
            0x1A => self.nop(Implied),
            0x1B => self.slo(AbsoluteY),
            0x1C => self.nop(AbsoluteX),
            0x1D => self.ora(AbsoluteX),
            0x1E => self.asl(AbsoluteX),
            0x1F => self.slo(AbsoluteX),

            0x20 => self.jsr(Absolute),
            0x21 => self.and(IndirectX),
            0x22 => self.jam(),
            0x23 => self.rla(IndirectX),
            0x24 => self.bit(ZeroPage),
            0x25 => self.and(ZeroPage),
            0x26 => self.rol(ZeroPage),
            0x27 => self.rla(ZeroPage),
            0x28 => self.plp(),
            0x29 => self.and(Immediate),
            0x2A => self.rol(Accumulator),
            0x2B => self.and(Immediate),
            0x2C => self.bit(Absolute),
            0x2D => self.and(Absolute),
            0x2E => self.rol(Absolute),
            0x2F => self.rla(Absolute),

            0x30 => self.bmi(),
            0x31 => self.and(IndirectY),
            0x32 => self.jam(),
            0x33 => self.rla(IndirectY),
            0x34 => self.nop(ZeroPageX),
            0x35 => self.and(ZeroPageX),
            0x36 => self.rol(ZeroPageX),
            0x37 => self.rla(ZeroPageX),
            0x38 => self.sec(),
            0x39 => self.and(AbsoluteY),
            0x3A => self.nop(Implied),
            0x3B => self.rla(AbsoluteY),
            0x3C => self.nop(AbsoluteX),
            0x3D => self.and(AbsoluteX),
            0x3E => self.rol(AbsoluteX),
            0x3F => self.rla(AbsoluteX),

            0x40 => self.rti(Implied),
            0x41 => self.eor(IndirectX),
            0x42 => self.jam(),
            0x43 => self.sre(IndirectX),
            0x44 => self.nop(ZeroPage),
            0x45 => self.eor(ZeroPage),
            0x46 => self.lsr(ZeroPage),
            0x47 => self.sre(ZeroPage),
            0x48 => self.pha(),
            0x49 => self.eor(Immediate),
            0x4A => self.lsr(Accumulator),
            0x4B => self.alr(Immediate),
            0x4C => self.jmp(Absolute),
            0x4D => self.eor(Absolute),
            0x4E => self.lsr(Absolute),
            0x4F => self.sre(Absolute),

            0x50 => self.bvc(),
            0x51 => self.eor(IndirectY),
            0x52 => self.jam(),
            0x53 => self.sre(IndirectY),
            0x54 => self.nop(ZeroPageX),
            0x55 => self.eor(ZeroPageX),
            0x56 => self.lsr(ZeroPageX),
            0x57 => self.sre(ZeroPageX),
            0x58 => self.cli(),
            0x59 => self.eor(AbsoluteY),
            0x5A => self.nop(Implied),
            0x5B => self.sre(AbsoluteY),
            0x5C => self.nop(AbsoluteX),
            0x5D => self.eor(AbsoluteX),
            0x5E => self.lsr(AbsoluteX),
            0x5F => self.sre(AbsoluteX),

            0x60 => self.rts(Implied),
            0x61 => self.adc(IndirectX),
            0x62 => self.jam(),
            0x63 => self.rra(IndirectX),
            0x64 => self.nop(ZeroPage),
            0x65 => self.adc(ZeroPage),
            0x66 => self.ror(ZeroPage),
            0x67 => self.rra(ZeroPage),
            0x68 => self.pla(Implied),
            0x69 => self.adc(Immediate),
            0x6A => self.ror(Accumulator),
            0x6B => self.arr(Immediate),
            0x6C => self.jmp(Indirect),
            0x6D => self.adc(Absolute),
            0x6E => self.ror(Absolute),
            0x6F => self.rra(Absolute),

            0x70 => self.bvs(),
            0x71 => self.adc(IndirectY),
            0x72 => self.jam(),
            0x73 => self.rra(IndirectY),
            0x74 => self.nop(ZeroPageX),
            0x75 => self.adc(ZeroPageX),
            0x76 => self.ror(ZeroPageX),
            0x77 => self.rra(ZeroPageX),
            0x78 => self.sei(),
            0x79 => self.adc(AbsoluteY),
            0x7A => self.nop(Implied),
            0x7B => self.rra(AbsoluteY),
            0x7C => self.nop(AbsoluteX),
            0x7D => self.adc(AbsoluteX),
            0x7E => self.ror(AbsoluteX),
            0x7F => self.rra(AbsoluteX),

            0x80 => self.nop(Immediate),
            0x81 => self.sta(IndirectX),
            0x82 => self.nop(Immediate),
            0x83 => self.sax(IndirectX),
            0x84 => self.sty(ZeroPage),
            0x85 => self.sta(ZeroPage),
            0x86 => self.stx(ZeroPage),
            0x87 => self.sax(ZeroPage),
            0x88 => self.dey(),
            0x89 => self.nop(Immediate),
            0x8A => self.txa(),
            0x8B => self.ane(Immediate),
            0x8C => self.sty(Absolute),
            0x8D => self.sta(Absolute),
            0x8E => self.stx(Absolute),
            0x8F => self.sax(Absolute),

            0x90 => self.bcc(),
            0x91 => self.sta(IndirectY),
            0x92 => self.jam(),
            0x93 => self.sha(IndirectY),
            0x94 => self.sty(ZeroPageX),
            0x95 => self.sta(ZeroPageX),
            0x96 => self.stx(ZeroPageY),
            0x97 => self.sax(ZeroPageY),
            0x98 => self.tya(),
            0x99 => self.sta(AbsoluteY),
            0x9A => self.txs(),
            0x9B => self.tas(AbsoluteY),
            0x9C => self.shy(AbsoluteX),
            0x9D => self.sta(AbsoluteX),
            0x9E => self.shx(AbsoluteY),
            0x9F => self.sha(AbsoluteY),

            0xA0 => self.ldy(Immediate),
            0xA1 => self.lda(IndirectX),
            0xA2 => self.ldx(Immediate),
            0xA3 => self.lax(IndirectX),
            0xA4 => self.ldy(ZeroPage),
            0xA5 => self.lda(ZeroPage),
            0xA6 => self.ldx(ZeroPage),
            0xA7 => self.lax(ZeroPage),
            0xA8 => self.tay(),
            0xA9 => self.lda(Immediate),
            0xAA => self.tax(),
            0xAB => self.lxa(Immediate),
            0xAC => self.ldy(Absolute),
            0xAD => self.lda(Absolute),
            0xAE => self.ldx(Absolute),
            0xAF => self.lax(Absolute),

            0xB0 => self.bcs(),
            0xB1 => self.lda(IndirectY),
            0xB2 => self.jam(),
            0xB3 => self.lax(IndirectY),
            0xB4 => self.ldy(ZeroPageX),
            0xB5 => self.lda(ZeroPageX),
            0xB6 => self.ldx(ZeroPageY),
            0xB7 => self.lax(ZeroPageY),
            0xB8 => self.clv(),
            0xB9 => self.lda(AbsoluteY),
            0xBA => self.tsx(),
            0xBB => self.las(AbsoluteY),
            0xBC => self.ldy(AbsoluteX),
            0xBD => self.lda(AbsoluteX),
            0xBE => self.ldx(AbsoluteY),
            0xBF => self.lax(AbsoluteY),

            0xC0 => self.cpy(Immediate),
            0xC1 => self.cmp(IndirectX),
            0xC2 => self.nop(Immediate),
            0xC3 => self.dcp(IndirectX),
            0xC4 => self.cpy(ZeroPage),
            0xC5 => self.cmp(ZeroPage),
            0xC6 => self.dec(ZeroPage),
            0xC7 => self.dcp(ZeroPage),
            0xC8 => self.iny(),
            0xC9 => self.cmp(Immediate),
            0xCA => self.dex(),
            0xCB => self.sbx(Immediate),
            0xCC => self.cpy(Absolute),
            0xCD => self.cmp(Absolute),
            0xCE => self.dec(Absolute),
            0xCF => self.dcp(Absolute),

            0xD0 => self.bne(),
            0xD1 => self.cmp(IndirectY),
            0xD2 => self.jam(),
            0xD3 => self.dcp(IndirectY),
            0xD4 => self.nop(ZeroPageX),
            0xD5 => self.cmp(ZeroPageX),
            0xD6 => self.dec(ZeroPageX),
            0xD7 => self.dcp(ZeroPageX),
            0xD8 => self.cld(Implied),
            0xD9 => self.cmp(AbsoluteY),
            0xDA => self.nop(Implied),
            0xDB => self.dcp(AbsoluteY),
            0xDC => self.nop(AbsoluteX),
            0xDD => self.cmp(AbsoluteX),
            0xDE => self.dec(AbsoluteX),
            0xDF => self.dcp(AbsoluteX),

            0xE0 => self.cpx(Immediate),
            0xE1 => self.sbc(IndirectX),
            0xE2 => self.nop(Immediate),
            0xE3 => self.isc(IndirectX),
            0xE4 => self.cpx(ZeroPage),
            0xE5 => self.sbc(ZeroPage),
            0xE6 => self.inc(ZeroPage),
            0xE7 => self.isc(ZeroPage),
            0xE8 => self.inx(),
            0xE9 => self.sbc(Immediate),
            0xEA => self.nop(Implied),
            0xEB => self.usbc(Immediate),
            0xEC => self.cpx(Absolute),
            0xED => self.sbc(Absolute),
            0xEE => self.inc(Absolute),
            0xEF => self.isc(Absolute),

            0xF0 => self.beq(),
            0xF1 => self.sbc(IndirectY),
            0xF2 => self.jam(),
            0xF3 => self.isc(IndirectY),
            0xF4 => self.nop(ZeroPageX),
            0xF5 => self.sbc(ZeroPageX),
            0xF6 => self.inc(ZeroPageX),
            0xF7 => self.isc(ZeroPageX),
            0xF8 => self.sed(),
            0xF9 => self.sbc(AbsoluteY),
            0xFA => self.nop(Implied),
            0xFB => self.isc(AbsoluteY),
            0xFC => self.nop(AbsoluteX),
            0xFD => self.sbc(AbsoluteX),
            0xFE => self.inc(AbsoluteX),
            0xFF => self.isc(AbsoluteX),

            _ => panic!("unmapped opcode: {:#06X}", opcode)
        }
    }

    // Unofficial Opcodes

    fn jam(&mut self) -> u8 {
        0
    }

    fn usbc(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn isc(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn dcp(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn sbx(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn lax(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn las(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn lxa(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn sha(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn shx(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn shy(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn tas(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn sax(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn ane(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn rra(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn arr(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn sre(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn alr(&mut self, _mode: OperandMode) -> u8 {
        0
    }

    fn rla(&mut self, _mode: OperandMode) -> u8{
        0
    }

    fn slo(&mut self, _mode: OperandMode) -> u8{
        0
    }

    fn anc(&mut self, _mode: OperandMode) -> u8{
        0
    }

}