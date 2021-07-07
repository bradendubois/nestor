use crate::nestor::traits::MemoryMap;
use super::APU;


pub struct Triangle {

    // 0x4008
    r_4008: u8,
    halt: bool,             // Bit 7
    linear_load: u8,        // Bit 6-0

    timer_low: u8,          // 0x400A

    // 0x400B
    r_400b: u8,
    pub length_counter: u8, // Bit 7-3
    timer_high: u8,         // Bit 2-0

    timer: u16,
    silence: bool
}


impl Triangle {
    
    pub fn new() -> Triangle {
        Triangle {
            r_4008: 0,
            halt: false,
            linear_load: 0,
            timer_low: 0,
            timer_high: 0,
            r_400b: 0,
            length_counter: 0,
            timer: 0,
            silence: true
        }
    }

    pub fn set_enabled(&mut self, _enabled: bool) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn length_sweep_tick(&mut self) { todo!() }

    #[allow(dead_code)]
    pub fn envelope_tick(&mut self) { todo!() }

    /// Length Counter unit
    pub fn length_counter_tick(&mut self) {
        if !self.halt && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.silence = true;
            }
        }
    }
}


impl MemoryMap for Triangle {

    fn read(&mut self, address: u16) -> u8 {
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
                self.halt = value & 0x80 != 0;
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