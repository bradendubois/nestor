use crate::nestor::cpu::CPU6502;

use crate::nestor::enums::OperandMode::*;
use crate::nestor::enums::OperandMode;
use crate::nestor::traits::MemoryMap;

impl CPU6502 {

    pub fn call(&mut self, opcode: u8) -> u8 {

        #[allow(unreachable_patterns)]
        match opcode {
            0x00 => self.brk(),
            0x01 => self.ora(IndirectX),
            0x02 => self.jam(),
            0x03 => self.slo(IndirectX),
            0x04 => self.nop_unofficial(ZeroPage),
            0x05 => self.ora(ZeroPage),
            0x06 => self.asl(ZeroPage),
            0x07 => self.slo(ZeroPage),
            0x08 => self.php(),
            0x09 => self.ora(Immediate),
            0x0A => self.asl(Accumulator),
            0x0B => self.anc(Immediate),
            0x0C => self.nop_unofficial(Absolute),
            0x0D => self.ora(Absolute),
            0x0E => self.asl(Absolute),
            0x0F => self.slo(Absolute),

            0x10 => self.bpl(),
            0x11 => self.ora(IndirectY),
            0x12 => self.jam(),
            0x13 => self.slo(IndirectY),
            0x14 => self.nop_unofficial(ZeroPageX),
            0x15 => self.ora(ZeroPageX),
            0x16 => self.asl(ZeroPageX),
            0x17 => self.slo(ZeroPageX),
            0x18 => self.clc(),
            0x19 => self.ora(AbsoluteY),
            0x1A => self.nop_unofficial(Implied),
            0x1B => self.slo(AbsoluteY),
            0x1C => self.nop_unofficial(AbsoluteX),
            0x1D => self.ora(AbsoluteX),
            0x1E => self.asl(AbsoluteX),
            0x1F => self.slo(AbsoluteX),

            0x20 => self.jsr(),
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
            0x34 => self.nop_unofficial(ZeroPageX),
            0x35 => self.and(ZeroPageX),
            0x36 => self.rol(ZeroPageX),
            0x37 => self.rla(ZeroPageX),
            0x38 => self.sec(),
            0x39 => self.and(AbsoluteY),
            0x3A => self.nop_unofficial(Implied),
            0x3B => self.rla(AbsoluteY),
            0x3C => self.nop_unofficial(AbsoluteX),
            0x3D => self.and(AbsoluteX),
            0x3E => self.rol(AbsoluteX),
            0x3F => self.rla(AbsoluteX),

            0x40 => self.rti(),
            0x41 => self.eor(IndirectX),
            0x42 => self.jam(),
            0x43 => self.sre(IndirectX),
            0x44 => self.nop_unofficial(ZeroPage),
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
            0x54 => self.nop_unofficial(ZeroPageX),
            0x55 => self.eor(ZeroPageX),
            0x56 => self.lsr(ZeroPageX),
            0x57 => self.sre(ZeroPageX),
            0x58 => self.cli(),
            0x59 => self.eor(AbsoluteY),
            0x5A => self.nop_unofficial(Implied),
            0x5B => self.sre(AbsoluteY),
            0x5C => self.nop_unofficial(AbsoluteX),
            0x5D => self.eor(AbsoluteX),
            0x5E => self.lsr(AbsoluteX),
            0x5F => self.sre(AbsoluteX),

            0x60 => self.rts(),
            0x61 => self.adc(IndirectX),
            0x62 => self.jam(),
            0x63 => self.rra(IndirectX),
            0x64 => self.nop_unofficial(ZeroPage),
            0x65 => self.adc(ZeroPage),
            0x66 => self.ror(ZeroPage),
            0x67 => self.rra(ZeroPage),
            0x68 => self.pla(),
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
            0x74 => self.nop_unofficial(ZeroPageX),
            0x75 => self.adc(ZeroPageX),
            0x76 => self.ror(ZeroPageX),
            0x77 => self.rra(ZeroPageX),
            0x78 => self.sei(),
            0x79 => self.adc(AbsoluteY),
            0x7A => self.nop_unofficial(Implied),
            0x7B => self.rra(AbsoluteY),
            0x7C => self.nop_unofficial(AbsoluteX),
            0x7D => self.adc(AbsoluteX),
            0x7E => self.ror(AbsoluteX),
            0x7F => self.rra(AbsoluteX),

            0x80 => self.nop_unofficial(Immediate),
            0x81 => self.sta(IndirectX),
            0x82 => self.nop_unofficial(Immediate),
            0x83 => self.sax(IndirectX),
            0x84 => self.sty(ZeroPage),
            0x85 => self.sta(ZeroPage),
            0x86 => self.stx(ZeroPage),
            0x87 => self.sax(ZeroPage),
            0x88 => self.dey(),
            0x89 => self.nop_unofficial(Immediate),
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
            0xC2 => self.nop_unofficial(Immediate),
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
            0xD4 => self.nop_unofficial(ZeroPageX),
            0xD5 => self.cmp(ZeroPageX),
            0xD6 => self.dec(ZeroPageX),
            0xD7 => self.dcp(ZeroPageX),
            0xD8 => self.cld(),
            0xD9 => self.cmp(AbsoluteY),
            0xDA => self.nop_unofficial(Implied),
            0xDB => self.dcp(AbsoluteY),
            0xDC => self.nop_unofficial(AbsoluteX),
            0xDD => self.cmp(AbsoluteX),
            0xDE => self.dec(AbsoluteX),
            0xDF => self.dcp(AbsoluteX),

            0xE0 => self.cpx(Immediate),
            0xE1 => self.sbc(IndirectX),
            0xE2 => self.nop_unofficial(Immediate),
            0xE3 => self.isc(IndirectX),
            0xE4 => self.cpx(ZeroPage),
            0xE5 => self.sbc(ZeroPage),
            0xE6 => self.inc(ZeroPage),
            0xE7 => self.isc(ZeroPage),
            0xE8 => self.inx(),
            0xE9 => self.sbc(Immediate),
            0xEA => self.nop(),
            0xEB => self.usbc(Immediate),
            0xEC => self.cpx(Absolute),
            0xED => self.sbc(Absolute),
            0xEE => self.inc(Absolute),
            0xEF => self.isc(Absolute),

            0xF0 => self.beq(),
            0xF1 => self.sbc(IndirectY),
            0xF2 => self.jam(),
            0xF3 => self.isc(IndirectY),
            0xF4 => self.nop_unofficial(ZeroPageX),
            0xF5 => self.sbc(ZeroPageX),
            0xF6 => self.inc(ZeroPageX),
            0xF7 => self.isc(ZeroPageX),
            0xF8 => self.sed(),
            0xF9 => self.sbc(AbsoluteY),
            0xFA => self.nop_unofficial(Implied),
            0xFB => self.isc(AbsoluteY),
            0xFC => self.nop_unofficial(AbsoluteX),
            0xFD => self.sbc(AbsoluteX),
            0xFE => self.inc(AbsoluteX),
            0xFF => self.isc(AbsoluteX),

            _ => panic!("unmapped opcode: {:#06X}", opcode)
        }
    }

}

/// Official Opcodes
impl CPU6502 {

    /// 0x61, 0x65, 0x69, 0x6D, 0x71, 0x75, 0x79, 0x7D  - Add with Carry
    pub fn adc(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_adc(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_adc(value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_adc(value);
                4
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_adc(value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_adc(value);
                4 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_adc(value);
                4 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_adc(value);
                6
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                let value = self.io.read(address);
                self.alu_adc(value);
                5 + if carry { 1 } else { 0 }
            }

            _ => panic!("unsupported mode for adc : {:?}", mode)
        }
    }

    /// 0x21, 0x25, 0x29, 0x2D, 0x31, 0x35, 0x39, 0x3D - Bitwise AND with Accumulator
    pub fn and(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_and(value);
                2
            },
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_and(value);
                3
            },
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_and(value);
                4
            },
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_and(value);
                4
            },
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_and(value);
                4 + if carry { 1 } else { 0 }
            },
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_and(value);
                4 + if carry { 1 } else { 0 }
            },
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_and(value);
                6
            },
            IndirectY => {
                let (address, carry) = self.indirect_y();
                let value = self.io.read(address);
                self.alu_and(value);
                5 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for and : {:?}", mode)
        }
    }

    /// 0x06, 0x0A, 0x0E, 0x16, 0x01E - Arithmetic Shift Left
    pub fn asl(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Accumulator => {
                let value = self.accumulator();
                self.alu_asl(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_asl(value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_asl(value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_asl(value);
                6
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_asl(value);
                6 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for asl : {:?}", mode)
        }
    }

    /// 0x24, 0x2C - BIT
    pub fn bit(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_bit(value);
                3
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_bit(value);
                4
            }
            _ => panic!("unsupported mode for bit : {:?}", mode)
        }
    }

    /// 0x00 - Break
    pub fn brk(&mut self) -> u8 {
        self.registers.set_interrupt(true);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        7
    }

    // Helper method to generalize CMP calls with A, X, Y registers
    fn cmp_general(&mut self, mode: OperandMode, source: u8) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_cmp(source, value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                4
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                4 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                4 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                6
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                let value = self.io.read(address);
                self.alu_cmp(source, value);
                5 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for cmp : {:?}", mode)
        }
    }

    /// 0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1 - Compare Accumulator
    pub fn cmp(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | ZeroPageX | Absolute | AbsoluteX | AbsoluteY | IndirectX | IndirectY => self.cmp_general(mode, self.registers.a),
            _ => panic!("unsupported mode for cmp : {:?}", mode)
        }
    }

    /// 0xE0, 0xE4, 0xEC - Compare X Register
    pub fn cpx(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | Absolute => self.cmp_general(mode, self.registers.x),
            _ => panic!("unsupported mode for cpx : {:?}", mode)
        }
    }

    /// 0xC0, 0xC4, 0xCC - Compare Y Register
    pub fn cpy(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | Absolute => self.cmp_general(mode, self.registers.y),
            _ => panic!("unsupported mode for cpy : {:?}", mode)
        }
    }

    /// 0xC6, 0xCE, 0xD6, 0xDE - Decrement Memory
    pub fn dec(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                let value = self.alu_dec(value);
                self.io.write(address, value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                let value = self.alu_dec(value);
                self.io.write(address, value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                let value = self.alu_dec(value);
                self.io.write(address, value);
                6
            }
            AbsoluteX => {
                let (address, _carry) = self.absolute_x();
                let value = self.io.read(address);
                let value = self.alu_dec(value);
                self.io.write(address, value);
                7
            }
            _ => panic!("unsupported mode for dec : {:?}", mode)
        }
    }

    /// 0x41, 0x45, 0x49, 0x4D, 0x51, 0x55, 0x59, 0x5D - Bitwise Exclusive OR
    pub fn eor(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_xor(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_xor(value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_xor(value);
                4
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_xor(value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_xor(value);
                4 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_xor(value);
                4 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_xor(value);
                6
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                let value = self.io.read(address);
                self.alu_xor(value);
                5 + if carry { 1 } else { 0 }
            }

            _ => panic!("unsupported mode for eor : {:?}", mode)
        }
    }

    /// 0xE6, 0xEE, 0xF6, 0xFE - Increment Memory
    pub fn inc(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                let value = self.alu_inc(value);
                self.io.write(address, value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                let value = self.alu_inc(value);
                self.io.write(address, value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                let value = self.alu_inc(value);
                self.io.write(address, value);
                6
            }
            AbsoluteX => {
                let (address, _carry) = self.absolute_x();
                let value = self.io.read(address);
                let value = self.alu_inc(value);
                self.io.write(address, value);
                7
            }
            _ => panic!("unsupported mode for dec : {:?}", mode)
        }
    }

    /// 0x4C, 0x6C - Jump
    pub fn jmp(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Absolute => {
                let address = self.absolute();
                self.registers.pc = address;
                3
            }
            Indirect => {
                let address = self.indirect();
                self.registers.pc = address;
                5
            }
            _ => panic!("unsupported mode for jmp : {:?}", mode)
        }
    }

    /// 0x20 - Jump to Subroutine
    pub fn jsr(&mut self) -> u8 {
        let address = self.absolute();
        self.push_word(self.registers.pc);
        self.registers.pc = address;
        6
    }

    /// 0x46, 0x4E, 0x4A, 0x56, 0x5E - Logical Shift Right
    pub fn lsr(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Accumulator => {
                let value = self.accumulator();
                self.alu_lsr(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_lsr(value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_lsr(value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_lsr(value);
                6
            }
            AbsoluteX => {
                let (address, _carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_lsr(value);
                7
            }
            _ => panic!("unsupported mode for lsr : {:?}", mode)
        }
    }

    /// 0xEA
    pub fn nop(&mut self) -> u8 {
        2
    }

    /// 0x01, 0x05, 0x09, 0x0D, 0x11, 0x15, 0x19, 0x1D - Bitwise OR with Accumulator
    pub fn ora(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_ora(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_ora(value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_ora(value);
                4
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_ora(value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_ora(value);
                4 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_ora(value);
                4 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_ora(value);
                6
            }
            IndirectY => {
                let (address, carry)= self.indirect_y();
                let value = self.io.read(address);
                self.alu_ora(value);
                5 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for ora : {:?}", mode)
        }
    }

    /// 0x26, 0x2A, 0x2E, 0x36, 0x3E - Rotate Left
    pub fn rol(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Accumulator => {
                let value = self.accumulator();
                self.alu_rol(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_rol(value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_rol(value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_rol(value);
                6
            }
            AbsoluteX => {
                let (address, _carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_rol(value);
                7
            }
            _ => panic!("unsupported mode for lsr : {:?}", mode)
        }
    }

    /// 0x66, 0x6A, 0x6E, 0x76, 0x7E - Rotate Right
    pub fn ror(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Accumulator => {
                let value = self.accumulator();
                self.alu_ror(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_ror(value);
                5
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_ror(value);
                6
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_ror(value);
                6
            }
            AbsoluteX => {
                let (address, _carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_ror(value);
                7
            }
            _ => panic!("unsupported mode for lsr : {:?}", mode)
        }
    }

    /// 0x40 - Return From Interrupt
    pub fn rti(&mut self) -> u8 {
        let p = self.pull();
        let pc = self.pull_word();
        self.registers.p = p;
        self.registers.pc = pc;
        6
    }

    /// 0x60 - Return from Subroutine
    pub fn rts(&mut self) -> u8 {
        self.registers.pc = self.pull_word().wrapping_add(1);
        6
    }

    /// 0xE1, 0xE5, 0xE9, 0xED, 0xF1, 0xF5, 0xF9, 0xFD - Subtract with Carry
    pub fn sbc(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate => {
                let value = self.immediate();
                self.alu_sbc(value);
                2
            }
            ZeroPage => {
                let address = self.zero_page();
                let value = self.io.read(address);
                self.alu_sbc(value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                let value = self.io.read(address);
                self.alu_sbc(value);
                4
            }
            Absolute => {
                let address = self.absolute();
                let value = self.io.read(address);
                self.alu_sbc(value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                let value = self.io.read(address);
                self.alu_sbc(value);
                4 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                let value = self.io.read(address);
                self.alu_sbc(value);
                4 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                let value = self.io.read(address);
                self.alu_sbc(value);
                6
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                let value = self.io.read(address);
                self.alu_sbc(value);
                5 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for sbc : {:?}", mode)
        }
    }

    /***** Stack Instruction *****/

    /// 0x08 - Push Processor Status
    pub fn php(&mut self) -> u8 {
        self.push(self.registers.p);
        3
    }

    /// 0x28 - Pull Processor Status
    pub fn plp(&mut self) -> u8 {
        self.registers.p = self.pull();
        4
    }

    /// 0x48 - Push Accumulator
    pub fn pha(&mut self) -> u8 {
        self.push(self.registers.a);
        3
    }

    /// 0x68 - Pull Accumulator
    pub fn pla(&mut self) -> u8 {
        self.registers.a = self.pull();
        4
    }

    /// 0x9A - Transfer X to SP
    pub fn txs(&mut self) -> u8 {
        self.registers.s = self.registers.x;
        2
    }

    /// 0xBA - Transfer X to SP
    pub fn tsx(&mut self) -> u8 {
        self.registers.s = self.registers.x;
        2
    }

    /***** Register Instructions *****/

    /// 0x88 - Decrement Y
    pub fn dey(&mut self) -> u8 {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.set_negative(true);
        self.registers.set_zero(self.registers.y == 0);
        2
    }

    /// 0x8A - Transfer X to A
    pub fn txa(&mut self) -> u8 {
        self.registers.a = self.registers.x;
        2
    }

    /// 0x98 - Transfer Y to A
    pub fn tya(&mut self) -> u8 {
        self.registers.a = self.registers.y;
        2
    }

    /// 0xAA - Transfer A to X
    pub fn tax(&mut self) -> u8 {
        self.registers.x = self.registers.a;
        2
    }

    /// 0xA8 - Transfer A to Y
    pub fn tay(&mut self) -> u8 {
        self.registers.y = self.registers.a;
        2
    }

    /// 0xCA - Decrement X
    pub fn dex(&mut self) -> u8 {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.set_negative(true);
        self.registers.set_zero(self.registers.x == 0);
        2
    }

    /// 0xC8 - Increment Y
    pub fn iny(&mut self) -> u8 {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.set_negative(false);
        self.registers.set_zero(self.registers.y == 0);
        2
    }

    /// 0xE8 - Increment X
    pub fn inx(&mut self) -> u8 {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.set_negative(false);
        self.registers.set_zero(self.registers.x == 0);
        2
    }

    /***** Load *****/

    fn load(&mut self, mode: OperandMode) -> (u8, u8) {
        match mode {
            Immediate => {
                (self.immediate(), 2)
            }
            ZeroPage => {
                let address = self.zero_page();
                (self.io.read(address), 3)
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                (self.io.read(address), 4)
            }
            ZeroPageY => {
                let address = self.zero_page_y();
                (self.io.read(address), 4)
            }
            Absolute => {
                let address = self.absolute();
                (self.io.read(address), 4)
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                (self.io.read(address), 4 + if carry { 1 } else { 0 })
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                (self.io.read(address), 4 + if carry { 1 } else { 0 })
            },
            IndirectX => {
                let address = self.x_indirect();
                (self.io.read(address), 6)
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                (self.io.read(address), 5 + if carry { 1 } else { 0 })
            }
            _ => panic!("unsupported load mode: {:?}", mode)
        }
    }

    pub fn lda(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | ZeroPageX | Absolute | AbsoluteX | AbsoluteY | IndirectX | IndirectY => {
                let (result, cycles) = self.load(mode);
                self.registers.a = result;
                cycles
            }
            _ => panic!("unsupported mode for lda : {:?}", mode)
        }
    }

    pub fn ldx(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | ZeroPageY | Absolute | AbsoluteY => {
                let (result, cycles) = self.load(mode);
                self.registers.x = result;
                cycles
            }
            _ => panic!("unsupported mode for ldx : {:?}", mode)
        }
    }

    pub fn ldy(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Immediate | ZeroPage | ZeroPageY | Absolute | AbsoluteY => {
                let (result, cycles) = self.load(mode);
                self.registers.y = result;
                cycles
            }
            _ => panic!("unsupported mode for ldy : {:?}", mode)
        }
    }

    /***** Store *****/

    fn store(&mut self, mode: OperandMode, value: u8) -> u8 {
        match mode {
            ZeroPage => {
                let address = self.zero_page();
                self.io.write(address, value);
                3
            }
            ZeroPageX => {
                let address = self.zero_page_x();
                self.io.write(address, value);
                4
            }
            Absolute => {
                let address = self.absolute();
                self.io.write(address, value);
                4
            }
            AbsoluteX => {
                let (address, carry) = self.absolute_x();
                self.io.write(address, value);
                5 + if carry { 1 } else { 0 }
            }
            AbsoluteY => {
                let (address, carry) = self.absolute_y();
                self.io.write(address, value);
                5 + if carry { 1 } else { 0 }
            }
            IndirectX => {
                let address = self.x_indirect();
                self.io.write(address, value);
                6
            }
            IndirectY => {
                let (address, carry) = self.indirect_y();
                self.io.write(address, value);
                6 + if carry { 1 } else { 0 }
            }

            _ => panic!("unsupported mode: {:?}", mode)
        }
    }

    /// 0x81, 0x85, 0x8D, 0x91, 0x96, 0x99, 0x9D - Store Accumulator
    pub fn sta(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage | ZeroPageX | Absolute | AbsoluteX | AbsoluteY | IndirectX | IndirectY => self.store(mode, self.registers.a),
            _ => panic!("unsupported mode for sta : {:?}", mode)
        }
    }

    /// 0x86, 0x8E, 0x96 - Store X Register
    pub fn stx(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage | ZeroPageY | Absolute => self.store(mode, self.registers.x),
            _ => panic!("unsupported mode for stx : {:?}", mode)
        }
    }

    /// 0x84, 0x8C, 0x94 - Store Y Register
    pub fn sty(&mut self, mode: OperandMode) -> u8 {
        match mode {
            ZeroPage | ZeroPageX | Absolute => self.store(mode, self.registers.y),
            _ => panic!("unsupported mode for sty : {:?}", mode)
        }
    }

    /***** Branching *****/

    fn branch_if(&mut self, condition: bool) -> u8 {
        let bb = self.byte() as i8;
        let (address, boundary) = self.relative(bb);
        match condition {
            true  => {
                self.registers.pc = address;
                3 + if boundary { 1 } else { 0 }
            },
            false => 2,
        }
    }

    /// 0x10 - Branch on Plus (Negative Unset)
    pub fn bpl(&mut self) -> u8 {
        self.branch_if(!self.registers.negative())
    }

    // 0x30 - Branch on Minus (Negative Set)
    pub fn bmi(&mut self) -> u8 {
        self.branch_if(self.registers.negative())
    }

    /// 0x50 - Branch on Overflow Clear
    pub fn bvc(&mut self) -> u8 {
        self.branch_if(!self.registers.overflow())
    }

    /// 0x70 - Branch on Overflow Set
    pub fn bvs(&mut self) -> u8 {
        self.branch_if(self.registers.overflow())
    }

    /// 0x90 - Branch on Carry Clear
    pub fn bcc(&mut self) -> u8 {
        self.branch_if(!self.registers.carry())
    }

    /// 0xB0 - Branch on Carry Set
    pub fn bcs(&mut self) -> u8 {
        self.branch_if(self.registers.carry())
    }

    /// 0xD0 - Branch on Not Equal (Zero Unset)
    pub fn bne(&mut self) -> u8 {
        self.branch_if(!self.registers.zero())
    }

    /// 0xF0 - Branch on Equal (Zero Set)
    pub fn beq(&mut self) -> u8 {
        self.branch_if(self.registers.zero())
    }

    /***** Flag (Processor Status) *****/

    /// 0x18 - Clear Carry
    pub fn clc(&mut self) -> u8 {
        self.registers.set_carry(false);
        2
    }

    /// 0x38 - Set Carry
    pub fn sec(&mut self) -> u8 {
        self.registers.set_carry(true);
        2
    }

    /// 0x58 - Clear Interrupt
    pub fn cli(&mut self) -> u8 {
        self.registers.set_interrupt(false);
        2
    }

    /// 0x78 - Set Interrupt
    pub fn sei(&mut self) -> u8 {
        self.registers.set_interrupt(true);
        2
    }

    /// 0xB8 - Clear Overflow
    pub fn clv(&mut self) -> u8 {
        self.registers.set_overflow(false);
        2
    }

    /// 0xD8 - Clear Decimal
    pub fn cld(&mut self) -> u8 {
        self.registers.set_decimal(false);
        2
    }

    /// 0xF8 - Set Decimal
    pub fn sed(&mut self) -> u8 {
        self.registers.set_decimal(true);
        2
    }
}


/// Unofficial Opcodes
impl CPU6502 {

    fn jam(&mut self) -> u8 {
        0
    }

    pub fn nop_unofficial(&mut self, mode: OperandMode) -> u8 {
        match mode {
            Implied => 2,
            Immediate => { let _addr = self.immediate(); 2 }
            ZeroPage => { let _addr = self.zero_page(); 3 }
            ZeroPageX => { let _addr = self.zero_page_x(); 4 }
            Absolute => { let _addr = self.absolute(); 4 }
            AbsoluteX => {
                let (_addr, carry) = self.absolute_x();
                4 + if carry { 1 } else { 0 }
            }
            _ => panic!("unsupported mode for unofficial nop : {:?}", mode)
        }
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