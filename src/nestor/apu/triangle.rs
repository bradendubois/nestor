use crate::nestor::traits::MemoryMap;

pub struct Triangle {

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