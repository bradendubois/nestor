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

| Test Name | Author | Status |
| `branch_timing_tests` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `cpu_dummy_reads` | [blargg](http://blargg.8bitalley.com/) |  <ul><li>[ ]</li></ul> |
| `cpu_dummy_writes` | [bisqwit](https://github.com/bisqwit) | <ul><li>[ ]</li></ul> |
| `cpu_exec_space` | [bisqwit](https://github.com/bisqwit) | <ul><li>[ ]</li></ul> |
| `cpu_interrupts_v2` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `cpu_reset` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `cpu_timing_test6` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `instr_misc` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `instr_test_v5` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `instr_timing` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `nestest` | [kevtris](http://kevtris.org/) | <ul><li>[ ]</li></ul> |
| `ram_retain` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |

### PPU Tests

| Test Name | Author | Status |
| `color_test` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `blargg_ppu_tests_2005.09.15b` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `full_palette` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `nmi_sync` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `ntsc_torture` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `oam_read` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `oam_stress` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `oamtest3` | lidnariq | <ul><li>[ ]</li></ul> |
| `palette` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `ppu_open_bus` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `ppu_read_buffer` | [bisqwit](https://github.com/bisqwit) | <ul><li>[ ]</li></ul> |
| `ppu_sprite_hit` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `ppu_sprite_overflow` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `ppu_vbl_nmi` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `scanline` | [Quietust](https://github.com/quietust) | <ul><li>[ ]</li></ul> |
| `sprdma_and_dmc_dma` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `sprite_hit_tests_2005.10.05` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `sprite_overflow_tests` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `tvpassfail` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `vbl_nmi_timing` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |



### Mapper Tests

### Input Tests
