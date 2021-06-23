# nestor

A WIP NES emulator, written in Rust.

## Description

Nestor is in early development, aiming to be a highly-accurate NES Emulator, with goals of passing the major of ROM tests publicly available.

- All official (and unofficial) opcodes implemented
- Nestor currently passes `nestest` (PPU notwithstanding)

## Testing Status

In extremely early development, Nestor does not pass almost *any* of these tests, nor does it even have many of the necessary components implemented. These tables are a 'target' to be reached *someday*.

Sources:
- ROM files: [github.com / christopherpow / nes-test-roms](https://github.com/christopherpow/nes-test-roms)
- Tables & ROM files: [Nes Dev Wiki](https://wiki.nesdev.com/w/index.php/Emulator_tests)

### CPU Tests

| Test Name | Author | Description | Status |
| `branch_timing_tests` | [blargg](http://blargg.8bitalley.com/) | "These ROMs test timing of the branch instruction, including edge cases" | <ul><li>[ ]</li></ul> |
| `cpu_dummy_reads` | [blargg](http://blargg.8bitalley.com/) | "TTests the CPU's dummy reads" | <ul><li>[ ]</li></ul> |
| `cpu_dummy_writes` | [bisqwit](https://github.com/bisqwit) | "Tests the CPU's dummy writes" | <ul><li>[ ]</li></ul> |
| `cpu_exec_space` | [bisqwit](https://github.com/bisqwit) | "Verifies that the CPU can execute code from any possible memory location, even if that is mapped as I/O space" | <ul><li>[ ]</li></ul> |
| `cpu_interrupts_v2` | [blargg](http://blargg.8bitalley.com/) | "Tests the behavior and timing of CPU in the presence of interrupts, both IRQ and NMI; see CPU interrupts." | <ul><li>[ ]</li></ul> |
| `cpu_reset` | [blargg](http://blargg.8bitalley.com/) | "Tests CPU registers just after power and changes during reset, and that RAM isn't changed during reset." | <ul><li>[ ]</li></ul> |
| `cpu_timing_test6` | [blargg](http://blargg.8bitalley.com/) | "This program tests instruction timing for all official and unofficial NES 6502 instructions except the 8 branch instructions (Bxx) and the 12 halt instructions (HLT)" | <ul><li>[ ]</li></ul> |
| `instr_misc` | [blargg](http://blargg.8bitalley.com/) | "Tests some miscellaneous aspects of instructions, including behavior when 16-bit address wraps around, and dummy reads." | <ul><li>[ ]</li></ul> |
| `instr_test_v5` | [blargg](http://blargg.8bitalley.com/) | "Tests official and unofficial CPU instructions and lists which ones failed. It will work even if emulator has no PPU and only supports NROM, writing a copy of output to $6000 (see readme). This more thoroughly tests instructions, but can't help you figure out what's wrong beyond what instruction(s) are failing, so it's better for testing mature CPU emulators." | <ul><li>[ ]</li></ul> |
| `instr_timing` | [blargg](http://blargg.8bitalley.com/) | "Tests timing of all instructions, including unofficial ones, page-crossing, etc." | <ul><li>[ ]</li></ul> |
| `nestest` | [kevtris](http://kevtris.org/) | "fairly thoroughly tests CPU operation. This is the best test to start with when getting a CPU emulator working for the first time. Start execution at $C000 and compare execution with a [known good log](https://www.qmtpro.com/~nes/misc/nestest.log) (created using [Nintendulator](https://wiki.nesdev.com/w/index.php/Nintendulator), an emulator chosen by the test's author because its CPU was verified to function correctly, aside from some minor details of the power-up state)." | <ul><li>[ ]</li></ul> |
| `ram_retain` | [rainwarrior](http://rainwarrior.ca/) | "	RAM contents test, for displaying contents of RAM at power-on or after reset" | <ul><li>[ ]</li></ul> |

### PPU Tests

### Mapper Tests

### Input Tests
