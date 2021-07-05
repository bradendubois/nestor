use crate::nestor::traits::MemoryMap;

pub struct DMC {

}

impl DMC {


}


impl MemoryMap for DMC {

    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {

        match address {
            0x4010 => {
                // self.dmc.r_4010 = value;
                // self.dmc.irq_enable = value & 0x80 != 0;
                // self.dmc.loop = value & 0x40 != 0;
                // self.dmc.frequency = value & 0x0F;
            }

            0x4011 => {
                // self.dmc.load_counter = value & 0x4F;
            }

            0x4012 => {
                // self.dmc.sample_address = value;
            }

            0x4013 => {
                // self.dmc.r_4013 = value;
                // self.dmc.sample_length = APU::length_table_lookup((value & 0xF8) >> 3);
            }

            _ => panic!("unmapped DMC address: {:#06X}", address)
        };
    }
}