use super::*;

#[test]
fn BPL_is_true_negative_test() {
    let mut registers = Registers::new();
    registers.P.negative = true;
    registers.PC = 0x00;

    Calculator::BPL(&mut registers, 0xCB);
    assert_eq!(registers.PC, 0x00);
}

#[test]
fn BPL_is_false_negative_test() {
    let mut registers = Registers::new();
    registers.P.negative = false;

    Calculator::BPL(&mut registers, 0xCB);
    assert_eq!(registers.PC, 0xCB);
}