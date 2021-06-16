use crate::nestor::traits::Power;

#[allow(dead_code)]
pub struct Registers {
    pc: u16,    // Program Counter
    a: u8,      // Accumulator
    x: u8,      // X Index
    y: u8,      // Y Index
    s: u8,      // Stack Pointer
    p: u8       // Status Register
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            p: 0x34
        }
    }
}


impl Power for Registers {

    fn power_up(&mut self) {
        self.pc = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.s = 0xFD;
        self.p = 0x34;
    }

    fn reset(&mut self) {
        self.s = self.s.wrapping_sub(3);
        self.p |= 0x04;
    }
}