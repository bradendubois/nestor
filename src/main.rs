mod nestor;

use nestor::cpu::CPU6502;

fn main() {
    println!("Hello, world!");

    let mut cpu = CPU6502::new();

    cpu.run();
}
