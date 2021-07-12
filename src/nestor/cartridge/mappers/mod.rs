pub mod nrom;
mod mmc1;

use crate::nestor::traits::Mapper;
use crate::nestor::cartridge::mappers::nrom::NROMTypes::Mapper0;
use crate::nestor::cartridge::ines::INes;


pub fn get_mapper(data: Vec<u8>) -> Box<dyn Mapper> {

    let i_nes = INes::new(data);

    // panic!("{}", i_nes.mapper);

    match i_nes.mapper {
        0x00 => nrom::NROM::from(Mapper0, i_nes),
        0x01 => mmc1::MMC1::from(i_nes),
        _ => panic!("unsupported mapper type: {}", i_nes.mapper)
    }
}