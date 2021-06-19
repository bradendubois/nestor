use crate::nestor::traits::Power;

#[allow(dead_code)]
pub struct Registers {
    pub pc: u16,    // Program Counter
    pub a: u8,      // Accumulator
    pub x: u8,      // X Index
    pub y: u8,      // Y Index
    pub s: u8,      // Stack Pointer
    pub p: u8       // Status Register
}

#[allow(dead_code)]
impl Registers {

    pub fn new() -> Registers {
        Registers {
            // pc: 0,
            pc: 0xC000,
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            // p: 0x34
            p: 0x24
        }
    }

    /// Negative (V) - Bit 7 (0x80)
    pub fn set_negative(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x80,
            false => self.p &= !0x80
        }
    }

    pub fn negative(&self) -> bool { self.p & 0x80 != 0 }

    /// Overflow (V) - Bit 6 (0x40)
    pub fn set_overflow(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x40,
            false => self.p &= !0x40
        }
    }

    pub fn overflow(&self) -> bool { self.p & 0x40 != 0 }

    /// Ignored (-) - Bit 5 (0x12)  - Not so 'ignored' if we're here, huh?
    pub fn set_ignored(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x20,
            false => self.p &= !0x20
        }
    }

    pub fn ignored(&mut self) -> bool { self.p & 0x20 != 0 }

    /// Break (B) - Bit 4 (0x10)
    pub fn set_break(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x10,
            false => self.p &= !0x10
        }
    }

    pub fn break_flag(&self) -> bool { self.p & 0x10 != 0 }

    /// Decimal (D) - Bit 3 (0x08)
    pub fn set_decimal(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x08,
            false => self.p &= !0x08
        }
    }

    pub fn decimal(&self) -> bool { self.p & 0x08 != 0 }

    /// Interrupt (I) - Bit 2 (0x04)
    pub fn set_interrupt(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x04,
            false => self.p &= !0x04
        }
    }

    pub fn interrupt(&self) -> bool { self.p & 0x04 != 0 }

    /// Zero (Z) - Bit 1 (0x02)
    pub fn set_zero(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x02,
            false => self.p &= !0x02
        }
    }

    pub fn zero(&self) -> bool { self.p & 0x02 != 0 }

    /// Carry (C) - Bit 0 (0x01)
    pub fn set_carry(&mut self, enabled: bool) {
        match enabled {
            true  => self.p |=  0x01,
            false => self.p &= !0x01
        }
    }

    pub fn carry(&self) -> bool { self.p & 0x01 != 0 }
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