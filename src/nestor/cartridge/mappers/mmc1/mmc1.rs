use crate::nestor::traits::Mapper;

use super::{MMC1};


impl Mapper for MMC1 {

    fn read(&self, address: u16) -> u8 {
        match address {
            0x6000..=0x7FFF => match self.ram_enabled {
                true => self.ram[(address & 0x1FFF) as usize],
                false => 0x00
            },
            0x8000..=0xBFFF => match self.prg_bankmode {

                0b00 => self.rom[(address & 0x3FFF) as usize],
                0b01 => self.rom[(address & 0x3FFF) as usize],
                0b10 => self.rom[(address & 0x3FFF) as usize],
                0b11 => self.rom[(address & 0x3FFF) as usize],
                
                _ => panic!("impossible bankmode: {:#04X}", self.prg_bankmode)
            }
            0xC000..=0xFFFF => self.rom[(address & 0x3FFF) as usize],
            _ => panic!("unmapped: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // Nothing below 0x6000
            0x4020..=0x5FFF => (),

            // RAM
            0x6000..=0x7FFF => if self.ram_enabled {
                self.ram[(address & 0x1FFF) as usize] = value;
            }

            // ROM
            0x8000..=0xFFFF => {

                // Bit 7 set: clear
                if value & 0x80 != 0 {
                    self.shift_register = 0;
                    self.shift_count = 0;
                    return
                }

                let bit: u8 = if value & 0x01 != 0 { 0x80 } else { 0x00 };
                self.shift_register >>= 1;
                self.shift_register |= bit;
                self.shift_count += 1;

                // On fifth write, shift register is fully updated
                if self.shift_count == 5 {

                    let result  = (self.shift_register & 0xF8) >> 3;
                    let bits_14_13 = address & 0x6000 >> 13;

                    match bits_14_13 {

                        // Control - 0x8000-0x9FFF
                        0b00 => {
                            self.control = result;
                            self.mirror = result & 0b00011;
                            self.prg_bankmode = (result & 0b01100) >> 2;
                            self.chr_bankmode = (result & 0b10000) >> 4;
                        },

                        // CHR Bank 0
                        0b01 => self.chr_bank0 = result,

                        // CHR Bank 1
                        0b10 => self.chr_bank1 = result,

                        // PRG Bank
                        0b11 => {
                            self.ram_enabled = result & 0x10 != 0;
                            self.rom_bank = result;
                        },

                        _ => panic!("unmapped: {:#06X}", address)
                    }

                    self.shift_register = 0x80;
                    self.shift_count = 0;
                }
            }

            _ => panic!("unmapped: {:#06X}", address)
        }
    }
}