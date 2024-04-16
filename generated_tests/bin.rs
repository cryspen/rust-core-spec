//! This module contains 533 tests, organized in functions.
#![allow(arithmetic_overflow)]
use core_spec::lifts::*;
use core_spec::*;

/// Properties for [`Option::is_some`]
pub fn option_is_some(){
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
        Option::< u8 >::None.is_some() == (match Option::< u8 >::None { Some(_) => true,
        None => false, })
    );
    assert!(
        Some(0u8).is_some() == (match Some(0u8) { Some(_) => true, None => false, })
    );
    assert!(
        Some(255u8).is_some() == (match Some(255u8) { Some(_) => true, None => false, })
    );
    assert!(
        Some(101u8).is_some() == (match Some(101u8) { Some(_) => true, None => false, })
    );
    assert!(
        Some(91u8).is_some() == (match Some(91u8) { Some(_) => true, None => false, })
    );
    assert!(
        Some(82u8).is_some() == (match Some(82u8) { Some(_) => true, None => false, })
    );
    assert!(
        Some(26u8).is_some() == (match Some(26u8) { Some(_) => true, None => false, })
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::is_none`]
pub fn option_is_none(){
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
        Option::< u8 >::None.is_none() == (match Option::< u8 >::None { Some(_) => false,
        None => true, })
    );
    assert!(
        Some(0u8).is_none() == (match Some(0u8) { Some(_) => false, None => true, })
    );
    assert!(
        Some(255u8).is_none() == (match Some(255u8) { Some(_) => false, None => true, })
    );
    assert!(
        Some(206u8).is_none() == (match Some(206u8) { Some(_) => false, None => true, })
    );
    assert!(
        Some(174u8).is_none() == (match Some(174u8) { Some(_) => false, None => true, })
    );
    assert!(
        Some(213u8).is_none() == (match Some(213u8) { Some(_) => false, None => true, })
    );
    assert!(
        Some(189u8).is_none() == (match Some(189u8) { Some(_) => false, None => true, })
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::expect`]
pub fn option_expect(){
  eprintln!(r#"Testing "Properties for [`Option::expect`]"... (3 contracts)"#);
  eprint!("  ");
  // ## Unwrapping a [`None`] with `expect` always panic
  // __Inputs:__ `v : Option<T>`  
  // __Precondition:__ `v.is_none()`  
  // __Postcondition:__ `panics!(v.expect("message"))`
  {
    assert!(panics!(Option::< u8 >::None.expect("message")));
  }
  eprint!("... ");
  // ## Unwrapping a [`Some(_)`] with `expect` always succeeds
  // __Inputs:__ `v : Option<T>`  
  // __Precondition:__ `v.is_some()`  
  // __Postcondition:__ `doesn_t_panic!(v.expect("message"))`
  {
    assert!(doesn_t_panic!(Some(0u8).expect("message")));
    assert!(doesn_t_panic!(Some(255u8).expect("message")));
    assert!(doesn_t_panic!(Some(9u8).expect("message")));
    assert!(doesn_t_panic!(Some(105u8).expect("message")));
    assert!(doesn_t_panic!(Some(212u8).expect("message")));
    assert!(doesn_t_panic!(Some(227u8).expect("message")));
    assert!(doesn_t_panic!(Some(128u8).expect("message")));
  }
  eprint!("... ");
  // ## Wrapping a value in a `Some` and unwrapping is identity
  // __Inputs:__ `v : T`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `Some(v).unwrap() == v`
  {
    assert!(Some(0u8).unwrap() == 0u8);
    assert!(Some(255u8).unwrap() == 255u8);
    assert!(Some(200u8).unwrap() == 200u8);
    assert!(Some(9u8).unwrap() == 9u8);
    assert!(Some(160u8).unwrap() == 160u8);
    assert!(Some(3u8).unwrap() == 3u8);
    assert!(Some(190u8).unwrap() == 190u8);
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::map`]
pub fn option_map(){
  eprintln!(r#"Testing "Properties for [`Option::map`]"... (2 contracts)"#);
  eprint!("  ");
  // ## Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`
  // __Inputs:__ `v : T, f : Fn1<T, T>`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `Some(v).map(f) == Some((f)(v))`
  {
    assert!(
        Some(0u8).map((| x : u8 | x.wrapping_add(x))) == Some(((| x : u8 | x
        .wrapping_add(x))) (0u8))
    );
    assert!(
        Some(0u8).map((| x : u8 | x.wrapping_add(0u8))) == Some(((| x : u8 | x
        .wrapping_add(0u8))) (0u8))
    );
    assert!(
        Some(0u8).map((| x : u8 | x.wrapping_add(255u8))) == Some(((| x : u8 | x
        .wrapping_add(255u8))) (0u8))
    );
    assert!(
        Some(255u8).map((| x : u8 | x.wrapping_add(x))) == Some(((| x : u8 | x
        .wrapping_add(x))) (255u8))
    );
    assert!(
        Some(255u8).map((| x : u8 | x.wrapping_add(0u8))) == Some(((| x : u8 | x
        .wrapping_add(0u8))) (255u8))
    );
    assert!(
        Some(255u8).map((| x : u8 | x.wrapping_add(255u8))) == Some(((| x : u8 | x
        .wrapping_add(255u8))) (255u8))
    );
    assert!(Some(151u8).map((| x : u8 | x)) == Some(((| x : u8 | x)) (151u8)));
  }
  eprint!("... ");
  // ## Mapping a `None` is the identity
  // __Inputs:__ `v : Option<T>, f : Fn1<T, T>`  
  // __Precondition:__ `v.is_none()`  
  // __Postcondition:__ `v.map(f) == v`
  {
    assert!(
        Option::< u8 >::None.map((| x : u8 | x.wrapping_add(x))) == Option::< u8 >::None
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::filter`]
pub fn option_filter(){
  eprintln!(r#"Testing "Properties for [`Option::filter`]"... (2 contracts)"#);
  eprint!("  ");
  // ## The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`
  // __Inputs:__ `v : T, f : FnR1<T, bool>`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `Some(v).filter(f).is_some() == f(&v)`
  {
    assert!(
        Some(0u8).filter((| x : & u8 | * x < 0u8)).is_some() == (| x : & u8 | * x < 0u8)
        (& 0u8)
    );
    assert!(
        Some(0u8).filter((| x : & u8 | * x < 255u8)).is_some() == (| x : & u8 | * x <
        255u8) (& 0u8)
    );
    assert!(
        Some(255u8).filter((| x : & u8 | * x < 0u8)).is_some() == (| x : & u8 | * x <
        0u8) (& 255u8)
    );
    assert!(
        Some(255u8).filter((| x : & u8 | * x < 255u8)).is_some() == (| x : & u8 | * x <
        255u8) (& 255u8)
    );
    assert!(
        Some(90u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x > 128)
        (& 90u8)
    );
    assert!(
        Some(243u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x >
        128) (& 243u8)
    );
    assert!(
        Some(178u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x >
        128) (& 178u8)
    );
  }
  eprint!("... ");
  // ## Filtering a `None` is the identity
  // __Inputs:__ `v : Option<T>, f : FnR1<T, bool>`  
  // __Precondition:__ `v.is_none()`  
  // __Postcondition:__ `v.filter(f) == v`
  {
    assert!(
        Option::< u8 >::None.filter((| x : & u8 | * x < 0u8)) == Option::< u8 >::None
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::flatten`]
pub fn option_flatten(){
  eprintln!(r#"Testing "Properties for [`Option::flatten`]"... (2 contracts)"#);
  eprint!("  ");
  // ## Nested `Some`s
  // __Inputs:__ `x : T`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `Some(Some(x)).flatten() == Some(x)`
  {
    assert!(Some(Some(0u8)).flatten() == Some(0u8));
    assert!(Some(Some(255u8)).flatten() == Some(255u8));
    assert!(Some(Some(6u8)).flatten() == Some(6u8));
    assert!(Some(Some(85u8)).flatten() == Some(85u8));
    assert!(Some(Some(246u8)).flatten() == Some(246u8));
    assert!(Some(Some(223u8)).flatten() == Some(223u8));
    assert!(Some(Some(130u8)).flatten() == Some(130u8));
  }
  eprint!("... ");
  // ## Nested or direct `None` flattens to None
  // __Inputs:__ `x : Option<Option<T>>`  
  // __Precondition:__ `x.is_none() || x.unwrap().is_none()`  
  // __Postcondition:__ `x.flatten() == None`
  {
    assert!(Option::< Option < u8 >>::None.flatten() == None);
    assert!(Some(Option::< u8 >::None).flatten() == None);
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::take`]
pub fn option_take(){
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
    assert!(
        { let mut y = Option::< u8 >::None.clone(); y.take() == Option::< u8 >::None && y
        .is_none() }
    );
    assert!({ let mut y = Some(0u8).clone(); y.take() == Some(0u8) && y.is_none() });
    assert!({ let mut y = Some(255u8).clone(); y.take() == Some(255u8) && y.is_none() });
    assert!({ let mut y = Some(110u8).clone(); y.take() == Some(110u8) && y.is_none() });
    assert!({ let mut y = Some(54u8).clone(); y.take() == Some(54u8) && y.is_none() });
    assert!({ let mut y = Some(6u8).clone(); y.take() == Some(6u8) && y.is_none() });
    assert!({ let mut y = Some(203u8).clone(); y.take() == Some(203u8) && y.is_none() });
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::zip`]
pub fn option_zip(){
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
    assert!(Some(40u8).zip(Some(206u8)) == Some((40u8, 206u8)));
    assert!(Some(168u8).zip(Some(21u8)) == Some((168u8, 21u8)));
    assert!(Some(201u8).zip(Some(62u8)) == Some((201u8, 62u8)));
  }
  eprint!("... ");
  // ## Zipping two options when one is a `None` makes `None`
  // __Inputs:__ `x : Option<T>, y : Option<T>`  
  // __Precondition:__ `x.is_none() || y.is_none()`  
  // __Postcondition:__ `x.zip(y).is_none()`
  {
    assert!(Option::< u8 >::None.zip(Option::< u8 >::None).is_none());
    assert!(Option::< u8 >::None.zip(Some(0u8)).is_none());
    assert!(Option::< u8 >::None.zip(Some(255u8)).is_none());
    assert!(Some(0u8).zip(Option::< u8 >::None).is_none());
    assert!(Some(255u8).zip(Option::< u8 >::None).is_none());
    assert!(Option::< u8 >::None.zip(Some(232u8)).is_none());
    assert!(Option::< u8 >::None.zip(Some(100u8)).is_none());
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::unwrap`]
pub fn option_unwrap(){
  eprintln!(r#"Testing "Properties for [`Option::unwrap`]"... (2 contracts)"#);
  eprint!("  ");
  // ## Unwrapping a [`None`] always panic
  // __Inputs:__ `v : Option<T>`  
  // __Precondition:__ `v.is_none()`  
  // __Postcondition:__ `panics!(v.unwrap())`
  {
    assert!(panics!(Option::< u8 >::None.unwrap()));
  }
  eprint!("... ");
  // ## Unwrapping a [`Some(_)`] always succeeds
  // __Inputs:__ `v : Option<T>`  
  // __Precondition:__ `v.is_some()`  
  // __Postcondition:__ `doesn_t_panic!(v.unwrap())`
  {
    assert!(doesn_t_panic!(Some(0u8).unwrap()));
    assert!(doesn_t_panic!(Some(255u8).unwrap()));
    assert!(doesn_t_panic!(Some(28u8).unwrap()));
    assert!(doesn_t_panic!(Some(138u8).unwrap()));
    assert!(doesn_t_panic!(Some(241u8).unwrap()));
    assert!(doesn_t_panic!(Some(41u8).unwrap()));
    assert!(doesn_t_panic!(Some(144u8).unwrap()));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::as_mut`]
pub fn option_as_mut(){
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
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8)); * v_mut
        .as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(38u8).unwrap().clone(), Some(38u8)); *
        v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(5u8).unwrap().clone(), Some(5u8)); * v_mut
        .as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(18u8).unwrap().clone(), Some(18u8)); *
        v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(21u8).unwrap().clone(), Some(21u8)); *
        v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(29u8).unwrap().clone(), Some(29u8)); *
        v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
    assert!(
        { let (v_unwrapped, mut v_mut) = (Some(9u8).unwrap().clone(), Some(9u8)); * v_mut
        .as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Option::as_slice`]
pub fn option_as_slice(){
  eprintln!(r#"Testing "Properties for [`Option::as_slice`]"... (2 contracts)"#);
  eprint!("  ");
  // ## [`None.as_slice()`] should always result in an empty slice
  // __Inputs:__ `v : Option<T>`  
  // __Precondition:__ `v.is_none()`  
  // __Postcondition:__ `{ v.as_slice().is_empty() }`
  {
    assert!({ Option::< u8 >::None.as_slice().is_empty() });
  }
  eprint!("... ");
  // ## [`Some(v).as_slice()`] should always result in a slice containing exactly `v`
  // __Inputs:__ `v : T`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `{ Some(v).as_slice() == [v] }`
  {
    assert!({ Some(0u8).as_slice() == [0u8] });
    assert!({ Some(255u8).as_slice() == [255u8] });
    assert!({ Some(42u8).as_slice() == [42u8] });
    assert!({ Some(114u8).as_slice() == [114u8] });
    assert!({ Some(173u8).as_slice() == [173u8] });
    assert!({ Some(85u8).as_slice() == [85u8] });
    assert!({ Some(102u8).as_slice() == [102u8] });
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`Vec`]
pub fn slice_get(){
  eprintln!(r#"Testing "Properties for [`Vec`]"... (1 contracts)"#);
  eprint!("  ");
  // ## Indexing
  // __Inputs:__ `v : Vec<u8>, i : usize`  
  // __Precondition:__ `v.len() > 0`  
  // __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`
  {
    assert!(vec![0u8] .get(0usize) == Some(& 0u8));
    assert!(vec![0u8] .get(0usize) == Some(& 0u8));
    assert!(vec![255u8] .get(0usize) == Some(& 255u8));
    assert!(vec![255u8] .get(0usize) == Some(& 255u8));
    assert!(vec![247u8] .get(0usize) == Some(& 247u8));
    assert!(vec![119u8, 184u8, 198u8] .get(0usize) == Some(& 119u8));
    assert!(vec![196u8, 196u8] .get(1usize) == Some(& 196u8));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Sub::<i8>::sub`]
pub fn core_ops_sub_i8_sub(){
  eprintln!(r#"Testing "Properties for [`core::ops::Sub::<i8>::sub`]"... (2 contracts)"#);
  eprint!("  ");
  // ## Semantics of non-overflowing subtraction
  // __Inputs:__ `x : i8, y : i8`  
  // __Precondition:__ `x.up() - y.up() < 128.up() && x.up() - y.up() >= -128i32.up()`  
  // __Postcondition:__ `x - y == i8::down(x.up() - y.up())`
  {
    assert!(- 128i8 - - 128i8 == 0i8);
    assert!(- 128i8 - 0i8 == - 128i8);
    assert!(0i8 - 0i8 == 0i8);
    assert!(0i8 - 127i8 == - 127i8);
    assert!(127i8 - 0i8 == 127i8);
    assert!(127i8 - 127i8 == 0i8);
    assert!(- 10i8 - - 79i8 == 69i8);
  }
  eprint!("... ");
  // ## Overflowing subtraction panics
  // __Inputs:__ `x : i8, y : i8`  
  // __Precondition:__ `x.up() - y.up() >= 128.up() || x.up() - y.up() < -128i32.up()`  
  // __Postcondition:__ `panics!(x - y)`
  {
    assert!(panics!(- 128i8 - 127i8));
    assert!(panics!(0i8 - - 128i8));
    assert!(panics!(127i8 - - 128i8));
    assert!(panics!(39i8 - - 112i8));
    assert!(panics!(- 69i8 - 124i8));
    assert!(panics!(119i8 - - 71i8));
    assert!(panics!(101i8 - - 123i8));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<i8>::add`]
pub fn core_ops_add_i8_add(){
  eprintln!(r#"Testing "Properties for [`core::ops::Add::<i8>::add`]"... (2 contracts)"#);
  eprint!("  ");
  // ## Semantics of non-overflowing addition
  // __Inputs:__ `x : i8, y : i8`  
  // __Precondition:__ `x.up() + y.up() < 128.up() && x.up() + y.up() >= -128i32.up()`  
  // __Postcondition:__ `x + y == i8::down(x.up() + y.up())`
  {
    assert!(- 128i8 + 0i8 == - 128i8);
    assert!(- 128i8 + 127i8 == - 1i8);
    assert!(0i8 + - 128i8 == - 128i8);
    assert!(0i8 + 0i8 == 0i8);
    assert!(0i8 + 127i8 == 127i8);
    assert!(127i8 + - 128i8 == - 1i8);
    assert!(127i8 + 0i8 == 127i8);
  }
  eprint!("... ");
  // ## Overflowing addition panics
  // __Inputs:__ `x : i8, y : i8`  
  // __Precondition:__ `x.up() + y.up() >= 128.up() || x.up() + y.up() < -128i32.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(panics!(- 128i8 + - 128i8));
    assert!(panics!(127i8 + 127i8));
    assert!(panics!(83i8 + 58i8));
    assert!(panics!(- 78i8 + - 100i8));
    assert!(panics!(- 48i8 + - 82i8));
    assert!(panics!(- 109i8 + - 65i8));
    assert!(panics!(84i8 + 53i8));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u8>::add`]
pub fn core_ops_add_u8_add(){
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
    assert!(36u8 + 38u8 == 74u8);
    assert!(150u8 + 56u8 == 206u8);
    assert!(17u8 + 171u8 == 188u8);
    assert!(102u8 + 13u8 == 115u8);
  }
  eprint!("... ");
  // ## Panics when overflowing
  // __Inputs:__ `x : u8, y : u8`  
  // __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(panics!(255u8 + 255u8));
    assert!(panics!(195u8 + 251u8));
    assert!(panics!(199u8 + 71u8));
    assert!(panics!(212u8 + 237u8));
    assert!(panics!(61u8 + 253u8));
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
    assert!(196u8 + 44u8 == 44u8 + 196u8);
    assert!(3u8 + 163u8 == 163u8 + 3u8);
    assert!(14u8 + 233u8 == 233u8 + 14u8);
    assert!(145u8 + 5u8 == 5u8 + 145u8);
  }
  eprint!("... ");
  // ## Left identity
  // __Inputs:__ `x : u8`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x + 0 == x`
  {
    assert!(0u8 + 0 == 0u8);
    assert!(255u8 + 0 == 255u8);
    assert!(6u8 + 0 == 6u8);
    assert!(78u8 + 0 == 78u8);
    assert!(148u8 + 0 == 148u8);
    assert!(83u8 + 0 == 83u8);
    assert!(63u8 + 0 == 63u8);
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u8`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0 + x == x`
  {
    assert!(0 + 0u8 == 0u8);
    assert!(0 + 255u8 == 255u8);
    assert!(0 + 220u8 == 220u8);
    assert!(0 + 187u8 == 187u8);
    assert!(0 + 85u8 == 85u8);
    assert!(0 + 60u8 == 60u8);
    assert!(0 + 227u8 == 227u8);
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
    assert!((10u8 + 135u8) + 103u8 == 10u8 + (135u8 + 103u8));
    assert!((21u8 + 93u8) + 102u8 == 21u8 + (93u8 + 102u8));
    assert!((189u8 + 18u8) + 7u8 == 189u8 + (18u8 + 7u8));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`u8::checked_add`]
pub fn core_ops_add_u8_checked_add(){
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
    assert!(41u8.checked_add(165u8) == Some(206u8));
    assert!(28u8.checked_add(45u8) == Some(73u8));
    assert!(30u8.checked_add(94u8) == Some(124u8));
    assert!(10u8.checked_add(230u8) == Some(240u8));
  }
  eprint!("... ");
  // ## None when overflowing
  // __Inputs:__ `x : u8, y : u8`  
  // __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
  // __Postcondition:__ `x.checked_add(y) == None`
  {
    assert!(255u8.checked_add(255u8) == None);
    assert!(132u8.checked_add(138u8) == None);
    assert!(39u8.checked_add(237u8) == None);
    assert!(248u8.checked_add(91u8) == None);
    assert!(182u8.checked_add(133u8) == None);
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
    assert!(223u8.checked_add(150u8) == 150u8.checked_add(223u8));
    assert!(172u8.checked_add(201u8) == 201u8.checked_add(172u8));
    assert!(93u8.checked_add(169u8) == 169u8.checked_add(93u8));
  }
  eprint!("... ");
  // ## Left identity
  // __Inputs:__ `x : u8`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(0u8) == Some(x)`
  {
    assert!(0u8.checked_add(0u8) == Some(0u8));
    assert!(255u8.checked_add(0u8) == Some(255u8));
    assert!(6u8.checked_add(0u8) == Some(6u8));
    assert!(245u8.checked_add(0u8) == Some(245u8));
    assert!(56u8.checked_add(0u8) == Some(56u8));
    assert!(187u8.checked_add(0u8) == Some(187u8));
    assert!(43u8.checked_add(0u8) == Some(43u8));
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u8`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0u8.checked_add(x) == Some(x)`
  {
    assert!(0u8.checked_add(0u8) == Some(0u8));
    assert!(0u8.checked_add(255u8) == Some(255u8));
    assert!(0u8.checked_add(205u8) == Some(205u8));
    assert!(0u8.checked_add(178u8) == Some(178u8));
    assert!(0u8.checked_add(106u8) == Some(106u8));
    assert!(0u8.checked_add(168u8) == Some(168u8));
    assert!(0u8.checked_add(166u8) == Some(166u8));
  }
  eprint!("... ");
  // ## Associativity
  // __Inputs:__ `x : u8, y : u8, z : u8`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
  //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
  {
    assert!(
        0u8.checked_add(0u8).and_then(| iv | iv.checked_add(0u8)) == 0u8.checked_add(0u8)
        .and_then(| iv | 0u8.checked_add(iv))
    );
    assert!(
        0u8.checked_add(0u8).and_then(| iv | iv.checked_add(255u8)) == 0u8
        .checked_add(255u8).and_then(| iv | 0u8.checked_add(iv))
    );
    assert!(
        0u8.checked_add(255u8).and_then(| iv | iv.checked_add(0u8)) == 255u8
        .checked_add(0u8).and_then(| iv | 0u8.checked_add(iv))
    );
    assert!(
        0u8.checked_add(255u8).and_then(| iv | iv.checked_add(255u8)) == 255u8
        .checked_add(255u8).and_then(| iv | 0u8.checked_add(iv))
    );
    assert!(
        255u8.checked_add(0u8).and_then(| iv | iv.checked_add(0u8)) == 0u8
        .checked_add(0u8).and_then(| iv | 255u8.checked_add(iv))
    );
    assert!(
        255u8.checked_add(0u8).and_then(| iv | iv.checked_add(255u8)) == 0u8
        .checked_add(255u8).and_then(| iv | 255u8.checked_add(iv))
    );
    assert!(
        255u8.checked_add(255u8).and_then(| iv | iv.checked_add(0u8)) == 255u8
        .checked_add(0u8).and_then(| iv | 255u8.checked_add(iv))
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u16>::add`]
pub fn core_ops_add_u16_add(){
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
    assert!(4657u16 + 57252u16 == 61909u16);
    assert!(7938u16 + 6618u16 == 14556u16);
    assert!(8220u16 + 26649u16 == 34869u16);
    assert!(10159u16 + 7144u16 == 17303u16);
  }
  eprint!("... ");
  // ## Panics when overflowing
  // __Inputs:__ `x : u16, y : u16`  
  // __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(panics!(65535u16 + 65535u16));
    assert!(panics!(31015u16 + 63879u16));
    assert!(panics!(65430u16 + 33748u16));
    assert!(panics!(29937u16 + 61359u16));
    assert!(panics!(55869u16 + 27303u16));
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
    assert!(41749u16 + 23482u16 == 23482u16 + 41749u16);
    assert!(34217u16 + 17957u16 == 17957u16 + 34217u16);
    assert!(21445u16 + 1395u16 == 1395u16 + 21445u16);
    assert!(18825u16 + 7416u16 == 7416u16 + 18825u16);
  }
  eprint!("... ");
  // ## Left identity
  // __Inputs:__ `x : u16`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x + 0 == x`
  {
    assert!(0u16 + 0 == 0u16);
    assert!(65535u16 + 0 == 65535u16);
    assert!(29669u16 + 0 == 29669u16);
    assert!(17625u16 + 0 == 17625u16);
    assert!(60111u16 + 0 == 60111u16);
    assert!(45438u16 + 0 == 45438u16);
    assert!(14306u16 + 0 == 14306u16);
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u16`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0 + x == x`
  {
    assert!(0 + 0u16 == 0u16);
    assert!(0 + 65535u16 == 65535u16);
    assert!(0 + 30615u16 == 30615u16);
    assert!(0 + 14097u16 == 14097u16);
    assert!(0 + 2981u16 == 2981u16);
    assert!(0 + 32944u16 == 32944u16);
    assert!(0 + 29872u16 == 29872u16);
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
    assert!((18429u16 + 7867u16) + 14472u16 == 18429u16 + (7867u16 + 14472u16));
    assert!((12273u16 + 16461u16) + 1409u16 == 12273u16 + (16461u16 + 1409u16));
    assert!((15275u16 + 19048u16) + 30714u16 == 15275u16 + (19048u16 + 30714u16));
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`u16::checked_add`]
pub fn core_ops_add_u16_checked_add(){
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
    assert!(18988u16.checked_add(23929u16) == Some(42917u16));
    assert!(21335u16.checked_add(43150u16) == Some(64485u16));
    assert!(10246u16.checked_add(9324u16) == Some(19570u16));
    assert!(13779u16.checked_add(49779u16) == Some(63558u16));
  }
  eprint!("... ");
  // ## None when overflowing
  // __Inputs:__ `x : u16, y : u16`  
  // __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
  // __Postcondition:__ `x.checked_add(y) == None`
  {
    assert!(65535u16.checked_add(65535u16) == None);
    assert!(39237u16.checked_add(56870u16) == None);
    assert!(63011u16.checked_add(6738u16) == None);
    assert!(34385u16.checked_add(45008u16) == None);
    assert!(42332u16.checked_add(44677u16) == None);
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
    assert!(56243u16.checked_add(27519u16) == 27519u16.checked_add(56243u16));
    assert!(62069u16.checked_add(56952u16) == 56952u16.checked_add(62069u16));
    assert!(59345u16.checked_add(49146u16) == 49146u16.checked_add(59345u16));
  }
  eprint!("... ");
  // ## Left identity
  // __Inputs:__ `x : u16`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(0u16) == Some(x)`
  {
    assert!(0u16.checked_add(0u16) == Some(0u16));
    assert!(65535u16.checked_add(0u16) == Some(65535u16));
    assert!(64512u16.checked_add(0u16) == Some(64512u16));
    assert!(24120u16.checked_add(0u16) == Some(24120u16));
    assert!(26997u16.checked_add(0u16) == Some(26997u16));
    assert!(35576u16.checked_add(0u16) == Some(35576u16));
    assert!(44474u16.checked_add(0u16) == Some(44474u16));
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u16`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0u16.checked_add(x) == Some(x)`
  {
    assert!(0u16.checked_add(0u16) == Some(0u16));
    assert!(0u16.checked_add(65535u16) == Some(65535u16));
    assert!(0u16.checked_add(51651u16) == Some(51651u16));
    assert!(0u16.checked_add(64244u16) == Some(64244u16));
    assert!(0u16.checked_add(13505u16) == Some(13505u16));
    assert!(0u16.checked_add(28269u16) == Some(28269u16));
    assert!(0u16.checked_add(13258u16) == Some(13258u16));
  }
  eprint!("... ");
  // ## Associativity
  // __Inputs:__ `x : u16, y : u16, z : u16`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
  //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
  {
    assert!(
        0u16.checked_add(0u16).and_then(| iv | iv.checked_add(0u16)) == 0u16
        .checked_add(0u16).and_then(| iv | 0u16.checked_add(iv))
    );
    assert!(
        0u16.checked_add(0u16).and_then(| iv | iv.checked_add(65535u16)) == 0u16
        .checked_add(65535u16).and_then(| iv | 0u16.checked_add(iv))
    );
    assert!(
        0u16.checked_add(65535u16).and_then(| iv | iv.checked_add(0u16)) == 65535u16
        .checked_add(0u16).and_then(| iv | 0u16.checked_add(iv))
    );
    assert!(
        0u16.checked_add(65535u16).and_then(| iv | iv.checked_add(65535u16)) == 65535u16
        .checked_add(65535u16).and_then(| iv | 0u16.checked_add(iv))
    );
    assert!(
        65535u16.checked_add(0u16).and_then(| iv | iv.checked_add(0u16)) == 0u16
        .checked_add(0u16).and_then(| iv | 65535u16.checked_add(iv))
    );
    assert!(
        65535u16.checked_add(0u16).and_then(| iv | iv.checked_add(65535u16)) == 0u16
        .checked_add(65535u16).and_then(| iv | 65535u16.checked_add(iv))
    );
    assert!(
        65535u16.checked_add(65535u16).and_then(| iv | iv.checked_add(0u16)) == 65535u16
        .checked_add(0u16).and_then(| iv | 65535u16.checked_add(iv))
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u32>::add`]
pub fn core_ops_add_u32_add(){
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
    assert!(1916838581u32 + 1867326820u32 == 3784165401u32);
    assert!(2403193023u32 + 1378792873u32 == 3781985896u32);
    assert!(2056028193u32 + 717535420u32 == 2773563613u32);
    assert!(818836294u32 + 3184656551u32 == 4003492845u32);
  }
  eprint!("... ");
  // ## Panics when overflowing
  // __Inputs:__ `x : u32, y : u32`  
  // __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(panics!(4294967295u32 + 4294967295u32));
    assert!(panics!(1732794631u32 + 4125656264u32));
    assert!(panics!(2644610124u32 + 3552763161u32));
    assert!(panics!(1413599579u32 + 3268056546u32));
    assert!(panics!(3657998284u32 + 3175825949u32));
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
    assert!(2182951120u32 + 1986764447u32 == 1986764447u32 + 2182951120u32);
    assert!(1048514664u32 + 2692993596u32 == 2692993596u32 + 1048514664u32);
    assert!(1300112244u32 + 673170742u32 == 673170742u32 + 1300112244u32);
    assert!(871484658u32 + 543611400u32 == 543611400u32 + 871484658u32);
  }
  eprint!("... ");
  // ## Left identity
  // __Inputs:__ `x : u32`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x + 0 == x`
  {
    assert!(0u32 + 0 == 0u32);
    assert!(4294967295u32 + 0 == 4294967295u32);
    assert!(2271285329u32 + 0 == 2271285329u32);
    assert!(1626925841u32 + 0 == 1626925841u32);
    assert!(2257870636u32 + 0 == 2257870636u32);
    assert!(2983762624u32 + 0 == 2983762624u32);
    assert!(2542836994u32 + 0 == 2542836994u32);
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u32`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0 + x == x`
  {
    assert!(0 + 0u32 == 0u32);
    assert!(0 + 4294967295u32 == 4294967295u32);
    assert!(0 + 3678527288u32 == 3678527288u32);
    assert!(0 + 15199u32 == 15199u32);
    assert!(0 + 3095489077u32 == 3095489077u32);
    assert!(0 + 2750823793u32 == 2750823793u32);
    assert!(0 + 3655940132u32 == 3655940132u32);
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
        (47090513u32 + 2645340346u32) + 1447744816u32 == 47090513u32 + (2645340346u32 +
        1447744816u32)
    );
    assert!(
        (366201493u32 + 1551837092u32) + 602416881u32 == 366201493u32 + (1551837092u32 +
        602416881u32)
    );
    assert!(
        (2611690411u32 + 538885865u32) + 872512866u32 == 2611690411u32 + (538885865u32 +
        872512866u32)
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`u32::checked_add`]
pub fn core_ops_add_u32_checked_add(){
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
    assert!(496604356u32.checked_add(287701069u32) == Some(784305425u32));
    assert!(469626658u32.checked_add(1485686854u32) == Some(1955313512u32));
    assert!(4263077847u32.checked_add(1414639u32) == Some(4264492486u32));
    assert!(2019928432u32.checked_add(2222499980u32) == Some(4242428412u32));
  }
  eprint!("... ");
  // ## None when overflowing
  // __Inputs:__ `x : u32, y : u32`  
  // __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
  // __Postcondition:__ `x.checked_add(y) == None`
  {
    assert!(4294967295u32.checked_add(4294967295u32) == None);
    assert!(3480170470u32.checked_add(4141953202u32) == None);
    assert!(4253646812u32.checked_add(2952265509u32) == None);
    assert!(4068877698u32.checked_add(3615948754u32) == None);
    assert!(3100627445u32.checked_add(3971421084u32) == None);
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
        4294967295u32.checked_add(4294967295u32) == 4294967295u32
        .checked_add(4294967295u32)
    );
    assert!(
        3081563449u32.checked_add(3091548329u32) == 3091548329u32
        .checked_add(3081563449u32)
    );
    assert!(
        3233973771u32.checked_add(3683754918u32) == 3683754918u32
        .checked_add(3233973771u32)
    );
    assert!(
        3772028530u32.checked_add(3468063212u32) == 3468063212u32
        .checked_add(3772028530u32)
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
    assert!(2660804380u32.checked_add(0u32) == Some(2660804380u32));
    assert!(1053185591u32.checked_add(0u32) == Some(1053185591u32));
    assert!(3414494668u32.checked_add(0u32) == Some(3414494668u32));
    assert!(2605373754u32.checked_add(0u32) == Some(2605373754u32));
    assert!(1973552644u32.checked_add(0u32) == Some(1973552644u32));
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u32`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0u32.checked_add(x) == Some(x)`
  {
    assert!(0u32.checked_add(0u32) == Some(0u32));
    assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
    assert!(0u32.checked_add(2467364219u32) == Some(2467364219u32));
    assert!(0u32.checked_add(3867635198u32) == Some(3867635198u32));
    assert!(0u32.checked_add(3286980667u32) == Some(3286980667u32));
    assert!(0u32.checked_add(2565097253u32) == Some(2565097253u32));
    assert!(0u32.checked_add(1607931383u32) == Some(1607931383u32));
  }
  eprint!("... ");
  // ## Associativity
  // __Inputs:__ `x : u32, y : u32, z : u32`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
  //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
  {
    assert!(
        0u32.checked_add(0u32).and_then(| iv | iv.checked_add(0u32)) == 0u32
        .checked_add(0u32).and_then(| iv | 0u32.checked_add(iv))
    );
    assert!(
        0u32.checked_add(0u32).and_then(| iv | iv.checked_add(4294967295u32)) == 0u32
        .checked_add(4294967295u32).and_then(| iv | 0u32.checked_add(iv))
    );
    assert!(
        0u32.checked_add(4294967295u32).and_then(| iv | iv.checked_add(0u32)) ==
        4294967295u32.checked_add(0u32).and_then(| iv | 0u32.checked_add(iv))
    );
    assert!(
        0u32.checked_add(4294967295u32).and_then(| iv | iv.checked_add(4294967295u32)) ==
        4294967295u32.checked_add(4294967295u32).and_then(| iv | 0u32.checked_add(iv))
    );
    assert!(
        4294967295u32.checked_add(0u32).and_then(| iv | iv.checked_add(0u32)) == 0u32
        .checked_add(0u32).and_then(| iv | 4294967295u32.checked_add(iv))
    );
    assert!(
        4294967295u32.checked_add(0u32).and_then(| iv | iv.checked_add(4294967295u32)) ==
        0u32.checked_add(4294967295u32).and_then(| iv | 4294967295u32.checked_add(iv))
    );
    assert!(
        4294967295u32.checked_add(4294967295u32).and_then(| iv | iv.checked_add(0u32)) ==
        4294967295u32.checked_add(0u32).and_then(| iv | 4294967295u32.checked_add(iv))
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u64>::add`]
pub fn core_ops_add_u64_add(){
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
    assert!(13125140763534286593u64 + 3927769232320196337u64 == 17052909995854482930u64);
    assert!(9093381719757337827u64 + 620948497547885633u64 == 9714330217305223460u64);
    assert!(1934212261962110729u64 + 11673961412482495690u64 == 13608173674444606419u64);
    assert!(6766101589916358075u64 + 6200036605893040190u64 == 12966138195809398265u64);
  }
  eprint!("... ");
  // ## Panics when overflowing
  // __Inputs:__ `x : u64, y : u64`  
  // __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
    assert!(panics!(6448567070522900651u64 + 15470470475406116552u64));
    assert!(panics!(14148468244294055977u64 + 15677571747955404748u64));
    assert!(panics!(17943903405453721814u64 + 9559765792233331028u64));
    assert!(panics!(17114909009124663695u64 + 9413751319251859184u64));
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
        5882701380255680948u64 + 5323447308076762143u64 == 5323447308076762143u64 +
        5882701380255680948u64
    );
    assert!(
        13025422064699287192u64 + 3376340481103581704u64 == 3376340481103581704u64 +
        13025422064699287192u64
    );
    assert!(
        10012257027772040881u64 + 2562233125230032418u64 == 2562233125230032418u64 +
        10012257027772040881u64
    );
    assert!(
        2165916656086874440u64 + 11169927090654556061u64 == 11169927090654556061u64 +
        2165916656086874440u64
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
    assert!(12728106104009289020u64 + 0 == 12728106104009289020u64);
    assert!(8831262569610964997u64 + 0 == 8831262569610964997u64);
    assert!(18154579220548667250u64 + 0 == 18154579220548667250u64);
    assert!(4269198553623692496u64 + 0 == 4269198553623692496u64);
    assert!(3623985269915848117u64 + 0 == 3623985269915848117u64);
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u64`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0 + x == x`
  {
    assert!(0 + 0u64 == 0u64);
    assert!(0 + 18446744073709551615u64 == 18446744073709551615u64);
    assert!(0 + 302309546184672550u64 == 302309546184672550u64);
    assert!(0 + 13896292345132402197u64 == 13896292345132402197u64);
    assert!(0 + 5489813207023984314u64 == 5489813207023984314u64);
    assert!(0 + 4529177259527971793u64 == 4529177259527971793u64);
    assert!(0 + 4439743563818569357u64 == 4439743563818569357u64);
  }
  eprint!("... ");
  // ## Associativity
  // __Inputs:__ `x : u64, y : u64, z : u64`  
  // __Precondition:__ `x.up() + y.up() + z.up() <= u64::MAX.up()`  
  // __Postcondition:__ `(x + y) + z == x + (y + z)`
  {
    assert!((0u64 + 0u64) + 0u64 == 0u64 + (0u64 + 0u64));
    assert!(
        (0u64 + 0u64) + 18446744073709551615u64 == 0u64 + (0u64 +
        18446744073709551615u64)
    );
    assert!(
        (0u64 + 18446744073709551615u64) + 0u64 == 0u64 + (18446744073709551615u64 +
        0u64)
    );
    assert!(
        (18446744073709551615u64 + 0u64) + 0u64 == 18446744073709551615u64 + (0u64 +
        0u64)
    );
    assert!(
        (778731238713794576u64 + 5510787770492739566u64) + 10216325256439029411u64 ==
        778731238713794576u64 + (5510787770492739566u64 + 10216325256439029411u64)
    );
    assert!(
        (12345141418044287607u64 + 4131064766553685364u64) + 59372124706784222u64 ==
        12345141418044287607u64 + (4131064766553685364u64 + 59372124706784222u64)
    );
    assert!(
        (434215247792223641u64 + 2831757766042833068u64) + 8822908435009789685u64 ==
        434215247792223641u64 + (2831757766042833068u64 + 8822908435009789685u64)
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`u64::checked_add`]
pub fn core_ops_add_u64_checked_add(){
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
        10319665142740526495u64.checked_add(1580939060803786461u64) ==
        Some(11900604203544312956u64)
    );
    assert!(
        96040861103887606u64.checked_add(17096831315995372080u64) ==
        Some(17192872177099259686u64)
    );
    assert!(
        10247670434930013634u64.checked_add(2768708255389041973u64) ==
        Some(13016378690319055607u64)
    );
    assert!(
        16580826385761171881u64.checked_add(1395676480227006999u64) ==
        Some(17976502865988178880u64)
    );
  }
  eprint!("... ");
  // ## None when overflowing
  // __Inputs:__ `x : u64, y : u64`  
  // __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
  // __Postcondition:__ `x.checked_add(y) == None`
  {
    assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
    assert!(3837264180014193847u64.checked_add(17695739075348268205u64) == None);
    assert!(5319007015111890645u64.checked_add(15494180103804553178u64) == None);
    assert!(17657311451316839294u64.checked_add(16639217354312975208u64) == None);
    assert!(7970518524142412284u64.checked_add(17471945994664784480u64) == None);
  }
  eprint!("... ");
  // ## Commutativity
  // __Inputs:__ `x : u64, y : u64`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`
  {
    assert!(0u64.checked_add(0u64) == 0u64.checked_add(0u64));
    assert!(
        0u64.checked_add(18446744073709551615u64) == 18446744073709551615u64
        .checked_add(0u64)
    );
    assert!(
        18446744073709551615u64.checked_add(0u64) == 0u64
        .checked_add(18446744073709551615u64)
    );
    assert!(
        18446744073709551615u64.checked_add(18446744073709551615u64) ==
        18446744073709551615u64.checked_add(18446744073709551615u64)
    );
    assert!(
        10173333614985470077u64.checked_add(4663152924529571316u64) ==
        4663152924529571316u64.checked_add(10173333614985470077u64)
    );
    assert!(
        538869268897202571u64.checked_add(216463633744338667u64) == 216463633744338667u64
        .checked_add(538869268897202571u64)
    );
    assert!(
        14139591821137162892u64.checked_add(17836696054002117003u64) ==
        17836696054002117003u64.checked_add(14139591821137162892u64)
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
    assert!(814894684741255764u64.checked_add(0u64) == Some(814894684741255764u64));
    assert!(16315219662698286557u64.checked_add(0u64) == Some(16315219662698286557u64));
    assert!(16885662844312017565u64.checked_add(0u64) == Some(16885662844312017565u64));
    assert!(3532464429668122453u64.checked_add(0u64) == Some(3532464429668122453u64));
    assert!(12906718864756491514u64.checked_add(0u64) == Some(12906718864756491514u64));
  }
  eprint!("... ");
  // ## Right identity
  // __Inputs:__ `x : u64`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `0u64.checked_add(x) == Some(x)`
  {
    assert!(0u64.checked_add(0u64) == Some(0u64));
    assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
    assert!(0u64.checked_add(2703397565941799802u64) == Some(2703397565941799802u64));
    assert!(0u64.checked_add(9130468968030618558u64) == Some(9130468968030618558u64));
    assert!(0u64.checked_add(2666648179673026508u64) == Some(2666648179673026508u64));
    assert!(0u64.checked_add(13924031752538595728u64) == Some(13924031752538595728u64));
    assert!(0u64.checked_add(3168860215794057572u64) == Some(3168860215794057572u64));
  }
  eprint!("... ");
  // ## Associativity
  // __Inputs:__ `x : u64, y : u64, z : u64`  
  // __Precondition:__ `true`  
  // __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
  //         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`
  {
    assert!(
        0u64.checked_add(0u64).and_then(| iv | iv.checked_add(0u64)) == 0u64
        .checked_add(0u64).and_then(| iv | 0u64.checked_add(iv))
    );
    assert!(
        0u64.checked_add(0u64).and_then(| iv | iv.checked_add(18446744073709551615u64))
        == 0u64.checked_add(18446744073709551615u64).and_then(| iv | 0u64
        .checked_add(iv))
    );
    assert!(
        0u64.checked_add(18446744073709551615u64).and_then(| iv | iv.checked_add(0u64))
        == 18446744073709551615u64.checked_add(0u64).and_then(| iv | 0u64
        .checked_add(iv))
    );
    assert!(
        0u64.checked_add(18446744073709551615u64).and_then(| iv | iv
        .checked_add(18446744073709551615u64)) == 18446744073709551615u64
        .checked_add(18446744073709551615u64).and_then(| iv | 0u64.checked_add(iv))
    );
    assert!(
        18446744073709551615u64.checked_add(0u64).and_then(| iv | iv.checked_add(0u64))
        == 0u64.checked_add(0u64).and_then(| iv | 18446744073709551615u64
        .checked_add(iv))
    );
    assert!(
        18446744073709551615u64.checked_add(0u64).and_then(| iv | iv
        .checked_add(18446744073709551615u64)) == 0u64
        .checked_add(18446744073709551615u64).and_then(| iv | 18446744073709551615u64
        .checked_add(iv))
    );
    assert!(
        18446744073709551615u64.checked_add(18446744073709551615u64).and_then(| iv | iv
        .checked_add(0u64)) == 18446744073709551615u64.checked_add(0u64).and_then(| iv |
        18446744073709551615u64.checked_add(iv))
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u128>::add`]
pub fn core_ops_add_u128_add(){
  eprintln!(r#"Testing "Properties for [`core::ops::Add::<u128>::add`]"... (6 contracts)"#);
  eprint!("  ");
  // ## Semantics of non-overflowing addition
  // __Inputs:__ `x : u128, y : u128`  
  // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
  // __Postcondition:__ `x + y == u128::down(x.up() + y.up())`
  {
    assert!(0u128 + 0u128 == 0u128);
    assert!(
        0u128 + 340282366920938463463374607431768211455u128 ==
        340282366920938463463374607431768211455u128
    );
    assert!(
        340282366920938463463374607431768211455u128 + 0u128 ==
        340282366920938463463374607431768211455u128
    );
    assert!(
        301370909848594279591461150471121578737u128 +
        35814339722372214555284862383047721617u128 ==
        337185249570966494146746012854169300354u128
    );
    assert!(
        89996932787694552297624275082161448516u128 +
        37518987540574558085768394449524015429u128 ==
        127515920328269110383392669531685463945u128
    );
    assert!(
        27427478810616094779597024591837868989u128 +
        103963563277872155549982150581077579381u128 ==
        131391042088488250329579175172915448370u128
    );
    assert!(
        94805089649375580165582462057439883094u128 +
        242867375558892211929159567015371862266u128 ==
        337672465208267792094742029072811745360u128
    );
  }
  eprint!("... ");
  // ## Panics when overflowing
  // __Inputs:__ `x : u128, y : u128`  
  // __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
  // __Postcondition:__ `panics!(x + y)`
  {
    assert!(
        panics!(340282366920938463463374607431768211455u128 +
        340282366920938463463374607431768211455u128)
    );
    assert!(
        panics!(326273368693551272326556222344464025486u128 +
        154459877113479028141523241753164958278u128)
    );
    assert!(
        panics!(261017019014281949839602196930541122982u128 +
        142195501228092516616387760748284664758u128)
    );
    assert!(
        panics!(314800563423903301313530256031374157400u128 +
        110119266231012759780169682656513567754u128)
    );
    assert!(
        panics!(274086314388992267114151700761031247044u128 +
        181779057023191838419533310439417359642u128)
    );
  }
  eprint!("... ");
  // ## Commutativity
  // __Inputs:__ `x : u128, y : u128`  
  // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
  // __Postcondition:__ `x + y == y + x`
  {
    assert!(0u128 + 0u128 == 0u128 + 0u128);
    assert!(
        0u128 + 340282366920938463463374607431768211455u128 ==
        340282366920938463463374607431768211455u128 + 0u128
    );
    assert!(
        340282366920938463463374607431768211455u128 + 0u128 == 0u128 +
        340282366920938463463374607431768211455u128
    );
    assert!(
        63526415030570092104962920347736433408u128 +
        179391049630257919629323978810269345142u128 ==
        179391049630257919629323978810269345142u128 +
        63526415030570092104962920347736433408u128
    );
    assert!(
        684359504521534864090066651360731354u128 +
        35083182456310223587822506539428191534u128 ==
        35083182456310223587822506539428191534u128 +
        684359504521534864090066651360731354u128
    );
    assert!(
        16769600507509701152812101393450189451u128 +
        305348258432879712481442439179017547540u128 ==
        305348258432879712481442439179017547540u128 +
        16769600507509701152812101393450189451u128
    );
    assert!(
        250633948371291310669717748444841449430u128 +
        526821924908191179154921606076349079u128 ==
        526821924908191179154921606076349079u128 +
        250633948371291310669717748444841449430u128
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
        340282366920938463463374607431768211455u128 + 0 ==
        340282366920938463463374607431768211455u128
    );
    assert!(
        80949615260197109582061920586998211808u128 + 0 ==
        80949615260197109582061920586998211808u128
    );
    assert!(
        61504431792962955670018075684658585207u128 + 0 ==
        61504431792962955670018075684658585207u128
    );
    assert!(
        77221703645708040405395057765183635454u128 + 0 ==
        77221703645708040405395057765183635454u128
    );
    assert!(
        74543511943943993381311655523035702163u128 + 0 ==
        74543511943943993381311655523035702163u128
    );
    assert!(
        117960521058624222994091659979971570436u128 + 0 ==
        117960521058624222994091659979971570436u128
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
        0 + 340282366920938463463374607431768211455u128 ==
        340282366920938463463374607431768211455u128
    );
    assert!(
        0 + 263062052446594260353303310653847111455u128 ==
        263062052446594260353303310653847111455u128
    );
    assert!(
        0 + 266980034559934049329024821582288219950u128 ==
        266980034559934049329024821582288219950u128
    );
    assert!(
        0 + 29575459189974817250905722628623740110u128 ==
        29575459189974817250905722628623740110u128
    );
    assert!(
        0 + 164367468580495925957286847163985073389u128 ==
        164367468580495925957286847163985073389u128
    );
    assert!(
        0 + 233950753207145698002439731824951226187u128 ==
        233950753207145698002439731824951226187u128
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
        (0u128 + 0u128) + 340282366920938463463374607431768211455u128 == 0u128 + (0u128 +
        340282366920938463463374607431768211455u128)
    );
    assert!(
        (0u128 + 340282366920938463463374607431768211455u128) + 0u128 == 0u128 +
        (340282366920938463463374607431768211455u128 + 0u128)
    );
    assert!(
        (340282366920938463463374607431768211455u128 + 0u128) + 0u128 ==
        340282366920938463463374607431768211455u128 + (0u128 + 0u128)
    );
    assert!(
        (66213640771777056249716421577125162345u128 +
        96685414438477518421727851335648538050u128) +
        130977045267309913877969376160726992878u128 ==
        66213640771777056249716421577125162345u128 +
        (96685414438477518421727851335648538050u128 +
        130977045267309913877969376160726992878u128)
    );
    assert!(
        (326078578398515731130953541416336176543u128 +
        7243771087818523031993866226308964512u128) +
        2176130121709356634192940911036418865u128 ==
        326078578398515731130953541416336176543u128 +
        (7243771087818523031993866226308964512u128 +
        2176130121709356634192940911036418865u128)
    );
    assert!(
        (98619006774975771709075126065309763503u128 +
        126890652920525934930516729366883939708u128) +
        31455374901545913470370967480236109185u128 ==
        98619006774975771709075126065309763503u128 +
        (126890652920525934930516729366883939708u128 +
        31455374901545913470370967480236109185u128)
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
/// Properties for [`u128::checked_add`]
pub fn core_ops_add_u128_checked_add(){
  eprintln!(r#"Testing "Properties for [`u128::checked_add`]"... (6 contracts)"#);
  eprint!("  ");
  // ## Semantics of non-overflowing checked addition
  // __Inputs:__ `x : u128, y : u128`  
  // __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
  // __Postcondition:__ `x.checked_add(y) == Some(u128::down(x.up() + y.up()))`
  {
    assert!(0u128.checked_add(0u128) == Some(0u128));
    assert!(
        0u128.checked_add(340282366920938463463374607431768211455u128) ==
        Some(340282366920938463463374607431768211455u128)
    );
    assert!(
        340282366920938463463374607431768211455u128.checked_add(0u128) ==
        Some(340282366920938463463374607431768211455u128)
    );
    assert!(
        54612447081117559865967542658311279741u128
        .checked_add(195263932934173560795176109448775097946u128) ==
        Some(249876380015291120661143652107086377687u128)
    );
    assert!(
        217682747879594299709948525807931757780u128
        .checked_add(82409755854691091437725797019488812379u128) ==
        Some(300092503734285391147674322827420570159u128)
    );
    assert!(
        118146600004434510218378985476528641493u128
        .checked_add(219587934433786544932460097221550197577u128) ==
        Some(337734534438221055150839082698078839070u128)
    );
    assert!(
        140390201758942030562582956931042616463u128
        .checked_add(78280835050497905126706607081453375073u128) ==
        Some(218671036809439935689289564012495991536u128)
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
        .checked_add(340282366920938463463374607431768211455u128) == None
    );
    assert!(
        176028053346526115377000430127089688940u128
        .checked_add(293872891822758795691996906048037747208u128) == None
    );
    assert!(
        248414921339935319392826660922356600063u128
        .checked_add(157970557453681754829954574212273029475u128) == None
    );
    assert!(
        224087302189027287733454619052593287115u128
        .checked_add(172374818419951960248733910265576699565u128) == None
    );
    assert!(
        324416747375774709211039594627319068596u128
        .checked_add(216224128245603375903183565757174422218u128) == None
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
        0u128.checked_add(340282366920938463463374607431768211455u128) ==
        340282366920938463463374607431768211455u128.checked_add(0u128)
    );
    assert!(
        340282366920938463463374607431768211455u128.checked_add(0u128) == 0u128
        .checked_add(340282366920938463463374607431768211455u128)
    );
    assert!(
        340282366920938463463374607431768211455u128
        .checked_add(340282366920938463463374607431768211455u128) ==
        340282366920938463463374607431768211455u128
        .checked_add(340282366920938463463374607431768211455u128)
    );
    assert!(
        186480696479260285981833345181261660792u128
        .checked_add(227705548393058304404556623422706413009u128) ==
        227705548393058304404556623422706413009u128
        .checked_add(186480696479260285981833345181261660792u128)
    );
    assert!(
        267411779757966411400764072616204604065u128
        .checked_add(285521260725945353483379815649352803587u128) ==
        285521260725945353483379815649352803587u128
        .checked_add(267411779757966411400764072616204604065u128)
    );
    assert!(
        31848357900658053707279435732143190922u128
        .checked_add(255494916345677679703442168575315068620u128) ==
        255494916345677679703442168575315068620u128
        .checked_add(31848357900658053707279435732143190922u128)
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
        340282366920938463463374607431768211455u128.checked_add(0u128) ==
        Some(340282366920938463463374607431768211455u128)
    );
    assert!(
        40309663057292881741452557762238726100u128.checked_add(0u128) ==
        Some(40309663057292881741452557762238726100u128)
    );
    assert!(
        97609709790548622651826528609563641946u128.checked_add(0u128) ==
        Some(97609709790548622651826528609563641946u128)
    );
    assert!(
        14991169416687567040366556387936363443u128.checked_add(0u128) ==
        Some(14991169416687567040366556387936363443u128)
    );
    assert!(
        82207581687985246518185744470061901435u128.checked_add(0u128) ==
        Some(82207581687985246518185744470061901435u128)
    );
    assert!(
        281517665166188637085664908117546103039u128.checked_add(0u128) ==
        Some(281517665166188637085664908117546103039u128)
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
        0u128.checked_add(340282366920938463463374607431768211455u128) ==
        Some(340282366920938463463374607431768211455u128)
    );
    assert!(
        0u128.checked_add(190348595346634282447952766768023210136u128) ==
        Some(190348595346634282447952766768023210136u128)
    );
    assert!(
        0u128.checked_add(104526475574802544089919081626493162633u128) ==
        Some(104526475574802544089919081626493162633u128)
    );
    assert!(
        0u128.checked_add(279772134261981111028453807662240722673u128) ==
        Some(279772134261981111028453807662240722673u128)
    );
    assert!(
        0u128.checked_add(81577281226549620568237091137384152319u128) ==
        Some(81577281226549620568237091137384152319u128)
    );
    assert!(
        0u128.checked_add(36173597367687954834377331272500591766u128) ==
        Some(36173597367687954834377331272500591766u128)
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
        0u128.checked_add(0u128).and_then(| iv | iv.checked_add(0u128)) == 0u128
        .checked_add(0u128).and_then(| iv | 0u128.checked_add(iv))
    );
    assert!(
        0u128.checked_add(0u128).and_then(| iv | iv
        .checked_add(340282366920938463463374607431768211455u128)) == 0u128
        .checked_add(340282366920938463463374607431768211455u128).and_then(| iv | 0u128
        .checked_add(iv))
    );
    assert!(
        0u128.checked_add(340282366920938463463374607431768211455u128).and_then(| iv | iv
        .checked_add(0u128)) == 340282366920938463463374607431768211455u128
        .checked_add(0u128).and_then(| iv | 0u128.checked_add(iv))
    );
    assert!(
        0u128.checked_add(340282366920938463463374607431768211455u128).and_then(| iv | iv
        .checked_add(340282366920938463463374607431768211455u128)) ==
        340282366920938463463374607431768211455u128
        .checked_add(340282366920938463463374607431768211455u128).and_then(| iv | 0u128
        .checked_add(iv))
    );
    assert!(
        340282366920938463463374607431768211455u128.checked_add(0u128).and_then(| iv | iv
        .checked_add(0u128)) == 0u128.checked_add(0u128).and_then(| iv |
        340282366920938463463374607431768211455u128.checked_add(iv))
    );
    assert!(
        340282366920938463463374607431768211455u128.checked_add(0u128).and_then(| iv | iv
        .checked_add(340282366920938463463374607431768211455u128)) == 0u128
        .checked_add(340282366920938463463374607431768211455u128).and_then(| iv |
        340282366920938463463374607431768211455u128.checked_add(iv))
    );
    assert!(
        340282366920938463463374607431768211455u128
        .checked_add(340282366920938463463374607431768211455u128).and_then(| iv | iv
        .checked_add(0u128)) == 340282366920938463463374607431768211455u128
        .checked_add(0u128).and_then(| iv | 340282366920938463463374607431768211455u128
        .checked_add(iv))
    );
  }
  eprint!("... ");
  eprintln!("✓\n");
}
fn main(){option_is_some();
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
core_ops_add_u128_checked_add();}