use crate::nestor::cartridge::ines::INes;
use crate::nestor::traits::Mapper;

mod mmc1;

pub struct MMC1 {
    ram: Vec<u8>,
    ram_enabled: bool,
    rom: Vec<u8>,

    // Control
    control: u8,
    mirror: u8,
    prg_bankmode: u8,
    chr_bankmode: u8,

    shift_register: u8,
    shift_count: u8,

    // ram_bank: u8,
    rom_bank: u8,
    chr_bank0: u8,
    chr_bank1: u8
}


impl MMC1 {

    pub fn new(i_nes: INes) -> MMC1 {

        MMC1 {
            ram: std::iter::repeat(0).take(0x2000).collect(),
            ram_enabled: false,
            rom: i_nes.prg_rom,

            // Control
            control: 0,
            mirror: 0,
            prg_bankmode: 3,
            chr_bankmode: 0,

            shift_register: 0,
            shift_count: 0,

            // ram_bank: 0,
            rom_bank: 0,
            chr_bank0: 0,
            chr_bank1: 0
        }
    }

    pub fn from(i_nes: INes) -> Box<dyn Mapper> {
        Box::new(MMC1::new(i_nes))
    }
}


