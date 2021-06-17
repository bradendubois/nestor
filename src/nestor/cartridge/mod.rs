mod mappers;

use crate::nestor::traits::MemoryMap;


pub struct Cartridge {
    data: Vec<u8>
}


impl Cartridge {

    pub fn new(rom_path: &str) -> Cartridge {
        Cartridge {
            data: std::fs::read(rom_path).unwrap()
        }
    }
}


impl MemoryMap for Cartridge {

    fn read(&self, address: u16) -> u8 {

        let header = &self.data[0..=15];

        println!("NES: {:#04X} {:#04X} {:#04X} {:#04X}", header[0], header[1], header[2], header[3]);
        println!("PRG ROM Size (16KB Units): {:#04X}", header[4]);
        println!("CHR ROM Size  (8KB Units): {:#04X}", header[5]);
        println!("Flags  6: {:#010b}", header[6]);
        println!("Flags  7: {:#010b}", header[7]);
        println!("Flags  8: {:#010b}", header[8]);
        println!("Flags  9: {:#010b}", header[9]);
        println!("Flags 10: {:#010b}", header[10]);

        println!("{:#04X} {:#04X} {:#04X} {:#04X}", self.data[0], self.data[1], self.data[2], self.data[3]);
        println!("Cartridge read for address: {:#06X} in cartridge of size {:#06X}", address, self.data.len());
        println!("{:?}", self.data);
        std::process::exit(0);
        self.data[address as usize]
    }

    fn write(&mut self, _address: u16, _value: u8) {    }
}