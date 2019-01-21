use super::*;

#[test]
fn STX_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();
    let opeland = 0x90;

    registers.X = 0x89;
    Calculator::STX(&registers, &mut bus, opeland);

    let actual = bus.read(opeland as u16);
    assert_eq!(actual, registers.X);
}