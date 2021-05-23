use std::collections::HashMap;
use once_cell::sync::Lazy;

pub struct Opecode {
    pub command: Command,
    pub mode: AddressingMode,
    pub cycle: usize,
}


#[derive(PartialEq, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectAbsolute,
    PreIndexedIndirect,
    PostIndexedIndirect,
}

#[derive(Debug)]
pub enum Command {
    BNE,
    DEY,
    INX,
    JMP,
    LDA,
    LDX,
    LDY,
    SEI,
    STA,
    TXS,
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    EOR,
    INC,
    INY,
    ISB,
    JSR,
    LAX,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SLO,
    SRE,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TYA,
}

pub static OPECODE_MAP: Lazy<HashMap<u8, Opecode>> = Lazy::new(|| {
    let cycles: Vec<usize> =
        vec![7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6,
                2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8,
                4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
                2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2,
                4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3,
                2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, 2, 6, 2, 8,
                3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
                2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                4, 4, 7, 7];

    let mut m = HashMap::new();
    m.insert(0xA9, Opecode { command: Command::LDA, mode: AddressingMode::Immediate, cycle: cycles[0xA9] });
    m.insert(0xA5, Opecode { command: Command::LDA, mode: AddressingMode::ZeroPage, cycle: cycles[0xA5] });
    m.insert(0xB5, Opecode { command: Command::LDA, mode: AddressingMode::ZeroPageX, cycle: cycles[0xB5] });
    m.insert(0xAD, Opecode { command: Command::LDA, mode: AddressingMode::Absolute, cycle: cycles[0xAD] });
    m.insert(0xBD, Opecode { command: Command::LDA, mode: AddressingMode::AbsoluteX, cycle: cycles[0xBD] });
    m.insert(0xB9, Opecode { command: Command::LDA, mode: AddressingMode::AbsoluteY, cycle: cycles[0xB9] });
    m.insert(0xA1, Opecode { command: Command::LDA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xA1] });
    m.insert(0xB1, Opecode { command: Command::LDA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xB1] });
    m.insert(0xA2, Opecode { command: Command::LDX, mode: AddressingMode::Immediate, cycle: cycles[0xA2] });
    m.insert(0xA6, Opecode { command: Command::LDX, mode: AddressingMode::ZeroPage, cycle: cycles[0xA6] });
    m.insert(0xAE, Opecode { command: Command::LDX, mode: AddressingMode::Absolute, cycle: cycles[0xAE] });
    m.insert(0xB6, Opecode { command: Command::LDX, mode: AddressingMode::ZeroPageY, cycle: cycles[0xB6] });
    m.insert(0xBE, Opecode { command: Command::LDX, mode: AddressingMode::AbsoluteY, cycle: cycles[0xBE] });
    m.insert(0xA0, Opecode { command: Command::LDY, mode: AddressingMode::Immediate, cycle: cycles[0xA0] });
    m.insert(0xA4, Opecode { command: Command::LDY, mode: AddressingMode::ZeroPage, cycle: cycles[0xA4] });
    m.insert(0xAC, Opecode { command: Command::LDY, mode: AddressingMode::Absolute, cycle: cycles[0xAC] });
    m.insert(0xB4, Opecode { command: Command::LDY, mode: AddressingMode::ZeroPageX, cycle: cycles[0xB4] });
    m.insert(0xBC, Opecode { command: Command::LDY, mode: AddressingMode::AbsoluteX, cycle: cycles[0xBC] });
    m.insert(0x85, Opecode { command: Command::STA, mode: AddressingMode::ZeroPage, cycle: cycles[0x85] });
    m.insert(0x8D, Opecode { command: Command::STA, mode: AddressingMode::Absolute, cycle: cycles[0x8D] });
    m.insert(0x95, Opecode { command: Command::STA, mode: AddressingMode::ZeroPageX, cycle: cycles[0x95] });
    m.insert(0x9D, Opecode { command: Command::STA, mode: AddressingMode::AbsoluteX, cycle: cycles[0x9D] });
    m.insert(0x99, Opecode { command: Command::STA, mode: AddressingMode::AbsoluteY, cycle: cycles[0x99] });
    m.insert(0x81, Opecode { command: Command::STA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x81] });
    m.insert(0x91, Opecode { command: Command::STA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x91] });
    m.insert(0x86, Opecode { command: Command::STX, mode: AddressingMode::ZeroPage, cycle: cycles[0x86] });
    m.insert(0x8E, Opecode { command: Command::STX, mode: AddressingMode::Absolute, cycle: cycles[0x8E] });
    m.insert(0x96, Opecode { command: Command::STX, mode: AddressingMode::ZeroPageY, cycle: cycles[0x96] });
    m.insert(0x84, Opecode { command: Command::STY, mode: AddressingMode::ZeroPage, cycle: cycles[0x84] });
    m.insert(0x8C, Opecode { command: Command::STY, mode: AddressingMode::Absolute, cycle: cycles[0x8C] });
    m.insert(0x94, Opecode { command: Command::STY, mode: AddressingMode::ZeroPageX, cycle: cycles[0x94] });
    m.insert(0x8A, Opecode { command: Command::TXA, mode: AddressingMode::Implied, cycle: cycles[0x8A] });
    m.insert(0x98, Opecode { command: Command::TYA, mode: AddressingMode::Implied, cycle: cycles[0x98] });
    m.insert(0x9A, Opecode { command: Command::TXS, mode: AddressingMode::Implied, cycle: cycles[0x9A] });
    m.insert(0xA8, Opecode { command: Command::TAY, mode: AddressingMode::Implied, cycle: cycles[0xA8] });
    m.insert(0xAA, Opecode { command: Command::TAX, mode: AddressingMode::Implied, cycle: cycles[0xAA] });
    m.insert(0xBA, Opecode { command: Command::TSX, mode: AddressingMode::Implied, cycle: cycles[0xBA] });
    m.insert(0x08, Opecode { command: Command::PHP, mode: AddressingMode::Implied, cycle: cycles[0x08] });
    m.insert(0x28, Opecode { command: Command::PLP, mode: AddressingMode::Implied, cycle: cycles[0x28] });
    m.insert(0x48, Opecode { command: Command::PHA, mode: AddressingMode::Implied, cycle: cycles[0x48] });
    m.insert(0x68, Opecode { command: Command::PLA, mode: AddressingMode::Implied, cycle: cycles[0x68] });
    m.insert(0x69, Opecode { command: Command::ADC, mode: AddressingMode::Immediate, cycle: cycles[0x69] });
    m.insert(0x65, Opecode { command: Command::ADC, mode: AddressingMode::ZeroPage, cycle: cycles[0x65] });
    m.insert(0x6D, Opecode { command: Command::ADC, mode: AddressingMode::Absolute, cycle: cycles[0x6D] });
    m.insert(0x75, Opecode { command: Command::ADC, mode: AddressingMode::ZeroPageX, cycle: cycles[0x75] });
    m.insert(0x7D, Opecode { command: Command::ADC, mode: AddressingMode::AbsoluteX, cycle: cycles[0x7D] });
    m.insert(0x79, Opecode { command: Command::ADC, mode: AddressingMode::AbsoluteY, cycle: cycles[0x79] });
    m.insert(0x61, Opecode { command: Command::ADC, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x61] });
    m.insert(0x71, Opecode { command: Command::ADC, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x71] });
    m.insert(0xE9, Opecode { command: Command::SBC, mode: AddressingMode::Immediate, cycle: cycles[0xE9] });
    m.insert(0xE5, Opecode { command: Command::SBC, mode: AddressingMode::ZeroPage, cycle: cycles[0xE5] });
    m.insert(0xED, Opecode { command: Command::SBC, mode: AddressingMode::Absolute, cycle: cycles[0xED] });
    m.insert(0xF5, Opecode { command: Command::SBC, mode: AddressingMode::ZeroPageX, cycle: cycles[0xF5] });
    m.insert(0xFD, Opecode { command: Command::SBC, mode: AddressingMode::AbsoluteX, cycle: cycles[0xFD] });
    m.insert(0xF9, Opecode { command: Command::SBC, mode: AddressingMode::AbsoluteY, cycle: cycles[0xF9] });
    m.insert(0xE1, Opecode { command: Command::SBC, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xE1] });
    m.insert(0xF1, Opecode { command: Command::SBC, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xF1] });
    m.insert(0xE0, Opecode { command: Command::CPX, mode: AddressingMode::Immediate, cycle: cycles[0xE0] });
    m.insert(0xE4, Opecode { command: Command::CPX, mode: AddressingMode::ZeroPage, cycle: cycles[0xE4] });
    m.insert(0xEC, Opecode { command: Command::CPX, mode: AddressingMode::Absolute, cycle: cycles[0xEC] });
    m.insert(0xC0, Opecode { command: Command::CPY, mode: AddressingMode::Immediate, cycle: cycles[0xC0] });
    m.insert(0xC4, Opecode { command: Command::CPY, mode: AddressingMode::ZeroPage, cycle: cycles[0xC4] });
    m.insert(0xCC, Opecode { command: Command::CPY, mode: AddressingMode::Absolute, cycle: cycles[0xCC] });
    m.insert(0xC9, Opecode { command: Command::CMP, mode: AddressingMode::Immediate, cycle: cycles[0xC9] });
    m.insert(0xC5, Opecode { command: Command::CMP, mode: AddressingMode::ZeroPage, cycle: cycles[0xC5] });
    m.insert(0xCD, Opecode { command: Command::CMP, mode: AddressingMode::Absolute, cycle: cycles[0xCD] });
    m.insert(0xD5, Opecode { command: Command::CMP, mode: AddressingMode::ZeroPageX, cycle: cycles[0xD5] });
    m.insert(0xDD, Opecode { command: Command::CMP, mode: AddressingMode::AbsoluteX, cycle: cycles[0xDD] });
    m.insert(0xD9, Opecode { command: Command::CMP, mode: AddressingMode::AbsoluteY, cycle: cycles[0xD9] });
    m.insert(0xC1, Opecode { command: Command::CMP, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xC1] });
    m.insert(0xD1, Opecode { command: Command::CMP, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xD1] });
    m.insert(0x29, Opecode { command: Command::AND, mode: AddressingMode::Immediate, cycle: cycles[0x29] });
    m.insert(0x25, Opecode { command: Command::AND, mode: AddressingMode::ZeroPage, cycle: cycles[0x25] });
    m.insert(0x2D, Opecode { command: Command::AND, mode: AddressingMode::Absolute, cycle: cycles[0x2D] });
    m.insert(0x35, Opecode { command: Command::AND, mode: AddressingMode::ZeroPageX, cycle: cycles[0x35] });
    m.insert(0x3D, Opecode { command: Command::AND, mode: AddressingMode::AbsoluteX, cycle: cycles[0x3D] });
    m.insert(0x39, Opecode { command: Command::AND, mode: AddressingMode::AbsoluteY, cycle: cycles[0x39] });
    m.insert(0x21, Opecode { command: Command::AND, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x21] });
    m.insert(0x31, Opecode { command: Command::AND, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x31] });
    m.insert(0x49, Opecode { command: Command::EOR, mode: AddressingMode::Immediate, cycle: cycles[0x49] });
    m.insert(0x45, Opecode { command: Command::EOR, mode: AddressingMode::ZeroPage, cycle: cycles[0x45] });
    m.insert(0x4D, Opecode { command: Command::EOR, mode: AddressingMode::Absolute, cycle: cycles[0x4D] });
    m.insert(0x55, Opecode { command: Command::EOR, mode: AddressingMode::ZeroPageX, cycle: cycles[0x55] });
    m.insert(0x5D, Opecode { command: Command::EOR, mode: AddressingMode::AbsoluteX, cycle: cycles[0x5D] });
    m.insert(0x59, Opecode { command: Command::EOR, mode: AddressingMode::AbsoluteY, cycle: cycles[0x59] });
    m.insert(0x41, Opecode { command: Command::EOR, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x41] });
    m.insert(0x51, Opecode { command: Command::EOR, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x51] });
    m.insert(0x09, Opecode { command: Command::ORA, mode: AddressingMode::Immediate, cycle: cycles[0x09] });
    m.insert(0x05, Opecode { command: Command::ORA, mode: AddressingMode::ZeroPage, cycle: cycles[0x05] });
    m.insert(0x0D, Opecode { command: Command::ORA, mode: AddressingMode::Absolute, cycle: cycles[0x0D] });
    m.insert(0x15, Opecode { command: Command::ORA, mode: AddressingMode::ZeroPageX, cycle: cycles[0x15] });
    m.insert(0x1D, Opecode { command: Command::ORA, mode: AddressingMode::AbsoluteX, cycle: cycles[0x1D] });
    m.insert(0x19, Opecode { command: Command::ORA, mode: AddressingMode::AbsoluteY, cycle: cycles[0x19] });
    m.insert(0x01, Opecode { command: Command::ORA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x01] });
    m.insert(0x11, Opecode { command: Command::ORA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x11] });
    m.insert(0x24, Opecode { command: Command::BIT, mode: AddressingMode::ZeroPage, cycle: cycles[0x24] });
    m.insert(0x2C, Opecode { command: Command::BIT, mode: AddressingMode::Absolute, cycle: cycles[0x2C] });
    m.insert(0x0A, Opecode { command: Command::ASL, mode: AddressingMode::Accumulator, cycle: cycles[0x0A] });
    m.insert(0x06, Opecode { command: Command::ASL, mode: AddressingMode::ZeroPage, cycle: cycles[0x06] });
    m.insert(0x0E, Opecode { command: Command::ASL, mode: AddressingMode::Absolute, cycle: cycles[0x0E] });
    m.insert(0x16, Opecode { command: Command::ASL, mode: AddressingMode::ZeroPageX, cycle: cycles[0x16] });
    m.insert(0x1E, Opecode { command: Command::ASL, mode: AddressingMode::AbsoluteX, cycle: cycles[0x1E] });
    m.insert(0x4A, Opecode { command: Command::LSR, mode: AddressingMode::Accumulator, cycle: cycles[0x4A] });
    m.insert(0x46, Opecode { command: Command::LSR, mode: AddressingMode::ZeroPage, cycle: cycles[0x46] });
    m.insert(0x4E, Opecode { command: Command::LSR, mode: AddressingMode::Absolute, cycle: cycles[0x4E] });
    m.insert(0x56, Opecode { command: Command::LSR, mode: AddressingMode::ZeroPageX, cycle: cycles[0x56] });
    m.insert(0x5E, Opecode { command: Command::LSR, mode: AddressingMode::AbsoluteX, cycle: cycles[0x5E] });
    m.insert(0x2A, Opecode { command: Command::ROL, mode: AddressingMode::Accumulator, cycle: cycles[0x2A] });
    m.insert(0x26, Opecode { command: Command::ROL, mode: AddressingMode::ZeroPage, cycle: cycles[0x26] });
    m.insert(0x2E, Opecode { command: Command::ROL, mode: AddressingMode::Absolute, cycle: cycles[0x2E] });
    m.insert(0x36, Opecode { command: Command::ROL, mode: AddressingMode::ZeroPageX, cycle: cycles[0x36] });
    m.insert(0x3E, Opecode { command: Command::ROL, mode: AddressingMode::AbsoluteX, cycle: cycles[0x3E] });
    m.insert(0x6A, Opecode { command: Command::ROR, mode: AddressingMode::Accumulator, cycle: cycles[0x6A] });
    m.insert(0x66, Opecode { command: Command::ROR, mode: AddressingMode::ZeroPage, cycle: cycles[0x66] });
    m.insert(0x6E, Opecode { command: Command::ROR, mode: AddressingMode::Absolute, cycle: cycles[0x6E] });
    m.insert(0x76, Opecode { command: Command::ROR, mode: AddressingMode::ZeroPageX, cycle: cycles[0x76] });
    m.insert(0x7E, Opecode { command: Command::ROR, mode: AddressingMode::AbsoluteX, cycle: cycles[0x7E] });
    m.insert(0xE8, Opecode { command: Command::INX, mode: AddressingMode::Implied, cycle: cycles[0xE8] });
    m.insert(0xC8, Opecode { command: Command::INY, mode: AddressingMode::Implied, cycle: cycles[0xC8] });
    m.insert(0xE6, Opecode { command: Command::INC, mode: AddressingMode::ZeroPage, cycle: cycles[0xE6] });
    m.insert(0xEE, Opecode { command: Command::INC, mode: AddressingMode::Absolute, cycle: cycles[0xEE] });
    m.insert(0xF6, Opecode { command: Command::INC, mode: AddressingMode::ZeroPageX, cycle: cycles[0xF6] });
    m.insert(0xFE, Opecode { command: Command::INC, mode: AddressingMode::AbsoluteX, cycle: cycles[0xFE] });
    m.insert(0xCA, Opecode { command: Command::DEX, mode: AddressingMode::Implied, cycle: cycles[0xCA] });
    m.insert(0x88, Opecode { command: Command::DEY, mode: AddressingMode::Implied, cycle: cycles[0x88] });
    m.insert(0xC6, Opecode { command: Command::DEC, mode: AddressingMode::ZeroPage, cycle: cycles[0xC6] });
    m.insert(0xCE, Opecode { command: Command::DEC, mode: AddressingMode::Absolute, cycle: cycles[0xCE] });
    m.insert(0xD6, Opecode { command: Command::DEC, mode: AddressingMode::ZeroPageX, cycle: cycles[0xD6] });
    m.insert(0xDE, Opecode { command: Command::DEC, mode: AddressingMode::AbsoluteX, cycle: cycles[0xDE] });
    m.insert(0x18, Opecode { command: Command::CLC, mode: AddressingMode::Implied, cycle: cycles[0x18] });
    m.insert(0x58, Opecode { command: Command::CLI, mode: AddressingMode::Implied, cycle: cycles[0x58] });
    m.insert(0xB8, Opecode { command: Command::CLV, mode: AddressingMode::Implied, cycle: cycles[0xB8] });
    m.insert(0x38, Opecode { command: Command::SEC, mode: AddressingMode::Implied, cycle: cycles[0x38] });
    m.insert(0x78, Opecode { command: Command::SEI, mode: AddressingMode::Implied, cycle: cycles[0x78] });
    m.insert(0xEA, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xEA] });
    m.insert(0x00, Opecode { command: Command::BRK, mode: AddressingMode::Implied, cycle: cycles[0x00] });
    m.insert(0x20, Opecode { command: Command::JSR, mode: AddressingMode::Absolute, cycle: cycles[0x20] });
    m.insert(0x4C, Opecode { command: Command::JMP, mode: AddressingMode::Absolute, cycle: cycles[0x4C] });
    m.insert(0x6C, Opecode { command: Command::JMP, mode: AddressingMode::IndirectAbsolute, cycle: cycles[0x6C] });
    m.insert(0x40, Opecode { command: Command::RTI, mode: AddressingMode::Implied, cycle: cycles[0x40] });
    m.insert(0x60, Opecode { command: Command::RTS, mode: AddressingMode::Implied, cycle: cycles[0x60] });
    m.insert(0x10, Opecode { command: Command::BPL, mode: AddressingMode::Relative, cycle: cycles[0x10] });
    m.insert(0x30, Opecode { command: Command::BMI, mode: AddressingMode::Relative, cycle: cycles[0x30] });
    m.insert(0x50, Opecode { command: Command::BVC, mode: AddressingMode::Relative, cycle: cycles[0x50] });
    m.insert(0x70, Opecode { command: Command::BVS, mode: AddressingMode::Relative, cycle: cycles[0x70] });
    m.insert(0x90, Opecode { command: Command::BCC, mode: AddressingMode::Relative, cycle: cycles[0x90] });
    m.insert(0xB0, Opecode { command: Command::BCS, mode: AddressingMode::Relative, cycle: cycles[0xB0] });
    m.insert(0xD0, Opecode { command: Command::BNE, mode: AddressingMode::Relative, cycle: cycles[0xD0] });
    m.insert(0xF0, Opecode { command: Command::BEQ, mode: AddressingMode::Relative, cycle: cycles[0xF0] });
    m.insert(0xF8, Opecode { command: Command::SED, mode: AddressingMode::Implied, cycle: cycles[0xF8] });
    m.insert(0xD8, Opecode { command: Command::CLD, mode: AddressingMode::Implied, cycle: cycles[0xD8] });
    m.insert(0x1A, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x1A] });
    m.insert(0x3A, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x3A] });
    m.insert(0x5A, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x5A] });
    m.insert(0x7A, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x7A] });
    m.insert(0xDA, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xDA] });
    m.insert(0xFA, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xFA] });
    m.insert(0x02, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x02] });
    m.insert(0x12, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x12] });
    m.insert(0x22, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x22] });
    m.insert(0x32, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x32] });
    m.insert(0x42, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x42] });
    m.insert(0x52, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x52] });
    m.insert(0x62, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x62] });
    m.insert(0x72, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x72] });
    m.insert(0x92, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x92] });
    m.insert(0xB2, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xB2] });
    m.insert(0xD2, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xD2] });
    m.insert(0xF2, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xF2] });
    m.insert(0x80, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x80] });
    m.insert(0x82, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x82] });
    m.insert(0x89, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x89] });
    m.insert(0xC2, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xC2] });
    m.insert(0xE2, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xE2] });
    m.insert(0x04, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x04] });
    m.insert(0x44, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x44] });
    m.insert(0x64, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x64] });
    m.insert(0x14, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x14] });
    m.insert(0x34, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x34] });
    m.insert(0x54, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x54] });
    m.insert(0x74, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x74] });
    m.insert(0xD4, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xD4] });
    m.insert(0xF4, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xF4] });
    m.insert(0x0C, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x0C] });
    m.insert(0x1C, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x1C] });
    m.insert(0x3C, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x3C] });
    m.insert(0x5C, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x5C] });
    m.insert(0x7C, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0x7C] });
    m.insert(0xDC, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xDC] });
    m.insert(0xFC, Opecode { command: Command::NOP, mode: AddressingMode::Implied, cycle: cycles[0xFC] });
    m.insert(0xA7, Opecode { command: Command::LAX, mode: AddressingMode::ZeroPage, cycle: cycles[0xA7] });
    m.insert(0xB7, Opecode { command: Command::LAX, mode: AddressingMode::ZeroPageY, cycle: cycles[0xB7] });
    m.insert(0xAF, Opecode { command: Command::LAX, mode: AddressingMode::Absolute, cycle: cycles[0xAF] });
    m.insert(0xBF, Opecode { command: Command::LAX, mode: AddressingMode::AbsoluteY, cycle: cycles[0xBF] });
    m.insert(0xA3, Opecode { command: Command::LAX, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xA3] });
    m.insert(0xB3, Opecode { command: Command::LAX, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xB3] });
    m.insert(0x87, Opecode { command: Command::SAX, mode: AddressingMode::ZeroPage, cycle: cycles[0x87] });
    m.insert(0x97, Opecode { command: Command::SAX, mode: AddressingMode::ZeroPageY, cycle: cycles[0x97] });
    m.insert(0x8F, Opecode { command: Command::SAX, mode: AddressingMode::Absolute, cycle: cycles[0x8F] });
    m.insert(0x83, Opecode { command: Command::SAX, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x83] });
    m.insert(0xEB, Opecode { command: Command::SBC, mode: AddressingMode::Immediate, cycle: cycles[0xEB] });
    m.insert(0xC7, Opecode { command: Command::DCP, mode: AddressingMode::ZeroPage, cycle: cycles[0xC7] });
    m.insert(0xD7, Opecode { command: Command::DCP, mode: AddressingMode::ZeroPageX, cycle: cycles[0xD7] });
    m.insert(0xCF, Opecode { command: Command::DCP, mode: AddressingMode::Absolute, cycle: cycles[0xCF] });
    m.insert(0xDF, Opecode { command: Command::DCP, mode: AddressingMode::AbsoluteX, cycle: cycles[0xDF] });
    m.insert(0xDB, Opecode { command: Command::DCP, mode: AddressingMode::AbsoluteY, cycle: cycles[0xD8] });
    m.insert(0xC3, Opecode { command: Command::DCP, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xC3] });
    m.insert(0xD3, Opecode { command: Command::DCP, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xD3] });
    m.insert(0xE7, Opecode { command: Command::ISB, mode: AddressingMode::ZeroPage, cycle: cycles[0xE7] });
    m.insert(0xF7, Opecode { command: Command::ISB, mode: AddressingMode::ZeroPageX, cycle: cycles[0xF7] });
    m.insert(0xEF, Opecode { command: Command::ISB, mode: AddressingMode::Absolute, cycle: cycles[0xEF] });
    m.insert(0xFF, Opecode { command: Command::ISB, mode: AddressingMode::AbsoluteX, cycle: cycles[0xFF] });
    m.insert(0xFB, Opecode { command: Command::ISB, mode: AddressingMode::AbsoluteY, cycle: cycles[0xF8] });
    m.insert(0xE3, Opecode { command: Command::ISB, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xE3] });
    m.insert(0xF3, Opecode { command: Command::ISB, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xF3] });
    m.insert(0x07, Opecode { command: Command::SLO, mode: AddressingMode::ZeroPage, cycle: cycles[0x07] });
    m.insert(0x17, Opecode { command: Command::SLO, mode: AddressingMode::ZeroPageX, cycle: cycles[0x17] });
    m.insert(0x0F, Opecode { command: Command::SLO, mode: AddressingMode::Absolute, cycle: cycles[0x0F] });
    m.insert(0x1F, Opecode { command: Command::SLO, mode: AddressingMode::AbsoluteX, cycle: cycles[0x1F] });
    m.insert(0x1B, Opecode { command: Command::SLO, mode: AddressingMode::AbsoluteY, cycle: cycles[0x1B] });
    m.insert(0x03, Opecode { command: Command::SLO, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x03] });
    m.insert(0x13, Opecode { command: Command::SLO, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x13] });
    m.insert(0x27, Opecode { command: Command::RLA, mode: AddressingMode::ZeroPage, cycle: cycles[0x27] });
    m.insert(0x37, Opecode { command: Command::RLA, mode: AddressingMode::ZeroPageX, cycle: cycles[0x37] });
    m.insert(0x2F, Opecode { command: Command::RLA, mode: AddressingMode::Absolute, cycle: cycles[0x2F] });
    m.insert(0x3F, Opecode { command: Command::RLA, mode: AddressingMode::AbsoluteX, cycle: cycles[0x3F] });
    m.insert(0x3B, Opecode { command: Command::RLA, mode: AddressingMode::AbsoluteY, cycle: cycles[0x3B] });
    m.insert(0x23, Opecode { command: Command::RLA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x23] });
    m.insert(0x33, Opecode { command: Command::RLA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x33] }, );
    m.insert(0x47, Opecode { command: Command::SRE, mode: AddressingMode::ZeroPage, cycle: cycles[0x47] });
    m.insert(0x57, Opecode { command: Command::SRE, mode: AddressingMode::ZeroPageX, cycle: cycles[0x57] });
    m.insert(0x4F, Opecode { command: Command::SRE, mode: AddressingMode::Absolute, cycle: cycles[0x4F] });
    m.insert(0x5F, Opecode { command: Command::SRE, mode: AddressingMode::AbsoluteX, cycle: cycles[0x5F] });
    m.insert(0x5B, Opecode { command: Command::SRE, mode: AddressingMode::AbsoluteY, cycle: cycles[0x5B] });
    m.insert(0x43, Opecode { command: Command::SRE, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x43] });
    m.insert(0x53, Opecode { command: Command::SRE, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x53] }, );
    m.insert(0x67, Opecode { command: Command::RRA, mode: AddressingMode::ZeroPage, cycle: cycles[0x67] });
    m.insert(0x77, Opecode { command: Command::RRA, mode: AddressingMode::ZeroPageX, cycle: cycles[0x77] });
    m.insert(0x6F, Opecode { command: Command::RRA, mode: AddressingMode::Absolute, cycle: cycles[0x6F] });
    m.insert(0x7F, Opecode { command: Command::RRA, mode: AddressingMode::AbsoluteX, cycle: cycles[0x7F] });
    m.insert(0x7B, Opecode { command: Command::RRA, mode: AddressingMode::AbsoluteY, cycle: cycles[0x7B] });
    m.insert(0x63, Opecode { command: Command::RRA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x63] });
    m.insert(0x73, Opecode { command: Command::RRA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x73] });
    m
});