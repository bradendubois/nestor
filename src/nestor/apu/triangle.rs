use crate::nestor::traits::MemoryMap;

pub struct Triangle {

    // Triangle Channel
    triangle_main: u8,               // 0x4008
    triangle_period_low: u8,         // 0x400A
    triangle_period_upper: u8,       // 0x400B
    triangle_length: u8,

    pub length_counter: u8
}


impl Triangle {
    
    pub fn new() -> Triangle {
        Triangle {
            triangle_main: 0,
            triangle_period_low: 0,
            triangle_period_upper: 0,
            triangle_length: 0,

            length_counter: 0
        }
    }
}

impl MemoryMap for Triangle {

    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            0x4008 => self.triangle_main = value,
            0x400A => self.triangle_period_low = value,
            0x400B => {
                self.triangle_period_upper = value;
                self.triangle_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            _ => panic!("unimplemented Triangle register: {:#06X}", address)
        }
    }
}