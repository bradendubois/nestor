use crate::nestor::traits::MemoryMap;
use super::APU;


pub struct Triangle {

    // 0x4008
    r_4008: u8,
    length_control: bool,
    linear_load: u8,

    timer_low: u8,      // 0x400A
    timer_high: u8,     // 0x400B(0-2)
    timer: u16,

    // 0x400B
    r_400b: u8,
    pub length_counter: u8
}


impl Triangle {
    
    pub fn new() -> Triangle {
        Triangle {
            r_4008: 0,
            length_control: false,
            linear_load: 0,
            timer_low: 0,
            timer_high: 0,
            timer: 0,
            r_400b: 0,
            length_counter: 0
        }
    }
}


impl MemoryMap for Triangle {

    fn read(&self, address: u16) -> u8 {
        match address {
            0x4008 => self.r_4008,
            0x400A => self.timer_low,
            0x400B => self.r_400b,

            _ => panic!("unimplemented Triangle register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x4008 => {
                self.r_4008 = value;
                self.length_control = value & 0x80 != 0;
                self.linear_load = value & 0x7F;
            }
            0x400A => {
                self.timer_low = value;
                self.timer = ((self.timer_high as u16) << 8) | (self.timer_low as u16);
            }
            0x400B => {
                self.r_400b = value;
                self.length_counter = APU::length_table_lookup((value & 0xF8) >> 3);
                self.timer_high = value & 0x07;
                self.timer = ((self.timer_high as u16) << 8) | (self.timer_low as u16);
            },

            _ => panic!("unimplemented Triangle register: {:#06X}", address)
        }
    }
}