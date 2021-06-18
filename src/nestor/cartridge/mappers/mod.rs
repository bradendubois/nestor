pub mod nrom;

use crate::nestor::traits::Mapper;
use crate::nestor::cartridge::mappers::nrom::NROMTypes::Mapper0;


pub fn get_mapper(data: Vec<u8>) -> Box<dyn Mapper> {

    let header = &data[0..=15];

    /*
    println!("NES: {:#04X} {:#04X} {:#04X} {:#04X}", header[0], header[1], header[2], header[3]);
    println!("PRG ROM Size (16KB Units): {:#04X}", header[4]);
    println!("CHR ROM Size  (8KB Units): {:#04X}", header[5]);
    println!("Flags  6: {:#010b}", header[6]);
    println!("Flags  7: {:#010b}", header[7]);
    println!("Flags  8: {:#010b}", header[8]);
    println!("Flags  9: {:#010b}", header[9]);
    println!("Flags 10: {:#010b}", header[10]);

    println!("{:#04X} {:#04X} {:#04X} {:#04X}", header[0], header[1], header[2], header[3]);


     */
    let mapper_number = (header[7] & 0xF0) | ((header[6] & 0xF0) >> 4);

    match mapper_number {
        0x00 => nrom::NROM::from(Mapper0, data),
        _ => panic!("unsupported mapper type: {}", mapper_number)
    }
}