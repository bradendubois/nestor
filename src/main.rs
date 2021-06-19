mod nestor;


fn main() {
    let mut nestor = nestor::Nestor::new("./roms/testing/nestest.nes");

    nestor.run();
    nestor.power_up();
    nestor.reset();
}
