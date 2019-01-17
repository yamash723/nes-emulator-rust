use super::*;

#[test]
fn CLD_test() {
    let mut registers = Registers::new();
    registers.P.decimal = true;

    Calculator::CLD(&mut registers);
    assert_eq!(registers.P.decimal, false);
}