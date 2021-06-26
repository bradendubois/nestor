use crate::nestor::traits::Mapper;
use crate::nestor::cartridge::ines::INes;


pub struct MMC1 {
    ram: Vec<u8>,
    rom: Vec<u8>
}


impl MMC1 {

    pub fn new(data: Vec<u8>) -> MMC1 {

        let ines = INes::new(data.clone());

        MMC1 {
            ram: std::iter::repeat(0).take(0x2000).collect(),
            rom: ines.data
        }
    }

    pub fn from(data: Vec<u8>) -> Box<dyn Mapper> {
        Box::new(MMC1::new(data))
    }
}


impl Mapper for MMC1 {

    fn read(&self, address: u16) -> u8 {
        match address {
            0x4020..=0x5FFF => 0x00,  // TODO - Cartridge Expansion
            0x6000..=0x7FFF => 0x00,  // TODO - SRAM
            0x8000..=0xFFFF => self.rom[(address & 0x3FFF) as usize],
            _ => panic!("unmapped: {:#06X}", address)
        }
    }

    fn write(&mut self, _address: u16, _value: u8) {
        panic!("DID THIS GET CALLED?");
    }
}