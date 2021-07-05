use crate::nestor::traits::MemoryMap;

const     CONV: usize = 1000000;
const NTSC_CPU: usize = 1.789773 * CONV;
const  PAL_CPU: usize = 1.662607 * CONV;


pub struct Pulse {
    pub duty: u8,
    pub halt: bool,
    pub constant_flag: bool,
    pub volume: u8,
    pub length: u8,

    pulse_1_main: u8,                // 0x4000
    pulse_1_sweep: u8,               // 0x4001
    pulse_1_period_low: u8,          // 0x4002
    pulse_1_period_upper: u8,        // 0x4003


    pub length_counter: u8,
}


impl Pulse {
    
    pub fn new() -> Pulse {
        Pulse {
            duty: 0,
            halt: false,
            constant_flag: false,
            volume: 0,
            length: 0,
            pulse_1_main: 0,
            pulse_1_sweep: 0,
            pulse_1_period_low: 0,
            pulse_1_period_upper: 0,
            length_counter: 0
        }
    }
}


impl MemoryMap for Pulse {

    fn read(&self, address: u16) -> u8 {
        match address % 4 {
            0 => self.main,
            1 => self.sweep,
            2 => self.period_low,
            3 => self.period_upper,

            _ => panic!("unimplemented Pulse register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => self.main = value,
            1 => self.sweep = value,
            2 => self.period_low = value,
            3 => {
                self.period_upper = value;
                self.length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            _ => panic!("unimplemented Pulse register: {:#06X}", address)
        }
    }
}