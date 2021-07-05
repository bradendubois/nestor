use crate::nestor::traits::MemoryMap;
use crate::nestor::apu::APU;

#[allow(dead_code)]
const NTSC_CPU: usize = 1789773;
#[allow(dead_code)]
const  PAL_CPU: usize = 1662607;


pub struct Pulse {

    // 0x400{0,4}
    r_4000: u8,
    duty: u8,
    halt: bool,
    constant_flag: bool,
    volume: u8,

    // 0x400{1,5}
    r_4001: u8,
    enabled: bool,
    period: u8,
    negate: bool,
    shift: u8,

    timer_low: u8,  // 0x400{2,6}

    // 0x400{3,7}
    r_4003: u8,
    pub length_counter: u8,
    timer_high: u8,

    timer: u16
}


impl Pulse {
    
    pub fn new() -> Pulse {
        Pulse {
            r_4000: 0,
            duty: 0,
            halt: false,
            constant_flag: false,
            volume: 0,
            r_4001: 0,
            enabled: false,
            period: 0,
            negate: false,
            shift: 0,
            timer_low: 0,
            r_4003: 0,
            length_counter: 0,
            timer_high: 0,
            timer: 0
        }
    }

    pub fn set_enabled(&mut self, _enabled: bool) {
        todo!()
    }
}


impl MemoryMap for Pulse {

    fn read(&mut self, address: u16) -> u8 {
        match address % 4 {
            0 => self.r_4000,
            1 => self.r_4001,
            2 => self.timer_low,
            3 => self.r_4003,

            _ => panic!("unimplemented Pulse register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.r_4000 = value;
                self.duty = (value & 0xC0) >> 6;
                self.halt = value & 0x20 != 0;
                self.constant_flag = value & 0x10 != 0;
                self.volume = value & 0x0F;
            }
            1 => {
                self.r_4001 = value;
                self.enabled = value & 0x80 != 0;
                self.period = (value & 0x70) >> 4;
                self.negate = value & 0x08 != 0;
                self.shift = value & 0x07;
            }
            2 => {
                self.timer_low = value;
                self.timer = ((self.timer_high as u16) << 8) | (self.timer_low as u16);
            }
            3 => {
                self.r_4003 = value;
                self.timer_high = value & 0x07;
                self.timer = ((self.timer_high as u16) << 8) | (self.timer_low as u16);
                self.length_counter = APU::length_table_lookup((value & 0xF8) >> 3);
            }

            _ => panic!("unimplemented Pulse register: {:#06X}", address)
        }
    }
}