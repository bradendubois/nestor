pub trait MemoryMap {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub trait Power {

    fn power_up(&mut self);
    fn reset(&mut self);
}


pub trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
