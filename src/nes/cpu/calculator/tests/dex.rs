use super::*;

#[test]
fn DEX_test() {
    let mut registers = Registers::new();
    let opeland = 0x20;
    registers.X = opeland + 1;

    Calculator::DEX(&mut registers);
    assert_eq!(registers.X, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn DEX_test_overflow() {
    let mut registers = Registers::new();
    registers.X = 0x00;

    Calculator::DEX(&mut registers);
    assert_eq!(registers.X, 0xFF); // 0 - decriment-> 255
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn DEX_update_zero_test() {
    let mut registers = Registers::new();
    let opeland = 0x00; // Zero operand
    registers.X = opeland + 1;

    Calculator::DEX(&mut registers);
    assert_eq!(registers.X, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, true);
}

#[test]
fn DEX_update_negative_test() {
    let mut registers = Registers::new();
    let opeland = 0x90; // Nagative opeland(over than 0x80)
    registers.X = opeland + 1;

    Calculator::DEX(&mut registers);
    assert_eq!(registers.X, opeland);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}