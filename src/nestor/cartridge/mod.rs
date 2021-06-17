mod mappers;
mod ines;

use crate::nestor::traits::{MemoryMap, Mapper};
use crate::nestor::cartridge::mappers::get_mapper;


pub struct Cartridge {
    // data: Vec<u8>
    mapper: Box<dyn Mapper>
}


impl Cartridge {

    pub fn new(rom_path: &str) -> Cartridge {

        let data = std::fs::read(rom_path).unwrap(); //.into_vec()[0x0010..].to_vec();

        Cartridge {
            mapper: get_mapper(data)
        }
    }
}


impl MemoryMap for Cartridge {

    fn read(&self, address: u16) -> u8 {
        self.mapper.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.write(address, value);
    }
}