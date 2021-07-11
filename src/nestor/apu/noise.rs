use crate::nestor::traits::MemoryMap;
use super::APU;


pub struct Noise {

    // 0x400C
    r_400c: u8,
    envelop_halt: bool,         // Bit 5
    constant_volume: bool,      // Bit 4
    volume: u8,                 // Bit 3-0

    // 0x400E
    r_400e: u8,
    loop_noise: bool,           // Bit 7
    noise_period: u8,           // Bit 3-0

    // 0x400F
    r_400f: u8,
    pub length_counter: u8,     // Bit 7-3

    enabled: bool,
    silence: bool
}


impl Noise {

    pub fn new() -> Noise {
        Noise {
            r_400c: 0,
            envelop_halt: false,
            constant_volume: false,
            volume: 0,
            r_400e: 0,
            loop_noise: false,
            noise_period: 0,
            r_400f: 0,
            length_counter: 0,

            enabled: false,
            silence: true
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[allow(dead_code)]
    pub fn length_sweep_tick(&mut self) { todo!() }

    #[allow(dead_code)]
    pub fn envelope_tick(&mut self) { todo!() }

    /// Length Counter unit
    pub fn length_counter_tick(&mut self) {
        if self.length_counter != 0 {
            self.length_counter -= 1;
        } else {
            self.silence = true;
        }
    }
}


impl MemoryMap for Noise {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x400C => self.r_400c,
            0x400E => self.r_400e,
            0x400F => self.r_400f,

            _ => panic!("unmapped Noise register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x400C => {
                self.r_400c = value;
                self.envelop_halt = value & 0x20 != 0;
                self.constant_volume = value & 0x10 != 0;
                self.volume = value & 0x0F;
            }
            0x400E => {
                self.r_400e = value;
                self.loop_noise = value & 0x80 != 0;
                self.noise_period = value & 0x0F;
            }
            0x400F => {
                self.r_400f = value;
                self.length_counter = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            _ => panic!("unmapped Noise register: {:#06X}", address)
        }
    }
}