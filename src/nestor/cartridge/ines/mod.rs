
#[allow(dead_code)]
pub struct INesHeader {
    constant: Vec<u8>,      // Bytes 0-3
    prg_rom_units: u8,      // Byte 4
    chr_rom_units: u8,      // Byte 5

    // Flags 6 (0x06)
    mirroring: u8,          // Bit 0 - 0: Horizontal, 1: Vertical
    battery_prg_ram: bool,  // Bit 1 - 0: False, 1: True (at 0x6000-0x7FFF)
    trainer: bool,          // Bit 2 - 0: False, 1: True (at 0x7000-0x71FF)
    ignore_mirror: bool,    // Bit 3 - 0: False, 1: True
    lower_map_nibble: u8,   // Bit 4-7

    // Flags 7 (0x07)
    vs_unisystem: bool,     // Bit 0
    playchoice_10: bool,    // Bit 1 - 0: False, 1: True (hint data after CHR data)
    nes_2_format: bool,     // Bit 2-3: 2 = Flags 8-15 in 2.0 Format
    upper_map_nibble: u8,   // Bit 4-7

    // Flags 8 (0x08)
    program_ram: u8,        // 8 KB units

    // Flags 9 (0x09)
    tv_system: bool,        // Bit 0 - 0 (False): NTSC, 1 (True) = PAL

    // Flags 10 (0x10)
    tv_system_2: u8,        // Bit 0-1: 0: NTSC, 2: PAL, 1 or 3: DUAL Compatible
    prg_ram: bool,          // Bit 4: 0 (True): Present (at 0x6000-0x7000), 1 (False): Not Present
    bus_conflicts: bool,    // Bit 5: 0 (False): No conflicts, 1 (True): Board has bus conflicts
}

impl INesHeader {

    pub fn new(header_data: &Vec<u8>) -> INesHeader {
        let flags_6 = header_data[6];
        let flags_7 = header_data[7];
        let flags_8 = header_data[8];
        let flags_9 = header_data[9];
        let flags_10 = header_data[10];

        let upper_map_nibble = flags_7 & 0xF0;
        let lower_map_nibble = flags_6 & 0xF0 >> 4;

        INesHeader {
            constant: header_data[0..4].to_vec(),
            prg_rom_units: header_data[4],
            chr_rom_units: header_data[5],

            // Flags 6
            mirroring: flags_6 & 0x01,
            battery_prg_ram: flags_6 & 0x02 != 0,
            trainer: flags_6 & 04 != 0,
            ignore_mirror: flags_6 & 08 != 0,
            lower_map_nibble,

            // Flags 7
            vs_unisystem: flags_7 & 0x01 != 0,
            playchoice_10: flags_7 & 0x02 != 0,
            nes_2_format: flags_7 & 0x0C == 0x0C,
            upper_map_nibble,

            program_ram: flags_8,             // Flags 8
            tv_system: flags_9 & 0x01 != 0,   // Flags 9

            // Flags 10
            tv_system_2: flags_10 & 0x03,
            prg_ram: flags_10 & 0x10 != 0,
            bus_conflicts: flags_10 & 0x20 != 0,
        }
    }
}


#[allow(dead_code)]
pub struct INes {
    header: INesHeader,
    trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    instr_rom: Option<Vec<u8>>,
    prom: Option<Vec<u8>>,
    pub data: Vec<u8>,

    pub mapper: u8          // iNES Mapper Number
}

impl INes {

    pub fn new(data: Vec<u8>) -> INes {

        let slice = data[0..=15].to_vec();

        let header = INesHeader::new(&slice);

        let real_data = data[0x10..].to_vec();
        let mut i = 0;

        let trainer = match header.trainer {
            true  => {
                let res = &real_data[i..512];
                i += 512;
                Some(res.to_vec())
            },
            false => None
        };

        let prg_rom_size = header.prg_rom_units as usize * 0x4000;
        let prg_rom = real_data[i..i+prg_rom_size].to_vec();
        i += prg_rom_size;

        let chr_rom_size = header.chr_rom_units as usize * 0x2000;
        let chr_rom = real_data[i..i+chr_rom_size].to_vec();

        let mut data = prg_rom.clone();
        data.extend(chr_rom.iter());

        let mapper = header.upper_map_nibble | header.lower_map_nibble;

        // panic!("{:?} {:#06X}", prg_rom, prg_rom.len());

        INes {
            header,
            trainer,
            prg_rom,
            chr_rom,
            instr_rom: None,
            prom: None,
            data,
            mapper
        }
    }
}
