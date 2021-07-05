mod dmc;
mod noise;
mod pulse;
mod triangle;

use crate::nestor::traits::{Power, MemoryMap};

use dmc::DMC;
use noise::Noise;
use pulse::Pulse;
use triangle::Triangle;


pub struct APU {
    pulse_1: Pulse,
    pulse_2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: DMC,

    frame_interrupt: bool,

    frame_counter: u8,
    mode: u8,
    irq_inhibit: bool,
}


impl APU {

    pub fn new() -> APU {
        APU {
            pulse_1: Pulse::new(),
            pulse_2: Pulse::new(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: DMC::new(),

            frame_interrupt: false,

            frame_counter: 0,
            mode: 0,
            irq_inhibit: false
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

    fn read(&mut self, address: u16) -> u8 {
        match address {

            // TODO - What happens when you read a write-only register? 0's, garbage?
            // Channels
            0x4000..=0x4003 => self.pulse_1.read(address),      // Pulse 1
            0x4004..=0x4007 => self.pulse_2.read(address),      // Pulse 2
            0x4008..=0x400B => self.triangle.read(address),     // Triangle
            0x400C..=0x400F => self.noise.read(address),        // Noise
            0x4010..=0x4013 => self.dmc.read(address),          // DMC

            // Status
            0x4015 => {
                let mut result = 0;

                if self.pulse_1.length_counter > 0 { result |= 0x01 };
                if self.pulse_2.length_counter > 0 { result |= 0x02 };
                if self.noise.length_counter > 0 { result |= 0x04 };
                if self.triangle.length_counter > 0 { result |= 0x08 };
                if self.dmc.bytes_remaining > 0 { result |= 0x10 };
                if self.frame_interrupt {
                     result |= 0x40;
                     self.frame_interrupt = false;
                }
                if self.dmc.interrupt { result |= 0x80; }

                result
            }

            0x4017 => self.frame_counter,

            _ => 0 // panic!("unmapped apu register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // Channels
            0x4000..=0x4003 => self.pulse_1.write(address, value),      // Pulse 1
            0x4004..=0x4007 => self.pulse_2.write(address, value),      // Pulse 2
            0x4008..=0x400B => self.triangle.write(address, value),     // Triangle
            0x400C..=0x400F => self.noise.write(address, value),        // Noise
            0x4010..=0x4013 => self.dmc.write(address, value),          // DMC

            // Status
            0x4015 => {
                self.pulse_1.set_enabled(value & 0x01 != 0);
                self.pulse_2.set_enabled(value & 0x02 != 0);
                self.noise.set_enabled(value & 0x04 != 0);
                self.triangle.set_enabled(value & 0x08 != 0);
                self.dmc.set_enabled(value & 0x10 != 0);
                self.dmc.interrupt = false;
            },

            // Frame Counter
            0x4017 => {
                self.frame_counter = value & 0xC0;
                self.mode = if value & 0x80 != 0 { 5 } else { 4 };
                self.irq_inhibit = value & 0x40 != 0;
            },

            _ => panic!("unmapped apu register: {:#06X}", address)
        }
    }
}