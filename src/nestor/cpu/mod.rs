mod instructions;
mod registers;

use registers::Registers;


#[allow(dead_code)]
pub struct CPU6502 {
    registers: Registers
}


impl CPU6502 {

    pub fn new() -> CPU6502 {
        CPU6502 {
            registers: Registers::new()
        }
    }

    pub fn run(&mut self) {
        let x = 0;
        self.call(x);
    }
}