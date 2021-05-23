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
            Command::DEX => Calculator::DEX(registers),
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
        registers.A = opeland as u8;
        registers.update_negative(registers.A);
        registers.update_zero(registers.A);
    }

    fn LDX<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        Calculator::LDX_immediate(registers, bus.read(opeland) as u16);
    }

    fn LDX_immediate(registers: &mut Registers, opeland: u16) {
        registers.X = opeland as u8;
        registers.update_negative(registers.X);
        registers.update_zero(registers.X);
    }

    fn LDY<T: CpuBus>(registers: &mut Registers, bus: &mut T, opeland: u16) {
        Calculator::LDY_immediate(registers, bus.read(opeland) as u16);
    }

    fn LDY_immediate(registers: &mut Registers, opeland: u16) {
        registers.Y = opeland as u8;
        registers.update_negative(registers.Y);
        registers.update_zero(registers.Y);
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

    fn DEX(registers: &mut Registers) {
        registers.X = registers.X.wrapping_sub(1);
        registers.update_negative(registers.X);
        registers.update_zero(registers.X);
    }

    fn DEY(registers: &mut Registers) {
        registers.Y = registers.Y.wrapping_sub(1);
        registers.update_negative(registers.Y);
        registers.update_zero(registers.Y);
    }

    fn INX(registers: &mut Registers) {
        registers.X = registers.X.wrapping_add(1);
        registers.update_negative(registers.X);
        registers.update_zero(registers.X);
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
        registers.A = registers.Y;
        registers.update_negative(registers.A);
        registers.update_zero(registers.A);
    }

    fn push<T: CpuBus>(data: u8, registers: &mut Registers, bus: &mut T) {
        let addr = registers.S as u16;
        bus.write(addr | 0x0100, data);
        registers.S -= 1;
    }
}

#[cfg(test)]
mod tests;
