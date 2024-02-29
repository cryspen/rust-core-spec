use crate::helpers::*;

pub fn specs() {
    tests! {
        header: "Properties for [`core::ops::Sub::<i8>::sub`]",
        ident: core_ops_sub_i8_sub,
        contract! {
            header: "Semantics of non-overflowing subtraction",
            inputs: [x: i8, y: i8],
            precondition: x.up() - y.up() < 128.up() && x.up() - y.up() >= -128i32.up(),
            postcondition: x - y == eval(i8::down(x.up() - y.up())),
        },
        contract! {
            header: "Overflowing subtraction panics",
            inputs: [x: i8, y: i8],
            precondition: x.up() - y.up() >= 128.up() || x.up() - y.up() < -128i32.up(),
            postcondition: panics!(x - y),
        },
    }
    tests! {
        header: "Properties for [`core::ops::Add::<i8>::add`]",
        ident: core_ops_add_i8_add,
        contract! {
            header: "Semantics of non-overflowing addition",
            inputs: [x: i8, y: i8],
            precondition: x.up() + y.up() < 128.up() && x.up() + y.up() >= -128i32.up(),
            postcondition: x + y == eval(i8::down(x.up() + y.up())),
        },
        contract! {
            header: "Overflowing addition panics",
            inputs: [x: i8, y: i8],
            precondition: x.up() + y.up() >= 128.up() || x.up() + y.up() < -128i32.up(),
            postcondition: panics!(x + y),
        },
    }
    tests! {
        header: "Properties for [`core::ops::Add::<u8>::add`]",
        ident: core_ops_add_u8_add,
        contract! {
            header: "Semantics of non-overflowing addition",
            inputs: [x: u8, y: u8],
            precondition: x.up() + y.up() < 256.up(),
            postcondition: x + y == eval(u8::down(x.up() + y.up())),
        },
        contract! {
            header: "Panics when overflowing",
            inputs: [x: u8, y: u8],
            precondition: x.up() + y.up() >= 256.up(),
            postcondition: panics!(x + y),
        },
        contract! {
            header: "Commutativity",
            inputs: [x: u8, y: u8],
            precondition: x.up() + y.up() < 256.up(),
            postcondition: x + y == y + x,
        },
        contract! {
            header: "Left identity",
            inputs: [x: u8],
            precondition: true,
            postcondition: x + 0 == x,
        },
        contract! {
            header: "Right identity",
            inputs: [x: u8],
            precondition: true,
            postcondition: 0 + x == x,
        },
        contract! {
            header: "Associativity",
            inputs: [x: u8, y: u8, z: u8],
            precondition: x.up() + y.up() + z.up() < 256.up(),
            postcondition: (x + y) + z == x + (y + z),
        },
    }
}
