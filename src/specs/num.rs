use crate::helpers::*;
use paste::paste;

macro_rules! mk_common_tests {
    ($typ:ident) => {
        paste! {
            tests! {
                header: concat!("Properties for [`core::cmp::PartialOrd::<", stringify!($typ), ">::cmp`]"),
                ident: [<core_cmp_partial_ord_ $typ _cmp>],
                contract! {
                    header: "Semantics of comparaison",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.cmp(&(y)) == eval(x.up().cmp(&(y).up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::cmp::PartialOrd::<", stringify!($typ), ">::lt`]"),
                ident: [<core_cmp_partial_ord_ $typ _lt>],
                contract! {
                    header: "Semantics of comparaison",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.lt(&(y)) == eval(x.up().lt(&(y).up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::cmp::PartialOrd::<", stringify!($typ), ">::gt`]"),
                ident: [<core_cmp_partial_ord_ $typ _gt>],
                contract! {
                    header: "Semantics of comparaison",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.gt(&(y)) == eval(x.up().gt(&(y).up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::cmp::PartialOrd::<", stringify!($typ), ">::ge`]"),
                ident: [<core_cmp_partial_ord_ $typ _ge>],
                contract! {
                    header: "Semantics of comparaison",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.ge(&(y)) == eval(x.up().ge(&(y).up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::cmp::PartialOrd::<", stringify!($typ), ">::le`]"),
                ident: [<core_cmp_partial_ord_ $typ _le>],
                contract! {
                    header: "Semantics of comparaison",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.le(&(y)) == eval(x.up().le(&(y).up())),
                },
            }



            tests! {
                header: concat!("Properties for [`core::ops::BitXor::<", stringify!($typ), ">::bitxor`]"),
                ident: [<core_ops_bit_xor_ $typ _xor>],
                contract! {
                    header: "Semantics of bitwise or",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x ^ y == eval($typ::down(x.up() ^ y.up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::BitAnd::<", stringify!($typ), ">::bitand`]"),
                ident: [<core_ops_bit_and_ $typ _and>],
                contract! {
                    header: "Semantics of bitwise and",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x & y == eval($typ::down(x.up() & y.up())),
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::BitAnd::<", stringify!($typ), ">::bitor`]"),
                ident: [<core_ops_bit_and_ $typ _or>],
                contract! {
                    header: "Semantics of bitwise or",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x ^ y == eval($typ::down(x.up() | y.up())),
                },
            }

        }
    }
}

macro_rules! mk_signed_tests {
    ($typ:ident) => {
        paste! {
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_neg`]"),
                ident: [<t_ $typ _checked_neg>],
                contract! {
                    header: "Semantics of checked neg",
                    inputs: [x: $typ],
                    precondition: x != $typ::MIN,
                    postcondition: x.checked_neg() == Some(eval($typ::down(-x.up()))),
                },
                contract! {
                    header: "Semantics of checked neg when out of bounds",
                    inputs: [x: $typ],
                    precondition: x == $typ::MIN,
                    postcondition: x.checked_neg() == None,
                    n: 1,
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::neg`]"),
                ident: [<t_ $typ _neg>],
                contract! {
                    header: "Semantics of checked neg",
                    inputs: [x: $typ],
                    precondition: x != $typ::MIN,
                    postcondition: {
                        use std::ops::Neg;
                        x.neg() == eval($typ::down(-x.up()))
                    },
                },
                contract! {
                    header: "Semantics of checked neg when out of bounds",
                    inputs: [x: $typ],
                    precondition: x == $typ::MIN,
                    postcondition: {
                        use std::ops::Neg;
                        panics!(x.neg())
                    },
                    n: 1,
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::overflowing_neg`]"),
                ident: [<t_ $typ _overflowing_neg>],
                contract! {
                    header: "Semantics of overflowing neg",
                    inputs: [x: $typ],
                    precondition: x != $typ::MIN,
                    postcondition: x.overflowing_neg() == (eval($typ::down(-x.up())), false),
                },
                contract! {
                    header: "Semantics of overflowing neg when out of bounds",
                    inputs: [x: $typ],
                    precondition: x == $typ::MIN,
                    postcondition: x.overflowing_neg() == (eval($typ::MIN), true),
                    n: 1,
                },
            }

            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::sub`]"),
                ident: [<core_ops_sub_ $typ _sub>],
                contract! {
                    header: "Semantics of non-overflowing subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() <= $typ::MAX.up() && x.up() - y.up() >= $typ::MIN.up(),
                    postcondition: x - y == eval($typ::down(x.up() - y.up())),
                },
                contract! {
                    header: "Overflowing subtraction panics",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() > $typ::MAX.up() || x.up() - y.up() < $typ::MIN.up(),
                    postcondition: panics!(x - y),
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_sub`]"),
                ident: [<core_ops_sub_ $typ _checked_sub>],
                contract! {
                    header: "Semantics of non-overflowing checked subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() <= $typ::MAX.up() && x.up() - y.up() >= $typ::MIN.up(),
                    postcondition: x.checked_sub(y) == Some(eval($typ::down(x.up() - y.up()))),
                },
                contract! {
                    header: "Overflowing subtraction panics",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() > $typ::MAX.up() || x.up() - y.up() < $typ::MIN.up(),
                    postcondition: x.checked_sub(y) == None,
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::wrapping_sub`]"),
                ident: [<core_ops_sub_ $typ _wrapping_sub>],
                contract! {
                    header: "Semantics of non-overflowing wrapping subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() <= $typ::MAX.up() && x.up() - y.up() >= $typ::MIN.up(),
                    postcondition: x.wrapping_sub(y) == eval($typ::down(x.up() - y.up())),
                },
                contract! {
                    header: "Semantics of overflowing wrapping subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() > $typ::MAX.up() || x.up() - y.up() < $typ::MIN.up(),
                    postcondition: x.wrapping_sub(y) == eval(
                        $typ::down(
                            (-$typ::MIN.up() + x.up() - y.up()).rem_euclid(&(-$typ::MIN.up() * 2))
                                + $typ::MIN.up()
                        )
                    ),
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::overflowing_sub`]"),
                ident: [<core_ops_sub_ $typ _overflowing_sub>],
                contract! {
                    header: "Semantics of overflowing subtraction when in bounds",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() <= $typ::MAX.up() && x.up() - y.up() >= $typ::MIN.up(),
                    postcondition: x.overflowing_sub(y) == (eval($typ::down(x.up() - y.up())), false),
                },
                contract! {
                    header: "Semantics of overflowing subtraction when not in bounds",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() > $typ::MAX.up() || x.up() - y.up() < $typ::MIN.up(),
                    postcondition: x.overflowing_sub(y) == (eval(
                        $typ::down(
                            (-$typ::MIN.up() + x.up() - y.up()).rem_euclid(&(-$typ::MIN.up() * 2))
                                + $typ::MIN.up()
                        )
                    ), true),
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::add`]"),
                ident: [<core_ops_add_ $typ _add>],
                contract! {
                    header: "Semantics of non-overflowing addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $typ::MAX.up() && x.up() + y.up() >= $typ::MIN.up(),
                    postcondition: x + y == eval($typ::down(x.up() + y.up())),
                },
                contract! {
                    header: "Overflowing addition panics",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() > $typ::MAX.up() || x.up() + y.up() < $typ::MIN.up(),
                    postcondition: panics!(x + y),
                },
            }
        }
    };
}
macro_rules! mk_unsigned_tests {
    ($typ:ident, $ident:ident) => {
        mk_unsigned_tests!($typ, $ident, $typ::MAX);
    };
    ($typ:ident, $ident:ident, $max:expr) => {
        paste! {
            tests! {
                header: concat!("Properties for [`core::ops::Rem::<", stringify!($typ), ">::rem`]"),
                ident: [<core_ops_rem_ $typ _rem>],
                contract! {
                    header: "Semantics of rem",
                    inputs: [x: $typ, y: $typ],
                    precondition: y != 0,
                    postcondition: x % y == eval($typ::down(x.up() % y.up())),
                },
                contract! {
                    header: "Semantics of rem",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: {
                        #[allow(unconditional_panic)]
                        {
                            panics!(x % 0)
                        }
                    },
                    n: 1,
                },
            }

            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_rem`]"),
                ident: [<core_ops_rem_ $typ _checked_rem>],
                contract! {
                    header: "Semantics of checked_rem",
                    inputs: [x: $typ, y: $typ],
                    precondition: y != 0,
                    postcondition: x.checked_rem(y) == Some(eval($typ::down(x.up() % y.up()))),
                },
                contract! {
                    header: "Semantics of checked_rem",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: {
                        #[allow(unconditional_panic)]
                        {
                            x.checked_rem(0) == None
                        }
                    },
                    n: 1,
                },
            }

            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_neg`]"),
                ident: [<t_ $typ _checked_neg>],
                contract! {
                    header: "Semantics of checked neg when out of bounds",
                    inputs: [x: $typ],
                    precondition: x == $typ::MIN,
                    postcondition: x.checked_neg() == Some(eval($typ::down(-x.up()))),
                    n: 1,
                },
                contract! {
                    header: "Semantics of checked neg",
                    inputs: [x: $typ],
                    precondition: x != $typ::MIN,
                    postcondition: x.checked_neg() == None,
                },
            }

            // Shl
            tests! {
                header: concat!("Properties for [`core::ops::Shl::<", stringify!($typ), ">::shl`]"),
                ident: [<core_ops_shl_ $typ _shl>],
                contract! {
                    header: "Semantics of the left shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x << y == eval($typ::down((x.up() << y) & $max.up())),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the left shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: panics!(x << y),
                    strategy: Id_MicroInt,
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::Shl::<", stringify!($typ), ">::checked_shl`]"),
                ident: [<core_ops_shl_ $typ _checked_shl>],
                contract! {
                    header: "Semantics of the left shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x.checked_shl(y) == Some(eval($typ::down((x.up() << y) & $max.up()))),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the left shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: x.checked_shl(y) == None,
                    strategy: Id_MicroInt,
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::Shl::<", stringify!($typ), ">::overflowing_shl`]"),
                ident: [<core_ops_shl_ $typ _overflowing_shl>],
                contract! {
                    header: "Semantics of the left shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x.overflowing_shl(y) == (eval($typ::down((x.up() << y) & $max.up())), false),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the left shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: x.overflowing_shl(y) == (eval($typ::down((x.up() << (y & ($typ::BITS - 1)) & $max.up()))), true),
                    strategy: Id_MicroInt,
                },
            }

            // Shr
            tests! {
                header: concat!("Properties for [`core::ops::Shr::<", stringify!($typ), ">::shr`]"),
                ident: [<core_ops_shr_ $typ _shr>],
                contract! {
                    header: "Semantics of the right shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x >> y == eval($typ::down((x.up() >> y) & $max.up())),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the right shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: panics!(x >> y),
                    strategy: Id_MicroInt,
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::Shr::<", stringify!($typ), ">::checked_shr`]"),
                ident: [<core_ops_shr_ $typ _checked_shr>],
                contract! {
                    header: "Semantics of the right shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x.checked_shr(y) == Some(eval($typ::down((x.up() >> y) & $max.up()))),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the right shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: x.checked_shr(y) == None,
                    strategy: Id_MicroInt,
                },
            }
            tests! {
                header: concat!("Properties for [`core::ops::Shr::<", stringify!($typ), ">::overflowing_shr`]"),
                ident: [<core_ops_shr_ $typ _overflowing_shr>],
                contract! {
                    header: "Semantics of the right shift when the number of bits is right",
                    inputs: [x: $typ, y: u32],
                    precondition: y < $typ::BITS,
                    postcondition: x.overflowing_shr(y) == (eval($typ::down((x.up() >> y) & $max.up())), false),
                    strategy: Id_MicroInt,
                },
                contract! {
                    header: "Semantics of the right shift otherwise",
                    inputs: [x: $typ, y: u32],
                    precondition: y >= $typ::BITS,
                    postcondition: x.overflowing_shr(y) == (eval($typ::down((x.up() >> (y & ($typ::BITS - 1)) & $max.up()))), true),
                    strategy: Id_MicroInt,
                },
            }

            // Division
            tests! {
                header: concat!("Properties for [`core::ops::Div::<", stringify!($typ), ">::div`]"),
                ident: [<core_ops_div_ $typ _div>],
                contract! {
                    header: "Semantics of the division by non-zero",
                    inputs: [x: $typ, y: $typ],
                    precondition: y != 0,
                    postcondition: x / y == eval($typ::down(x.up() / y.up())),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the division by zero",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: {
                        #[allow(unconditional_panic)]
                        {
                            panics!(x / 0)
                        }
                    },
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::saturating_div`]"),
                ident: [<core_ops_div_ $typ _saturating_div>],
                contract! {
                    header: "Semantics of the saturating division by non-zero",
                    inputs: [x: $typ, y: $typ],
                    precondition: y != 0,
                    postcondition: x.saturating_div(y) == eval($typ::down(x.up() / y.up())),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the saturating division by zero",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: {
                        #[allow(unconditional_panic)]
                        {
                            panics!(x.saturating_div(0))
                        }
                    },
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_div`]"),
                ident: [<core_ops_div_ $typ _checked_div>],
                contract! {
                    header: "Semantics of the checked division by non-zero",
                    inputs: [x: $typ, y: $typ],
                    precondition: y != 0,
                    postcondition: x.checked_div(y) == Some(eval($typ::down(x.up() / y.up()))),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the checked division by zero",
                    inputs: [x: $typ],
                    precondition: true,
                    postcondition: x.checked_div(0) == None,
                },
            }


            // Multiplication
            tests! {
                header: concat!("Properties for [`core::ops::Mul::<", stringify!($typ), ">::mul`]"),
                ident: [<core_ops_mul_ $typ _mul>],
                contract! {
                    header: "Semantics of non-overflowing multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() <= $max.up(),
                    postcondition: x * y == eval($typ::down(x.up() * y.up())),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Panics when overflowing",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() > $max.up(),
                    postcondition: panics!(x * y),
                },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x * 1 == x,
                // },
                // contract! {
                //     header: "Right identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: 1 * x == x,
                // },
                // contract! {
                //     header: "Commutativity",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: x.up() * y.up() <= $max.up(),
                //     postcondition: x * y == y * x,
                //     strategy: SmallNumbers,
                // },
                // contract! {
                //     header: "Associativity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: (
                //         x.up() * y.up() * z.up() <= $max.up() &&
                //             x > 0 && y > 0 && z > 0
                //     ),
                //     postcondition: (x * y) * z == x * (y * z),
                //     strategy: TinyNumbers,
                // },
                // contract! {
                //     header: "Distributivity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: (
                //         x.up() * (y.up() + z.up()) <= $max.up()
                //             && x > 0
                //     ),
                //     postcondition: x * (y + z) == x * y + x * z,
                //     strategy: SmallNumbers,
                // },
                // contract! {
                //     header: "Zero",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x * 0 == 0,
                //     strategy: SmallNumbers,
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_mul`]"),
                ident: [<core_ops_mul_ $typ _checked_mul>],
                contract! {
                    header: "Semantics of the non-overflowing checked multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() <= $max.up(),
                    postcondition: x.checked_mul(y) == Some(eval($typ::down(x.up() * y.up()))),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the overflowing checked multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() > $max.up(),
                    postcondition: x.checked_mul(y) == None,
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::overflowing_mul`]"),
                ident: [<core_ops_mul_ $typ _overflowing_mul>],
                contract! {
                    header: "Semantics of the overflowing multiplication when in bounds",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() <= $max.up(),
                    postcondition: x.overflowing_mul(y) == (eval($typ::down(x.up() * y.up())), false),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the overflowing multiplication when out of bounds",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() > $max.up(),
                    postcondition: x.overflowing_mul(y) == (eval(x.wrapping_mul(y)), true),
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::saturating_mul`]"),
                ident: [<core_ops_mul_ $typ _saturating_mul>],
                contract! {
                    header: "Semantics of the saturating multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.saturating_mul(y) == eval($typ::down((x.up() * y.up()).min($max.up()))),
                },
                contract! {
                    header: "Semantics of the non-overflowing saturating multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() <= $max.up(),
                    postcondition: x.saturating_mul(y) == eval($typ::down(x.up() * y.up())),
                    strategy: SmallInt_SmallInt,
                },
                contract! {
                    header: "Semantics of the overflowing saturating multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() > $max.up(),
                    postcondition: x.saturating_mul(y) == $max,
                },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::wrapping_mul`]"),
                ident: [<core_ops_mul_ $typ _wrapped_mul>],
                contract! {
                    header: "Semantics of the wrapping multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.wrapping_mul(y) == eval($typ::down((x.up() * y.up()) % ($max.up() + 1))),
                },
                contract! {
                    header: "Semantics of the non-overflowing wrapping multiplication",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() * y.up() <= $max.up(),
                    postcondition: x.wrapping_mul(y) == eval($typ::down(x.up() * y.up())),

                    strategy: SmallInt_SmallInt,
                },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x * 1 == x,
                // },
                // contract! {
                //     header: "Right identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: 1 * x == x,
                // },
                // contract! {
                //     header: "Commutativity",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: x.up() * y.up() <= $max.up(),
                //     postcondition: x * y == y * x,
                //     strategy: SmallNumbers,
                // },
                // contract! {
                //     header: "Associativity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: (
                //         x.up() * y.up() * z.up() <= $max.up() &&
                //             x > 0 && y > 0 && z > 0
                //     ),
                //     postcondition: (x * y) * z == x * (y * z),
                //     strategy: TinyNumbers,
                // },
                // contract! {
                //     header: "Distributivity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: (
                //         x.up() * (y.up() + z.up()) <= $max.up()
                //             && x > 0
                //     ),
                //     postcondition: x * (y + z) == x * y + x * z,
                //     strategy: SmallNumbers,
                // },
                // contract! {
                //     header: "Zero",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x * 0 == 0,
                //     strategy: SmallNumbers,
                // },
            }


            // Subtraction
            tests! {
                header: concat!("Properties for [`core::ops::Sub::<", stringify!($typ), ">::sub`]"),
                ident: [<core_ops_add_ $typ _sub>],
                contract! {
                    header: "Semantics of non-underflowing subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() >= 0u8.up(),
                    postcondition: x - y == eval($typ::down(x.up() - y.up())),
                },
                contract! {
                    header: "Panics when underflowing",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() < 0u8.up(),
                    postcondition: panics!(x - y),
                },
                // contract! {
                //     header: "Subtraction is the reverse of addition",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: x.up() - y.up() >= 0u8.up(),
                //     postcondition: (x - y) + y == x,
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x - 0 == x,
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::wrapping_sub`]"),
                ident: [<core_ops_add_ $typ _wrapping_sub>],
                contract! {
                    header: "Semantics of non-underflowing wrapping subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() >= 0u8.up(),
                    postcondition: x.wrapping_sub(y) == eval($typ::down(x.up() - y.up())),
                },
                contract! {
                    header: "Semantics of underflowing wrapping subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() < 0u8.up(),
                    postcondition: x.wrapping_sub(y) == eval($typ::down(x.up() - y.up() + $max + 1)),
                },
                contract! {
                    header: "Semantics of wrapping subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.wrapping_sub(y) == eval($typ::down((x.up() - y.up()).rem_euclid(&($max.up() + 1)))),
                },
                // contract! {
                //     header: "Wrapping subtraction is the reverse of wrapping subtraction",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: true,
                //     postcondition: (x.wrapping_sub(y)).wrapping_add(y) == x,
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x.wrapping_sub(0) == x,
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::checked_sub`]"),
                ident: [<core_ops_add_ $typ _checked_sub>],
                contract! {
                    header: "Semantics of non-underflowing checked subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() >= 0u8.up(),
                    postcondition: x.checked_sub(y) == Some(eval($typ::down(x.up() - y.up()))),
                },
                contract! {
                    header: "Semantics of underflowing checked subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() < 0u8.up(),
                    postcondition: x.checked_sub(y) == None,
                },
                // contract! {
                //     header: "Checked subtraction is the reverse of checked addition",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: x.up() - y.up() >= 0u8.up(),
                //     postcondition: (x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x),
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x.checked_sub(0) == Some(x),
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::saturating_sub`]"),
                ident: [<core_ops_sub_ $typ _saturating_sub>],
                contract! {
                    header: "Semantics of the saturating subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.saturating_sub(y) == eval($typ::down((x.up() - y.up()).max(0u8.up()))),
                },
                contract! {
                    header: "Semantics of the non-overflowing saturating subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() >= 0u8.up(),
                    postcondition: x.saturating_sub(y) == eval($typ::down(x.up() - y.up())),
                },
                contract! {
                    header: "Semantics of the overflowing saturating subtraction",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() - y.up() < 0u8.up(),
                    postcondition: x.saturating_sub(y) == 0,
                },
            }

            // Addition
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
                // contract! {
                //     header: "Commutativity",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: x.up() + y.up() <= $max.up(),
                //     postcondition: x + y == y + x,
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x + 0 == x,
                // },
                // contract! {
                //     header: "Right identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: 0 + x == x,
                // },
                // contract! {
                //     header: "Associativity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: x.up() + y.up() + z.up() <= $max.up(),
                //     postcondition: (x + y) + z == x + (y + z),
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::wrapping_add`]"),
                ident: [<core_ops_add_ $typ _wrapping_add>],
                contract! {
                    header: "Semantics of the wrapping addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.wrapping_add(y) == eval($typ::down((x.up() + y.up()) % ($max.up() + 1))),
                },
                contract! {
                    header: "Semantics of non-overflowing wrapping addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $max.up(),
                    postcondition: x.wrapping_add(y) == eval($typ::down(x.up() + y.up())),
                },
                contract! {
                    header: "Semantics of the overflowing wrapping addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() > $max.up(),
                    postcondition: x.wrapping_add(y) == eval($typ::down(x.up() + y.up() - $max - 1)),
                },
                // contract! {
                //     header: "Commutativity",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: true,
                //     postcondition: x.wrapping_add(y) == y.wrapping_add(x),
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x.wrapping_add(0) == x,
                // },
                // contract! {
                //     header: "Right identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: {
                //         let zero: $typ = 0;
                //         zero.wrapping_add(x) == x
                //     },
                // },
                // contract! {
                //     header: "Associativity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: x.up() + y.up() + z.up() <= $max.up(),
                //     postcondition: (x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z)),
                // },
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
                // contract! {
                //     header: "Commutativity",
                //     inputs: [x: $typ, y: $typ],
                //     precondition: true,
                //     postcondition: x.checked_add(y) == y.checked_add(x),
                // },
                // contract! {
                //     header: "Left identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: x.checked_add([<0 $typ>]) == Some(x),
                // },
                // contract! {
                //     header: "Right identity",
                //     inputs: [x: $typ],
                //     precondition: true,
                //     postcondition: [<0 $typ>].checked_add(x) == Some(x),
                // },
                // contract! {
                //     header: "Associativity",
                //     inputs: [x: $typ, y: $typ, z: $typ],
                //     precondition: true,
                //     postcondition: x.checked_add(y).and_then(|iv| iv.checked_add(z)) == y.checked_add(z).and_then(|iv| x.checked_add(iv)),
                // },
            }
            tests! {
                header: concat!("Properties for [`", stringify!($typ), "::saturating_add`]"),
                ident: [<core_ops_add_ $typ _saturating_add>],
                contract! {
                    header: "Semantics of the saturating addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: true,
                    postcondition: x.saturating_add(y) == eval($typ::down((x.up() + y.up()).min($max.up()))),
                },
                contract! {
                    header: "Semantics of the non-overflowing saturating addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() <= $max.up(),
                    postcondition: x.saturating_add(y) == eval($typ::down(x.up() + y.up())),
                },
                contract! {
                    header: "Semantics of the overflowing saturating addition",
                    inputs: [x: $typ, y: $typ],
                    precondition: x.up() + y.up() > $max.up(),
                    postcondition: x.saturating_add(y) == $max,
                },
            }
        }
    };
}

pub fn specs() {
    mk_unsigned_tests!(u8, core_ops_add_u8_add);
    mk_unsigned_tests!(u16, core_ops_add_u16_add);
    mk_unsigned_tests!(u32, core_ops_add_u32_add);
    mk_unsigned_tests!(u64, core_ops_add_u64_add);
    mk_unsigned_tests!(u128, core_ops_add_u128_add);
    mk_unsigned_tests!(usize, core_ops_add_u8_add);

    mk_signed_tests!(i8);
    mk_signed_tests!(i16);
    mk_signed_tests!(i32);
    mk_signed_tests!(i64);
    mk_signed_tests!(i128);
    mk_signed_tests!(isize);

    mk_common_tests!(u8);
    mk_common_tests!(u16);
    mk_common_tests!(u32);
    mk_common_tests!(u64);
    mk_common_tests!(u128);
    mk_common_tests!(usize);
    mk_common_tests!(i8);
    mk_common_tests!(i16);
    mk_common_tests!(i32);
    mk_common_tests!(i64);
    mk_common_tests!(i128);
    mk_common_tests!(isize);
}
