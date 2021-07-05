use crate::nestor::traits::MemoryMap;

pub struct Noise {
    pub length_counter: u8
}


impl Noise {

    pub fn new() -> Noise {
        Noise {
            length_counter: 0
        }
    }
}

impl MemoryMap for Noise {

    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x400C => self.noise_main = value,
            0x400E => self.noise_loop_period = value,
            0x400F => {
                self.noise_length_register = value;
                self.noise_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            _ => panic!("unmapped Noise register: {:#06X}", address)
        }
    }
}