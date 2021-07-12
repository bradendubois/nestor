use crate::nestor::traits::Mapper;
use crate::nestor::cartridge::ines::INes;

#[allow(dead_code)]
pub enum NROMTypes {
    Mapper0,
    NROM128,
    NROM256
}

#[allow(dead_code)]
pub struct NROM {
    ram: Vec<u8>,
    rom: Vec<u8>
}


impl NROM {

    pub fn new(i_nes: INes) -> NROM {
        NROM {
            ram: std::iter::repeat(0).take(0x8000).collect(),
            rom: i_nes.prg_rom
        }
    }

    pub fn from(mapper_type: NROMTypes, i_nes: INes) -> Box<dyn Mapper> {
        match mapper_type {
            NROMTypes::Mapper0 => Box::new(NROM::new(i_nes)),
            NROMTypes::NROM128 => panic!("NROM 128 not implemented!"),
            NROMTypes::NROM256 => panic!("NROM 256 not implemented!")
        }
    }
}


impl Mapper for NROM {

    fn read(&self, address: u16) -> u8 {
        match address {
            0x4020..=0x5FFF => 0x00,  // TODO - Cartridge Expansion
            0x6000..=0x7FFF => 0x00,  // TODO - SRAM
            0x8000..=0xFFFF => self.rom[(address & 0x3FFF) as usize],
            _ => panic!("unmapped: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        // self.ram[(address & 0x5FFF) as usize] = value;
    }
}