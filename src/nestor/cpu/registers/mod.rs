
#[allow(dead_code)]
pub struct Registers {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0
        }
    }
}