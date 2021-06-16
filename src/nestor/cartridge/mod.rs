use crate::nestor::traits::MemoryMap;


pub struct Cartridge {
    data: Vec<u8>
}


impl MemoryMap for Cartridge {

    fn read(&self, address: u16) -> u8 {

        match address {
            0x4020..=0xFFFF => *self.data.get((address - 0x4020) as usize).unwrap_or(&0),
            _=> panic!("unmapped address: {:#06X}", address)
        }
    }

    fn write(&mut self, _address: u16, _value: u8) {
        todo!()
    }
}