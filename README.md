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
| `branch_timing_tests` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `cpu_dummy_reads` | [blargg](http://blargg.8bitalley.com/) |  :x: |
| `cpu_dummy_writes` | [bisqwit](https://github.com/bisqwit) | :x: |
| `cpu_exec_space` | [bisqwit](https://github.com/bisqwit) | :x: |
| `cpu_interrupts_v2` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `cpu_reset` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `cpu_timing_test6` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `instr_misc` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `instr_test_v5` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `instr_timing` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `nestest` | [kevtris](http://kevtris.org/) | :heavy_check_mark:\* |
| `ram_retain` | [rainwarrior](http://rainwarrior.ca/) | :x: |

\* Nestor does pass nestest's CPU component, as the PPU / APU are not implemented,

### PPU Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `color_test` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `blargg_ppu_tests_2005.09.15b` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `full_palette` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `nmi_sync` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `ntsc_torture` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `oam_read` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `oam_stress` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `oamtest3` | lidnariq | :x: |
| `palette` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `ppu_open_bus` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `ppu_read_buffer` | [bisqwit](https://github.com/bisqwit) | :x: |
| `ppu_sprite_hit` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `ppu_sprite_overflow` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `ppu_vbl_nmi` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `scanline` | [Quietust](https://github.com/quietust) | :x: |
| `sprdma_and_dmc_dma` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `sprite_hit_tests_2005.10.05` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `sprite_overflow_tests` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `tvpassfail` | [tepples](https://pineight.com/) | :x: |
| `vbl_nmi_timing` | [blargg](http://blargg.8bitalley.com/) | :x: |

### APU Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `apu_mixer` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `apu_phase_reset` | Rahsennor | :x: |
| `apu_reset` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `apu_test` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `blargg_apu_2005.07.30` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `dmc_dma_during_read4` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `dpcmletterbox` | [tepples](https://pineight.com/) | :x: |
| `square_timer_div2` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `test_apu_2 (1-10)` | x0000 | :x: |
| `test_apu_env` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `test_apu_sweep` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `test_apu_timers` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `test_tri_lin_ctr` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `volume_tests` | [tepples](https://pineight.com/) | :x: |

### Mapper Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `31_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `BNTest` | [tepples](https://pineight.com/) |  :x: |
| `bxrom_512k_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `FdslrqTests (v7)` | Sour |  :x: |
| `exram` | [Quietust](https://github.com/quietust) |  :x: |
| `famicom_audio_swap_tests` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `fme7acktest-r1` | [tepples](https://pineight.com/) |  :x: |
| `fme7ramtest-r1` | [tepples](https://pineight.com/) |  :x: |
| `Holy Mapperel` | [tepples](https://pineight.com/) |  :x: |
| `mmc3bigchrram` | [tepples](https://pineight.com/) |  :x: |
| `mmc3_test` | [blargg](http://blargg.8bitalley.com/) |  :x: |
| `mmc5test` | Drag |  :x: |
| `mmc5test_v2` | AWJ |  :x: |
| `serom` | lidnariq |  :x: |
| `NES 2.0 Submapper Test - 2_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `NES 2.0 Submapper Test - 3_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `NES 2.0 Submapper Test - 7_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `NES 2.0 Submapper Test - 34_test` | [rainwarrior](http://rainwarrior.ca/) |  :x: |
| `test28` | [tepples](https://pineight.com/) |  :x: |
| `vrc24test` | AWJ |  :x: |
| `vrc6test` | natt |  :x: |
| `mmc5ramsize` | [rainwarrior](http://rainwarrior.ca/) | :x: |

### Input Tests

| Test Name | Author | Status |
| :-: | :-: | :-: |
| `allpads` | [tepples](https://pineight.com/) | :x: |
| `dma_sync_test_v2` | Rahsennor | :x: |
| `PaddleTest3` | 3gengames | :x: |
| `read_joy3` | [blargg](http://blargg.8bitalley.com/) | :x: |
| `Zap Ruder` | [tepples](https://pineight.com/) | :x: |
| `spadtest-nes` | [tepples](https://pineight.com/) | :x: |
| `vaus_test` | [tepples](https://pineight.com/) | :x: |
| `mset` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `mict` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `Telling LYs` | [tepples](https://pineight.com/) | :x: |
| `crtltest` | [rainwarrior](http://rainwarrior.ca/) | :x: |
| `raw` | lidnariq | :x: |

## Acknowledgements

- The [Nesdev Wiki](http://wiki.nesdev.com/w/index.php/Nesdev_Wiki) for being an invaluable resource for every detail of development.
- Test ROM authors such as [blargg](http://blargg.8bitalley.com/), [rainwarrior](http://rainwarrior.ca/), and others, for providing extremely accessible means of testing and validation.
