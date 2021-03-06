use crate::nestor::traits::MemoryMap;
use crate::nestor::cartridge::Cartridge;
use crate::nestor::apu::APU;

pub struct IO {
    apu: APU,
    cartridge: Cartridge,
    ram: Vec<u8>,
}

impl IO {

    pub fn new(cartridge: Cartridge) -> IO {
        IO {
            apu: APU::new(),
            cartridge,
            ram: std::iter::repeat(0).take(0x0800).collect(),
        }
    }
}


// Allow flag is to please IntelliJ not realizing the pattern is exhaustive
#[allow(unreachable_patterns)]
impl MemoryMap for IO {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => self.ram[address as usize],
            0x0800..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x2000..=0x2007 => 0x00,
            0x2008..=0x3FFF => 0x00,
            0x4000..=0x4017 => self.apu.read(address),
            0x4018..=0x401F => 0x00,
            0x4020..=0xFFFF => self.cartridge.read(address),

            _ => panic!("unmapped address: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x07FF => self.ram[address as usize] = value,
            0x0800..=0x1FFF => self.ram[(address % 0x07FF) as usize] = value,
            0x2000..=0x2007 => (),
            0x2008..=0x3FFF => (),
            0x4000..=0x4017 => self.apu.write(address, value),
            0x4018..=0x401F => (),
            0x4020..=0xFFFF => self.cartridge.write(address, value),

            _ => panic!("unmapped address: {:#06X}", address)
        };
    }
}