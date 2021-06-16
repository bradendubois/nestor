pub struct CPU6502 {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8
}


impl CPU6502 {

    pub fn call(&mut self) {

        // TODO Placeholder
        let opcode: u8 = 0x00;

        let x = match opcode {
            0x00 => self.brk_impl(),
            0x01 => self.ora_x_ind(),
            0x02 => self.jam(),
            0x03 => self.slo_x_ind(),
            0x04 => self.nop_zpg(),
            0x05 => self.ora_zpg(),
            0x06 => self.asl_zpg(),
            0x07 => self.slo_zpg(),
            0x08 => self.php_impl(),
            0x09 => self.ora_num(),
            0x0A => self.asl_a(),
            0x0B => self.anc_num(),
            0x0C => self.nop_abs(),
            0x0D => self.ora_abs(),
            0x0E => self.asl_abs(),
            0x0F => self.slo_abs(),

            0x10 => self.bpl_rel(),
            0x11 => self.ora_ind_y(),
            0x12 => self.jam(),
            0x13 => self.slo_ind_y(),
            0x14 => self.nop_zpg_x(),
            0x15 => self.ora_zpg_x(),
            0x16 => self.asl_zpg_x(),
            0x17 => self.slo_zpg_x(),
            0x18 => self.clc_impl(),
            0x19 => self.ora_abs_y(),
            0x1A => self.nop_impl(),
            0x1B => self.slo_abs_y(),
            0x1C => self.nop_abs_x(),
            0x1D => self.ora_abs_x(),
            0x1E => self.asl_abs_x(),
            0x1F => self.slo_abs_x(),

            0x20 => self.jsr_abs(),
            0x21 => self.and_x_ind(),
            0x22 => self.jam(),
            0x23 => self.rla_x_ind(),
            0x24 => self.bit_zpg(),
            0x25 => self.and_zpg(),
            0x26 => self.rol_zpg(),
            0x27 => self.rla_zpg(),
            0x28 => self.plp_impl(),
            0x29 => self.and_num(),
            0x2A => self.rol_a(),
            0x2B => self.and_num(),
            0x2C => self.bit_abs(),
            0x2D => self.and_abs(),
            0x2E => self.rol_abs(),
            0x2F => self.rla_abs(),

            0x30 => self.bmi_rel(),
            0x31 => self.and_ind_y(),
            0x32 => self.jam(),
            0x33 => self.rla_ind_y(),
            0x34 => self.nop_zpg_x(),
            0x35 => self.and_zpg_x(),
            0x36 => self.rol_zpg_x(),
            0x37 => self.rla_zpg_x(),
            0x38 => self.sec_impl(),
            0x39 => self.and_abs_y(),
            0x3A => self.nop_impl(),
            0x3B => self.rla_abs_y(),
            0x3C => self.nop_abs_x(),
            0x3D => self.and_abs_x(),
            0x3E => self.rol_abs_x(),
            0x3F => self.rla_abs_x(),

            0x40 => self.rti_impl(),
            0x41 => self.eor_x_ind(),
            0x42 => self.jam(),
            0x43 => self.sre_x_ind(),
            0x44 => self.nop_zpg(),
            0x45 => self.eor_zpg(),
            0x46 => self.lsr_zpg(),
            0x47 => self.sre_zpg(),
            0x48 => self.pha_impl(),
            0x49 => self.eor_num(),
            0x4A => self.lsr_a(),
            0x4B => self.alr_num(),
            0x4C => self.jmp_abs(),
            0x4D => self.eor_abs(),
            0x4E => self.lsr_abs(),
            0x4F => self.sre_abs(),

            0x50 => self.bvc_rel(),
            0x51 => self.eor_ind_y(),
            0x52 => self.jam(),
            0x53 => self.sre_ind_y(),
            0x54 => self.nop_zpg_x(),
            0x55 => self.eor_zpg_x(),
            0x56 => self.lsr_zpg_x(),
            0x57 => self.sre_zpg_x(),
            0x58 => self.cli_impl(),
            0x59 => self.eor_abs_y(),
            0x5A => self.nop_impl(),
            0x5B => self.sre_abs_y(),
            0x5C => self.nop_abs_x(),
            0x5D => self.eor_abs_x(),
            0x5E => self.lsr_abs_x(),
            0x5F => self.sre_abs_x(),

            0x60 => self.rts_impl(),
            0x61 => self.adc_x_ind(),
            0x62 => self.jam(),
            0x63 => self.rra_x_ind(),
            0x64 => self.nop_zpg(),
            0x65 => self.adc_zpg(),
            0x66 => self.ror_zpg(),
            0x67 => self.rra_zpg(),
            0x68 => self.pla_impl(),
            0x69 => self.adc_num(),
            0x6A => self.ror_a(),
            0x6B => self.arr_num(),
            0x6C => self.jmp_ind(),
            0x6D => self.adc_abs(),
            0x6E => self.ror_abs(),
            0x6F => self.rra_abs(),

            0x70 => self.bvs_rel(),
            0x71 => self.adc_ind_y(),
            0x72 => self.jam(),
            0x73 => self.rra_ind_y(),
            0x74 => self.nop_zpg_x(),
            0x75 => self.adc_zpg_x(),
            0x76 => self.ror_zpg_x(),
            0x77 => self.rra_zpg_x(),
            0x78 => self.sei_impl(),
            0x79 => self.adc_abs_y(),
            0x7A => self.nop_impl(),
            0x7B => self.rra_abs_y(),
            0x7C => self.nop_abs_x(),
            0x7D => self.adc_abs_x(),
            0x7E => self.ror_abs_x(),
            0x7F => self.rra_abs_x(),

            0x80 => self.nop_num(),
            0x81 => self.sta_x_ind(),
            0x82 => self.nop_num(),
            0x83 => self.sax_x_ind(),
            0x84 => self.sty_zpg(),
            0x85 => self.sta_zpg(),
            0x86 => self.stx_zpg(),
            0x87 => self.sax_zpg(),
            0x88 => self.dey_impl(),
            0x89 => self.nop_num(),
            0x8A => self.txa_impl(),
            0x8B => self.ane_num(),
            0x8C => self.sty_abs(),
            0x8D => self.sta_abs(),
            0x8E => self.stx_abs(),
            0x8F => self.sax_abs(),

            0x90 => self.bcc_rel(),
            0x91 => self.sta_ind_y(),
            0x92 => self.jam(),
            0x93 => self.sha_ind_y(),
            0x94 => self.sty_zpg_x(),
            0x95 => self.sta_zpg_x(),
            0x96 => self.stx_zpg_y(),
            0x97 => self.sax_zpg_y(),
            0x98 => self.tya_impl(),
            0x99 => self.sta_abs_y(),
            0x9A => self.txs_impl(),
            0x9B => self.tas_abs_y(),
            0x9C => self.nop_abs_x(),
            0x9D => self.sta_abs_x(),
            0x9E => self.shx_abs_y(),
            0x9F => self.sha_abs_y(),

            0xA0 => self.ldy_num(),
            0xA1 => self.lda_x_ind(),
            0xA2 => self.ldx_num(),
            0xA3 => self.lax_x_ind(),
            0xA4 => self.ldy_zpg(),
            0xA5 => self.lda_zpg(),
            0xA6 => self.ldx_zpg(),
            0xA7 => self.lax_zpg(),
            0xA8 => self.tay_impl(),
            0xA9 => self.lda_num(),
            0xAA => self.tax_impl(),
            0xAB => self.lxa_num(),
            0xAC => self.ldy_abs(),
            0xAD => self.lda_abs(),
            0xAE => self.ldx_abs(),
            0xAF => self.lax_abs(),

            0xB0 => self.bcs_rel(),
            0xB1 => self.lda_ind_y(),
            0xB2 => self.jam(),
            0xB3 => self.lax_ind_y(),
            0xB4 => self.ldy_zpg_x(),
            0xB5 => self.lda_zpg_x(),
            0xB6 => self.ldx_zpg_y(),
            0xB7 => self.lax_zpg_y(),
            0xB8 => self.clv_impl(),
            0xB9 => self.lda_abs_y(),
            0xBA => self.tsx_impl(),
            0xBB => self.las_abs_y(),
            0xBC => self.ldy_abs_x(),
            0xBD => self.lda_abs_x(),
            0xBE => self.ldx_abs_y(),
            0xBF => self.lax_abs_y(),

            0xC0 => self.cpy_num(),
            0xC1 => self.cmp_x_ind(),
            0xC2 => self.nop_num(),
            0xC3 => self.dcp_x_ind(),
            0xC4 => self.cpy_zpg(),
            0xC5 => self.cmp_zpg(),
            0xC6 => self.dec_zpg(),
            0xC7 => self.dcp_zpg(),
            0xC8 => self.iny_impl(),
            0xC9 => self.cmp_num(),
            0xCA => self.dex_impl(),
            0xCB => self.sbx_num(),
            0xCC => self.cpy_abs(),
            0xCD => self.cmp_abs(),
            0xCE => self.dec_abs(),
            0xCF => self.dcp_abs(),

            0xD0 => self.bne_rel(),
            0xD1 => self.cmp_ind_y(),
            0xD2 => self.jam(),
            0xD3 => self.dcp_ind_y(),
            0xD4 => self.nop_zpg_x(),
            0xD5 => self.cmp_zpg_x(),
            0xD6 => self.dec_zpg_x(),
            0xD7 => self.dcp_zpg_x(),
            0xD8 => self.cld_impl(),
            0xD9 => self.cmp_abs_y(),
            0xDA => self.nop_impl(),
            0xDB => self.dcp_abs_y(),
            0xDC => self.nop_abs_x(),
            0xDD => self.cmp_abs_x(),
            0xDE => self.dec_abs_x(),
            0xDF => self.dcp_abs_x(),

            0xE0 => self.cpx_num(),
            0xE1 => self.sbc_x_ind(),
            0xE2 => self.nop_num(),
            0xE3 => self.isc_x_ind(),
            0xE4 => self.cpx_zpg(),
            0xE5 => self.sbc_zpg(),
            0xE6 => self.inc_zpg(),
            0xE7 => self.shx_abs_y(),
            0xE8 => self.inx_impl(),
            0xE9 => self.sbc_num(),
            0xEA => self.nop_impl(),
            0xEB => self.usbc_num(),
            0xEC => self.cpx_abs(),
            0xED => self.sbc_abs(),
            0xEE => self.inc_abs(),
            0xEF => self.isc_abs(),

            0xF0 => self.beq_rel(),
            0xF1 => self.sbc_ind_y(),
            0xF2 => self.jam(),
            0xF3 => self.isc_ind_y(),
            0xF4 => self.nop_zpg_x(),
            0xF5 => self.sbc_zpg_x(),
            0xF6 => self.inc_zpg_x(),
            0xF7 => self.isc_zpg_x(),
            0xF8 => self.sed_impl(),
            0xF9 => self.sbc_abs_y(),
            0xFA => self.nop_impl(),
            0xFB => self.isc_abs_y(),
            0xFC => self.nop_abs_x(),
            0xFD => self.sbc_abs_x(),
            0xFE => self.inc_abs_x(),
            0xFF => self.isc_abs_x(),

            _ => panic!("unmapped opcode: {:#06X}", opcode)
        };
    }

    // Official Opcodes

    /* Official 0x0X */

    fn brk_impl(&mut self) -> u8 {
        0
    }

    fn ora_x_ind(&mut self) -> u8 {
        0
    }

    fn ora_zpg(&mut self) -> u8 {
        0
    }

    fn asl_zpg(&mut self) -> u8 {
        0
    }

    fn php_impl(&mut self) -> u8 {
        0
    }

    fn ora_num(&mut self) -> u8 {
        0
    }

    fn asl_a(&mut self) -> u8 {
        0
    }

    fn ora_abs(&mut self) -> u8 {
        0
    }

    fn asl_abs(&mut self) -> u8 {
        0
    }

    /* Official 0x1X */

    fn bpl_rel(&mut self) -> u8 {
        0
    }

    fn ora_ind_y(&mut self) -> u8 {
        0
    }

    fn ora_zpg_x(&mut self) -> u8 {
        0
    }

    fn asl_zpg_x(&mut self) -> u8 {
        0
    }

    fn clc_impl(&mut self) -> u8 {
        0
    }

    fn ora_abs_y(&mut self) -> u8 {
        0
    }

    fn ora_abs_x(&mut self) -> u8 {
        0
    }

    fn asl_abs_x(&mut self) -> u8 {
        0
    }

    /* Official 0x2X */

    fn jsr_abs(&mut self) -> u8 {
        0
    }

    fn and_x_ind(&mut self) -> u8 {
        0
    }

    fn bit_zpg(&mut self) -> u8 {
        0
    }

    fn and_zpg(&mut self) -> u8 {
        0
    }

    fn rol_zpg(&mut self) -> u8 {
        0
    }

    fn plp_impl(&mut self) -> u8 {
        0
    }

    fn and_num(&mut self) -> u8 {
        0
    }

    fn rol_a(&mut self) -> u8 {
        0
    }

    fn bit_abs(&mut self) -> u8 {
        0
    }

    fn and_abs(&mut self) -> u8 {
        0
    }

    fn rol_abs(&mut self) -> u8 {
        0
    }

    /* Official 0x3X */

    fn bmi_rel(&mut self) -> u8 {
        0
    }

    fn and_ind_y(&mut self) -> u8 {
        0
    }

    fn and_zpg_x(&mut self) -> u8 {
        0
    }

    fn rol_zpg_x(&mut self) -> u8 {
        0
    }

    fn sec_impl(&mut self) -> u8 {
        0
    }

    fn and_abs_y(&mut self) -> u8 {
        0
    }

    fn and_abs_x(&mut self) -> u8 {
        0
    }

    fn rol_abs_x(&mut self) -> u8 {
        0
    }

    /* Official 0x4X */

    fn rti_impl(&mut self) -> u8 {
        0
    }

    fn eor_x_ind(&mut self) -> u8 {
        0
    }

    fn eor_zpg(&mut self) -> u8 {
        0
    }

    fn lsr_zpg(&mut self) -> u8 {
        0
    }

    fn pha_impl(&mut self) -> u8 {
        0
    }

    fn eor_num(&mut self) -> u8 {
        0
    }

    fn lsr_a(&mut self) -> u8 {
        0
    }

    fn jmp_abs(&mut self) -> u8 {
        0
    }

    fn eor_abs(&mut self) -> u8 {
        0
    }

    fn lsr_abs(&mut self) -> u8 {
        0
    }

    /* Official 0x5X */

    fn bvc_rel(&mut self) -> u8 {
        0
    }

    fn eor_ind_y(&mut self) -> u8 {
        0
    }

    fn eor_zpg_x(&mut self) -> u8 {
        0
    }

    fn lsr_zpg_x(&mut self) -> u8 {
        0
    }

    fn cli_impl(&mut self) -> u8 {
        0
    }

    fn eor_abs_y(&mut self) -> u8 {
        0
    }

    fn eor_abs_x(&mut self) -> u8 {
        0
    }

    fn lsr_abs_x(&mut self) -> u8 {
        0
    }

    /* Official 0x6X */

    fn rts_impl(&mut self) -> u8 {
        0
    }
    
    fn adc_x_ind(&mut self) -> u8 {
        0
    }
    
    fn adc_zpg(&mut self) -> u8 {
        0
    }
    
    fn ror_zpg(&mut self) -> u8 {
        0
    }
    
    fn pla_impl(&mut self) -> u8 {
        0
    }
    
    fn adc_num(&mut self) -> u8 {
        0
    }
    
    fn ror_a(&mut self) -> u8 {
        0
    }
    
    fn jmp_ind(&mut self) -> u8 {
        0
    }
    
    fn adc_abs(&mut self) -> u8 {
        0
    }
    
    fn ror_abs(&mut self) -> u8 {
        0
    }
    
    /* Official 0x7X */
    
    fn bvs_rel(&mut self) -> u8 {
        0
    }
    
    fn adc_ind_y(&mut self) -> u8 {
        0
    }
    
    fn adc_zpg_x(&mut self) -> u8 {
        0
    }
    
    fn ror_zpg_x(&mut self) -> u8 {
        0
    }
    
    fn sei_impl(&mut self) -> u8 {
        0
    }
    
    fn adc_abs_y(&mut self) -> u8 {
        0
    }
    
    fn adc_abs_x(&mut self) -> u8 {
        0
    }
    
    fn ror_abs_x(&mut self) -> u8 { 
        0
    }

    /* Official 0x8X */

    fn sta_x_ind(&mut self) -> u8 {
        0
    }

    fn sty_zpg(&mut self) -> u8 {
        0
    }

    fn sta_zpg(&mut self) -> u8 {
        0
    }

    fn stx_zpg(&mut self) -> u8 {
        0
    }

    fn dey_impl(&mut self) -> u8 {
        0
    }

    fn txa_impl(&mut self) -> u8 {
        0
    }

    fn sty_abs(&mut self) -> u8 {
        0
    }

    fn sta_abs(&mut self) -> u8 {
        0
    }

    fn stx_abs(&mut self) -> u8 {
        0
    }

    /* Official 0x9X */

    fn bcc_rel(&mut self) -> u8 {
        0
    }

    fn sta_ind_y(&mut self) -> u8 {
        0
    }

    fn sty_zpg_x(&mut self) -> u8 {
        0
    }

    fn sta_zpg_x(&mut self) -> u8 {
        0
    }

    fn stx_zpg_y(&mut self) -> u8 {
        0
    }

    fn tya_impl(&mut self) -> u8 {
        0
    }

    fn sta_abs_y(&mut self) -> u8 {
        0
    }

    fn txs_impl(&mut self) -> u8 {
        0
    }

    fn sta_abs_x(&mut self) -> u8 {
        0
    }

    /* Official 0xAX */

    fn ldy_num(&mut self) -> u8 {
        0
    }

    fn lda_x_ind(&mut self) -> u8 {
        0
    }

    fn ldx_num(&mut self) -> u8 {
        0
    }

    fn ldy_zpg(&mut self) -> u8 {
        0
    }

    fn lda_zpg(&mut self) -> u8 {
        0
    }

    fn ldx_zpg(&mut self) -> u8 {
        0
    }

    fn tay_impl(&mut self) -> u8 {
        0
    }

    fn lda_num(&mut self) -> u8 {
        0
    }

    fn tax_impl(&mut self) -> u8 {
        0
    }

    fn ldy_abs(&mut self) -> u8 {
        0
    }

    fn lda_abs(&mut self) -> u8 {
        0
    }

    fn ldx_abs(&mut self) -> u8 {
        0
    }

    /* Official 0xBX */

    fn bcs_rel(&mut self) -> u8 {
        0
    }

    fn lda_ind_y(&mut self) -> u8 {
        0
    }

    fn ldy_zpg_x(&mut self) -> u8 {
        0
    }

    fn lda_zpg_x(&mut self) -> u8 {
        0
    }

    fn ldx_zpg_y(&mut self) -> u8 {
        0
    }

    fn clv_impl(&mut self) -> u8 {
        0
    }

    fn lda_abs_y(&mut self) -> u8 {
        0
    }

    fn tsx_impl(&mut self) -> u8 {
        0
    }

    fn ldy_abs_x(&mut self) -> u8 {
        0
    }

    fn lda_abs_x(&mut self) -> u8 {
        0
    }

    fn ldx_abs_y(&mut self) -> u8 {
        0
    }

    /* Official 0xCX */

    fn cpy_num(&mut self) -> u8 {
        0
    }

    fn cmp_x_ind(&mut self) -> u8 {
        0
    }

    fn cpy_zpg(&mut self) -> u8 {
        0
    }

    fn cmp_zpg(&mut self) -> u8 {
        0
    }

    fn dec_zpg(&mut self) -> u8 {
        0
    }

    fn iny_impl(&mut self) -> u8 {
        0
    }

    fn cmp_num(&mut self) -> u8 {
        0
    }

    fn dex_impl(&mut self) -> u8 {
        0
    }

    fn cpy_abs(&mut self) -> u8 {
        0
    }

    fn cmp_abs(&mut self) -> u8 {
        0
    }

    fn dec_abs(&mut self) -> u8 {
        0
    }

    /* Official 0xDX */

    fn bne_rel(&mut self) -> u8 {
        0
    }

    fn cmp_ind_y(&mut self) -> u8 {
        0
    }

    fn cmp_zpg_x(&mut self) -> u8 {
        0
    }

    fn dec_zpg_x(&mut self) -> u8 {
        0
    }

    fn cld_impl(&mut self) -> u8 {
        0
    }

    fn cmp_abs_y(&mut self) -> u8 {
        0
    }

    fn cmp_abs_x(&mut self) -> u8 {
        0
    }

    fn dec_abs_x(&mut self) -> u8 {
        0
    }

    /* Official 0EX */

    fn cpx_num(&mut self) -> u8 {
        0
    }

    fn sbc_x_ind(&mut self) -> u8 {
        0
    }

    fn cpx_zpg(&mut self) -> u8 {
        0
    }

    fn sbc_zpg(&mut self) -> u8 {
        0
    }

    fn inc_zpg(&mut self) -> u8 {
        0
    }

    fn inx_impl(&mut self) -> u8 {
        0
    }

    fn sbc_num(&mut self) -> u8 {
        0
    }

    fn nop_impl(&mut self) -> u8 {
        0
    }

    fn cpx_abs(&mut self) -> u8 {
        0
    }

    fn sbc_abs(&mut self) -> u8 {
        0
    }

    fn inc_abs(&mut self) -> u8 {
        0
    }

    /* Official 0xFX */

    fn beq_rel(&mut self) -> u8 {
        0
    }

    fn sbc_ind_y(&mut self) -> u8 {
        0
    }

    fn sbc_zpg_x(&mut self) -> u8 {
        0
    }

    fn inc_zpg_x(&mut self) -> u8 {
        0
    }

    fn sed_impl(&mut self) -> u8 {
        0
    }

    fn sbc_abs_y(&mut self) -> u8 {
        0
    }

    fn sbc_abs_x(&mut self) -> u8 {
        0
    }

    fn inc_abs_x(&mut self) -> u8 {
        0
    }

    // Unofficial Opcodes

    fn jam(&mut self) -> u8 {
        0
    }

    fn nop_num(&mut self) -> u8 {
        0
    }

    fn slo_x_ind(&mut self) -> u8 {
        0
    }

    fn nop_zpg(&mut self) -> u8 {
        0
    }

    fn slo_zpg(&mut self) -> u8 {
        0
    }

    fn anc_num(&mut self) -> u8 {
        0
    }

    fn nop_abs(&mut self) -> u8 {
        0
    }

    fn slo_abs(&mut self) -> u8 {
        0
    }

    fn slo_ind_y(&mut self) -> u8 {
        0
    }

    fn zop_zpg_x (&mut self) -> u8 {
        0
    }

    fn slo_zpg_x(&mut self) -> u8 {
        0
    }

    fn slo_abs_y(&mut self) -> u8 {
        0
    }

    fn nop_abs_x(&mut self) -> u8 {
        0
    }

    fn slo_abs_x(&mut self) -> u8 {
        0
    }

    fn rla_x_ind(&mut self) -> u8 {
        0
    }

    fn rla_zpg(&mut self) -> u8 {
        0
    }

    fn rla_abs(&mut self) -> u8 {
        0
    }

    fn rla_ind_y(&mut self) -> u8 {
        0
    }

    fn nop_zpg_x(&mut self) -> u8 {
        0
    }

    fn rla_zpg_x(&mut self) -> u8 {
        0
    }

    fn rla_abs_y(&mut self) -> u8 {
        0
    }

    fn rla_abs_x(&mut self) -> u8 {
        0
    }

    fn sre_x_ind(&mut self) -> u8 {
        0
    }

    fn sre_zpg(&mut self) -> u8 {
        0
    }

    fn alr_num(&mut self) -> u8 {
        0
    }

    fn sre_abs(&mut self) -> u8 {
        0
    }

    fn sre_ind_y(&mut self) -> u8 {
        0
    }

    fn sre_zpg_x(&mut self) -> u8 {
        0
    }

    fn sre_abs_y(&mut self) -> u8 {
        0
    }

    fn sre_abs_x(&mut self) -> u8 {
        0
    }

    fn rra_x_ind(&mut self) -> u8 {
        0
    }

    fn rra_zpg(&mut self) -> u8 {
        0
    }

    fn arr_num(&mut self) -> u8 {
        0
    }

    fn rra_abs(&mut self) -> u8 {
        0
    }

    fn rra_ind_y(&mut self) -> u8 {
        0
    }

    fn rra_zpg_x(&mut self) -> u8 {
        0
    }

    fn rra_abs_y(&mut self) -> u8 {
        0
    }

    fn rra_abs_x(&mut self) -> u8 {
        0
    }

    fn sax_x_ind(&mut self) -> u8 {
        0
    }

    fn sax_zpg(&mut self) -> u8 {
        0
    }

    fn ane_num(&mut self) -> u8 {
        0
    }

    fn sax_abs(&mut self) -> u8 {
        0
    }

    fn sha_ind_y(&mut self) -> u8 {
        0
    }

    fn sax_zpg_y(&mut self) -> u8 {
        0
    }

    fn tas_abs_y(&mut self) -> u8 {
        0
    }

    fn shy_abs_x(&mut self) -> u8 {
        0
    }

    fn shx_abs_y(&mut self) -> u8 {
        0
    }

    fn sha_abs_y(&mut self) -> u8 {
        0
    }

    fn lax_x_ind(&mut self) -> u8 {
        0
    }

    fn lax_zpg(&mut self) -> u8 {
        0
    }

    fn lxa_num(&mut self) -> u8 {
        0
    }

    fn lax_abs(&mut self) -> u8 {
        0
    }

    fn lax_ind_y(&mut self) -> u8 {
        0
    }

    fn lax_zpg_y(&mut self) -> u8 {
        0
    }

    fn las_abs_y(&mut self) -> u8 {
        0
    }

    fn lax_abs_y(&mut self) -> u8 {
        0
    }

    fn dcp_x_ind(&mut self) -> u8 {
        0
    }

    fn dcp_zpg(&mut self) -> u8 {
        0
    }

    fn sbx_num(&mut self) -> u8 {
        0
    }

    fn dcp_abs(&mut self) -> u8 {
        0
    }

    fn dcp_ind_y(&mut self) -> u8 {
        0
    }


    fn dcp_zpg_x(&mut self) -> u8 {
        0
    }

    fn dcp_abs_y(&mut self) -> u8 {
        0
    }

    fn dcp_abs_x(&mut self) -> u8 {
        0
    }

    fn isc_x_ind(&mut self) -> u8 {
        0
    }

    fn isc_zpg(&mut self) -> u8 {
        0
    }

    fn usbc_num(&mut self) -> u8 {
        0
    }

    fn isc_abs(&mut self) -> u8 {
        0
    }

    fn isc_ind_y(&mut self) -> u8 {
        0
    }

    fn isc_zpg_x(&mut self) -> u8 {
        0
    }

    fn isc_abs_y(&mut self) -> u8 {
        0
    }

    fn isc_abs_x(&mut self) -> u8 {
        0
    }
}