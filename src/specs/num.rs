use crate::helpers::*;
use paste::paste;

macro_rules! mk_unsigned_tests {
    ($typ:ident, $ident:ident) => {
        mk_unsigned_tests!($typ, $ident, $typ::MAX);
    };
    ($typ:ident, $ident:ident, $max:expr) => {
        paste! {
            tests! {
                header: concat!("Properties for [`core::ops::Add::<", stringify!($typ), ">::add`]"),
                ident: [<core_ops_add_ $typ _add>],
                contract! {
                    header: "Semantics of non-overflowing addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $max.up(),
                    postcondition: x + y == eval($typ::down(x.up() + y.up())),
                },
                contract! {
                    header: "Panics when overflowing",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() > $max.up(),
                    postcondition: panics!(x + y),
                },
                contract! {
                    header: "Commutativity",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $max.up(),
                    postcondition: x + y == y + x,
                },
                contract! {
                    header: "Left identity",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: x + 0 == x,
                },
                contract! {
                    header: "Right identity",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: 0 + x == x,
                },
                contract! {
                    header: "Associativity",
                    inputs: [x: $typ, y: $typ, z: $typ],
                    precondition: x.up() + y.up() + z.up() <= $max.up(),
                    postcondition: (x + y) + z == x + (y + z),
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_add`]"),
                ident: [<core_ops_add_ $typ _checked_add>],
                contract! {
                    header: "Semantics of non-overflowing checked addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $max.up(),
                    postcondition: x.checked_add(y) == Some(eval($typ::down(x.up() + y.up()))),
                },
                contract! {
                    header: "None when overflowing",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() > $max.up(),
                    postcondition: x.checked_add(y) == None,
                },
                contract! {
                    header: "Commutativity",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.checked_add(y) == y.checked_add(x),
                },
                contract! {
                    header: "Left identity",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: x.checked_add([<0 $typ>]) == Some(x),
                },
                contract! {
                    header: "Right identity",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: [<0 $typ>].checked_add(x) == Some(x),
                },
                contract! {
                    header: "Associativity",
                    inputs: [x: $typ, y: $typ, z: $typ],
                    precondition: true,
                    postcondition: x.checked_add(y).and_then(|iv| iv.checked_add(z)) == y.checked_add(z).and_then(|iv| x.checked_add(iv)),
                },
            }
        }
    };
}

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
    mk_unsigned_tests!(u8, core_ops_add_u8_add);
    mk_unsigned_tests!(u16, core_ops_add_u16_add);
    mk_unsigned_tests!(u32, core_ops_add_u32_add);
    mk_unsigned_tests!(u64, core_ops_add_u64_add);
    mk_unsigned_tests!(u128, core_ops_add_u128_add);
}
