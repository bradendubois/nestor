mod pulse;

use crate::nestor::traits::{Power, MemoryMap};

pub struct APU {

    // Pulse 1 Channel
    pulse_1_main: u8,                // 0x4000
    pulse_1_sweep: u8,               // 0x4001
    pulse_1_period_low: u8,          // 0x4002
    pulse_1_period_upper: u8,        // 0x4003
    pulse_1_length: u8,

    // Pulse 2 Channel
    pulse_2_main: u8,                // 0x4004
    pulse_2_sweep: u8,               // 0x4005
    pulse_2_period_low: u8,          // 0x4006
    pulse_2_period_upper: u8,        // 0x4007
    pulse_2_length: u8,

    // Triangle Channel
    triangle_main: u8,               // 0x4008
    triangle_period_low: u8,         // 0x400A
    triangle_period_upper: u8,       // 0x400B
    triangle_length: u8,

    // Noise Channel
    noise_main: u8,                 // 0x400C
    noise_loop_period: u8,          // 0x400E
    noise_length_register: u8,      // 0x400F
    noise_length: u8,

    // DMC Channel
    dmc_irq_main: u8,               // 0x4010
    dmc_direct: u8,                 // 0x4011
    dmc_sample_address: u8,         // 0x4012
    dmc_sample_length_register: u8, // 0x4013
    dmc_sample_length: u8,

    // Other
    control: u8,                    // 0x4015
    status: u8,                     // 0x4016
    frame_counter: u8               // 0x4017
}

impl APU {

    pub fn new() -> APU {
        APU {
            pulse_1_main: 0,
            pulse_1_sweep: 0,
            pulse_1_period_low: 0,
            pulse_1_period_upper: 0,
            pulse_1_length: 0,
            
            pulse_2_main: 0,
            pulse_2_sweep: 0,
            pulse_2_period_low: 0,
            pulse_2_period_upper: 0,
            pulse_2_length: 0,

            triangle_main: 0,
            triangle_period_low: 0,
            triangle_period_upper: 0,
            triangle_length: 0,

            noise_main: 0,
            noise_loop_period: 0,
            noise_length_register: 0,
            noise_length: 0,
            
            dmc_irq_main: 0,
            dmc_direct: 0,
            dmc_sample_address: 0,
            dmc_sample_length_register: 0,
            dmc_sample_length: 0,

            control: 0,
            status: 0,
            frame_counter: 0,
        }
    }


    fn length_table_lookup(value: u8) -> u8 {
        match value {

            0x00 => 10,
            0x01 => 254,
            0x02 => 20,
            0x03 => 2,
            0x04 => 40,
            0x05 => 4,
            0x06 => 80,
            0x07 => 6,
            0x08 => 160,
            0x09 => 8,
            0x0A => 60,
            0x0B => 10,
            0x0C => 14,
            0x0D => 12,
            0x0E => 26,
            0x0F => 14,

            0x10 => 12,
            0x11 => 16,
            0x12 => 24,
            0x13 => 18,
            0x14 => 48,
            0x15 => 20,
            0x16 => 96,
            0x17 => 22,
            0x18 => 192,
            0x19 => 24,
            0x1A => 72,
            0x1B => 26,
            0x1C => 16,
            0x1D => 28,
            0x1E => 32,
            0x1F => 30,

            _ => panic!("impossible length value: {:#04X}", value)
        }
    }
}


impl Power for APU {

    fn power_up(&mut self) {
        self.write(0x4000, 0x00);
        self.write(0x4001, 0x00);
        self.write(0x4002, 0x00);
        self.write(0x4003, 0x00);
        self.write(0x4004, 0x00);
        self.write(0x4005, 0x00);
        self.write(0x4006, 0x00);
        self.write(0x4007, 0x00);
        self.write(0x4008, 0x00);
        self.write(0x400A, 0x00);
        self.write(0x400B, 0x00);
        self.write(0x400C, 0x00);
        self.write(0x400E, 0x00);
        self.write(0x400F, 0x00);
        self.write(0x4010, 0x00);
        self.write(0x4011, 0x00);
        self.write(0x4012, 0x00);
        self.write(0x4013, 0x00);
    }

    fn reset(&mut self) {
        self.write(0x4015, 0x00);
        self.write(0x4017, 0x00);
        // TODO - Triangle Phase, DPCM Output AND 1
    }
}


impl MemoryMap for APU {

    fn read(&self, address: u16) -> u8 {
        match address {

            // Status
            0x4015 => {
                let mut _result = 0;
                // if self.pulse_1.length_counter > 0 { result |= 0x01 };
                // if self.pulse_2.length_counter > 0 { result |= 0x02 };
                // if self.noise.length_counter > 0 { result |= 0x04 };
                // if self.triangle.length_counter > 0 { result |= 0x08 };
                // if self.dmc.bytes_remaining > 0 { result |= 0x10 };
                // if self.frame_interrupt {
                //      result |= 0x40;
                //      self.frame_interrupt = false;
                // }
                // if self.dmc_interrupt { result |= 0x80; }
                0
            },

            _ => 0 // panic!("unmapped apu register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x4000 => self.pulse_1_main = value,
            0x4001 => self.pulse_1_sweep = value,
            0x4002 => self.pulse_1_period_low = value,
            0x4003 => {
                self.pulse_1_period_upper = value;
                self.pulse_1_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },
            
            0x4004 => self.pulse_2_main = value,
            0x4005 => self.pulse_2_sweep = value,
            0x4006 => self.pulse_2_period_low = value,
            0x4007 => {
                self.pulse_2_period_upper = value;
                self.pulse_2_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            0x4008 => self.triangle_main = value,
            0x400A => self.triangle_period_low = value,
            0x400B => {
                self.triangle_period_upper = value;
                self.triangle_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            0x400C => self.noise_main = value,
            0x400E => self.noise_loop_period = value,
            0x400F => {
                self.noise_length_register = value;
                self.noise_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            0x4010 => self.dmc_irq_main = value,
            0x4011 => self.dmc_direct = value,
            0x4012 => self.dmc_sample_address = value,
            0x4013 => {
                self.dmc_sample_length_register = value;
                self.dmc_sample_length = APU::length_table_lookup((value & 0xF8) >> 3);
            },

            // Status
            0x4015 => {
                // self.pulse_1.set_enabled(value & 0x01 != 0);
                // self.pulse_2.set_enabled(value & 0x02 != 0);
                // self.noise.set_enabled(value & 0x04 != 0);
                // self.triangle.set_enabled(value & 0x08 != 0);
                // self.dmc.set_enabled(value & 0x10 != 0);
                // self.dmc_interrupt = false;
            },

            0x4017 => {
                self.frame_counter = value;
                // self.mode = if value & 0x80 != 0 { ;
            },

            _ => panic!("unmapped apu register: {:#06X}", address)
        }
    }
}