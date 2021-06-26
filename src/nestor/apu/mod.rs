use crate::nestor::traits::{Power, MemoryMap};

pub struct APU {

    // Pulse 1 Channel
    pulse_1_main: u8,               // 0x4000
    pulse_1_sweep: u8,              // 0x4001
    pulse_1_timer_low: u8,          // 0x4002
    pulse_1_timer_upper: u8,        // 0x4003

    // Pulse 2 Channel
    pulse_2_main: u8,               // 0x4004
    pulse_2_sweep: u8,              // 0x4005
    pulse_2_timer_low: u8,          // 0x4006
    pulse_2_timer_upper: u8,        // 0x4007

    // Triangle Channel
    triangle_main: u8,              // 0x4008
    triangle_timer_low: u8,         // 0x400A
    triangle_timer_upper: u8,       // 0x400B

    // Noise Channel
    noise_main: u8,                 // 0x400C
    noise_loop_period: u8,          // 0x400E
    noise_length: u8,               // 0x400F

    // DMC Channel
    dmc_irq_main: u8,               // 0x4010
    dmc_direct: u8,                 // 0x4011
    dmc_sample_address: u8,         // 0x4012
    dmc_sample_length: u8,          // 0x4013

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
            pulse_1_timer_low: 0,
            pulse_1_timer_upper: 0,
            pulse_2_main: 0,
            pulse_2_sweep: 0,
            pulse_2_timer_low: 0,
            pulse_2_timer_upper: 0,
            triangle_main: 0,
            triangle_timer_low: 0,
            triangle_timer_upper: 0,
            noise_main: 0,
            noise_loop_period: 0,
            noise_length: 0,
            dmc_irq_main: 0,
            dmc_direct: 0,
            dmc_sample_address: 0,
            dmc_sample_length: 0,
            control: 0,
            status: 0,
            frame_counter: 0
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

            0x4015 => self.status,

            _ => 0 // panic!("unmapped apu register: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x4000 => self.pulse_1_main = value,
            0x4001 => self.pulse_1_sweep = value,
            0x4002 => self.pulse_1_timer_low = value,
            0x4003 => self.pulse_1_timer_upper = value,

            0x4004 => self.pulse_2_main = value,
            0x4005 => self.pulse_2_sweep = value,
            0x4006 => self.pulse_2_timer_low = value,
            0x4007 => self.pulse_2_timer_upper = value,

            0x4008 => self.triangle_main = value,
            0x400A => self.triangle_timer_low = value,
            0x400B => self.triangle_timer_upper = value,

            0x400C => self.noise_main = value,
            0x400E => self.noise_loop_period = value,
            0x400F => self.noise_length = value,

            0x4010 => self.dmc_irq_main = value,
            0x4011 => self.dmc_direct = value,
            0x4012 => self.dmc_sample_address = value,
            0x4013 => self.dmc_sample_length = value,

            0x4015 => self.control = value,
            0x4017 => self.frame_counter = value,

            _ => panic!("unmapped apu register: {:#06X}", address)
        }
    }
}