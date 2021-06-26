pub mod nrom;
mod mmc1;

use crate::nestor::traits::Mapper;
use crate::nestor::cartridge::mappers::nrom::NROMTypes::Mapper0;


pub fn get_mapper(data: Vec<u8>) -> Box<dyn Mapper> {

    let header = &data[0..=15];

    let mapper_number = (header[7] & 0xF0) | ((header[6] & 0xF0) >> 4);

    match mapper_number {
        0x00 => nrom::NROM::from(Mapper0, data),
        0x01 => mmc1::MMC1::from(data),

        _ => panic!("unsupported mapper type: {}", mapper_number)
    }
}