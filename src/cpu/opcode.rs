use crate::cpu::cpu::{Cpu, InterruptType};
use crate::cpu::opcode::Opcode::*;
use crate::cpu::registers::*;
use crate::mmu::bus::Bus;
use std::borrow::BorrowMut;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    // miscellaneous
    NOP,
    STOP,
    HALT,
    SCF,
    CCF,
    DAA,
    CPL,
    DI,
    EI,
    // 8bit load
    LD_B_B,
    LD_B_C,
    LD_B_D,
    LD_B_E,
    LD_B_H,
    LD_B_L,
    LD_B__HL_,
    LD_B_A,
    LD_C_B,
    LD_C_C,
    LD_C_D,
    LD_C_E,
    LD_C_H,
    LD_C_L,
    LD_C__HL_,
    LD_C_A,
    LD_D_B,
    LD_D_C,
    LD_D_D,
    LD_D_E,
    LD_D_H,
    LD_D_L,
    LD_D__HL_,
    LD_D_A,
    LD_E_B,
    LD_E_C,
    LD_E_D,
    LD_E_E,
    LD_E_H,
    LD_E_L,
    LD_E__HL_,
    LD_E_A,
    LD_H_B,
    LD_H_C,
    LD_H_D,
    LD_H_E,
    LD_H_H,
    LD_H_L,
    LD_H__HL_,
    LD_H_A,
    LD_L_B,
    LD_L_C,
    LD_L_D,
    LD_L_E,
    LD_L_H,
    LD_L_L,
    LD_L__HL_,
    LD_L_A,
    LD__HL__B,
    LD__HL__C,
    LD__HL__D,
    LD__HL__E,
    LD__HL__H,
    LD__HL__L,
    LD__HL__A,
    LD_A_B,
    LD_A_C,
    LD_A_D,
    LD_A_E,
    LD_A_H,
    LD_A_L,
    LD_A__HL_,
    LD_A_A,
    LD_B_n,
    LD_C_n,
    LD_D_n,
    LD_E_n,
    LD_H_n,
    LD_L_n,
    LD__HL__n,
    LD_A_n,
    LD_A__BC_,
    LD_A__DE_,
    LD__BC__A,
    LD__DE__A,
    LD_A__nn_,
    LD__nn__A,
    LDH_A__C_,
    LDH__C__A,
    LDH_A__n_,
    LDH__n__A,
    LD_A__HLDEC_,
    LD__HLDEC__A,
    LD_A__HLINC_,
    LD__HLINC__A,
    // 16bit load
    LD_BC_nn,
    LD_DE_nn,
    LD_HL_nn,
    LD_SP_nn,
    LD__nn__SP,
    LD_SP_HL,
    PUSH_BC,
    PUSH_DE,
    PUSH_HL,
    PUSH_AF,
    POP_BC,
    POP_DE,
    POP_HL,
    POP_AF,
    LD_HL_SP_e,
    // 8 bit arithmetic
    INC_B,
    INC_C,
    INC_D,
    INC_E,
    INC_H,
    INC_L,
    INC__HL_,
    INC_A,
    DEC_B,
    DEC_C,
    DEC_D,
    DEC_E,
    DEC_H,
    DEC_L,
    DEC__HL_,
    DEC_A,
    ADD_B,
    ADD_C,
    ADD_D,
    ADD_E,
    ADD_H,
    ADD_L,
    ADD__HL_,
    ADD_A,
    ADC_B,
    ADC_C,
    ADC_D,
    ADC_E,
    ADC_H,
    ADC_L,
    ADC__HL_,
    ADC_A,
    SUB_B,
    SUB_C,
    SUB_D,
    SUB_E,
    SUB_H,
    SUB_L,
    SUB__HL_,
    SUB_A,
    SBC_B,
    SBC_C,
    SBC_D,
    SBC_E,
    SBC_H,
    SBC_L,
    SBC__HL_,
    SBC_A,
    AND_B,
    AND_C,
    AND_D,
    AND_E,
    AND_H,
    AND_L,
    AND__HL_,
    AND_A,
    XOR_B,
    XOR_C,
    XOR_D,
    XOR_E,
    XOR_H,
    XOR_L,
    XOR__HL_,
    XOR_A,
    OR_B,
    OR_C,
    OR_D,
    OR_E,
    OR_H,
    OR_L,
    OR__HL_,
    OR_A,
    CP_B,
    CP_C,
    CP_D,
    CP_E,
    CP_H,
    CP_L,
    CP__HL_,
    CP_A,
    ADD_n,
    ADC_n,
    SUB_n,
    SBC_n,
    AND_n,
    XOR_n,
    OR_n,
    CP_n,
    // 16bit arithmetic
    INC_BC,
    INC_DE,
    INC_HL,
    INC_SP,
    DEC_BC,
    DEC_DE,
    DEC_HL,
    DEC_SP,
    ADD_HL_BC,
    ADD_HL_DE,
    ADD_HL_HL,
    ADD_HL_SP,
    ADD_SP_e,
    // Rotate, shifts and bit operations
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC_B,
    RLC_C,
    RLC_D,
    RLC_E,
    RLC_H,
    RLC_L,
    RLC__HL_,
    RLC_A,
    RRC_B,
    RRC_C,
    RRC_D,
    RRC_E,
    RRC_H,
    RRC_L,
    RRC__HL_,
    RRC_A,
    RL_B,
    RL_C,
    RL_D,
    RL_E,
    RL_H,
    RL_L,
    RL__HL_,
    RL_A,
    RR_B,
    RR_C,
    RR_D,
    RR_E,
    RR_H,
    RR_L,
    RR__HL_,
    RR_A,
    SLA_B,
    SLA_C,
    SLA_D,
    SLA_E,
    SLA_H,
    SLA_L,
    SLA__HL_,
    SLA_A,
    SRA_B,
    SRA_C,
    SRA_D,
    SRA_E,
    SRA_H,
    SRA_L,
    SRA__HL_,
    SRA_A,
    SWAP_B,
    SWAP_C,
    SWAP_D,
    SWAP_E,
    SWAP_H,
    SWAP_L,
    SWAP__HL_,
    SWAP_A,
    SRL_B,
    SRL_C,
    SRL_D,
    SRL_E,
    SRL_H,
    SRL_L,
    SRL__HL_,
    SRL_A,
    BIT_0_B,
    BIT_0_C,
    BIT_0_D,
    BIT_0_E,
    BIT_0_H,
    BIT_0_L,
    BIT_0__HL_,
    BIT_0_A,
    BIT_1_B,
    BIT_1_C,
    BIT_1_D,
    BIT_1_E,
    BIT_1_H,
    BIT_1_L,
    BIT_1__HL_,
    BIT_1_A,
    BIT_2_B,
    BIT_2_C,
    BIT_2_D,
    BIT_2_E,
    BIT_2_H,
    BIT_2_L,
    BIT_2__HL_,
    BIT_2_A,
    BIT_3_B,
    BIT_3_C,
    BIT_3_D,
    BIT_3_E,
    BIT_3_H,
    BIT_3_L,
    BIT_3__HL_,
    BIT_3_A,
    BIT_4_B,
    BIT_4_C,
    BIT_4_D,
    BIT_4_E,
    BIT_4_H,
    BIT_4_L,
    BIT_4__HL_,
    BIT_4_A,
    BIT_5_B,
    BIT_5_C,
    BIT_5_D,
    BIT_5_E,
    BIT_5_H,
    BIT_5_L,
    BIT_5__HL_,
    BIT_5_A,
    BIT_6_B,
    BIT_6_C,
    BIT_6_D,
    BIT_6_E,
    BIT_6_H,
    BIT_6_L,
    BIT_6__HL_,
    BIT_6_A,
    BIT_7_B,
    BIT_7_C,
    BIT_7_D,
    BIT_7_E,
    BIT_7_H,
    BIT_7_L,
    BIT_7__HL_,
    BIT_7_A,
    RES_0_B,
    RES_0_C,
    RES_0_D,
    RES_0_E,
    RES_0_H,
    RES_0_L,
    RES_0__HL_,
    RES_0_A,
    RES_1_B,
    RES_1_C,
    RES_1_D,
    RES_1_E,
    RES_1_H,
    RES_1_L,
    RES_1__HL_,
    RES_1_A,
    RES_2_B,
    RES_2_C,
    RES_2_D,
    RES_2_E,
    RES_2_H,
    RES_2_L,
    RES_2__HL_,
    RES_2_A,
    RES_3_B,
    RES_3_C,
    RES_3_D,
    RES_3_E,
    RES_3_H,
    RES_3_L,
    RES_3__HL_,
    RES_3_A,
    RES_4_B,
    RES_4_C,
    RES_4_D,
    RES_4_E,
    RES_4_H,
    RES_4_L,
    RES_4__HL_,
    RES_4_A,
    RES_5_B,
    RES_5_C,
    RES_5_D,
    RES_5_E,
    RES_5_H,
    RES_5_L,
    RES_5__HL_,
    RES_5_A,
    RES_6_B,
    RES_6_C,
    RES_6_D,
    RES_6_E,
    RES_6_H,
    RES_6_L,
    RES_6__HL_,
    RES_6_A,
    RES_7_B,
    RES_7_C,
    RES_7_D,
    RES_7_E,
    RES_7_H,
    RES_7_L,
    RES_7__HL_,
    RES_7_A,
    SET_0_B,
    SET_0_C,
    SET_0_D,
    SET_0_E,
    SET_0_H,
    SET_0_L,
    SET_0__HL_,
    SET_0_A,
    SET_1_B,
    SET_1_C,
    SET_1_D,
    SET_1_E,
    SET_1_H,
    SET_1_L,
    SET_1__HL_,
    SET_1_A,
    SET_2_B,
    SET_2_C,
    SET_2_D,
    SET_2_E,
    SET_2_H,
    SET_2_L,
    SET_2__HL_,
    SET_2_A,
    SET_3_B,
    SET_3_C,
    SET_3_D,
    SET_3_E,
    SET_3_H,
    SET_3_L,
    SET_3__HL_,
    SET_3_A,
    SET_4_B,
    SET_4_C,
    SET_4_D,
    SET_4_E,
    SET_4_H,
    SET_4_L,
    SET_4__HL_,
    SET_4_A,
    SET_5_B,
    SET_5_C,
    SET_5_D,
    SET_5_E,
    SET_5_H,
    SET_5_L,
    SET_5__HL_,
    SET_5_A,
    SET_6_B,
    SET_6_C,
    SET_6_D,
    SET_6_E,
    SET_6_H,
    SET_6_L,
    SET_6__HL_,
    SET_6_A,
    SET_7_B,
    SET_7_C,
    SET_7_D,
    SET_7_E,
    SET_7_H,
    SET_7_L,
    SET_7__HL_,
    SET_7_A,
    // control flow
    JP_nn,
    JP_HL,
    JP_Z_nn,
    JP_C_nn,
    JP_NZ_nn,
    JP_NC_nn,
    JR_e,
    JR_Z_e,
    JR_C_e,
    JR_NZ_e,
    JR_NC_e,
    CALL_nn,
    CALL_Z_nn,
    CALL_C_nn,
    CALL_NZ_nn,
    CALL_NC_nn,
    RET,
    RET_Z,
    RET_C,
    RET_NZ,
    RET_NC,
    RETI,
    RST_0x00,
    RST_0x08,
    RST_0x10,
    RST_0x18,
    RST_0x20,
    RST_0x28,
    RST_0x30,
    RST_0x38,
    RST_0x40,
    RST_0x48,
    RST_0x50,
    RST_0x58,
    RST_0x60,
}

impl Opcode {
    pub fn fetch_cb(cb: u8) -> Opcode {
        match cb {
            0x00 => RLC_B,
            0x01 => RLC_C,
            0x02 => RLC_D,
            0x03 => RLC_E,
            0x04 => RLC_H,
            0x05 => RLC_L,
            0x06 => RLC__HL_,
            0x07 => RLC_A,
            0x08 => RRC_B,
            0x09 => RRC_C,
            0x0A => RRC_D,
            0x0B => RRC_E,
            0x0C => RRC_H,
            0x0D => RRC_L,
            0x0E => RRC__HL_,
            0x0F => RRC_A,
            0x10 => RL_B,
            0x11 => RL_C,
            0x12 => RL_D,
            0x13 => RL_E,
            0x14 => RL_H,
            0x15 => RL_L,
            0x16 => RL__HL_,
            0x17 => RL_A,
            0x18 => RR_B,
            0x19 => RR_C,
            0x1A => RR_D,
            0x1B => RR_E,
            0x1C => RR_H,
            0x1D => RR_L,
            0x1E => RR__HL_,
            0x1F => RR_A,
            0x20 => SLA_B,
            0x21 => SLA_C,
            0x22 => SLA_D,
            0x23 => SLA_E,
            0x24 => SLA_H,
            0x25 => SLA_L,
            0x26 => SLA__HL_,
            0x27 => SLA_A,
            0x28 => SRA_B,
            0x29 => SRA_C,
            0x2A => SRA_D,
            0x2B => SRA_E,
            0x2C => SRA_H,
            0x2D => SRA_L,
            0x2E => SRA__HL_,
            0x2F => SRA_A,
            0x30 => SWAP_B,
            0x31 => SWAP_C,
            0x32 => SWAP_D,
            0x33 => SWAP_E,
            0x34 => SWAP_H,
            0x35 => SWAP_L,
            0x36 => SWAP__HL_,
            0x37 => SWAP_A,
            0x38 => SRL_B,
            0x39 => SRL_C,
            0x3A => SRL_D,
            0x3B => SRL_E,
            0x3C => SRL_H,
            0x3D => SRL_L,
            0x3E => SRL__HL_,
            0x3F => SRL_A,
            0x40 => BIT_0_B,
            0x41 => BIT_0_C,
            0x42 => BIT_0_D,
            0x43 => BIT_0_E,
            0x44 => BIT_0_H,
            0x45 => BIT_0_L,
            0x46 => BIT_0__HL_,
            0x47 => BIT_0_A,
            0x48 => BIT_1_B,
            0x49 => BIT_1_C,
            0x4A => BIT_1_D,
            0x4B => BIT_1_E,
            0x4C => BIT_1_H,
            0x4D => BIT_1_L,
            0x4E => BIT_1__HL_,
            0x4F => BIT_1_A,
            0x50 => BIT_2_B,
            0x51 => BIT_2_C,
            0x52 => BIT_2_D,
            0x53 => BIT_2_E,
            0x54 => BIT_2_H,
            0x55 => BIT_2_L,
            0x56 => BIT_2__HL_,
            0x57 => BIT_2_A,
            0x58 => BIT_3_B,
            0x59 => BIT_3_C,
            0x5A => BIT_3_D,
            0x5B => BIT_3_E,
            0x5C => BIT_3_H,
            0x5D => BIT_3_L,
            0x5E => BIT_3__HL_,
            0x5F => BIT_3_A,
            0x60 => BIT_4_B,
            0x61 => BIT_4_C,
            0x62 => BIT_4_D,
            0x63 => BIT_4_E,
            0x64 => BIT_4_H,
            0x65 => BIT_4_L,
            0x66 => BIT_4__HL_,
            0x67 => BIT_4_A,
            0x68 => BIT_5_B,
            0x69 => BIT_5_C,
            0x6A => BIT_5_D,
            0x6B => BIT_5_E,
            0x6C => BIT_5_H,
            0x6D => BIT_5_L,
            0x6E => BIT_5__HL_,
            0x6F => BIT_5_A,
            0x70 => BIT_6_B,
            0x71 => BIT_6_C,
            0x72 => BIT_6_D,
            0x73 => BIT_6_E,
            0x74 => BIT_6_H,
            0x75 => BIT_6_L,
            0x76 => BIT_6__HL_,
            0x77 => BIT_6_A,
            0x78 => BIT_7_B,
            0x79 => BIT_7_C,
            0x7A => BIT_7_D,
            0x7B => BIT_7_E,
            0x7C => BIT_7_H,
            0x7D => BIT_7_L,
            0x7E => BIT_7__HL_,
            0x7F => BIT_7_A,
            0x80 => RES_0_B,
            0x81 => RES_0_C,
            0x82 => RES_0_D,
            0x83 => RES_0_E,
            0x84 => RES_0_H,
            0x85 => RES_0_L,
            0x86 => RES_0__HL_,
            0x87 => RES_0_A,
            0x88 => RES_1_B,
            0x89 => RES_1_C,
            0x8A => RES_1_D,
            0x8B => RES_1_E,
            0x8C => RES_1_H,
            0x8D => RES_1_L,
            0x8E => RES_1__HL_,
            0x8F => RES_1_A,
            0x90 => RES_2_B,
            0x91 => RES_2_C,
            0x92 => RES_2_D,
            0x93 => RES_2_E,
            0x94 => RES_2_H,
            0x95 => RES_2_L,
            0x96 => RES_2__HL_,
            0x97 => RES_2_A,
            0x98 => RES_3_B,
            0x99 => RES_3_C,
            0x9A => RES_3_D,
            0x9B => RES_3_E,
            0x9C => RES_3_H,
            0x9D => RES_3_L,
            0x9E => RES_3__HL_,
            0x9F => RES_3_A,
            0xA0 => RES_4_B,
            0xA1 => RES_4_C,
            0xA2 => RES_4_D,
            0xA3 => RES_4_E,
            0xA4 => RES_4_H,
            0xA5 => RES_4_L,
            0xA6 => RES_4__HL_,
            0xA7 => RES_4_A,
            0xA8 => RES_5_B,
            0xA9 => RES_5_C,
            0xAA => RES_5_D,
            0xAB => RES_5_E,
            0xAC => RES_5_H,
            0xAD => RES_5_L,
            0xAE => RES_5__HL_,
            0xAF => RES_5_A,
            0xB0 => RES_6_B,
            0xB1 => RES_6_C,
            0xB2 => RES_6_D,
            0xB3 => RES_6_E,
            0xB4 => RES_6_H,
            0xB5 => RES_6_L,
            0xB6 => RES_6__HL_,
            0xB7 => RES_6_A,
            0xB8 => RES_7_B,
            0xB9 => RES_7_C,
            0xBA => RES_7_D,
            0xBB => RES_7_E,
            0xBC => RES_7_H,
            0xBD => RES_7_L,
            0xBE => RES_7__HL_,
            0xBF => RES_7_A,
            0xC0 => SET_0_B,
            0xC1 => SET_0_C,
            0xC2 => SET_0_D,
            0xC3 => SET_0_E,
            0xC4 => SET_0_H,
            0xC5 => SET_0_L,
            0xC6 => SET_0__HL_,
            0xC7 => SET_0_A,
            0xC8 => SET_1_B,
            0xC9 => SET_1_C,
            0xCA => SET_1_D,
            0xCB => SET_1_E,
            0xCC => SET_1_H,
            0xCD => SET_1_L,
            0xCE => SET_1__HL_,
            0xCF => SET_1_A,
            0xD0 => SET_2_B,
            0xD1 => SET_2_C,
            0xD2 => SET_2_D,
            0xD3 => SET_2_E,
            0xD4 => SET_2_H,
            0xD5 => SET_2_L,
            0xD6 => SET_2__HL_,
            0xD7 => SET_2_A,
            0xD8 => SET_3_B,
            0xD9 => SET_3_C,
            0xDA => SET_3_D,
            0xDB => SET_3_E,
            0xDC => SET_3_H,
            0xDD => SET_3_L,
            0xDE => SET_3__HL_,
            0xDF => SET_3_A,
            0xE0 => SET_4_B,
            0xE1 => SET_4_C,
            0xE2 => SET_4_D,
            0xE3 => SET_4_E,
            0xE4 => SET_4_H,
            0xE5 => SET_4_L,
            0xE6 => SET_4__HL_,
            0xE7 => SET_4_A,
            0xE8 => SET_5_B,
            0xE9 => SET_5_C,
            0xEA => SET_5_D,
            0xEB => SET_5_E,
            0xEC => SET_5_H,
            0xED => SET_5_L,
            0xEE => SET_5__HL_,
            0xEF => SET_5_A,
            0xF0 => SET_6_B,
            0xF1 => SET_6_C,
            0xF2 => SET_6_D,
            0xF3 => SET_6_E,
            0xF4 => SET_6_H,
            0xF5 => SET_6_L,
            0xF6 => SET_6__HL_,
            0xF7 => SET_6_A,
            0xF8 => SET_7_B,
            0xF9 => SET_7_C,
            0xFA => SET_7_D,
            0xFB => SET_7_E,
            0xFC => SET_7_H,
            0xFD => SET_7_L,
            0xFE => SET_7__HL_,
            0xFF => SET_7_A
        }
    }
    pub fn fetch(d8: u8) -> Opcode {
        match d8 {
            0x00 => NOP,
            0x01 => LD_BC_nn,
            0x02 => LD__BC__A,
            0x03 => INC_BC,
            0x04 => INC_B,
            0x05 => DEC_B,
            0x06 => LD_B_n,
            0x07 => RLCA,
            0x08 => LD__nn__SP,
            0x09 => ADD_HL_BC,
            0x0A => LD_A__BC_,
            0x0B => DEC_BC,
            0x0C => INC_C,
            0x0D => DEC_C,
            0x0E => LD_C_n,
            0x0F => RRCA,
            0x10 => STOP,
            0x11 => LD_DE_nn,
            0x12 => LD__DE__A,
            0x13 => INC_DE,
            0x14 => INC_D,
            0x15 => DEC_D,
            0x16 => LD_D_n,
            0x17 => RLA,
            0x18 => JR_e,
            0x19 => ADD_HL_DE,
            0x1A => LD_A__DE_,
            0x1B => DEC_DE,
            0x1C => INC_E,
            0x1D => DEC_E,
            0x1E => LD_E_n,
            0x1F => RRA,
            0x20 => JR_NZ_e,
            0x21 => LD_HL_nn,
            0x22 => LD__HLINC__A,
            0x23 => INC_HL,
            0x24 => INC_H,
            0x25 => DEC_H,
            0x26 => LD_H_n,
            0x27 => DAA,
            0x28 => JR_Z_e,
            0x29 => ADD_HL_HL,
            0x2A => LD_A__HLINC_,
            0x2B => DEC_HL,
            0x2C => INC_L,
            0x2D => DEC_L,
            0x2E => LD_L_n,
            0x2F => CPL,
            0x30 => JR_NC_e,
            0x31 => LD_SP_nn,
            0x32 => LD__HLDEC__A,
            0x33 => INC_SP,
            0x34 => INC__HL_,
            0x35 => DEC__HL_,
            0x36 => LD__HL__n,
            0x37 => SCF,
            0x38 => JR_C_e,
            0x39 => ADD_HL_SP,
            0x3A => LD_A__HLDEC_,
            0x3B => DEC_SP,
            0x3C => INC_A,
            0x3D => DEC_A,
            0x3E => LD_A_n,
            0x3F => CCF,
            0x40 => LD_B_B,
            0x41 => LD_B_C,
            0x42 => LD_B_D,
            0x43 => LD_B_E,
            0x44 => LD_B_H,
            0x45 => LD_B_L,
            0x46 => LD_B__HL_,
            0x47 => LD_B_A,
            0x48 => LD_C_B,
            0x49 => LD_C_C,
            0x4A => LD_C_D,
            0x4B => LD_C_E,
            0x4C => LD_C_H,
            0x4D => LD_C_L,
            0x4E => LD_C__HL_,
            0x4F => LD_C_A,
            0x50 => LD_D_B,
            0x51 => LD_D_C,
            0x52 => LD_D_D,
            0x53 => LD_D_E,
            0x54 => LD_D_H,
            0x55 => LD_D_L,
            0x56 => LD_D__HL_,
            0x57 => LD_D_A,
            0x58 => LD_E_B,
            0x59 => LD_E_C,
            0x5A => LD_E_D,
            0x5B => LD_E_E,
            0x5C => LD_E_H,
            0x5D => LD_E_L,
            0x5E => LD_E__HL_,
            0x5F => LD_E_A,
            0x60 => LD_H_B,
            0x61 => LD_H_C,
            0x62 => LD_H_D,
            0x63 => LD_H_E,
            0x64 => LD_H_H,
            0x65 => LD_H_L,
            0x66 => LD_H__HL_,
            0x67 => LD_H_A,
            0x68 => LD_L_B,
            0x69 => LD_L_C,
            0x6A => LD_L_D,
            0x6B => LD_L_E,
            0x6C => LD_L_H,
            0x6D => LD_L_L,
            0x6E => LD_L__HL_,
            0x6F => LD_L_A,
            0x70 => LD__HL__B,
            0x71 => LD__HL__C,
            0x72 => LD__HL__D,
            0x73 => LD__HL__E,
            0x74 => LD__HL__H,
            0x75 => LD__HL__L,
            0x76 => HALT,
            0x77 => LD__HL__A,
            0x78 => LD_A_B,
            0x79 => LD_A_C,
            0x7A => LD_A_D,
            0x7B => LD_A_E,
            0x7C => LD_A_H,
            0x7D => LD_A_L,
            0x7E => LD_A__HL_,
            0x7F => LD_A_A,
            0x80 => ADD_B,
            0x81 => ADD_C,
            0x82 => ADD_D,
            0x83 => ADD_E,
            0x84 => ADD_H,
            0x85 => ADD_L,
            0x86 => ADD__HL_,
            0x87 => ADD_A,
            0x88 => ADC_B,
            0x89 => ADC_C,
            0x8A => ADC_D,
            0x8B => ADC_E,
            0x8C => ADC_H,
            0x8D => ADC_L,
            0x8E => ADC__HL_,
            0x8F => ADC_A,
            0x90 => SUB_B,
            0x91 => SUB_C,
            0x92 => SUB_D,
            0x93 => SUB_E,
            0x94 => SUB_H,
            0x95 => SUB_L,
            0x96 => SUB__HL_,
            0x97 => SUB_A,
            0x98 => SBC_B,
            0x99 => SBC_C,
            0x9A => SBC_D,
            0x9B => SBC_E,
            0x9C => SBC_H,
            0x9D => SBC_L,
            0x9E => SBC__HL_,
            0x9F => SBC_A,
            0xA0 => AND_B,
            0xA1 => AND_C,
            0xA2 => AND_D,
            0xA3 => AND_E,
            0xA4 => AND_H,
            0xA5 => AND_L,
            0xA6 => AND__HL_,
            0xA7 => AND_A,
            0xA8 => XOR_B,
            0xA9 => XOR_C,
            0xAA => XOR_D,
            0xAB => XOR_E,
            0xAC => XOR_H,
            0xAD => XOR_L,
            0xAE => XOR__HL_,
            0xAF => XOR_A,
            0xB0 => OR_B,
            0xB1 => OR_C,
            0xB2 => OR_D,
            0xB3 => OR_E,
            0xB4 => OR_H,
            0xB5 => OR_L,
            0xB6 => OR__HL_,
            0xB7 => OR_A,
            0xB8 => CP_B,
            0xB9 => CP_C,
            0xBA => CP_D,
            0xBB => CP_E,
            0xBC => CP_H,
            0xBD => CP_L,
            0xBE => CP__HL_,
            0xBF => CP_A,
            0xC0 => RET_NZ,
            0xC1 => POP_BC,
            0xC2 => JP_NZ_nn,
            0xC3 => JP_nn,
            0xC4 => CALL_NZ_nn,
            0xC5 => PUSH_BC,
            0xC6 => ADD_n,
            0xC7 => RST_0x00,
            0xC8 => RET_Z,
            0xC9 => RET,
            0xCA => JP_Z_nn,
            0xCC => CALL_Z_nn,
            0xCD => CALL_nn,
            0xCE => ADC_n,
            0xCF => RST_0x08,
            0xD0 => RET_NC,
            0xD1 => POP_DE,
            0xD2 => JP_NC_nn,
            0xD4 => CALL_NC_nn,
            0xD5 => PUSH_DE,
            0xD6 => SUB_n,
            0xD7 => RST_0x10,
            0xD8 => RET_C,
            0xD9 => RETI,
            0xDA => JP_C_nn,
            0xDC => CALL_C_nn,
            0xDE => SBC_n,
            0xDF => RST_0x18,
            0xE0 => LDH__n__A,
            0xE1 => POP_HL,
            0xE2 => LDH__C__A,
            0xE5 => PUSH_HL,
            0xE6 => AND_n,
            0xE7 => RST_0x20,
            0xE8 => ADD_SP_e,
            0xE9 => JP_HL,
            0xEA => LD__nn__A,
            0xEE => XOR_n,
            0xEF => RST_0x28,
            0xF0 => LDH_A__n_,
            0xF1 => POP_AF,
            0xF2 => LDH_A__C_,
            0xF3 => DI,
            0xF5 => PUSH_AF,
            0xF6 => OR_n,
            0xF7 => RST_0x30,
            0xF8 => LD_HL_SP_e,
            0xF9 => LD_SP_HL,
            0xFA => LD_A__nn_,
            0xFB => EI,
            0xFE => CP_n,
            0xFF => RST_0x38,
            _ => {
                println!("Opcode {:X?} is not implemented.", d8);
                NOP
            }
        }
    }
    pub fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let register = &mut cpu.registers;
        match self {
            // miscellaneous
            NOP => { 1 }
            STOP => {
                cpu.halted = true;
                println!("halted requested");
                0
            }
            HALT => {
                cpu.halted = true;
                println!("halted requested");
                0
            }
            SCF => scf(register),
            CCF => ccf(register),
            DAA => daa(register),
            CPL => cpl(register),
            DI => di(cpu),
            EI => ei(cpu),
            // 8bit load
            LD_B_B => ld_r_r(register, R::B, R::B),
            LD_B_C => ld_r_r(register, R::B, R::C),
            LD_B_D => ld_r_r(register, R::B, R::D),
            LD_B_E => ld_r_r(register, R::B, R::E),
            LD_B_H => ld_r_r(register, R::B, R::H),
            LD_B_L => ld_r_r(register, R::B, R::L),
            LD_B__HL_ => ld_r_d8(register, bus, R::B, RR::HL),
            LD_B_A => ld_r_r(register, R::B, R::A),
            LD_C_B => ld_r_r(register, R::C, R::B),
            LD_C_C => ld_r_r(register, R::C, R::C),
            LD_C_D => ld_r_r(register, R::C, R::D),
            LD_C_E => ld_r_r(register, R::C, R::E),
            LD_C_H => ld_r_r(register, R::C, R::H),
            LD_C_L => ld_r_r(register, R::C, R::L),
            LD_C__HL_ => ld_r_d8(register, bus, R::C, RR::HL),
            LD_C_A => ld_r_r(register, R::C, R::A),
            LD_D_B => ld_r_r(register, R::D, R::B),
            LD_D_C => ld_r_r(register, R::D, R::C),
            LD_D_D => ld_r_r(register, R::D, R::D),
            LD_D_E => ld_r_r(register, R::D, R::E),
            LD_D_H => ld_r_r(register, R::D, R::H),
            LD_D_L => ld_r_r(register, R::D, R::L),
            LD_D__HL_ => ld_r_d8(register, bus, R::D, RR::HL),
            LD_D_A => ld_r_r(register, R::D, R::A),
            LD_E_B => ld_r_r(register, R::E, R::B),
            LD_E_C => ld_r_r(register, R::E, R::C),
            LD_E_D => ld_r_r(register, R::E, R::D),
            LD_E_E => ld_r_r(register, R::E, R::E),
            LD_E_H => ld_r_r(register, R::E, R::H),
            LD_E_L => ld_r_r(register, R::E, R::L),
            LD_E__HL_ => ld_r_d8(register, bus, R::E, RR::HL),
            LD_E_A => ld_r_r(register, R::E, R::A),
            LD_H_B => ld_r_r(register, R::H, R::B),
            LD_H_C => ld_r_r(register, R::H, R::C),
            LD_H_D => ld_r_r(register, R::H, R::D),
            LD_H_E => ld_r_r(register, R::H, R::E),
            LD_H_H => ld_r_r(register, R::H, R::H),
            LD_H_L => ld_r_r(register, R::H, R::L),
            LD_H__HL_ => ld_r_d8(register, bus, R::H, RR::HL),
            LD_H_A => ld_r_r(register, R::H, R::A),
            LD_L_B => ld_r_r(register, R::L, R::B),
            LD_L_C => ld_r_r(register, R::L, R::C),
            LD_L_D => ld_r_r(register, R::L, R::D),
            LD_L_E => ld_r_r(register, R::L, R::E),
            LD_L_H => ld_r_r(register, R::L, R::H),
            LD_L_L => ld_r_r(register, R::L, R::L),
            LD_L__HL_ => ld_r_d8(register, bus, R::L, RR::HL),
            LD_L_A => ld_r_r(register, R::L, R::A),
            LD__HL__B => ld_d8_r(register, bus, RR::HL, R::B),
            LD__HL__C => ld_d8_r(register, bus, RR::HL, R::C),
            LD__HL__D => ld_d8_r(register, bus, RR::HL, R::D),
            LD__HL__E => ld_d8_r(register, bus, RR::HL, R::E),
            LD__HL__H => ld_d8_r(register, bus, RR::HL, R::H),
            LD__HL__L => ld_d8_r(register, bus, RR::HL, R::L),
            LD__HL__A => ld_d8_r(register, bus, RR::HL, R::A),
            LD_A_B => ld_r_r(register, R::A, R::B),
            LD_A_C => ld_r_r(register, R::A, R::C),
            LD_A_D => ld_r_r(register, R::A, R::D),
            LD_A_E => ld_r_r(register, R::A, R::E),
            LD_A_H => ld_r_r(register, R::A, R::H),
            LD_A_L => ld_r_r(register, R::A, R::L),
            LD_A__HL_ => ld_r_d8(register, bus, R::A, RR::HL),
            LD_A_A => ld_r_r(register, R::A, R::A),
            LD_B_n => ld_r_n(register, bus, R::B),
            LD_C_n => ld_r_n(register, bus, R::C),
            LD_D_n => ld_r_n(register, bus, R::D),
            LD_E_n => ld_r_n(register, bus, R::E),
            LD_H_n => ld_r_n(register, bus, R::H),
            LD_L_n => ld_r_n(register, bus, R::L),
            LD__HL__n => ld_hl_n(register, bus),
            LD_A_n => ld_r_n(register, bus, R::A),
            LD_A__BC_ => ld_r_d8(register, bus, R::A, RR::BC),
            LD_A__DE_ => ld_r_d8(register, bus, R::A, RR::DE),
            LD__BC__A => ld_d8_r(register, bus, RR::BC, R::A),
            LD__DE__A => ld_d8_r(register, bus, RR::DE, R::A),
            LD_A__nn_ => ld_r_nn(register, bus, R::A),
            LD__nn__A => ld_nn_r(register, bus, R::A),
            LDH_A__C_ => ldh_a_c(register, bus),
            LDH__C__A => ldh_c_a(register, bus),
            LDH_A__n_ => ldh_a_n(register, bus),
            LDH__n__A => ldh_n_a(register, bus),
            LD_A__HLDEC_ => ld_a_hldec(register, bus),
            LD__HLDEC__A => ld_hldec_a(register, bus),
            LD_A__HLINC_ => ld_a_hlinc(register, bus),
            LD__HLINC__A => ld_hlinc_a(register, bus),
            // 16bit load
            LD_BC_nn => ld_rr_nn(register, bus, RR::BC),
            LD_DE_nn => ld_rr_nn(register, bus, RR::DE),
            LD_HL_nn => ld_rr_nn(register, bus, RR::HL),
            LD_SP_nn => ld_rr_nn(register, bus, RR::SP),
            LD__nn__SP => ld_nn_sp(register, bus),
            LD_SP_HL => ld_sp_hl(register),
            PUSH_BC => push_rr(register, bus, RR::BC),
            PUSH_DE => push_rr(register, bus, RR::DE),
            PUSH_HL => push_rr(register, bus, RR::HL),
            PUSH_AF => push_rr(register, bus, RR::AF),
            POP_BC => pop_rr(register, bus, RR::BC),
            POP_DE => pop_rr(register, bus, RR::DE),
            POP_HL => pop_rr(register, bus, RR::HL),
            POP_AF => pop_rr(register, bus, RR::AF),
            LD_HL_SP_e => ldhl_sp_e8(register, bus),
            // 8 bit arithmetic
            INC_B => inc_r(register, R::B),
            INC_C => inc_r(register, R::C),
            INC_D => inc_r(register, R::D),
            INC_E => inc_r(register, R::E),
            INC_H => inc_r(register, R::H),
            INC_L => inc_r(register, R::L),
            INC__HL_ => inc_d8(register, bus),
            INC_A => inc_r(register, R::A),
            DEC_B => dec_r(register, R::B),
            DEC_C => dec_r(register, R::C),
            DEC_D => dec_r(register, R::D),
            DEC_E => dec_r(register, R::E),
            DEC_H => dec_r(register, R::H),
            DEC_L => dec_r(register, R::L),
            DEC__HL_ => dec_d8(register, bus),
            DEC_A => dec_r(register, R::A),
            ADD_B => add_r(register, R::B),
            ADD_C => add_r(register, R::C),
            ADD_D => add_r(register, R::D),
            ADD_E => add_r(register, R::E),
            ADD_H => add_r(register, R::H),
            ADD_L => add_r(register, R::L),
            ADD__HL_ => add_d8(register, bus),
            ADD_A => add_r(register, R::A),
            ADC_B => adc_r(register, R::B),
            ADC_C => adc_r(register, R::C),
            ADC_D => adc_r(register, R::D),
            ADC_E => adc_r(register, R::E),
            ADC_H => adc_r(register, R::H),
            ADC_L => adc_r(register, R::L),
            ADC__HL_ => adc_d8(register, bus),
            ADC_A => adc_r(register, R::A),
            SUB_B => sub_r(register, R::B),
            SUB_C => sub_r(register, R::C),
            SUB_D => sub_r(register, R::D),
            SUB_E => sub_r(register, R::E),
            SUB_H => sub_r(register, R::H),
            SUB_L => sub_r(register, R::L),
            SUB__HL_ => sub_d8(register, bus),
            SUB_A => sub_r(register, R::A),
            SBC_B => sbc_r(register, R::B),
            SBC_C => sbc_r(register, R::C),
            SBC_D => sbc_r(register, R::D),
            SBC_E => sbc_r(register, R::E),
            SBC_H => sbc_r(register, R::H),
            SBC_L => sbc_r(register, R::L),
            SBC__HL_ => sbc_d8(register, bus),
            SBC_A => sbc_r(register, R::A),
            AND_B => and_r(register, R::B),
            AND_C => and_r(register, R::C),
            AND_D => and_r(register, R::D),
            AND_E => and_r(register, R::E),
            AND_H => and_r(register, R::H),
            AND_L => and_r(register, R::L),
            AND__HL_ => and_d8(register, bus),
            AND_A => and_r(register, R::A),
            XOR_B => xor_r(register, R::B),
            XOR_C => xor_r(register, R::C),
            XOR_D => xor_r(register, R::D),
            XOR_E => xor_r(register, R::E),
            XOR_H => xor_r(register, R::H),
            XOR_L => xor_r(register, R::L),
            XOR__HL_ => xor_d8(register, bus),
            XOR_A => xor_r(register, R::A),
            OR_B => or_r(register, R::B),
            OR_C => or_r(register, R::C),
            OR_D => or_r(register, R::D),
            OR_E => or_r(register, R::E),
            OR_H => or_r(register, R::H),
            OR_L => or_r(register, R::L),
            OR__HL_ => or_d8(register, bus),
            OR_A => or_r(register, R::A),
            CP_B => cp_r(register, R::B),
            CP_C => cp_r(register, R::C),
            CP_D => cp_r(register, R::D),
            CP_E => cp_r(register, R::E),
            CP_H => cp_r(register, R::H),
            CP_L => cp_r(register, R::L),
            CP__HL_ => cp_d8(register, bus),
            CP_A => cp_r(register, R::A),
            ADD_n => add_n(register, bus),
            ADC_n => adc_n(register, bus),
            SUB_n => sub_n(register, bus),
            SBC_n => sbc_n(register, bus),
            AND_n => and_n(register, bus),
            XOR_n => xor_n(register, bus),
            OR_n => or_n(register, bus),
            CP_n => cp_n(register, bus),
            INC_BC => inc_rr(register, RR::BC),
            INC_DE => inc_rr(register, RR::DE),
            INC_HL => inc_rr(register, RR::HL),
            INC_SP => inc_rr(register, RR::SP),
            DEC_BC => dec_rr(register, RR::BC),
            DEC_DE => dec_rr(register, RR::DE),
            DEC_HL => dec_rr(register, RR::HL),
            DEC_SP => dec_rr(register, RR::SP),
            ADD_HL_BC => addhl_rr(register, RR::BC),
            ADD_HL_DE => addhl_rr(register, RR::DE),
            ADD_HL_HL => addhl_rr(register, RR::HL),
            ADD_HL_SP => addhl_rr(register, RR::SP),
            ADD_SP_e => addsp_e(register, bus),
            // Rotate, shifts and bit operations
            RLCA => rlca(register),
            RLA => rla(register),
            RRCA => rrca(register),
            RRA => rra(register),
            RLC_B => rlc_r8(register, R::B),
            RLC_C => rlc_r8(register, R::C),
            RLC_D => rlc_r8(register, R::D),
            RLC_E => rlc_r8(register, R::E),
            RLC_H => rlc_r8(register, R::H),
            RLC_L => rlc_r8(register, R::L),
            RLC__HL_ => rlc_d8(register, bus),
            RLC_A => rlc_r8(register, R::A),
            RRC_B => rrc_r8(register, R::B),
            RRC_C => rrc_r8(register, R::C),
            RRC_D => rrc_r8(register, R::D),
            RRC_E => rrc_r8(register, R::E),
            RRC_H => rrc_r8(register, R::H),
            RRC_L => rrc_r8(register, R::L),
            RRC__HL_ => rrc_d8(register, bus),
            RRC_A => rrc_r8(register, R::A),
            RL_B => rl_r8(register, R::B),
            RL_C => rl_r8(register, R::C),
            RL_D => rl_r8(register, R::D),
            RL_E => rl_r8(register, R::E),
            RL_H => rl_r8(register, R::H),
            RL_L => rl_r8(register, R::L),
            RL__HL_ => rl_d8(register, bus),
            RL_A => rl_r8(register, R::A),
            RR_B => rr_r8(register, R::B),
            RR_C => rr_r8(register, R::C),
            RR_D => rr_r8(register, R::D),
            RR_E => rr_r8(register, R::E),
            RR_H => rr_r8(register, R::H),
            RR_L => rr_r8(register, R::L),
            RR__HL_ => rr_d8(register, bus),
            RR_A => rr_r8(register, R::A),
            SLA_B => sla_r8(register, R::B),
            SLA_C => sla_r8(register, R::C),
            SLA_D => sla_r8(register, R::D),
            SLA_E => sla_r8(register, R::E),
            SLA_H => sla_r8(register, R::H),
            SLA_L => sla_r8(register, R::L),
            SLA__HL_ => sla_d8(register, bus),
            SLA_A => sla_r8(register, R::A),
            SRA_B => sra_r8(register, R::B),
            SRA_C => sra_r8(register, R::C),
            SRA_D => sra_r8(register, R::D),
            SRA_E => sra_r8(register, R::E),
            SRA_H => sra_r8(register, R::H),
            SRA_L => sra_r8(register, R::L),
            SRA__HL_ => sra_d8(register, bus),
            SRA_A => sra_r8(register, R::A),
            SWAP_B => swap_r8(register, R::B),
            SWAP_C => swap_r8(register, R::C),
            SWAP_D => swap_r8(register, R::D),
            SWAP_E => swap_r8(register, R::E),
            SWAP_H => swap_r8(register, R::H),
            SWAP_L => swap_r8(register, R::L),
            SWAP__HL_ => swap_d8(register, bus),
            SWAP_A => swap_r8(register, R::A),
            SRL_B => srl_r8(register, R::B),
            SRL_C => srl_r8(register, R::C),
            SRL_D => srl_r8(register, R::D),
            SRL_E => srl_r8(register, R::E),
            SRL_H => srl_r8(register, R::H),
            SRL_L => srl_r8(register, R::L),
            SRL__HL_ => srl_d8(register, bus),
            SRL_A => srl_r8(register, R::A),
            BIT_0_B => bit_r8(register, 0, R::B),
            BIT_0_C => bit_r8(register, 0, R::C),
            BIT_0_D => bit_r8(register, 0, R::D),
            BIT_0_E => bit_r8(register, 0, R::E),
            BIT_0_H => bit_r8(register, 0, R::H),
            BIT_0_L => bit_r8(register, 0, R::L),
            BIT_0__HL_ => bit_d8(register, 0, bus),
            BIT_0_A => bit_r8(register, 0, R::A),
            BIT_1_B => bit_r8(register, 1, R::B),
            BIT_1_C => bit_r8(register, 1, R::C),
            BIT_1_D => bit_r8(register, 1, R::D),
            BIT_1_E => bit_r8(register, 1, R::E),
            BIT_1_H => bit_r8(register, 1, R::H),
            BIT_1_L => bit_r8(register, 1, R::L),
            BIT_1__HL_ => bit_d8(register, 1, bus),
            BIT_1_A => bit_r8(register, 1, R::A),
            BIT_2_B => bit_r8(register, 2, R::B),
            BIT_2_C => bit_r8(register, 2, R::C),
            BIT_2_D => bit_r8(register, 2, R::D),
            BIT_2_E => bit_r8(register, 2, R::E),
            BIT_2_H => bit_r8(register, 2, R::H),
            BIT_2_L => bit_r8(register, 2, R::L),
            BIT_2__HL_ => bit_d8(register, 2, bus),
            BIT_2_A => bit_r8(register, 2, R::A),
            BIT_3_B => bit_r8(register, 3, R::B),
            BIT_3_C => bit_r8(register, 3, R::C),
            BIT_3_D => bit_r8(register, 3, R::D),
            BIT_3_E => bit_r8(register, 3, R::E),
            BIT_3_H => bit_r8(register, 3, R::H),
            BIT_3_L => bit_r8(register, 3, R::L),
            BIT_3__HL_ => bit_d8(register, 3, bus),
            BIT_3_A => bit_r8(register, 3, R::A),
            BIT_4_B => bit_r8(register, 4, R::B),
            BIT_4_C => bit_r8(register, 4, R::C),
            BIT_4_D => bit_r8(register, 4, R::D),
            BIT_4_E => bit_r8(register, 4, R::E),
            BIT_4_H => bit_r8(register, 4, R::H),
            BIT_4_L => bit_r8(register, 4, R::L),
            BIT_4__HL_ => bit_d8(register, 4, bus),
            BIT_4_A => bit_r8(register, 4, R::A),
            BIT_5_B => bit_r8(register, 5, R::B),
            BIT_5_C => bit_r8(register, 5, R::C),
            BIT_5_D => bit_r8(register, 5, R::D),
            BIT_5_E => bit_r8(register, 5, R::E),
            BIT_5_H => bit_r8(register, 5, R::H),
            BIT_5_L => bit_r8(register, 5, R::L),
            BIT_5__HL_ => bit_d8(register, 5, bus),
            BIT_5_A => bit_r8(register, 5, R::A),
            BIT_6_B => bit_r8(register, 6, R::B),
            BIT_6_C => bit_r8(register, 6, R::C),
            BIT_6_D => bit_r8(register, 6, R::D),
            BIT_6_E => bit_r8(register, 6, R::E),
            BIT_6_H => bit_r8(register, 6, R::H),
            BIT_6_L => bit_r8(register, 6, R::L),
            BIT_6__HL_ => bit_d8(register, 6, bus),
            BIT_6_A => bit_r8(register, 6, R::A),
            BIT_7_B => bit_r8(register, 7, R::B),
            BIT_7_C => bit_r8(register, 7, R::C),
            BIT_7_D => bit_r8(register, 7, R::D),
            BIT_7_E => bit_r8(register, 7, R::E),
            BIT_7_H => bit_r8(register, 7, R::H),
            BIT_7_L => bit_r8(register, 7, R::L),
            BIT_7__HL_ => bit_d8(register, 7, bus),
            BIT_7_A => bit_r8(register, 7, R::A),
            RES_0_B => res_r8(register, 0, R::B),
            RES_0_C => res_r8(register, 0, R::C),
            RES_0_D => res_r8(register, 0, R::D),
            RES_0_E => res_r8(register, 0, R::E),
            RES_0_H => res_r8(register, 0, R::H),
            RES_0_L => res_r8(register, 0, R::L),
            RES_0__HL_ => res_d8(register, 0, bus),
            RES_0_A => res_r8(register, 0, R::A),
            RES_1_B => res_r8(register, 1, R::B),
            RES_1_C => res_r8(register, 1, R::C),
            RES_1_D => res_r8(register, 1, R::D),
            RES_1_E => res_r8(register, 1, R::E),
            RES_1_H => res_r8(register, 1, R::H),
            RES_1_L => res_r8(register, 1, R::L),
            RES_1__HL_ => res_d8(register, 1, bus),
            RES_1_A => res_r8(register, 1, R::A),
            RES_2_B => res_r8(register, 2, R::B),
            RES_2_C => res_r8(register, 2, R::C),
            RES_2_D => res_r8(register, 2, R::D),
            RES_2_E => res_r8(register, 2, R::E),
            RES_2_H => res_r8(register, 2, R::H),
            RES_2_L => res_r8(register, 2, R::L),
            RES_2__HL_ => res_d8(register, 2, bus),
            RES_2_A => res_r8(register, 2, R::A),
            RES_3_B => res_r8(register, 3, R::B),
            RES_3_C => res_r8(register, 3, R::C),
            RES_3_D => res_r8(register, 3, R::D),
            RES_3_E => res_r8(register, 3, R::E),
            RES_3_H => res_r8(register, 3, R::H),
            RES_3_L => res_r8(register, 3, R::L),
            RES_3__HL_ => res_d8(register, 3, bus),
            RES_3_A => res_r8(register, 3, R::A),
            RES_4_B => res_r8(register, 4, R::B),
            RES_4_C => res_r8(register, 4, R::C),
            RES_4_D => res_r8(register, 4, R::D),
            RES_4_E => res_r8(register, 4, R::E),
            RES_4_H => res_r8(register, 4, R::H),
            RES_4_L => res_r8(register, 4, R::L),
            RES_4__HL_ => res_d8(register, 4, bus),
            RES_4_A => res_r8(register, 4, R::A),
            RES_5_B => res_r8(register, 5, R::B),
            RES_5_C => res_r8(register, 5, R::C),
            RES_5_D => res_r8(register, 5, R::D),
            RES_5_E => res_r8(register, 5, R::E),
            RES_5_H => res_r8(register, 5, R::H),
            RES_5_L => res_r8(register, 5, R::L),
            RES_5__HL_ => res_d8(register, 5, bus),
            RES_5_A => res_r8(register, 5, R::A),
            RES_6_B => res_r8(register, 6, R::B),
            RES_6_C => res_r8(register, 6, R::C),
            RES_6_D => res_r8(register, 6, R::D),
            RES_6_E => res_r8(register, 6, R::E),
            RES_6_H => res_r8(register, 6, R::H),
            RES_6_L => res_r8(register, 6, R::L),
            RES_6__HL_ => res_d8(register, 6, bus),
            RES_6_A => res_r8(register, 6, R::A),
            RES_7_B => res_r8(register, 7, R::B),
            RES_7_C => res_r8(register, 7, R::C),
            RES_7_D => res_r8(register, 7, R::D),
            RES_7_E => res_r8(register, 7, R::E),
            RES_7_H => res_r8(register, 7, R::H),
            RES_7_L => res_r8(register, 7, R::L),
            RES_7__HL_ => res_d8(register, 7, bus),
            RES_7_A => res_r8(register, 7, R::A),
            SET_0_B => set_r8(register, 0, R::B),
            SET_0_C => set_r8(register, 0, R::C),
            SET_0_D => set_r8(register, 0, R::D),
            SET_0_E => set_r8(register, 0, R::E),
            SET_0_H => set_r8(register, 0, R::H),
            SET_0_L => set_r8(register, 0, R::L),
            SET_0__HL_ => set_d8(register, 0, bus),
            SET_0_A => set_r8(register, 0, R::A),
            SET_1_B => set_r8(register, 1, R::B),
            SET_1_C => set_r8(register, 1, R::C),
            SET_1_D => set_r8(register, 1, R::D),
            SET_1_E => set_r8(register, 1, R::E),
            SET_1_H => set_r8(register, 1, R::H),
            SET_1_L => set_r8(register, 1, R::L),
            SET_1__HL_ => set_d8(register, 1, bus),
            SET_1_A => set_r8(register, 1, R::A),
            SET_2_B => set_r8(register, 2, R::B),
            SET_2_C => set_r8(register, 2, R::C),
            SET_2_D => set_r8(register, 2, R::D),
            SET_2_E => set_r8(register, 2, R::E),
            SET_2_H => set_r8(register, 2, R::H),
            SET_2_L => set_r8(register, 2, R::L),
            SET_2__HL_ => set_d8(register, 2, bus),
            SET_2_A => set_r8(register, 2, R::A),
            SET_3_B => set_r8(register, 3, R::B),
            SET_3_C => set_r8(register, 3, R::C),
            SET_3_D => set_r8(register, 3, R::D),
            SET_3_E => set_r8(register, 3, R::E),
            SET_3_H => set_r8(register, 3, R::H),
            SET_3_L => set_r8(register, 3, R::L),
            SET_3__HL_ => set_d8(register, 3, bus),
            SET_3_A => set_r8(register, 3, R::A),
            SET_4_B => set_r8(register, 4, R::B),
            SET_4_C => set_r8(register, 4, R::C),
            SET_4_D => set_r8(register, 4, R::D),
            SET_4_E => set_r8(register, 4, R::E),
            SET_4_H => set_r8(register, 4, R::H),
            SET_4_L => set_r8(register, 4, R::L),
            SET_4__HL_ => set_d8(register, 4, bus),
            SET_4_A => set_r8(register, 4, R::A),
            SET_5_B => set_r8(register, 5, R::B),
            SET_5_C => set_r8(register, 5, R::C),
            SET_5_D => set_r8(register, 5, R::D),
            SET_5_E => set_r8(register, 5, R::E),
            SET_5_H => set_r8(register, 5, R::H),
            SET_5_L => set_r8(register, 5, R::L),
            SET_5__HL_ => set_d8(register, 5, bus),
            SET_5_A => set_r8(register, 5, R::A),
            SET_6_B => set_r8(register, 6, R::B),
            SET_6_C => set_r8(register, 6, R::C),
            SET_6_D => set_r8(register, 6, R::D),
            SET_6_E => set_r8(register, 6, R::E),
            SET_6_H => set_r8(register, 6, R::H),
            SET_6_L => set_r8(register, 6, R::L),
            SET_6__HL_ => set_d8(register, 6, bus),
            SET_6_A => set_r8(register, 6, R::A),
            SET_7_B => set_r8(register, 7, R::B),
            SET_7_C => set_r8(register, 7, R::C),
            SET_7_D => set_r8(register, 7, R::D),
            SET_7_E => set_r8(register, 7, R::E),
            SET_7_H => set_r8(register, 7, R::H),
            SET_7_L => set_r8(register, 7, R::L),
            SET_7__HL_ => set_d8(register, 7, bus),
            SET_7_A => set_r8(register, 7, R::A),
            // control flow
            JP_nn => jp_nn(register, bus),
            JP_HL => jp_hl(register),
            JP_Z_nn => jp_cc_nn(register, F::Z, bus),
            JP_C_nn => jp_cc_nn(register, F::C, bus),
            JP_NZ_nn => jp_nc_nn(register, F::Z, bus),
            JP_NC_nn => jp_nc_nn(register, F::C, bus),
            JR_e => jr_e(register, bus),
            JR_Z_e => jr_cc_e(register, F::Z, bus),
            JR_C_e => jr_cc_e(register, F::C, bus),
            JR_NZ_e => jr_nc_e(register, F::Z, bus),
            JR_NC_e => jr_nc_e(register, F::C, bus),
            CALL_nn => call_nn(register, bus),
            CALL_Z_nn => call_cc_nn(register, F::Z, bus),
            CALL_C_nn => call_cc_nn(register, F::C, bus),
            CALL_NZ_nn => call_nc_nn(register, F::Z, bus),
            CALL_NC_nn => call_nc_nn(register, F::C, bus),
            RET => ret(register, bus),
            RET_Z => ret_cc(register, F::Z, bus),
            RET_C => ret_cc(register, F::C, bus),
            RET_NZ => ret_nc(register, F::Z, bus),
            RET_NC => ret_nc(register, F::C, bus),
            RETI => reti(cpu, bus),
            RST_0x00 => rst(cpu, 0x0000, bus),
            RST_0x08 => rst(cpu, 0x0008, bus),
            RST_0x10 => rst(cpu, 0x0010, bus),
            RST_0x18 => rst(cpu, 0x0018, bus),
            RST_0x20 => rst(cpu, 0x0020, bus),
            RST_0x28 => rst(cpu, 0x0028, bus),
            RST_0x30 => rst(cpu, 0x0030, bus),
            RST_0x38 => rst(cpu, 0x0038, bus),
            RST_0x40 => rst(cpu, 0x0040, bus),
            RST_0x48 => rst(cpu, 0x0048, bus),
            RST_0x50 => rst(cpu, 0x0050, bus),
            RST_0x58 => rst(cpu, 0x0058, bus),
            RST_0x60 => rst(cpu, 0x0060, bus),
        }
    }
    pub fn new_interrupt(interrupt_type: InterruptType) -> Opcode {
        match interrupt_type {
            InterruptType::VBLANK => Opcode::RST_0x40,
            InterruptType::LCD => Opcode::RST_0x48,
            InterruptType::TIMER => Opcode::RST_0x50,
            InterruptType::SERIAL => Opcode::RST_0x58,
            InterruptType::JOYPAD => Opcode::RST_0x60,
            _ => panic!("illegal interrupt"),
        }
    }
}

fn scf(register: &mut Registers) -> u8 {
    register.set_flag(F::C);
    register.unset_flag(F::H);
    register.unset_flag(F::N);
    1
}

fn ccf(register: &mut Registers) -> u8 {
    register.flag(F::C, !register.is_flag(F::C));
    register.unset_flag(F::H);
    register.unset_flag(F::N);
    1
}

fn daa(register: &mut Registers) -> u8 {
    let mut acc: u16 = register.get_r8(R::A) as u16;
    let c: bool = register.is_flag(F::C);
    let h: bool = register.is_flag(F::H);
    if register.is_flag(F::N) {
        if h { acc -= 0x6; };
        if c { acc -= 0x60; };
    } else {
        if h || ((acc & 0x0F) > 0x9) { acc += 0x6; };
        if c || (acc > 0x9F) {
            acc += 0x60;
            register.set_flag(F::C)
        };
    }
    let d8: u8 = (acc & 0xFF) as u8;
    register.set_r8(R::A, d8);
    register.flag(F::Z, d8 == 0);
    register.unset_flag(F::H);
    1
}

fn cpl(register: &mut Registers) -> u8 {
    register.set_r8(R::A, !register.get_r8(R::A));
    register.set_flag(F::N);
    register.set_flag(F::H);
    1
}

fn ei(cpu: &mut Cpu) -> u8 {
    cpu.ime_delay = true;
    1
}

fn di(cpu: &mut Cpu) -> u8 {
    cpu.ime_delay = false;
    cpu.ime = false;
    1
}

fn ld_r_r(register: &mut Registers, dst: R, src: R) -> u8 {
    let r: u8 = register.get_r8(src);
    register.set_r8(dst, r);
    1
}

fn ld_r_d8(register: &mut Registers, bus: &mut Bus, dst: R, src: RR) -> u8 {
    let rr: u16 = register.get_r16(src);
    let d8: u8 = bus.get_byte(rr);
    register.set_r8(dst, d8);
    2
}

fn ld_d8_r(register: &mut Registers, bus: &mut Bus, dst: RR, src: R) -> u8 {
    let r: u8 = register.get_r8(src);
    let d16: u16 = register.get_r16(dst);
    bus.set_byte(d16, r);
    2
}

fn ld_r_n(register: &mut Registers, bus: &mut Bus, dst: R) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    register.set_r8(dst, d8);
    2
}

fn ld_hl_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let d8: u8 = bus.get_byte(pc);
    let d16: u16 = register.get_r16(RR::HL);
    bus.set_byte(d16, d8);
    3
}

fn ld_r_nn(register: &mut Registers, bus: &mut Bus, r8: R) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let lsb: u8 = bus.get_byte(pc);
    let pc: u16 = register.get_pc_and_increase();
    let msb: u8 = bus.get_byte(pc);
    let address: u16 = (msb as u16) << 8 | lsb as u16;
    let d8: u8 = bus.get_byte(address);
    register.set_r8(r8, d8);
    4
}

fn ld_nn_r(register: &mut Registers, bus: &mut Bus, r8: R) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let lsb: u8 = bus.get_byte(pc);
    let pc: u16 = register.get_pc_and_increase();
    let msb: u8 = bus.get_byte(pc);
    let address: u16 = (msb as u16) << 8 | lsb as u16;
    bus.set_byte(address, register.get_r8(r8));
    4
}

fn ldh_a_c(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = 0xFF00 | register.get_r8(R::C) as u16;
    let d8: u8 = bus.get_byte(d16);
    register.set_r8(R::A, d8);
    2
}

fn ldh_c_a(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = register.get_r8(R::A);
    let d16: u16 = 0xFF00 | register.get_r8(R::C) as u16;
    bus.set_byte(d16, d8);
    2
}

fn ldh_a_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let d8: u8 = bus.get_byte(0xFF00 | bus.get_byte(pc) as u16);
    register.set_r8(R::A, d8);
    3
}

fn ldh_n_a(register: &mut Registers, bus: &mut Bus) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let d16: u16 = 0xFF00 | bus.get_byte(pc) as u16;
    bus.set_byte(d16, register.get_r8(R::A));
    3
}

fn ld_a_hldec(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_hl_and_decrease();
    let d8: u8 = bus.get_byte(d16);
    register.set_r8(R::A, d8);
    2
}

fn ld_hldec_a(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_hl_and_decrease();
    bus.set_byte(d16, register.get_r8(R::A));
    2
}

fn ld_a_hlinc(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_hl_and_increase();
    let d8: u8 = bus.get_byte(d16);
    register.set_r8(R::A, d8);
    2
}

fn ld_hlinc_a(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_hl_and_increase();
    bus.set_byte(d16, register.get_r8(R::A));
    2
}

fn ld_rr_nn(register: &mut Registers, bus: &mut Bus, rr: RR) -> u8 {
    let pc: u16 = register.get_pc_and_increase();
    let lsb: u8 = bus.get_byte(pc);
    let pc: u16 = register.get_pc_and_increase();
    let msb: u8 = bus.get_byte(pc);
    let d16: u16 = (msb as u16) << 8 | lsb as u16;
    register.set_r16(rr, d16);
    3
}

fn ld_nn_sp(register: &mut Registers, bus: &mut Bus) -> u8 {
    let rr: u16 = register.get_r16(RR::SP);
    let lsb = bus.get_byte(register.get_pc_and_increase());
    let msb = bus.get_byte(register.get_pc_and_increase());
    let nn: u16 = (msb as u16) << 8 | lsb as u16;
    bus.set_byte(nn, (rr & 0xFF) as u8);
    bus.set_byte(nn + 1, (rr >> 8) as u8);
    5
}

fn ld_sp_hl(register: &mut Registers) -> u8 {
    register.set_r16(RR::SP, register.get_r16(RR::HL));
    2
}

fn push_rr(register: &mut Registers, bus: &mut Bus, rr: RR) -> u8 {
    let d16: u16 = register.get_r16(rr);
    bus.set_byte(register.decrease_and_get_sp(), (d16 >> 8) as u8);
    bus.set_byte(register.decrease_and_get_sp(), (d16 & 0xFF) as u8);
    4
}

fn pop_rr(register: &mut Registers, bus: &mut Bus, rr: RR) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_sp_and_increase());
    let msb: u8 = bus.get_byte(register.get_sp_and_increase());
    register.set_r16(rr, (msb as u16) << 8 | lsb as u16);
    3
}

fn ldhl_sp_e8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let sp: u16 = register.get_r16(RR::SP);
    let e8: i8 = bus.get_byte(register.get_pc_and_increase()) as i8;
    register.set_r16(RR::HL, sp.wrapping_add(e8 as u16));
    register.unset_flags();
    if e8 > 0 {
        register.flag(F::H, (sp & 0xF).wrapping_add((e8 & 0xF) as u16) > 0xF);
        register.flag(F::C, (sp & 0xFF).wrapping_add(e8 as u16) > 0xFF);
    } else {
        register.flag(F::H, sp.wrapping_add(e8 as u16) & 0xF <= sp & 0xF);
        register.flag(F::C, sp.wrapping_add(e8 as u16) & 0xFF <= sp & 0xFF);
    }
    3
}

fn inc_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8.wrapping_add(1);
    register.set_r8(r, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((r8 & 0x0F) + 1) > 0x0F);
    1
}

fn inc_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let hl: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(hl);
    let value: u8 = d8.wrapping_add(1);
    bus.set_byte(hl, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F) + 1) > 0x0F);
    2
}

fn dec_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8.wrapping_sub(1);
    register.set_r8(r, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (r8 & 0x0F) == 0x00);
    1
}

fn dec_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let hl: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(hl);
    let value: u8 = d8.wrapping_sub(1);
    bus.set_byte(hl, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (d8 & 0x0F) == 0x00);
    2
}

fn add_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_add(r8);
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((r8 & 0x0F) + (a & 0x0F)) > 0x0F);
    register.flag(F::C, ((r8 & 0xFF) as u16 + (a & 0xFF) as u16) > 0xFF);
    1
}

fn add_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_add(d8);
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F) + (a & 0x0F)) > 0x0F);
    register.flag(F::C, ((d8 & 0xFF) as u16 + (a & 0xFF) as u16) > 0xFF);
    2
}

fn add_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_add(d8);
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F) + (a & 0x0F)) > 0x0F);
    register.flag(F::C, ((d8 & 0xFF) as u16 + (a & 0xFF) as u16) > 0xFF);
    2
}

fn adc_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };

    let value: u8 = a.wrapping_add(r8).wrapping_add(carry);

    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((r8 & 0x0F).wrapping_add(a & 0x0F).wrapping_add(carry)) > 0x0F);
    register.flag(F::C, (((r8 & 0xFF) as u16).wrapping_add((a & 0xFF) as u16).wrapping_add(carry as u16)) > 0xFF);
    1
}

fn adc_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };

    let value: u8 = a.wrapping_add(d8).wrapping_add(carry);

    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F).wrapping_add(a & 0x0F).wrapping_add(carry)) > 0x0F);
    register.flag(F::C, (((d8 & 0xFF) as u16).wrapping_add((a & 0xFF) as u16).wrapping_add(carry as u16)) > 0xFF);
    2
}

fn adc_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);

    let value: u8 = a.wrapping_add(d8).wrapping_add(carry);
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F).wrapping_add(a & 0x0F).wrapping_add(carry)) > 0x0F);
    register.flag(F::C, (((d8 & 0xFF) as u16).wrapping_add((a & 0xFF) as u16).wrapping_add(carry as u16)) > 0xFF);
    2
}

fn sub_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(r8);
    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (r8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, r8 > a);
    1
}

fn sub_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(d8);
    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (d8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, d8 > a);
    2
}

fn sub_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(d8);
    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (d8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, d8 > a);
    2
}

fn sbc_r(register: &mut Registers, r: R) -> u8 {
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);

    let value: u8 = a.wrapping_sub(r8).wrapping_sub(carry);
    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((r8 & 0x0F) + carry) > (a & 0x0F));
    register.flag(F::C, (r8 as u16 + carry as u16) > a as u16);
    1
}

fn sbc_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);

    let value: u8 = a.wrapping_sub(d8).wrapping_sub(carry);

    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F) + carry) > (a & 0x0F));
    register.flag(F::C, (d8 as u16 + carry as u16) > a as u16);
    2
}

fn sbc_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let carry: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);

    let value: u8 = a.wrapping_sub(d8).wrapping_sub(carry);

    register.set_r8(R::A, value);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, ((d8 & 0x0F) + carry) > (a & 0x0F));
    register.flag(F::C, (d8 as u16 + carry as u16) > a as u16);
    2
}

fn and_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a & r8;
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.set_flag(F::H);
    register.unset_flag(F::C);
    1
}

fn and_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a & d8;
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.set_flag(F::H);
    register.unset_flag(F::C);
    2
}

fn and_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a & d8;
    register.set_r8(R::A, value);
    register.unset_flag(F::N);
    register.flag(F::Z, value == 0);
    register.set_flag(F::H);
    register.unset_flag(F::C);
    2
}

fn xor_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a ^ r8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    1
}

fn xor_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a ^ d8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}

fn xor_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a ^ d8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}


fn or_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a | r8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    1
}

fn or_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a | d8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}

fn or_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a | d8;
    register.set_r8(R::A, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}

fn cp_r(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(r8);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (r8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, r8 > a);
    1
}

fn cp_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_r16(RR::HL));
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(d8);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (d8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, d8 > a);
    2
}

fn cp_n(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d8: u8 = bus.get_byte(register.get_pc_and_increase());
    let a: u8 = register.get_r8(R::A);
    let value: u8 = a.wrapping_sub(d8);
    register.set_flag(F::N);
    register.flag(F::Z, value == 0);
    register.flag(F::H, (d8 & 0x0F) > (a & 0x0F));
    register.flag(F::C, d8 > a);
    2
}

fn inc_rr(register: &mut Registers, rr: RR) -> u8 {
    register.set_r16(rr, register.get_r16(rr).wrapping_add(1));
    2
}

fn dec_rr(register: &mut Registers, rr: RR) -> u8 {
    register.set_r16(rr, register.get_r16(rr).wrapping_sub(1));
    2
}

fn addhl_rr(register: &mut Registers, rr: RR) -> u8 {
    let r16: u16 = register.get_r16(rr);
    let hl: u16 = register.get_r16(RR::HL);
    register.set_r16(RR::HL, hl.wrapping_add(r16));
    register.unset_flag(F::N);
    register.flag(F::H, (hl & 0xFFF).wrapping_add(r16 & 0xFFF) > 0xFFF);
    register.flag(F::C, (hl as u32).wrapping_add(r16 as u32) > 0xFFFF);
    2
}

fn addsp_e(register: &mut Registers, bus: &mut Bus) -> u8 {
    let sp: u16 = register.get_r16(RR::SP);
    let e8: i8 = bus.get_byte(register.get_pc_and_increase()) as i8;
    register.set_r16(RR::SP, sp.wrapping_add(e8 as u16));
    register.unset_flags();
    if e8 > 0 {
        register.flag(F::H, (sp & 0xF).wrapping_add((e8 & 0xF) as u16) > 0xF);
        register.flag(F::C, (sp & 0xFF).wrapping_add(e8 as u16) > 0xFF);
    } else {
        register.flag(F::H, sp.wrapping_add(e8 as u16) & 0xF <= sp & 0xF);
        register.flag(F::C, sp.wrapping_add(e8 as u16) & 0xFF <= sp & 0xFF);
    }
    4
}

fn rlca(register: &mut Registers) -> u8 {
    let a: u8 = register.get_r8(R::A);
    register.set_r8(R::A, a.rotate_left(1));
    register.unset_flags();
    register.flag(F::C, ((a >> 7) & 0x1) == 1);
    1
}

fn rlc_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8.rotate_left(1);
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, ((r8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rlc_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = d8.rotate_left(1);
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, ((d8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rla(register: &mut Registers) -> u8 {
    let a: u8 = register.get_r8(R::A);
    let c: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    register.set_r8(R::A, a << 1 | c);
    register.unset_flags();
    register.flag(F::C, ((a >> 7) & 0x1) == 1);
    1
}

fn rl_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let c: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let value: u8 = r8 << 1 | c;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, ((r8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rl_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let c: u8 = if register.is_flag(F::C) { 1 } else { 0 };
    let value: u8 = d8 << 1 | c;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, ((d8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rrca(register: &mut Registers) -> u8 {
    let a: u8 = register.get_r8(R::A);
    register.set_r8(R::A, a.rotate_right(1));
    register.unset_flags();
    register.flag(F::C, (a & 0x1) == 1);
    1
}

fn rrc_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8.rotate_right(1);
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, (r8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rrc_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = d8.rotate_right(1);
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, (d8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rra(register: &mut Registers) -> u8 {
    let a: u8 = register.get_r8(R::A);
    let c: u8 = if register.is_flag(F::C) { 0x80 } else { 0 };
    register.set_r8(R::A, c | a >> 1);
    register.unset_flags();
    register.flag(F::C, (a & 0x1) == 1);
    1
}

fn rr_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let c: u8 = if register.is_flag(F::C) { 0x80 } else { 0 };
    let value = c | r8 >> 1;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, (r8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn rr_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let c: u8 = if register.is_flag(F::C) { 0x80 } else { 0 };
    let value = c | d8 >> 1;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, (d8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn sla_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8 << 1;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, ((r8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn sla_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = d8 << 1;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, ((d8 >> 7) & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn sra_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8 >> 1 | r8 & 0x80;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, (r8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn sra_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = d8 >> 1 | d8 & 0x80;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, (d8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn swap_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = (r8 & 0xF) << 4 | (r8 & 0xF0) >> 4;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}

fn swap_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = (d8 & 0xF) << 4 | (d8 & 0xF0) >> 4;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::Z, value == 0);
    2
}

fn srl_r8(register: &mut Registers, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    let value: u8 = r8 >> 1;
    register.set_r8(r, value);
    register.unset_flags();
    register.flag(F::C, (r8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn srl_d8(register: &mut Registers, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    let value: u8 = d8 >> 1;
    bus.set_byte(d16, value);
    register.unset_flags();
    register.flag(F::C, (d8 & 0x1) == 1);
    register.flag(F::Z, value == 0);
    2
}

fn bit_r8(register: &mut Registers, u: u8, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    register.flag(F::Z, (r8 >> u) & 0x1 == 0);
    register.unset_flag(F::N);
    register.set_flag(F::H);
    2
}

fn bit_d8(register: &mut Registers, u: u8, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    register.flag(F::Z, (d8 >> u) & 0x1 == 0);
    register.unset_flag(F::N);
    register.set_flag(F::H);
    2
}

fn res_r8(register: &mut Registers, u: u8, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    register.set_r8(r, r8 & !(1 << u));
    2
}

fn res_d8(register: &mut Registers, u: u8, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    bus.set_byte(d16, d8 & !(1 << u));
    2
}

fn set_r8(register: &mut Registers, u: u8, r: R) -> u8 {
    let r8: u8 = register.get_r8(r);
    register.set_r8(r, r8 | (1 << u));
    2
}

fn set_d8(register: &mut Registers, u: u8, bus: &mut Bus) -> u8 {
    let d16: u16 = register.get_r16(RR::HL);
    let d8: u8 = bus.get_byte(d16);
    bus.set_byte(d16, d8 | (1 << u));
    2
}

fn jp_nn(register: &mut Registers, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
    4
}

fn jp_hl(register: &mut Registers) -> u8 {
    let hl: u16 = register.get_r16(RR::HL);
    register.set_r16(RR::PC, hl);
    1
}

fn jp_cc_nn(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    if register.is_flag(f) {
        register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
        return 4;
    }
    3
}

fn jp_nc_nn(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    if !register.is_flag(f) {
        register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
        return 4;
    }
    3
}

fn jr_e(register: &mut Registers, bus: &mut Bus) -> u8 {
    let e: i8 = bus.get_byte(register.get_pc_and_increase()) as i8;
    let pc: u16 = register.get_r16(RR::PC);
    register.set_r16(RR::PC, pc.wrapping_add(e as u16));
    3
}

fn jr_cc_e(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let e: i8 = bus.get_byte(register.get_pc_and_increase()) as i8;
    let pc: u16 = register.get_r16(RR::PC);
    if register.is_flag(f) {
        register.set_r16(RR::PC, pc.wrapping_add(e as u16));
        return 3;
    }
    2
}

fn jr_nc_e(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let e: i8 = bus.get_byte(register.get_pc_and_increase()) as i8;
    let pc: u16 = register.get_r16(RR::PC);
    if !register.is_flag(f) {
        register.set_r16(RR::PC, pc.wrapping_add(e as u16));
        return 3;
    }
    2
}

fn call_nn(register: &mut Registers, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    let pc: u16 = register.get_r16(RR::PC);
    bus.set_byte(register.decrease_and_get_sp(), (pc >> 8) as u8);
    bus.set_byte(register.decrease_and_get_sp(), (pc & 0xFF) as u8);
    register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
    6
}

fn call_cc_nn(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    if register.is_flag(f) {
        let pc: u16 = register.get_r16(RR::PC);
        bus.set_byte(register.decrease_and_get_sp(), (pc >> 8) as u8);
        bus.set_byte(register.decrease_and_get_sp(), (pc & 0xFF) as u8);
        register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
        return 6;
    }
    3
}

fn call_nc_nn(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_pc_and_increase());
    let msb: u8 = bus.get_byte(register.get_pc_and_increase());
    if !register.is_flag(f) {
        let pc: u16 = register.get_r16(RR::PC);
        bus.set_byte(register.decrease_and_get_sp(), (pc >> 8) as u8);
        bus.set_byte(register.decrease_and_get_sp(), (pc & 0xFF) as u8);
        register.set_r16(RR::PC, (msb as u16) << 8 | (lsb as u16));
        return 6;
    }
    3
}

fn ret(register: &mut Registers, bus: &mut Bus) -> u8 {
    let lsb: u8 = bus.get_byte(register.get_sp_and_increase());
    let msb: u8 = bus.get_byte(register.get_sp_and_increase());
    register.set_r16(RR::PC, (msb as u16) << 8 | lsb as u16);
    4
}

fn ret_cc(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    if register.is_flag(f) {
        let lsb: u8 = bus.get_byte(register.get_sp_and_increase());
        let msb: u8 = bus.get_byte(register.get_sp_and_increase());
        register.set_r16(RR::PC, (msb as u16) << 8 | lsb as u16);
        return 5;
    }
    2
}

fn ret_nc(register: &mut Registers, f: F, bus: &mut Bus) -> u8 {
    if !register.is_flag(f) {
        let lsb: u8 = bus.get_byte(register.get_sp_and_increase());
        let msb: u8 = bus.get_byte(register.get_sp_and_increase());
        register.set_r16(RR::PC, (msb as u16) << 8 | lsb as u16);
        return 5;
    }
    2
}

fn reti(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.ime_delay = true;
    let lsb: u8 = bus.get_byte(cpu.registers.get_sp_and_increase());
    let msb: u8 = bus.get_byte(cpu.registers.get_sp_and_increase());
    cpu.registers.set_r16(RR::PC, (msb as u16) << 8 | lsb as u16);
    2
}

fn rst(cpu: &mut Cpu, d16: u16, bus: &mut Bus) -> u8 {
    cpu.ime = true;
    cpu.ime_delay = false;
    let pc: u16 = cpu.registers.get_r16(RR::PC);
    bus.set_byte(cpu.registers.decrease_and_get_sp(), (pc >> 8) as u8);
    bus.set_byte(cpu.registers.decrease_and_get_sp(), (pc & 0xFF) as u8);
    cpu.registers.set_r16(RR::PC, d16);
    4
}