//! This module contains 533 tests, organized in functions.
#![allow(arithmetic_overflow)]
#![allow(unused_parens)]
#![allow(unused_comparisons)]
use core_spec::*;

/// Properties for [`Option::is_some`]
pub fn option_is_some() {
    eprintln!(r#"Testing "Properties for [`Option::is_some`]"... (1 contracts)"#);
    eprint!("  ");
    // ## `is_some` is a shorthand to pattern matching
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `true`
    // __Postcondition:__ `v.is_some()
    //         == (match v {
    //             Some(_) => true,
    //             None => false,
    //         })`
    {
        assert!(
            Option::<u8>::None.is_some()
                == (match Option::<u8>::None {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(0u8).is_some()
                == (match Some(0u8) {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(255u8).is_some()
                == (match Some(255u8) {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(78u8).is_some()
                == (match Some(78u8) {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(13u8).is_some()
                == (match Some(13u8) {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(187u8).is_some()
                == (match Some(187u8) {
                    Some(_) => true,
                    None => false,
                })
        );
        assert!(
            Some(35u8).is_some()
                == (match Some(35u8) {
                    Some(_) => true,
                    None => false,
                })
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::is_none`]
pub fn option_is_none() {
    eprintln!(r#"Testing "Properties for [`Option::is_none`]"... (1 contracts)"#);
    eprint!("  ");
    // ## `is_none` is a shorthand to pattern matching
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `true`
    // __Postcondition:__ `v.is_none()
    //         == (match v {
    //             Some(_) => false,
    //             None => true,
    //         })`
    {
        assert!(
            Option::<u8>::None.is_none()
                == (match Option::<u8>::None {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(0u8).is_none()
                == (match Some(0u8) {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(255u8).is_none()
                == (match Some(255u8) {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(67u8).is_none()
                == (match Some(67u8) {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(17u8).is_none()
                == (match Some(17u8) {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(157u8).is_none()
                == (match Some(157u8) {
                    Some(_) => false,
                    None => true,
                })
        );
        assert!(
            Some(48u8).is_none()
                == (match Some(48u8) {
                    Some(_) => false,
                    None => true,
                })
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::expect`]
pub fn option_expect() {
    eprintln!(r#"Testing "Properties for [`Option::expect`]"... (3 contracts)"#);
    eprint!("  ");
    // ## Unwrapping a [`None`] with `expect` always panic
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `v.is_none()`
    // __Postcondition:__ `panics!(v.expect("message"))`
    {
        assert!(panics!(Option::<u8>::None.expect("message")));
    }
    eprint!("... ");
    // ## Unwrapping a [`Some(_)`] with `expect` always succeeds
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `v.is_some()`
    // __Postcondition:__ `doesn_t_panic!(v.expect("message"))`
    {
        assert!(doesn_t_panic!(Some(0u8).expect("message")));
        assert!(doesn_t_panic!(Some(255u8).expect("message")));
        assert!(doesn_t_panic!(Some(189u8).expect("message")));
        assert!(doesn_t_panic!(Some(96u8).expect("message")));
        assert!(doesn_t_panic!(Some(6u8).expect("message")));
        assert!(doesn_t_panic!(Some(245u8).expect("message")));
        assert!(doesn_t_panic!(Some(82u8).expect("message")));
    }
    eprint!("... ");
    // ## Wrapping a value in a `Some` and unwrapping is identity
    // __Inputs:__ `v : T`
    // __Precondition:__ `true`
    // __Postcondition:__ `Some(v).unwrap() == v`
    {
        assert!(Some(0u8).unwrap() == 0u8);
        assert!(Some(255u8).unwrap() == 255u8);
        assert!(Some(10u8).unwrap() == 10u8);
        assert!(Some(2u8).unwrap() == 2u8);
        assert!(Some(149u8).unwrap() == 149u8);
        assert!(Some(45u8).unwrap() == 45u8);
        assert!(Some(155u8).unwrap() == 155u8);
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::map`]
pub fn option_map() {
    eprintln!(r#"Testing "Properties for [`Option::map`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`
    // __Inputs:__ `v : T, f : Fn1<T, T>`
    // __Precondition:__ `true`
    // __Postcondition:__ `Some(v).map(f) == Some((f)(v))`
    {
        assert!(
            Some(0u8).map((|x: u8| x.wrapping_add(x))) == Some((|x: u8| x.wrapping_add(x))(0u8))
        );
        assert!(
            Some(0u8).map((|x: u8| x.wrapping_add(0u8)))
                == Some((|x: u8| x.wrapping_add(0u8))(0u8))
        );
        assert!(
            Some(0u8).map((|x: u8| x.wrapping_add(255u8)))
                == Some((|x: u8| x.wrapping_add(255u8))(0u8))
        );
        assert!(
            Some(255u8).map((|x: u8| x.wrapping_add(x)))
                == Some((|x: u8| x.wrapping_add(x))(255u8))
        );
        assert!(
            Some(255u8).map((|x: u8| x.wrapping_add(0u8)))
                == Some((|x: u8| x.wrapping_add(0u8))(255u8))
        );
        assert!(
            Some(255u8).map((|x: u8| x.wrapping_add(255u8)))
                == Some((|x: u8| x.wrapping_add(255u8))(255u8))
        );
        assert!(Some(30u8).map((|x: u8| x)) == Some((|x: u8| x)(30u8)));
    }
    eprint!("... ");
    // ## Mapping a `None` is the identity
    // __Inputs:__ `v : Option<T>, f : Fn1<T, T>`
    // __Precondition:__ `v.is_none()`
    // __Postcondition:__ `v.map(f) == v`
    {
        assert!(Option::<u8>::None.map((|x: u8| x.wrapping_add(x))) == Option::<u8>::None);
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::filter`]
pub fn option_filter() {
    eprintln!(r#"Testing "Properties for [`Option::filter`]"... (2 contracts)"#);
    eprint!("  ");
    // ## The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`
    // __Inputs:__ `v : T, f : FnR1<T, bool>`
    // __Precondition:__ `true`
    // __Postcondition:__ `Some(v).filter(f).is_some() == f(&v)`
    {
        assert!(Some(0u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&0u8));
        assert!(Some(0u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&0u8));
        assert!(Some(255u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&255u8));
        assert!(
            Some(255u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&255u8)
        );
        assert!(Some(140u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&140u8));
        assert!(Some(45u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&45u8));
        assert!(Some(35u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&35u8));
    }
    eprint!("... ");
    // ## Filtering a `None` is the identity
    // __Inputs:__ `v : Option<T>, f : FnR1<T, bool>`
    // __Precondition:__ `v.is_none()`
    // __Postcondition:__ `v.filter(f) == v`
    {
        assert!(Option::<u8>::None.filter((|x: &u8| *x < 0u8)) == Option::<u8>::None);
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::flatten`]
pub fn option_flatten() {
    eprintln!(r#"Testing "Properties for [`Option::flatten`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Nested `Some`s
    // __Inputs:__ `x : T`
    // __Precondition:__ `true`
    // __Postcondition:__ `Some(Some(x)).flatten() == Some(x)`
    {
        assert!(Some(Some(0u8)).flatten() == Some(0u8));
        assert!(Some(Some(255u8)).flatten() == Some(255u8));
        assert!(Some(Some(163u8)).flatten() == Some(163u8));
        assert!(Some(Some(202u8)).flatten() == Some(202u8));
        assert!(Some(Some(80u8)).flatten() == Some(80u8));
        assert!(Some(Some(146u8)).flatten() == Some(146u8));
        assert!(Some(Some(219u8)).flatten() == Some(219u8));
    }
    eprint!("... ");
    // ## Nested or direct `None` flattens to None
    // __Inputs:__ `x : Option<Option<T>>`
    // __Precondition:__ `x.is_none() || x.unwrap().is_none()`
    // __Postcondition:__ `x.flatten() == None`
    {
        assert!(Option::<Option<u8>>::None.flatten() == None);
        assert!(Some(Option::<u8>::None).flatten() == None);
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::take`]
pub fn option_take() {
    eprintln!(r#"Testing "Properties for [`Option::take`]"... (1 contracts)"#);
    eprint!("  ");
    // ## Take steals a value
    // __Inputs:__ `x : Option<T>`
    // __Precondition:__ `true`
    // __Postcondition:__ `{
    //         let mut y = x.clone();
    //         y.take() == x && y.is_none()
    //     }`
    {
        assert!({
            let mut y = Option::<u8>::None.clone();
            y.take() == Option::<u8>::None && y.is_none()
        });
        assert!({
            let mut y = Some(0u8).clone();
            y.take() == Some(0u8) && y.is_none()
        });
        assert!({
            let mut y = Some(255u8).clone();
            y.take() == Some(255u8) && y.is_none()
        });
        assert!({
            let mut y = Some(183u8).clone();
            y.take() == Some(183u8) && y.is_none()
        });
        assert!({
            let mut y = Some(152u8).clone();
            y.take() == Some(152u8) && y.is_none()
        });
        assert!({
            let mut y = Some(116u8).clone();
            y.take() == Some(116u8) && y.is_none()
        });
        assert!({
            let mut y = Some(7u8).clone();
            y.take() == Some(7u8) && y.is_none()
        });
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::zip`]
pub fn option_zip() {
    eprintln!(r#"Testing "Properties for [`Option::zip`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Zipping two non-empty options
    // __Inputs:__ `x : T, y : T`
    // __Precondition:__ `true`
    // __Postcondition:__ `Some(x).zip(Some(y)) == Some((x, y))`
    {
        assert!(Some(0u8).zip(Some(0u8)) == Some((0u8, 0u8)));
        assert!(Some(0u8).zip(Some(255u8)) == Some((0u8, 255u8)));
        assert!(Some(255u8).zip(Some(0u8)) == Some((255u8, 0u8)));
        assert!(Some(255u8).zip(Some(255u8)) == Some((255u8, 255u8)));
        assert!(Some(224u8).zip(Some(108u8)) == Some((224u8, 108u8)));
        assert!(Some(92u8).zip(Some(54u8)) == Some((92u8, 54u8)));
        assert!(Some(73u8).zip(Some(180u8)) == Some((73u8, 180u8)));
    }
    eprint!("... ");
    // ## Zipping two options when one is a `None` makes `None`
    // __Inputs:__ `x : Option<T>, y : Option<T>`
    // __Precondition:__ `x.is_none() || y.is_none()`
    // __Postcondition:__ `x.zip(y).is_none()`
    {
        assert!(Option::<u8>::None.zip(Option::<u8>::None).is_none());
        assert!(Option::<u8>::None.zip(Some(0u8)).is_none());
        assert!(Option::<u8>::None.zip(Some(255u8)).is_none());
        assert!(Some(0u8).zip(Option::<u8>::None).is_none());
        assert!(Some(255u8).zip(Option::<u8>::None).is_none());
        assert!(Option::<u8>::None.zip(Some(107u8)).is_none());
        assert!(Some(157u8).zip(Option::<u8>::None).is_none());
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::unwrap`]
pub fn option_unwrap() {
    eprintln!(r#"Testing "Properties for [`Option::unwrap`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Unwrapping a [`None`] always panic
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `v.is_none()`
    // __Postcondition:__ `panics!(v.unwrap())`
    {
        assert!(panics!(Option::<u8>::None.unwrap()));
    }
    eprint!("... ");
    // ## Unwrapping a [`Some(_)`] always succeeds
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `v.is_some()`
    // __Postcondition:__ `doesn_t_panic!(v.unwrap())`
    {
        assert!(doesn_t_panic!(Some(0u8).unwrap()));
        assert!(doesn_t_panic!(Some(255u8).unwrap()));
        assert!(doesn_t_panic!(Some(209u8).unwrap()));
        assert!(doesn_t_panic!(Some(144u8).unwrap()));
        assert!(doesn_t_panic!(Some(240u8).unwrap()));
        assert!(doesn_t_panic!(Some(93u8).unwrap()));
        assert!(doesn_t_panic!(Some(16u8).unwrap()));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::as_mut`]
pub fn option_as_mut() {
    eprintln!(r#"Testing "Properties for [`Option::as_mut`]"... (1 contracts)"#);
    eprint!("  ");
    // ## In place update via `as_mut` is equivalent to functional update
    // __Inputs:__ `v : Option<u8>`
    // __Precondition:__ `v.is_some() && v.unwrap() < 50`
    // __Postcondition:__ `{
    //         let (v_unwrapped, mut v_mut) = (v.unwrap().clone(), v);
    //         *v_mut.as_mut().unwrap() += 10;
    //         v_mut.unwrap() == v_unwrapped + 10
    //     }`
    {
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(15u8).unwrap().clone(), Some(15u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(33u8).unwrap().clone(), Some(33u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(6u8).unwrap().clone(), Some(6u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(44u8).unwrap().clone(), Some(44u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(7u8).unwrap().clone(), Some(7u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
        assert!({
            let (v_unwrapped, mut v_mut) = (Some(37u8).unwrap().clone(), Some(37u8));
            *v_mut.as_mut().unwrap() += 10;
            v_mut.unwrap() == v_unwrapped + 10
        });
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Option::as_slice`]
pub fn option_as_slice() {
    eprintln!(r#"Testing "Properties for [`Option::as_slice`]"... (2 contracts)"#);
    eprint!("  ");
    // ## [`None.as_slice()`] should always result in an empty slice
    // __Inputs:__ `v : Option<T>`
    // __Precondition:__ `v.is_none()`
    // __Postcondition:__ `{ v.as_slice().is_empty() }`
    {
        assert!({ Option::<u8>::None.as_slice().is_empty() });
    }
    eprint!("... ");
    // ## [`Some(v).as_slice()`] should always result in a slice containing exactly `v`
    // __Inputs:__ `v : T`
    // __Precondition:__ `true`
    // __Postcondition:__ `{ Some(v).as_slice() == [v] }`
    {
        assert!({ Some(0u8).as_slice() == [0u8] });
        assert!({ Some(255u8).as_slice() == [255u8] });
        assert!({ Some(123u8).as_slice() == [123u8] });
        assert!({ Some(199u8).as_slice() == [199u8] });
        assert!({ Some(168u8).as_slice() == [168u8] });
        assert!({ Some(110u8).as_slice() == [110u8] });
        assert!({ Some(36u8).as_slice() == [36u8] });
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`Vec`]
pub fn slice_get() {
    eprintln!(r#"Testing "Properties for [`Vec`]"... (1 contracts)"#);
    eprint!("  ");
    // ## Indexing
    // __Inputs:__ `v : Vec<u8>, i : usize`
    // __Precondition:__ `v.len() > 0`
    // __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`
    {
        assert!(vec![0u8].get(0usize) == Some(&0u8));
        assert!(vec![0u8].get(0usize) == Some(&0u8));
        assert!(vec![255u8].get(0usize) == Some(&255u8));
        assert!(vec![255u8].get(0usize) == Some(&255u8));
        assert!(vec![219u8, 63u8, 52u8].get(1usize) == Some(&63u8));
        assert!(vec![31u8, 227u8].get(0usize) == Some(&31u8));
        assert!(vec![116u8].get(0usize) == Some(&116u8));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Sub::<i8>::sub`]
pub fn core_ops_sub_i8_sub() {
    eprintln!(r#"Testing "Properties for [`core::ops::Sub::<i8>::sub`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing subtraction
    // __Inputs:__ `x : i8, y : i8`
    // __Precondition:__ `x.up() - y.up() < 128.up() && x.up() - y.up() >= -128i32.up()`
    // __Postcondition:__ `x - y == i8::down(x.up() - y.up())`
    {
        assert!(-128i8 - -128i8 == 0i8);
        assert!(-128i8 - 0i8 == -128i8);
        assert!(0i8 - 0i8 == 0i8);
        assert!(0i8 - 127i8 == -127i8);
        assert!(127i8 - 0i8 == 127i8);
        assert!(127i8 - 127i8 == 0i8);
        assert!(-33i8 - -101i8 == 68i8);
    }
    eprint!("... ");
    // ## Overflowing subtraction panics
    // __Inputs:__ `x : i8, y : i8`
    // __Precondition:__ `x.up() - y.up() >= 128.up() || x.up() - y.up() < -128i32.up()`
    // __Postcondition:__ `panics!(x - y)`
    {
        assert!(panics!(-128i8 - 127i8));
        assert!(panics!(0i8 - -128i8));
        assert!(panics!(127i8 - -128i8));
        assert!(panics!(108i8 - -53i8));
        assert!(panics!(-61i8 - 108i8));
        assert!(panics!(-51i8 - 97i8));
        assert!(panics!(108i8 - -93i8));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<i8>::add`]
pub fn core_ops_add_i8_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<i8>::add`]"... (2 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : i8, y : i8`
    // __Precondition:__ `x.up() + y.up() < 128.up() && x.up() + y.up() >= -128i32.up()`
    // __Postcondition:__ `x + y == i8::down(x.up() + y.up())`
    {
        assert!(-128i8 + 0i8 == -128i8);
        assert!(-128i8 + 127i8 == -1i8);
        assert!(0i8 + -128i8 == -128i8);
        assert!(0i8 + 0i8 == 0i8);
        assert!(0i8 + 127i8 == 127i8);
        assert!(127i8 + -128i8 == -1i8);
        assert!(127i8 + 0i8 == 127i8);
    }
    eprint!("... ");
    // ## Overflowing addition panics
    // __Inputs:__ `x : i8, y : i8`
    // __Precondition:__ `x.up() + y.up() >= 128.up() || x.up() + y.up() < -128i32.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(-128i8 + -128i8));
        assert!(panics!(127i8 + 127i8));
        assert!(panics!(-97i8 + -106i8));
        assert!(panics!(75i8 + 95i8));
        assert!(panics!(-111i8 + -111i8));
        assert!(panics!(-40i8 + -105i8));
        assert!(panics!(116i8 + 62i8));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u8>::add`]
pub fn core_ops_add_u8_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<u8>::add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`
    // __Postcondition:__ `x + y == u8::down(x.up() + y.up())`
    {
        assert!(0u8 + 0u8 == 0u8);
        assert!(0u8 + 255u8 == 255u8);
        assert!(255u8 + 0u8 == 255u8);
        assert!(138u8 + 59u8 == 197u8);
        assert!(30u8 + 34u8 == 64u8);
        assert!(90u8 + 84u8 == 174u8);
        assert!(45u8 + 147u8 == 192u8);
    }
    eprint!("... ");
    // ## Panics when overflowing
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `x.up() + y.up() > u8::MAX.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(255u8 + 255u8));
        assert!(panics!(213u8 + 184u8));
        assert!(panics!(198u8 + 186u8));
        assert!(panics!(140u8 + 253u8));
        assert!(panics!(160u8 + 205u8));
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`
    // __Postcondition:__ `x + y == y + x`
    {
        assert!(0u8 + 0u8 == 0u8 + 0u8);
        assert!(0u8 + 255u8 == 255u8 + 0u8);
        assert!(255u8 + 0u8 == 0u8 + 255u8);
        assert!(176u8 + 54u8 == 54u8 + 176u8);
        assert!(31u8 + 74u8 == 74u8 + 31u8);
        assert!(166u8 + 81u8 == 81u8 + 166u8);
        assert!(97u8 + 72u8 == 72u8 + 97u8);
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `x + 0 == x`
    {
        assert!(0u8 + 0 == 0u8);
        assert!(255u8 + 0 == 255u8);
        assert!(180u8 + 0 == 180u8);
        assert!(149u8 + 0 == 149u8);
        assert!(74u8 + 0 == 74u8);
        assert!(86u8 + 0 == 86u8);
        assert!(2u8 + 0 == 2u8);
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `0 + x == x`
    {
        assert!(0 + 0u8 == 0u8);
        assert!(0 + 255u8 == 255u8);
        assert!(0 + 145u8 == 145u8);
        assert!(0 + 52u8 == 52u8);
        assert!(0 + 120u8 == 120u8);
        assert!(0 + 68u8 == 68u8);
        assert!(0 + 126u8 == 126u8);
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u8, y : u8, z : u8`
    // __Precondition:__ `x.up() + y.up() + z.up() <= u8::MAX.up()`
    // __Postcondition:__ `(x + y) + z == x + (y + z)`
    {
        assert!((0u8 + 0u8) + 0u8 == 0u8 + (0u8 + 0u8));
        assert!((0u8 + 0u8) + 255u8 == 0u8 + (0u8 + 255u8));
        assert!((0u8 + 255u8) + 0u8 == 0u8 + (255u8 + 0u8));
        assert!((255u8 + 0u8) + 0u8 == 255u8 + (0u8 + 0u8));
        assert!((36u8 + 40u8) + 106u8 == 36u8 + (40u8 + 106u8));
        assert!((46u8 + 18u8) + 133u8 == 46u8 + (18u8 + 133u8));
        assert!((89u8 + 41u8) + 118u8 == 89u8 + (41u8 + 118u8));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`u8::checked_add`]
pub fn core_ops_add_u8_checked_add() {
    eprintln!(r#"Testing "Properties for [`u8::checked_add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing checked addition
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == Some(u8::down(x.up() + y.up()))`
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(0u8.checked_add(255u8) == Some(255u8));
        assert!(255u8.checked_add(0u8) == Some(255u8));
        assert!(6u8.checked_add(195u8) == Some(201u8));
        assert!(45u8.checked_add(106u8) == Some(151u8));
        assert!(96u8.checked_add(126u8) == Some(222u8));
        assert!(115u8.checked_add(18u8) == Some(133u8));
    }
    eprint!("... ");
    // ## None when overflowing
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `x.up() + y.up() > u8::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == None`
    {
        assert!(255u8.checked_add(255u8) == None);
        assert!(95u8.checked_add(174u8) == None);
        assert!(229u8.checked_add(248u8) == None);
        assert!(215u8.checked_add(228u8) == None);
        assert!(101u8.checked_add(228u8) == None);
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u8, y : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
    {
        assert!(0u8.checked_add(0u8) == 0u8.checked_add(0u8));
        assert!(0u8.checked_add(255u8) == 255u8.checked_add(0u8));
        assert!(255u8.checked_add(0u8) == 0u8.checked_add(255u8));
        assert!(255u8.checked_add(255u8) == 255u8.checked_add(255u8));
        assert!(36u8.checked_add(18u8) == 18u8.checked_add(36u8));
        assert!(124u8.checked_add(162u8) == 162u8.checked_add(124u8));
        assert!(132u8.checked_add(27u8) == 27u8.checked_add(132u8));
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(0u8) == Some(x)`
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(255u8.checked_add(0u8) == Some(255u8));
        assert!(186u8.checked_add(0u8) == Some(186u8));
        assert!(52u8.checked_add(0u8) == Some(52u8));
        assert!(36u8.checked_add(0u8) == Some(36u8));
        assert!(62u8.checked_add(0u8) == Some(62u8));
        assert!(193u8.checked_add(0u8) == Some(193u8));
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `0u8.checked_add(x) == Some(x)`
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(0u8.checked_add(255u8) == Some(255u8));
        assert!(0u8.checked_add(8u8) == Some(8u8));
        assert!(0u8.checked_add(227u8) == Some(227u8));
        assert!(0u8.checked_add(34u8) == Some(34u8));
        assert!(0u8.checked_add(19u8) == Some(19u8));
        assert!(0u8.checked_add(229u8) == Some(229u8));
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u8, y : u8, z : u8`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
    //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
    {
        assert!(
            0u8.checked_add(0u8).and_then(|iv| iv.checked_add(0u8))
                == 0u8.checked_add(0u8).and_then(|iv| 0u8.checked_add(iv))
        );
        assert!(
            0u8.checked_add(0u8).and_then(|iv| iv.checked_add(255u8))
                == 0u8.checked_add(255u8).and_then(|iv| 0u8.checked_add(iv))
        );
        assert!(
            0u8.checked_add(255u8).and_then(|iv| iv.checked_add(0u8))
                == 255u8.checked_add(0u8).and_then(|iv| 0u8.checked_add(iv))
        );
        assert!(
            0u8.checked_add(255u8).and_then(|iv| iv.checked_add(255u8))
                == 255u8.checked_add(255u8).and_then(|iv| 0u8.checked_add(iv))
        );
        assert!(
            255u8.checked_add(0u8).and_then(|iv| iv.checked_add(0u8))
                == 0u8.checked_add(0u8).and_then(|iv| 255u8.checked_add(iv))
        );
        assert!(
            255u8.checked_add(0u8).and_then(|iv| iv.checked_add(255u8))
                == 0u8.checked_add(255u8).and_then(|iv| 255u8.checked_add(iv))
        );
        assert!(
            255u8.checked_add(255u8).and_then(|iv| iv.checked_add(0u8))
                == 255u8.checked_add(0u8).and_then(|iv| 255u8.checked_add(iv))
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u16>::add`]
pub fn core_ops_add_u16_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<u16>::add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`
    // __Postcondition:__ `x + y == u16::down(x.up() + y.up())`
    {
        assert!(0u16 + 0u16 == 0u16);
        assert!(0u16 + 65535u16 == 65535u16);
        assert!(65535u16 + 0u16 == 65535u16);
        assert!(24124u16 + 18014u16 == 42138u16);
        assert!(26174u16 + 33499u16 == 59673u16);
        assert!(12090u16 + 26339u16 == 38429u16);
        assert!(4321u16 + 42782u16 == 47103u16);
    }
    eprint!("... ");
    // ## Panics when overflowing
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `x.up() + y.up() > u16::MAX.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(65535u16 + 65535u16));
        assert!(panics!(65504u16 + 51115u16));
        assert!(panics!(16421u16 + 57856u16));
        assert!(panics!(27649u16 + 45810u16));
        assert!(panics!(29899u16 + 39706u16));
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`
    // __Postcondition:__ `x + y == y + x`
    {
        assert!(0u16 + 0u16 == 0u16 + 0u16);
        assert!(0u16 + 65535u16 == 65535u16 + 0u16);
        assert!(65535u16 + 0u16 == 0u16 + 65535u16);
        assert!(16877u16 + 22765u16 == 22765u16 + 16877u16);
        assert!(11813u16 + 44705u16 == 44705u16 + 11813u16);
        assert!(3742u16 + 20717u16 == 20717u16 + 3742u16);
        assert!(22074u16 + 5249u16 == 5249u16 + 22074u16);
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `x + 0 == x`
    {
        assert!(0u16 + 0 == 0u16);
        assert!(65535u16 + 0 == 65535u16);
        assert!(18455u16 + 0 == 18455u16);
        assert!(10528u16 + 0 == 10528u16);
        assert!(19795u16 + 0 == 19795u16);
        assert!(63012u16 + 0 == 63012u16);
        assert!(3159u16 + 0 == 3159u16);
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `0 + x == x`
    {
        assert!(0 + 0u16 == 0u16);
        assert!(0 + 65535u16 == 65535u16);
        assert!(0 + 28600u16 == 28600u16);
        assert!(0 + 3279u16 == 3279u16);
        assert!(0 + 36767u16 == 36767u16);
        assert!(0 + 28200u16 == 28200u16);
        assert!(0 + 7540u16 == 7540u16);
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u16, y : u16, z : u16`
    // __Precondition:__ `x.up() + y.up() + z.up() <= u16::MAX.up()`
    // __Postcondition:__ `(x + y) + z == x + (y + z)`
    {
        assert!((0u16 + 0u16) + 0u16 == 0u16 + (0u16 + 0u16));
        assert!((0u16 + 0u16) + 65535u16 == 0u16 + (0u16 + 65535u16));
        assert!((0u16 + 65535u16) + 0u16 == 0u16 + (65535u16 + 0u16));
        assert!((65535u16 + 0u16) + 0u16 == 65535u16 + (0u16 + 0u16));
        assert!((24597u16 + 12960u16) + 1318u16 == 24597u16 + (12960u16 + 1318u16));
        assert!((18030u16 + 36649u16) + 8478u16 == 18030u16 + (36649u16 + 8478u16));
        assert!((42325u16 + 10528u16) + 1755u16 == 42325u16 + (10528u16 + 1755u16));
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`u16::checked_add`]
pub fn core_ops_add_u16_checked_add() {
    eprintln!(r#"Testing "Properties for [`u16::checked_add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing checked addition
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == Some(u16::down(x.up() + y.up()))`
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(0u16.checked_add(65535u16) == Some(65535u16));
        assert!(65535u16.checked_add(0u16) == Some(65535u16));
        assert!(45628u16.checked_add(9610u16) == Some(55238u16));
        assert!(35902u16.checked_add(15379u16) == Some(51281u16));
        assert!(38275u16.checked_add(7899u16) == Some(46174u16));
        assert!(6159u16.checked_add(39658u16) == Some(45817u16));
    }
    eprint!("... ");
    // ## None when overflowing
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `x.up() + y.up() > u16::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == None`
    {
        assert!(65535u16.checked_add(65535u16) == None);
        assert!(41361u16.checked_add(43021u16) == None);
        assert!(42249u16.checked_add(53632u16) == None);
        assert!(47706u16.checked_add(25555u16) == None);
        assert!(32218u16.checked_add(48415u16) == None);
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u16, y : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
    {
        assert!(0u16.checked_add(0u16) == 0u16.checked_add(0u16));
        assert!(0u16.checked_add(65535u16) == 65535u16.checked_add(0u16));
        assert!(65535u16.checked_add(0u16) == 0u16.checked_add(65535u16));
        assert!(65535u16.checked_add(65535u16) == 65535u16.checked_add(65535u16));
        assert!(42773u16.checked_add(32114u16) == 32114u16.checked_add(42773u16));
        assert!(40728u16.checked_add(33975u16) == 33975u16.checked_add(40728u16));
        assert!(31110u16.checked_add(27014u16) == 27014u16.checked_add(31110u16));
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(0u16) == Some(x)`
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(65535u16.checked_add(0u16) == Some(65535u16));
        assert!(19492u16.checked_add(0u16) == Some(19492u16));
        assert!(45095u16.checked_add(0u16) == Some(45095u16));
        assert!(15600u16.checked_add(0u16) == Some(15600u16));
        assert!(2433u16.checked_add(0u16) == Some(2433u16));
        assert!(33348u16.checked_add(0u16) == Some(33348u16));
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `0u16.checked_add(x) == Some(x)`
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(0u16.checked_add(65535u16) == Some(65535u16));
        assert!(0u16.checked_add(24497u16) == Some(24497u16));
        assert!(0u16.checked_add(11982u16) == Some(11982u16));
        assert!(0u16.checked_add(3809u16) == Some(3809u16));
        assert!(0u16.checked_add(42569u16) == Some(42569u16));
        assert!(0u16.checked_add(21268u16) == Some(21268u16));
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u16, y : u16, z : u16`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
    //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
    {
        assert!(
            0u16.checked_add(0u16).and_then(|iv| iv.checked_add(0u16))
                == 0u16.checked_add(0u16).and_then(|iv| 0u16.checked_add(iv))
        );
        assert!(
            0u16.checked_add(0u16)
                .and_then(|iv| iv.checked_add(65535u16))
                == 0u16
                    .checked_add(65535u16)
                    .and_then(|iv| 0u16.checked_add(iv))
        );
        assert!(
            0u16.checked_add(65535u16)
                .and_then(|iv| iv.checked_add(0u16))
                == 65535u16
                    .checked_add(0u16)
                    .and_then(|iv| 0u16.checked_add(iv))
        );
        assert!(
            0u16.checked_add(65535u16)
                .and_then(|iv| iv.checked_add(65535u16))
                == 65535u16
                    .checked_add(65535u16)
                    .and_then(|iv| 0u16.checked_add(iv))
        );
        assert!(
            65535u16
                .checked_add(0u16)
                .and_then(|iv| iv.checked_add(0u16))
                == 0u16
                    .checked_add(0u16)
                    .and_then(|iv| 65535u16.checked_add(iv))
        );
        assert!(
            65535u16
                .checked_add(0u16)
                .and_then(|iv| iv.checked_add(65535u16))
                == 0u16
                    .checked_add(65535u16)
                    .and_then(|iv| 65535u16.checked_add(iv))
        );
        assert!(
            65535u16
                .checked_add(65535u16)
                .and_then(|iv| iv.checked_add(0u16))
                == 65535u16
                    .checked_add(0u16)
                    .and_then(|iv| 65535u16.checked_add(iv))
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u32>::add`]
pub fn core_ops_add_u32_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<u32>::add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`
    // __Postcondition:__ `x + y == u32::down(x.up() + y.up())`
    {
        assert!(0u32 + 0u32 == 0u32);
        assert!(0u32 + 4294967295u32 == 4294967295u32);
        assert!(4294967295u32 + 0u32 == 4294967295u32);
        assert!(1888012904u32 + 905859874u32 == 2793872778u32);
        assert!(3034822593u32 + 578243239u32 == 3613065832u32);
        assert!(194511753u32 + 3338972016u32 == 3533483769u32);
        assert!(813008610u32 + 2380486259u32 == 3193494869u32);
    }
    eprint!("... ");
    // ## Panics when overflowing
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `x.up() + y.up() > u32::MAX.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(4294967295u32 + 4294967295u32));
        assert!(panics!(1900943365u32 + 3130999383u32));
        assert!(panics!(3482894212u32 + 1561794855u32));
        assert!(panics!(4128810562u32 + 3859426271u32));
        assert!(panics!(1801452783u32 + 3556315738u32));
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`
    // __Postcondition:__ `x + y == y + x`
    {
        assert!(0u32 + 0u32 == 0u32 + 0u32);
        assert!(0u32 + 4294967295u32 == 4294967295u32 + 0u32);
        assert!(4294967295u32 + 0u32 == 0u32 + 4294967295u32);
        assert!(2716784575u32 + 1363430664u32 == 1363430664u32 + 2716784575u32);
        assert!(768356999u32 + 2469258258u32 == 2469258258u32 + 768356999u32);
        assert!(2769207080u32 + 816340993u32 == 816340993u32 + 2769207080u32);
        assert!(2505921434u32 + 220667377u32 == 220667377u32 + 2505921434u32);
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `x + 0 == x`
    {
        assert!(0u32 + 0 == 0u32);
        assert!(4294967295u32 + 0 == 4294967295u32);
        assert!(45561805u32 + 0 == 45561805u32);
        assert!(3995834216u32 + 0 == 3995834216u32);
        assert!(4222344967u32 + 0 == 4222344967u32);
        assert!(3139919671u32 + 0 == 3139919671u32);
        assert!(560974991u32 + 0 == 560974991u32);
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `0 + x == x`
    {
        assert!(0 + 0u32 == 0u32);
        assert!(0 + 4294967295u32 == 4294967295u32);
        assert!(0 + 4256023399u32 == 4256023399u32);
        assert!(0 + 2773860066u32 == 2773860066u32);
        assert!(0 + 791823252u32 == 791823252u32);
        assert!(0 + 1644077956u32 == 1644077956u32);
        assert!(0 + 4084155989u32 == 4084155989u32);
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u32, y : u32, z : u32`
    // __Precondition:__ `x.up() + y.up() + z.up() <= u32::MAX.up()`
    // __Postcondition:__ `(x + y) + z == x + (y + z)`
    {
        assert!((0u32 + 0u32) + 0u32 == 0u32 + (0u32 + 0u32));
        assert!((0u32 + 0u32) + 4294967295u32 == 0u32 + (0u32 + 4294967295u32));
        assert!((0u32 + 4294967295u32) + 0u32 == 0u32 + (4294967295u32 + 0u32));
        assert!((4294967295u32 + 0u32) + 0u32 == 4294967295u32 + (0u32 + 0u32));
        assert!(
            (695360524u32 + 541461068u32) + 1243412878u32
                == 695360524u32 + (541461068u32 + 1243412878u32)
        );
        assert!(
            (1040936472u32 + 436229194u32) + 1299087906u32
                == 1040936472u32 + (436229194u32 + 1299087906u32)
        );
        assert!(
            (1905637959u32 + 1635700686u32) + 52099766u32
                == 1905637959u32 + (1635700686u32 + 52099766u32)
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`u32::checked_add`]
pub fn core_ops_add_u32_checked_add() {
    eprintln!(r#"Testing "Properties for [`u32::checked_add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing checked addition
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == Some(u32::down(x.up() + y.up()))`
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
        assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
        assert!(3343354073u32.checked_add(921320851u32) == Some(4264674924u32));
        assert!(1558342951u32.checked_add(1524221006u32) == Some(3082563957u32));
        assert!(2610301521u32.checked_add(872052952u32) == Some(3482354473u32));
        assert!(2894449244u32.checked_add(1334793935u32) == Some(4229243179u32));
    }
    eprint!("... ");
    // ## None when overflowing
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `x.up() + y.up() > u32::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == None`
    {
        assert!(4294967295u32.checked_add(4294967295u32) == None);
        assert!(1910421920u32.checked_add(2488900762u32) == None);
        assert!(3239641940u32.checked_add(2290008232u32) == None);
        assert!(1742799190u32.checked_add(3685449103u32) == None);
        assert!(1916948946u32.checked_add(4142886256u32) == None);
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u32, y : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
    {
        assert!(0u32.checked_add(0u32) == 0u32.checked_add(0u32));
        assert!(0u32.checked_add(4294967295u32) == 4294967295u32.checked_add(0u32));
        assert!(4294967295u32.checked_add(0u32) == 0u32.checked_add(4294967295u32));
        assert!(
            4294967295u32.checked_add(4294967295u32) == 4294967295u32.checked_add(4294967295u32)
        );
        assert!(
            1582309774u32.checked_add(2139154386u32) == 2139154386u32.checked_add(1582309774u32)
        );
        assert!(108776469u32.checked_add(311134778u32) == 311134778u32.checked_add(108776469u32));
        assert!(
            3812242969u32.checked_add(2312843734u32) == 2312843734u32.checked_add(3812242969u32)
        );
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(0u32) == Some(x)`
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
        assert!(1798486200u32.checked_add(0u32) == Some(1798486200u32));
        assert!(3212143869u32.checked_add(0u32) == Some(3212143869u32));
        assert!(90177170u32.checked_add(0u32) == Some(90177170u32));
        assert!(1021870448u32.checked_add(0u32) == Some(1021870448u32));
        assert!(285052149u32.checked_add(0u32) == Some(285052149u32));
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `0u32.checked_add(x) == Some(x)`
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
        assert!(0u32.checked_add(1491211847u32) == Some(1491211847u32));
        assert!(0u32.checked_add(4010030473u32) == Some(4010030473u32));
        assert!(0u32.checked_add(4034975175u32) == Some(4034975175u32));
        assert!(0u32.checked_add(2503021573u32) == Some(2503021573u32));
        assert!(0u32.checked_add(4248612815u32) == Some(4248612815u32));
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u32, y : u32, z : u32`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
    //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
    {
        assert!(
            0u32.checked_add(0u32).and_then(|iv| iv.checked_add(0u32))
                == 0u32.checked_add(0u32).and_then(|iv| 0u32.checked_add(iv))
        );
        assert!(
            0u32.checked_add(0u32)
                .and_then(|iv| iv.checked_add(4294967295u32))
                == 0u32
                    .checked_add(4294967295u32)
                    .and_then(|iv| 0u32.checked_add(iv))
        );
        assert!(
            0u32.checked_add(4294967295u32)
                .and_then(|iv| iv.checked_add(0u32))
                == 4294967295u32
                    .checked_add(0u32)
                    .and_then(|iv| 0u32.checked_add(iv))
        );
        assert!(
            0u32.checked_add(4294967295u32)
                .and_then(|iv| iv.checked_add(4294967295u32))
                == 4294967295u32
                    .checked_add(4294967295u32)
                    .and_then(|iv| 0u32.checked_add(iv))
        );
        assert!(
            4294967295u32
                .checked_add(0u32)
                .and_then(|iv| iv.checked_add(0u32))
                == 0u32
                    .checked_add(0u32)
                    .and_then(|iv| 4294967295u32.checked_add(iv))
        );
        assert!(
            4294967295u32
                .checked_add(0u32)
                .and_then(|iv| iv.checked_add(4294967295u32))
                == 0u32
                    .checked_add(4294967295u32)
                    .and_then(|iv| 4294967295u32.checked_add(iv))
        );
        assert!(
            4294967295u32
                .checked_add(4294967295u32)
                .and_then(|iv| iv.checked_add(0u32))
                == 4294967295u32
                    .checked_add(0u32)
                    .and_then(|iv| 4294967295u32.checked_add(iv))
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u64>::add`]
pub fn core_ops_add_u64_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<u64>::add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`
    // __Postcondition:__ `x + y == u64::down(x.up() + y.up())`
    {
        assert!(0u64 + 0u64 == 0u64);
        assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64);
        assert!(18446744073709551615u64 + 0u64 == 18446744073709551615u64);
        assert!(3299860708930978114u64 + 12692323002328864656u64 == 15992183711259842770u64);
        assert!(10144666043812578268u64 + 7432390731954471556u64 == 17577056775767049824u64);
        assert!(10846750238370482983u64 + 7019965563074851756u64 == 17866715801445334739u64);
        assert!(10992915065784168078u64 + 5238834649044861382u64 == 16231749714829029460u64);
    }
    eprint!("... ");
    // ## Panics when overflowing
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `x.up() + y.up() > u64::MAX.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
        assert!(panics!(15923628214049865041u64 + 4311773014773427905u64));
        assert!(panics!(13007882192139343136u64 + 5732437532938114461u64));
        assert!(panics!(17387321424306618021u64 + 16828153944607826775u64));
        assert!(panics!(13397581986533971019u64 + 8404817661524278422u64));
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`
    // __Postcondition:__ `x + y == y + x`
    {
        assert!(0u64 + 0u64 == 0u64 + 0u64);
        assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64 + 0u64);
        assert!(18446744073709551615u64 + 0u64 == 0u64 + 18446744073709551615u64);
        assert!(
            7698662027741304734u64 + 10391803119832643534u64
                == 10391803119832643534u64 + 7698662027741304734u64
        );
        assert!(
            1072442524335209033u64 + 12496007397455891270u64
                == 12496007397455891270u64 + 1072442524335209033u64
        );
        assert!(
            7950895310044927605u64 + 1918709978275983754u64
                == 1918709978275983754u64 + 7950895310044927605u64
        );
        assert!(
            2812983493656727798u64 + 5007012917011015880u64
                == 5007012917011015880u64 + 2812983493656727798u64
        );
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `x + 0 == x`
    {
        assert!(0u64 + 0 == 0u64);
        assert!(18446744073709551615u64 + 0 == 18446744073709551615u64);
        assert!(495857742072067951u64 + 0 == 495857742072067951u64);
        assert!(12809229157526760819u64 + 0 == 12809229157526760819u64);
        assert!(12277014783435348770u64 + 0 == 12277014783435348770u64);
        assert!(17657925416332202514u64 + 0 == 17657925416332202514u64);
        assert!(4913244799263321451u64 + 0 == 4913244799263321451u64);
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `0 + x == x`
    {
        assert!(0 + 0u64 == 0u64);
        assert!(0 + 18446744073709551615u64 == 18446744073709551615u64);
        assert!(0 + 8252462714173658888u64 == 8252462714173658888u64);
        assert!(0 + 11902420715959451291u64 == 11902420715959451291u64);
        assert!(0 + 4420086226218267278u64 == 4420086226218267278u64);
        assert!(0 + 698624162455574856u64 == 698624162455574856u64);
        assert!(0 + 14103583997526644828u64 == 14103583997526644828u64);
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u64, y : u64, z : u64`
    // __Precondition:__ `x.up() + y.up() + z.up() <= u64::MAX.up()`
    // __Postcondition:__ `(x + y) + z == x + (y + z)`
    {
        assert!((0u64 + 0u64) + 0u64 == 0u64 + (0u64 + 0u64));
        assert!((0u64 + 0u64) + 18446744073709551615u64 == 0u64 + (0u64 + 18446744073709551615u64));
        assert!((0u64 + 18446744073709551615u64) + 0u64 == 0u64 + (18446744073709551615u64 + 0u64));
        assert!((18446744073709551615u64 + 0u64) + 0u64 == 18446744073709551615u64 + (0u64 + 0u64));
        assert!(
            (7790594995114077096u64 + 4017388274301022720u64) + 1241573250800695577u64
                == 7790594995114077096u64 + (4017388274301022720u64 + 1241573250800695577u64)
        );
        assert!(
            (2429155475165511035u64 + 6137339344098257127u64) + 3338966807965607783u64
                == 2429155475165511035u64 + (6137339344098257127u64 + 3338966807965607783u64)
        );
        assert!(
            (6501488223978273510u64 + 2151436537100458863u64) + 6189493797324778780u64
                == 6501488223978273510u64 + (2151436537100458863u64 + 6189493797324778780u64)
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`u64::checked_add`]
pub fn core_ops_add_u64_checked_add() {
    eprintln!(r#"Testing "Properties for [`u64::checked_add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing checked addition
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == Some(u64::down(x.up() + y.up()))`
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
        assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
        assert!(
            2045216927147032824u64.checked_add(15344090651125280012u64)
                == Some(17389307578272312836u64)
        );
        assert!(
            3393950214562503539u64.checked_add(2567224232593280203u64)
                == Some(5961174447155783742u64)
        );
        assert!(
            70734489500176922u64.checked_add(12190634993922676139u64)
                == Some(12261369483422853061u64)
        );
        assert!(
            11434011010319675647u64.checked_add(3487256650707824099u64)
                == Some(14921267661027499746u64)
        );
    }
    eprint!("... ");
    // ## None when overflowing
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `x.up() + y.up() > u64::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == None`
    {
        assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
        assert!(14789270489629478186u64.checked_add(4384363171162322110u64) == None);
        assert!(17365288381133392010u64.checked_add(9090780507339722299u64) == None);
        assert!(13541054706099738120u64.checked_add(17046991290159799455u64) == None);
        assert!(16476132066803748047u64.checked_add(2081943244017026025u64) == None);
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u64, y : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
    {
        assert!(0u64.checked_add(0u64) == 0u64.checked_add(0u64));
        assert!(
            0u64.checked_add(18446744073709551615u64) == 18446744073709551615u64.checked_add(0u64)
        );
        assert!(
            18446744073709551615u64.checked_add(0u64) == 0u64.checked_add(18446744073709551615u64)
        );
        assert!(
            18446744073709551615u64.checked_add(18446744073709551615u64)
                == 18446744073709551615u64.checked_add(18446744073709551615u64)
        );
        assert!(
            13373667287347318054u64.checked_add(9355218010147905065u64)
                == 9355218010147905065u64.checked_add(13373667287347318054u64)
        );
        assert!(
            17956820294440337452u64.checked_add(7781346259158998315u64)
                == 7781346259158998315u64.checked_add(17956820294440337452u64)
        );
        assert!(
            8957271648020383305u64.checked_add(6973823902216697545u64)
                == 6973823902216697545u64.checked_add(8957271648020383305u64)
        );
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(0u64) == Some(x)`
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
        assert!(50873863113566978u64.checked_add(0u64) == Some(50873863113566978u64));
        assert!(14079590628493976630u64.checked_add(0u64) == Some(14079590628493976630u64));
        assert!(12804145556100127335u64.checked_add(0u64) == Some(12804145556100127335u64));
        assert!(6028742599419358975u64.checked_add(0u64) == Some(6028742599419358975u64));
        assert!(6604824945040226815u64.checked_add(0u64) == Some(6604824945040226815u64));
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `0u64.checked_add(x) == Some(x)`
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
        assert!(0u64.checked_add(445522585524504974u64) == Some(445522585524504974u64));
        assert!(0u64.checked_add(6311800884360669771u64) == Some(6311800884360669771u64));
        assert!(0u64.checked_add(3211309275946107738u64) == Some(3211309275946107738u64));
        assert!(0u64.checked_add(9907564542307625074u64) == Some(9907564542307625074u64));
        assert!(0u64.checked_add(5057699681057578800u64) == Some(5057699681057578800u64));
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u64, y : u64, z : u64`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
    //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
    {
        assert!(
            0u64.checked_add(0u64).and_then(|iv| iv.checked_add(0u64))
                == 0u64.checked_add(0u64).and_then(|iv| 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(0u64)
                .and_then(|iv| iv.checked_add(18446744073709551615u64))
                == 0u64
                    .checked_add(18446744073709551615u64)
                    .and_then(|iv| 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(18446744073709551615u64)
                .and_then(|iv| iv.checked_add(0u64))
                == 18446744073709551615u64
                    .checked_add(0u64)
                    .and_then(|iv| 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(18446744073709551615u64)
                .and_then(|iv| iv.checked_add(18446744073709551615u64))
                == 18446744073709551615u64
                    .checked_add(18446744073709551615u64)
                    .and_then(|iv| 0u64.checked_add(iv))
        );
        assert!(
            18446744073709551615u64
                .checked_add(0u64)
                .and_then(|iv| iv.checked_add(0u64))
                == 0u64
                    .checked_add(0u64)
                    .and_then(|iv| 18446744073709551615u64.checked_add(iv))
        );
        assert!(
            18446744073709551615u64
                .checked_add(0u64)
                .and_then(|iv| iv.checked_add(18446744073709551615u64))
                == 0u64
                    .checked_add(18446744073709551615u64)
                    .and_then(|iv| 18446744073709551615u64.checked_add(iv))
        );
        assert!(
            18446744073709551615u64
                .checked_add(18446744073709551615u64)
                .and_then(|iv| iv.checked_add(0u64))
                == 18446744073709551615u64
                    .checked_add(0u64)
                    .and_then(|iv| 18446744073709551615u64.checked_add(iv))
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u128>::add`]
pub fn core_ops_add_u128_add() {
    eprintln!(r#"Testing "Properties for [`core::ops::Add::<u128>::add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing addition
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`
    // __Postcondition:__ `x + y == u128::down(x.up() + y.up())`
    {
        assert!(0u128 + 0u128 == 0u128);
        assert!(
            0u128 + 340282366920938463463374607431768211455u128
                == 340282366920938463463374607431768211455u128
        );
        assert!(
            340282366920938463463374607431768211455u128 + 0u128
                == 340282366920938463463374607431768211455u128
        );
        assert!(
            96805561642699408289715414103947277480u128
                + 209185594251001669608504857623629926794u128
                == 305991155893701077898220271727577204274u128
        );
        assert!(
            10249101172100368802231193698826589865u128 + 37254652320344395712139742359118535859u128
                == 47503753492444764514370936057945125724u128
        );
        assert!(
            56351921073761587756265859320921138044u128 + 26017947435613524678510857331068265582u128
                == 82369868509375112434776716651989403626u128
        );
        assert!(
            5973189758581688234519759857640347023u128 + 322300171254737902970832375783165193413u128
                == 328273361013319591205352135640805540436u128
        );
    }
    eprint!("... ");
    // ## Panics when overflowing
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `x.up() + y.up() > u128::MAX.up()`
    // __Postcondition:__ `panics!(x + y)`
    {
        assert!(panics!(
            340282366920938463463374607431768211455u128
                + 340282366920938463463374607431768211455u128
        ));
        assert!(panics!(
            266696225175966572641575532798243562062u128
                + 269906937088700735232540706377232033685u128
        ));
        assert!(panics!(
            276012483884815600785285334613752460840u128
                + 101432446777267073669929498997700156714u128
        ));
        assert!(panics!(
            82837484715253088512591153405852548978u128
                + 337173891864812777565483154585202661851u128
        ));
        assert!(panics!(
            149156549730868366582709345154220385781u128
                + 217746358360438501760434538349585838310u128
        ));
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`
    // __Postcondition:__ `x + y == y + x`
    {
        assert!(0u128 + 0u128 == 0u128 + 0u128);
        assert!(
            0u128 + 340282366920938463463374607431768211455u128
                == 340282366920938463463374607431768211455u128 + 0u128
        );
        assert!(
            340282366920938463463374607431768211455u128 + 0u128
                == 0u128 + 340282366920938463463374607431768211455u128
        );
        assert!(
            31340014181806716894909855390823207549u128
                + 182507907077536741717087783321019549074u128
                == 182507907077536741717087783321019549074u128
                    + 31340014181806716894909855390823207549u128
        );
        assert!(
            181947992692513635965525928487452296159u128
                + 45698065207596151875926227268319870826u128
                == 45698065207596151875926227268319870826u128
                    + 181947992692513635965525928487452296159u128
        );
        assert!(
            165968195512674446731363910045051208768u128
                + 104502807287180588771950265924872320209u128
                == 104502807287180588771950265924872320209u128
                    + 165968195512674446731363910045051208768u128
        );
        assert!(
            163908985726565341089904241623386341211u128
                + 96772492889461788142153123479594954823u128
                == 96772492889461788142153123479594954823u128
                    + 163908985726565341089904241623386341211u128
        );
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `x + 0 == x`
    {
        assert!(0u128 + 0 == 0u128);
        assert!(
            340282366920938463463374607431768211455u128 + 0
                == 340282366920938463463374607431768211455u128
        );
        assert!(
            132434992799638416028634639665389523200u128 + 0
                == 132434992799638416028634639665389523200u128
        );
        assert!(
            58668134581679372051128504447721892304u128 + 0
                == 58668134581679372051128504447721892304u128
        );
        assert!(
            265344245021174327168600246646510213103u128 + 0
                == 265344245021174327168600246646510213103u128
        );
        assert!(
            18463907788069621370825865364876114018u128 + 0
                == 18463907788069621370825865364876114018u128
        );
        assert!(
            338209333978723804547030015115353026631u128 + 0
                == 338209333978723804547030015115353026631u128
        );
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `0 + x == x`
    {
        assert!(0 + 0u128 == 0u128);
        assert!(
            0 + 340282366920938463463374607431768211455u128
                == 340282366920938463463374607431768211455u128
        );
        assert!(
            0 + 326499171661444507830007649172204345410u128
                == 326499171661444507830007649172204345410u128
        );
        assert!(
            0 + 264864476148998129497084369356736937999u128
                == 264864476148998129497084369356736937999u128
        );
        assert!(
            0 + 253197617558911792939632490207905358325u128
                == 253197617558911792939632490207905358325u128
        );
        assert!(
            0 + 90915991978542813807649870823018046463u128
                == 90915991978542813807649870823018046463u128
        );
        assert!(
            0 + 111142855997773508804096052093293432502u128
                == 111142855997773508804096052093293432502u128
        );
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u128, y : u128, z : u128`
    // __Precondition:__ `x.up() + y.up() + z.up() <= u128::MAX.up()`
    // __Postcondition:__ `(x + y) + z == x + (y + z)`
    {
        assert!((0u128 + 0u128) + 0u128 == 0u128 + (0u128 + 0u128));
        assert!(
            (0u128 + 0u128) + 340282366920938463463374607431768211455u128
                == 0u128 + (0u128 + 340282366920938463463374607431768211455u128)
        );
        assert!(
            (0u128 + 340282366920938463463374607431768211455u128) + 0u128
                == 0u128 + (340282366920938463463374607431768211455u128 + 0u128)
        );
        assert!(
            (340282366920938463463374607431768211455u128 + 0u128) + 0u128
                == 340282366920938463463374607431768211455u128 + (0u128 + 0u128)
        );
        assert!(
            (67586145551107569718418202862124436227u128
                + 77231061255720903586666148880466240959u128)
                + 43683148548834333033073082548069690258u128
                == 67586145551107569718418202862124436227u128
                    + (77231061255720903586666148880466240959u128
                        + 43683148548834333033073082548069690258u128)
        );
        assert!(
            (50293008572074138310305159037401538315u128
                + 10696853186990730618357021849841588116u128)
                + 203980906056771636772388098891075467140u128
                == 50293008572074138310305159037401538315u128
                    + (10696853186990730618357021849841588116u128
                        + 203980906056771636772388098891075467140u128)
        );
        assert!(
            (182343174819687543135722292038614752780u128
                + 61319589385543974435466633831971999877u128)
                + 80548462345456071279691080693680597382u128
                == 182343174819687543135722292038614752780u128
                    + (61319589385543974435466633831971999877u128
                        + 80548462345456071279691080693680597382u128)
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
/// Properties for [`u128::checked_add`]
pub fn core_ops_add_u128_checked_add() {
    eprintln!(r#"Testing "Properties for [`u128::checked_add`]"... (6 contracts)"#);
    eprint!("  ");
    // ## Semantics of non-overflowing checked addition
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == Some(u128::down(x.up() + y.up()))`
    {
        assert!(0u128.checked_add(0u128) == Some(0u128));
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128)
                == Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128)
                == Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            111452244244825378868767612544763568365u128
                .checked_add(48856776945171172180184403901070498436u128)
                == Some(160309021189996551048952016445834066801u128)
        );
        assert!(
            231160723998664897308575346003338943233u128
                .checked_add(109001132373474805727731620937587492638u128)
                == Some(340161856372139703036306966940926435871u128)
        );
        assert!(
            31383275444616837248916341993526629888u128
                .checked_add(81384704576012473991906573957536666628u128)
                == Some(112767980020629311240822915951063296516u128)
        );
        assert!(
            24933477860649237301543104519802429015u128
                .checked_add(106827257148349188591215839658097717716u128)
                == Some(131760735008998425892758944177900146731u128)
        );
    }
    eprint!("... ");
    // ## None when overflowing
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `x.up() + y.up() > u128::MAX.up()`
    // __Postcondition:__ `x.checked_add(y) == None`
    {
        assert!(
            340282366920938463463374607431768211455u128
                .checked_add(340282366920938463463374607431768211455u128)
                == None
        );
        assert!(
            174414699033491351519140306682846088470u128
                .checked_add(277150305078209580065340664821951883276u128)
                == None
        );
        assert!(
            109239316794542850172656706309614748894u128
                .checked_add(232661073002213780150136623534797476277u128)
                == None
        );
        assert!(
            286365309802807092089821029391567575611u128
                .checked_add(127642791699691534971705408409884012639u128)
                == None
        );
        assert!(
            274096924952766639664339319596869208399u128
                .checked_add(267570563441714019921219110758729067816u128)
                == None
        );
    }
    eprint!("... ");
    // ## Commutativity
    // __Inputs:__ `x : u128, y : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
    {
        assert!(0u128.checked_add(0u128) == 0u128.checked_add(0u128));
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128)
                == 340282366920938463463374607431768211455u128.checked_add(0u128)
        );
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128)
                == 0u128.checked_add(340282366920938463463374607431768211455u128)
        );
        assert!(
            340282366920938463463374607431768211455u128
                .checked_add(340282366920938463463374607431768211455u128)
                == 340282366920938463463374607431768211455u128
                    .checked_add(340282366920938463463374607431768211455u128)
        );
        assert!(
            72746287508412695877219129351878740684u128
                .checked_add(200660078110688069883577712548080316239u128)
                == 200660078110688069883577712548080316239u128
                    .checked_add(72746287508412695877219129351878740684u128)
        );
        assert!(
            290736817028493669671095071184558650929u128
                .checked_add(243433186428784726815406230763806361742u128)
                == 243433186428784726815406230763806361742u128
                    .checked_add(290736817028493669671095071184558650929u128)
        );
        assert!(
            159193774535316038065624599533087053380u128
                .checked_add(180576800017919689853301489279097178601u128)
                == 180576800017919689853301489279097178601u128
                    .checked_add(159193774535316038065624599533087053380u128)
        );
    }
    eprint!("... ");
    // ## Left identity
    // __Inputs:__ `x : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(0u128) == Some(x)`
    {
        assert!(0u128.checked_add(0u128) == Some(0u128));
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128)
                == Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            61987719894141286443632177797911991829u128.checked_add(0u128)
                == Some(61987719894141286443632177797911991829u128)
        );
        assert!(
            54725086483903769532843984280907089254u128.checked_add(0u128)
                == Some(54725086483903769532843984280907089254u128)
        );
        assert!(
            249231190463534155424857940225459341282u128.checked_add(0u128)
                == Some(249231190463534155424857940225459341282u128)
        );
        assert!(
            210774036543381441125455998883284265369u128.checked_add(0u128)
                == Some(210774036543381441125455998883284265369u128)
        );
        assert!(
            184151495677124129448150497547260690764u128.checked_add(0u128)
                == Some(184151495677124129448150497547260690764u128)
        );
    }
    eprint!("... ");
    // ## Right identity
    // __Inputs:__ `x : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `0u128.checked_add(x) == Some(x)`
    {
        assert!(0u128.checked_add(0u128) == Some(0u128));
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128)
                == Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            0u128.checked_add(36082309401984328628637870493145027364u128)
                == Some(36082309401984328628637870493145027364u128)
        );
        assert!(
            0u128.checked_add(86384612236222460307304577762574758544u128)
                == Some(86384612236222460307304577762574758544u128)
        );
        assert!(
            0u128.checked_add(9041172171189308175158887031582054736u128)
                == Some(9041172171189308175158887031582054736u128)
        );
        assert!(
            0u128.checked_add(329624033775022326148096181444851568845u128)
                == Some(329624033775022326148096181444851568845u128)
        );
        assert!(
            0u128.checked_add(230907246437973186239904494356874508500u128)
                == Some(230907246437973186239904494356874508500u128)
        );
    }
    eprint!("... ");
    // ## Associativity
    // __Inputs:__ `x : u128, y : u128, z : u128`
    // __Precondition:__ `true`
    // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
    //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
    {
        assert!(
            0u128
                .checked_add(0u128)
                .and_then(|iv| iv.checked_add(0u128))
                == 0u128
                    .checked_add(0u128)
                    .and_then(|iv| 0u128.checked_add(iv))
        );
        assert!(
            0u128
                .checked_add(0u128)
                .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
                == 0u128
                    .checked_add(340282366920938463463374607431768211455u128)
                    .and_then(|iv| 0u128.checked_add(iv))
        );
        assert!(
            0u128
                .checked_add(340282366920938463463374607431768211455u128)
                .and_then(|iv| iv.checked_add(0u128))
                == 340282366920938463463374607431768211455u128
                    .checked_add(0u128)
                    .and_then(|iv| 0u128.checked_add(iv))
        );
        assert!(
            0u128
                .checked_add(340282366920938463463374607431768211455u128)
                .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
                == 340282366920938463463374607431768211455u128
                    .checked_add(340282366920938463463374607431768211455u128)
                    .and_then(|iv| 0u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128
                .checked_add(0u128)
                .and_then(|iv| iv.checked_add(0u128))
                == 0u128
                    .checked_add(0u128)
                    .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128
                .checked_add(0u128)
                .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
                == 0u128
                    .checked_add(340282366920938463463374607431768211455u128)
                    .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128
                .checked_add(340282366920938463463374607431768211455u128)
                .and_then(|iv| iv.checked_add(0u128))
                == 340282366920938463463374607431768211455u128
                    .checked_add(0u128)
                    .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv))
        );
    }
    eprint!("... ");
    eprintln!("✓\n");
}
fn main() {
    option_is_some();
    option_is_none();
    option_expect();
    option_map();
    option_filter();
    option_flatten();
    option_take();
    option_zip();
    option_unwrap();
    option_as_mut();
    option_as_slice();
    slice_get();
    core_ops_sub_i8_sub();
    core_ops_add_i8_add();
    core_ops_add_u8_add();
    core_ops_add_u8_checked_add();
    core_ops_add_u16_add();
    core_ops_add_u16_checked_add();
    core_ops_add_u32_add();
    core_ops_add_u32_checked_add();
    core_ops_add_u64_add();
    core_ops_add_u64_checked_add();
    core_ops_add_u128_add();
    core_ops_add_u128_checked_add();
}
