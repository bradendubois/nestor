use crate::nestor::traits::MemoryMap;

const     CONV: usize = 1000000;
const NTSC_CPU: usize = 1.789773 * CONV;
const  PAL_CPU: usize = 1.662607 * CONV;


pub struct Channel {
    pub duty: u8,
    pub halt: bool,
    pub constant_flag: bool,
    pub volume: u8,
    pub length: u8
}


impl MemoryMap for Channel {

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