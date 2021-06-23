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
| :-: | :-: | :-: |
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
| `nestest` | [kevtris](http://kevtris.org/) | <ul><li>[X]\*</li></ul> |
| `ram_retain` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |

\* Nestor does pass nestest's CPU component, as the PPU / APU are not implemented,

### PPU Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
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

### APU Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `apu_mixer` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `apu_phase_reset` | Rahsennor | <ul><li>[ ]</li></ul> |
| `apu_reset` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `apu_test` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `blargg_apu_2005.07.30` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `dmc_dma_during_read4` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `dpcmletterbox` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `square_timer_div2` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `test_apu_2 (1-10)` | x0000 | <ul><li>[ ]</li></ul> |
| `test_apu_env` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `test_apu_sweep` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `test_apu_timers` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `test_tri_lin_ctr` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `volume_tests` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |

### Mapper Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `31_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `BNTest` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `bxrom_512k_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `FdslrqTests (v7)` | Sour |  <ul><li>[ ]</li></ul> |
| `exram` | [Quietust](https://github.com/quietust) |  <ul><li>[ ]</li></ul> |
| `famicom_audio_swap_tests` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `fme7acktest-r1` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `fme7ramtest-r1` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `Holy Mapperel` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `mmc3bigchrram` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `mmc3_test` | [blargg](http://blargg.8bitalley.com/) |  <ul><li>[ ]</li></ul> |
| `mmc5test` | Drag |  <ul><li>[ ]</li></ul> |
| `mmc5test_v2` | AWJ |  <ul><li>[ ]</li></ul> |
| `serom` | lidnariq |  <ul><li>[ ]</li></ul> |
| `NES 2.0 Submapper Test - 2_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `NES 2.0 Submapper Test - 3_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `NES 2.0 Submapper Test - 7_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `NES 2.0 Submapper Test - 34_test` | [rainwarrior](http://rainwarrior.ca/) |  <ul><li>[ ]</li></ul> |
| `test28` | [tepples](https://pineight.com/) |  <ul><li>[ ]</li></ul> |
| `vrc24test` | AWJ |  <ul><li>[ ]</li></ul> |
| `vrc6test` | natt |  <ul><li>[ ]</li></ul> |
| `mmc5ramsize` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |

### Input Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `allpads` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `dma_sync_test_v2` | Rahsennor | <ul><li>[ ]</li></ul> |
| `PaddleTest3` | 3gengames | <ul><li>[ ]</li></ul> |
| `read_joy3` | [blargg](http://blargg.8bitalley.com/) | <ul><li>[ ]</li></ul> |
| `Zap Ruder` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `spadtest-nes` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `vaus_test` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `mset` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `mict` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `Telling LYs` | [tepples](https://pineight.com/) | <ul><li>[ ]</li></ul> |
| `crtltest` | [rainwarrior](http://rainwarrior.ca/) | <ul><li>[ ]</li></ul> |
| `raw` | lidnariq | <ul><li>[ ]</li></ul> |

## Acknowledgements

- The [Nesdev Wiki](http://wiki.nesdev.com/w/index.php/Nesdev_Wiki) for being an invaluable resource for every detail of development.
- Test ROM authors such as [blargg](http://blargg.8bitalley.com/), [rainwarrior](http://rainwarrior.ca/), and others, for providing extremely accessible means of testing and validation.
