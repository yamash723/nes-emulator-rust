use super::*;

#[test]
fn TYA_test() {
    let mut registers = Registers::new();
    let opeland = 0x10;
    registers.Y = opeland;

    Calculator::TYA(&mut registers);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn TYA_update_zero_test() {
    let mut registers = Registers::new();
    let opeland = 0x00; // Zero operand
    registers.Y = opeland;

    Calculator::TYA(&mut registers);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, true);
}

#[test]
fn TYA_update_negative_test() {
    let mut registers = Registers::new();
    let opeland = 0x90; // Nagative opeland(over than 0x80)
    registers.Y = opeland;

    Calculator::TYA(&mut registers);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}