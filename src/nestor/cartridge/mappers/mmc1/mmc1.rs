use crate::nestor::traits::Mapper;

use super::{MMC1};


impl Mapper for MMC1 {

    fn read(&self, address: u16) -> u8 {
        match address {

            0x0000..=0x0FFF => match self.chr_bankmode {
                0 => self.rom[(0x1FFF * (self.chr_bank0 & !1) as usize) * address as usize],
                1 => self.rom[(0x0FFF * (self.chr_bank0) as usize) * address as usize],

                _ => panic!("invalid chr bank mode: {}", self.chr_bankmode)
            }

            0x1000..=0x1FFF => match self.chr_bankmode {
                0 => self.rom[(0x1FFF * (self.chr_bank0 & !1) as usize) * address as usize],
                1 => self.rom[(0x0FFF * (self.chr_bank1) as usize) * address as usize],

                _ => panic!("invalid chr bank mode: {}", self.chr_bankmode)
            }

            0x6000..=0x7FFF => match self.ram_enabled {
                true => self.ram[(address & 0x1FFF) as usize],
                false => 0x00
            }

            0x8000..=0xBFFF => match self.prg_bankmode {

                // 0, 1: Switch 32 KB Bank, ignore LSB of Bank Number
                0b00 | 0b01 => self.rom[(0x8FFFF * (self.rom_bank & !1) as usize) * (address & 0x3FFF) as usize],

                // 3: First bank at 0x8000
                0b10 => self.rom[(address & 0x3FFF) as usize],

                // 2: Switch 16 KB bank
                0b11 => self.rom[(0xFFFF * self.rom_bank as usize) as usize * (address & 0x3FFF) as usize],
                
                _ => panic!("impossible bankmode: {:#04X}", self.prg_bankmode)
            }

            0xC000..=0xFFFF => match self.prg_bankmode {

                // 0, 1: Switch 32 KB Bank, ignore LSB of Bank Number
                0b00 | 0b01 => self.rom[(0x8FFFF * (self.rom_bank & !1) as usize) * (address & 0x3FFF) as usize],

                // 2: Switch 16 KB bank
                0b10 => self.rom[(0xFFFF * self.rom_bank as usize) as usize * (address & 0x3FFF) as usize],

                // 3: First bank at 0x8000
                0b11 => self.rom[&self.rom.len() - (address & 0x3FFF) as usize],

                _ => panic!("impossible bankmode: {:#04X}", self.prg_bankmode)
            },

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
                    self.control &= 0x0C;
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