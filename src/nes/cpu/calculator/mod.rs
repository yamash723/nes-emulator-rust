use crate::nes::cpu::bus::CpuBus;
use crate::nes::cpu::registers::Registers;
use crate::nes::cpu::controller::Controller;
use crate::nes::cpu::opecode::{Command, OPECODE_MAP, AddressingMode};

pub struct Calculator;

impl Calculator {
    pub fn execute<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> usize {
        let run_opecode = Controller::fetch(registers, bus);
        let opecode_rule = OPECODE_MAP.get(&run_opecode).unwrap();

        let (command, mode, cycle) = (&opecode_rule.command, &opecode_rule.mode, opecode_rule.cycle);
        let opeland = Controller::fetch_opeland(registers, bus, &mode);

        match *command {
            Command::LDA if *mode == AddressingMode::Immediate => Calculator::LDA_immediate(registers, opeland),
            Command::LDA => Calculator::LDA(registers, bus, opeland),
            Command::LDX if *mode == AddressingMode::Immediate => Calculator::LDX_immediate(registers, opeland),
            Command::LDX => Calculator::LDX(registers, bus, opeland),
            Command::LDY if *mode == AddressingMode::Immediate => Calculator::LDY_immediate(registers, opeland),
            Command::LDY => Calculator::LDY(registers, bus, opeland),
            Command::STA => Calculator::STA(registers, bus, opeland),
            Command::STX => Calculator::STX(registers, bus, opeland),
            Command::BNE => Calculator::BNE(registers, opeland),
            Command::DEY => Calculator::DEY(registers),
            Command::INX => Calculator::INX(registers),
            Command::JMP => Calculator::JMP(registers, opeland),
            Command::JSR => Calculator::JSR(registers, bus, opeland),
            Command::SEI => Calculator::SEI(registers),
            Command::TXS => Calculator::TXS(registers),
            Command::TYA => Calculator::TYA(registers),
            Command::CLD => Calculator::CLD(registers),
            Command::BPL => Calculator::BPL(registers, opeland),
            _ => panic!("not unimplement command: {:?}", &command),
        };

        cycle
    }

    fn LDA<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        Calculator::LDA_immediate(registers, bus.read(opeland) as u16);
    }

    fn LDA_immediate(registers: &mut Registers, opeland: u16) {
        let data = opeland as u8;
        registers.A = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn LDX<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        Calculator::LDX_immediate(registers, bus.read(opeland) as u16);
    }

    fn LDX_immediate(registers: &mut Registers, opeland: u16) {
        let data = opeland as u8;
        registers.X = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn LDY<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        Calculator::LDY_immediate(registers, bus.read(opeland) as u16);
    }

    fn LDY_immediate(registers: &mut Registers, opeland: u16) {
        let data = opeland as u8;
        registers.Y = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn STA<T: CpuBus>(registers: &Registers, bus: &mut T, opeland: u16) {
        bus.write(opeland, registers.A);
    }

    fn STX<T: CpuBus>(registers: &Registers, bus: &mut T, opeland: u16) {
        bus.write(opeland, registers.X);
    }

    fn TXS(registers: &mut Registers) {
        registers.S = registers.X;
    }

    fn BNE(registers: &mut Registers, opeland: u16) {
        if !registers.P.zero {
            registers.PC = opeland;
        }
    }

    fn DEY(registers: &mut Registers) {
        let data = registers.Y - 1;

        registers.Y = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn INX(registers: &mut Registers) {
        let data = registers.X + 1;

        registers.X = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn JMP(registers: &mut Registers, opeland: u16) {
        registers.PC = opeland;
    }

    fn JSR<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        let pc = registers.PC - 1;
        Calculator::push((pc >> 8) as u8, registers, bus);
        Calculator::push(pc as u8, registers, bus);
        registers.PC = opeland;
    }

    fn SEI(registers: &mut Registers) {
        registers.P.interrupt = true;
    }

    fn CLD(registers: &mut Registers) {
        registers.P.decimal = false;
    }

    fn BPL(registers: &mut Registers, opeland: u16) {
        if !registers.P.negative {
            registers.PC = opeland;
        }
    }

    fn TYA(registers: &mut Registers) {
        let data = registers.Y;

        registers.A = data;
        registers.P.negative = (data & 0x80) == 0x80;
        registers.P.zero = data == 0;
    }

    fn push<T: CpuBus>(data: u8, registers: &mut Registers, bus: &mut T) {
        let addr = registers.S as u16;
        bus.write(addr | 0x0100, data);
        registers.S -= 1;
    }
}

#[cfg(test)]
mod tests;
