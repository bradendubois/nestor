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

    pub fn new(data: Vec<u8>) -> NROM {

        let ines = INes::new(data.clone());

        /*
        for i in 0..=20 {
            println!("{:#06X}", ines.data[i as usize]);
        }

         */

        NROM {
            ram: std::iter::repeat(0).take(0x2000).collect(),
            rom: ines.data
        }
    }

    pub fn from(mapper_type: NROMTypes, data: Vec<u8>) -> Box<dyn Mapper> {
        match mapper_type {
            NROMTypes::Mapper0 => Box::new(NROM::new(data)),
            NROMTypes::NROM128 => panic!("NROM 128 not implemented!"),
            NROMTypes::NROM256 => panic!("NROM 256 not implemented!")
        }
    }
}


impl Mapper for NROM {

    fn read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0xFFFF => self.rom[(address & 0x3FFF) as usize],
            _ => panic!("unmapped")
        }
    }

    fn write(&mut self, _address: u16, _value: u8) {
        todo!()
    }
}