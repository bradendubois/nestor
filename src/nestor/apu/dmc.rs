use crate::nestor::traits::MemoryMap;
use super::APU;


#[allow(dead_code)]
pub struct DMC {

    // 0x4010
    r_4010: u8,
    irq_enable: bool,
    loop_enable: bool,
    frequency: u8,

    load_counter: u8,       // 0x4011
    sample_address: u8,     // 0x4012

    // 0x4013
    r_4013: u8,
    sample_length: u8,

    pub bytes_remaining: u8,
    pub interrupt: bool,
}

impl DMC {

    pub fn new() -> DMC {
        DMC {
            r_4010: 0,
            irq_enable: false,
            loop_enable: false,
            frequency: 0,
            load_counter: 0,
            sample_address: 0,
            r_4013: 0,
            sample_length: 0,
            bytes_remaining: 0,
            interrupt: false
        }
    }

    pub fn set_enabled(&mut self, _enabled: bool) {
        todo!()
    }
}


impl MemoryMap for DMC {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x4010 => self.r_4010,
            0x4011 => self.load_counter,
            0x4012 => self.sample_address,
            0x4013 => self.r_4013,

            _ => panic!("unmapped DMC address: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x4010 => {
                self.r_4010 = value;
                self.irq_enable = value & 0x80 != 0;
                self.loop_enable = value & 0x40 != 0;
                self.frequency = value & 0x0F;
            }
            0x4011 => self.load_counter = value & 0x4F,
            0x4012 => self.sample_address = value,
            0x4013 => {
                self.r_4013 = value;
                self.sample_length = APU::length_table_lookup((value & 0xF8) >> 3);
            }

            _ => panic!("unmapped DMC address: {:#06X}", address)
        };
    }
}
