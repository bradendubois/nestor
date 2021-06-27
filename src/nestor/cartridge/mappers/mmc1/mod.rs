use crate::nestor::cartridge::ines::INes;
use crate::nestor::traits::Mapper;

mod mmc1;

pub struct MMC1 {
    ram: Vec<u8>,
    ram_enabled: bool,
    rom: Vec<u8>,

    shift_register: u8,
    shift_count: u8,

    // ram_bank: u8,
    rom_bank: u8,
    chr_bank0: u8,
    chr_bank1: u8
}


impl MMC1 {

    pub fn new(data: Vec<u8>) -> MMC1 {

        let ines = INes::new(data.clone());

        MMC1 {
            ram: std::iter::repeat(0).take(0x2000).collect(),
            ram_enabled: false,
            rom: ines.data,

            shift_register: 0,
            shift_count: 0,

            // ram_bank: 0,
            rom_bank: 0,
            chr_bank0: 0,
            chr_bank1: 0
        }
    }

    pub fn from(data: Vec<u8>) -> Box<dyn Mapper> {
        Box::new(MMC1::new(data))
    }
}


