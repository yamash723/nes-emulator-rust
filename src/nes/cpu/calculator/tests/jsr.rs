use super::*;

#[test]
fn JSR_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();
    let opeland = 0x10;

    registers.PC = 0x02;
    registers.S = 0x05;

    Calculator::JSR(&mut registers, &mut bus, opeland);

    assert_eq!(registers.PC, opeland);
    assert_eq!(registers.S, 0x03);
}