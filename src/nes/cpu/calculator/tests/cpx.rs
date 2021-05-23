use super::*;

#[test]
fn CPX_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();

    struct PatternArgs {
        addr: u16,
        opeland: u8,
        x: u8,
        negative: bool,
        zero: bool,
        carry: bool,
    }

    let patterns = vec![
        PatternArgs { addr: 0x0010, x: 0x20, opeland: 0x30, negative: true,  zero: false, carry: false },
        PatternArgs { addr: 0x0010, x: 0x20, opeland: 0x20, negative: false, zero: true,  carry: true },
        PatternArgs { addr: 0x0010, x: 0x20, opeland: 0x10, negative: false, zero: false, carry: true  },
    ];

    for args in patterns.iter() {
        let addr = args.addr;
        let opeland = args.opeland;
        bus.write(addr, opeland);
    
        registers.X = args.x;
        Calculator::CPX(&mut registers, &mut bus, addr);
    
        assert_eq!(registers.P.negative, args.negative);
        assert_eq!(registers.P.zero, args.zero);
        assert_eq!(registers.P.carry, args.carry);
    }
}