//! This module contains 533 tests, organized in functions.
#![allow(arithmetic_overflow)]
use core_spec::lifts::*;
use core_spec::*;
/// Properties for [`Option::is_some`]
pub fn option_is_some() {
    eprintln!(r#"Testing "Properties for [`Option::is_some`]"... (1 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(
            Option::< u8 >::None.is_some() == (match Option::< u8 >::None { Some(_) =>
            true, None => false, })
        );
        assert!(
            Some(0u8).is_some() == (match Some(0u8) { Some(_) => true, None => false, })
        );
        assert!(
            Some(255u8).is_some() == (match Some(255u8) { Some(_) => true, None => false,
            })
        );
        assert!(
            Some(61u8).is_some() == (match Some(61u8) { Some(_) => true, None => false,
            })
        );
        assert!(
            Some(16u8).is_some() == (match Some(16u8) { Some(_) => true, None => false,
            })
        );
        assert!(
            Some(150u8).is_some() == (match Some(150u8) { Some(_) => true, None => false,
            })
        );
        assert!(
            Some(36u8).is_some() == (match Some(36u8) { Some(_) => true, None => false,
            })
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::is_none`]
pub fn option_is_none() {
    eprintln!(r#"Testing "Properties for [`Option::is_none`]"... (1 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(
            Option::< u8 >::None.is_none() == (match Option::< u8 >::None { Some(_) =>
            false, None => true, })
        );
        assert!(
            Some(0u8).is_none() == (match Some(0u8) { Some(_) => false, None => true, })
        );
        assert!(
            Some(255u8).is_none() == (match Some(255u8) { Some(_) => false, None => true,
            })
        );
        assert!(
            Some(70u8).is_none() == (match Some(70u8) { Some(_) => false, None => true,
            })
        );
        assert!(
            Some(216u8).is_none() == (match Some(216u8) { Some(_) => false, None => true,
            })
        );
        assert!(
            Some(253u8).is_none() == (match Some(253u8) { Some(_) => false, None => true,
            })
        );
        assert!(
            Some(25u8).is_none() == (match Some(25u8) { Some(_) => false, None => true,
            })
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::expect`]
pub fn option_expect() {
    eprintln!(r#"Testing "Properties for [`Option::expect`]"... (3 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(panics!(Option::< u8 >::None.expect("message")));
    }
    eprint!(r#"... "#);
    {
        assert!(doesn_t_panic!(Some(0u8).expect("message")));
        assert!(doesn_t_panic!(Some(255u8).expect("message")));
        assert!(doesn_t_panic!(Some(143u8).expect("message")));
        assert!(doesn_t_panic!(Some(198u8).expect("message")));
        assert!(doesn_t_panic!(Some(63u8).expect("message")));
        assert!(doesn_t_panic!(Some(40u8).expect("message")));
        assert!(doesn_t_panic!(Some(62u8).expect("message")));
    }
    eprint!(r#"... "#);
    {
        assert!(Some(0u8).unwrap() == 0u8);
        assert!(Some(255u8).unwrap() == 255u8);
        assert!(Some(228u8).unwrap() == 228u8);
        assert!(Some(69u8).unwrap() == 69u8);
        assert!(Some(233u8).unwrap() == 233u8);
        assert!(Some(167u8).unwrap() == 167u8);
        assert!(Some(187u8).unwrap() == 187u8);
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::map`]
pub fn option_map() {
    eprintln!(r#"Testing "Properties for [`Option::map`]"... (2 contracts)"#);
    eprint!(r#"  "#);
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
        assert!(Some(20u8).map((| x : u8 | x)) == Some(((| x : u8 | x)) (20u8)));
    }
    eprint!(r#"... "#);
    {
        assert!(
            Option::< u8 >::None.map((| x : u8 | x.wrapping_add(x))) == Option::< u8
            >::None
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::filter`]
pub fn option_filter() {
    eprintln!(r#"Testing "Properties for [`Option::filter`]"... (2 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(
            Some(0u8).filter((| x : & u8 | * x < 0u8)).is_some() == (| x : & u8 | * x <
            0u8) (& 0u8)
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
            Some(255u8).filter((| x : & u8 | * x < 255u8)).is_some() == (| x : & u8 | * x
            < 255u8) (& 255u8)
        );
        assert!(
            Some(13u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x >
            128) (& 13u8)
        );
        assert!(
            Some(147u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x >
            128) (& 147u8)
        );
        assert!(
            Some(54u8).filter((| x : & u8 | * x > 128)).is_some() == (| x : & u8 | * x >
            128) (& 54u8)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(
            Option::< u8 >::None.filter((| x : & u8 | * x < 0u8)) == Option::< u8 >::None
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::flatten`]
pub fn option_flatten() {
    eprintln!(r#"Testing "Properties for [`Option::flatten`]"... (2 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(Some(Some(0u8)).flatten() == Some(0u8));
        assert!(Some(Some(255u8)).flatten() == Some(255u8));
        assert!(Some(Some(61u8)).flatten() == Some(61u8));
        assert!(Some(Some(248u8)).flatten() == Some(248u8));
        assert!(Some(Some(184u8)).flatten() == Some(184u8));
        assert!(Some(Some(76u8)).flatten() == Some(76u8));
        assert!(Some(Some(214u8)).flatten() == Some(214u8));
    }
    eprint!(r#"... "#);
    {
        assert!(Option::< Option < u8 >>::None.flatten() == None);
        assert!(Some(Option::< u8 >::None).flatten() == None);
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::take`]
pub fn option_take() {
    eprintln!(r#"Testing "Properties for [`Option::take`]"... (1 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(
            { let mut y = Option::< u8 >::None.clone(); y.take() == Option::< u8 >::None
            && y.is_none() }
        );
        assert!({ let mut y = Some(0u8).clone(); y.take() == Some(0u8) && y.is_none() });
        assert!(
            { let mut y = Some(255u8).clone(); y.take() == Some(255u8) && y.is_none() }
        );
        assert!(
            { let mut y = Some(168u8).clone(); y.take() == Some(168u8) && y.is_none() }
        );
        assert!(
            { let mut y = Some(174u8).clone(); y.take() == Some(174u8) && y.is_none() }
        );
        assert!(
            { let mut y = Some(164u8).clone(); y.take() == Some(164u8) && y.is_none() }
        );
        assert!(
            { let mut y = Some(64u8).clone(); y.take() == Some(64u8) && y.is_none() }
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::zip`]
pub fn option_zip() {
    eprintln!(r#"Testing "Properties for [`Option::zip`]"... (2 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(Some(0u8).zip(Some(0u8)) == Some((0u8, 0u8)));
        assert!(Some(0u8).zip(Some(255u8)) == Some((0u8, 255u8)));
        assert!(Some(255u8).zip(Some(0u8)) == Some((255u8, 0u8)));
        assert!(Some(255u8).zip(Some(255u8)) == Some((255u8, 255u8)));
        assert!(Some(229u8).zip(Some(167u8)) == Some((229u8, 167u8)));
        assert!(Some(231u8).zip(Some(7u8)) == Some((231u8, 7u8)));
        assert!(Some(68u8).zip(Some(172u8)) == Some((68u8, 172u8)));
    }
    eprint!(r#"... "#);
    {
        assert!(Option::< u8 >::None.zip(Option::< u8 >::None).is_none());
        assert!(Option::< u8 >::None.zip(Some(0u8)).is_none());
        assert!(Option::< u8 >::None.zip(Some(255u8)).is_none());
        assert!(Some(0u8).zip(Option::< u8 >::None).is_none());
        assert!(Some(255u8).zip(Option::< u8 >::None).is_none());
        assert!(Option::< u8 >::None.zip(Some(24u8)).is_none());
        assert!(Some(227u8).zip(Option::< u8 >::None).is_none());
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::unwrap`]
pub fn option_unwrap() {
    eprintln!(r#"Testing "Properties for [`Option::unwrap`]"... (2 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(panics!(Option::< u8 >::None.unwrap()));
    }
    eprint!(r#"... "#);
    {
        assert!(doesn_t_panic!(Some(0u8).unwrap()));
        assert!(doesn_t_panic!(Some(255u8).unwrap()));
        assert!(doesn_t_panic!(Some(71u8).unwrap()));
        assert!(doesn_t_panic!(Some(226u8).unwrap()));
        assert!(doesn_t_panic!(Some(244u8).unwrap()));
        assert!(doesn_t_panic!(Some(26u8).unwrap()));
        assert!(doesn_t_panic!(Some(162u8).unwrap()));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::as_mut`]
pub fn option_as_mut() {
    eprintln!(r#"Testing "Properties for [`Option::as_mut`]"... (1 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(12u8).unwrap().clone(), Some(12u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(39u8).unwrap().clone(), Some(39u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(2u8).unwrap().clone(), Some(2u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(38u8).unwrap().clone(), Some(38u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(8u8).unwrap().clone(), Some(8u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
        assert!(
            { let (v_unwrapped, mut v_mut) = (Some(16u8).unwrap().clone(), Some(16u8)); *
            v_mut.as_mut().unwrap() += 10; v_mut.unwrap() == v_unwrapped + 10 }
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Option::as_slice`]
pub fn option_as_slice() {
    eprintln!(r#"Testing "Properties for [`Option::as_slice`]"... (2 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!({ Option::< u8 >::None.as_slice().is_empty() });
    }
    eprint!(r#"... "#);
    {
        assert!({ Some(0u8).as_slice() == [0u8] });
        assert!({ Some(255u8).as_slice() == [255u8] });
        assert!({ Some(18u8).as_slice() == [18u8] });
        assert!({ Some(215u8).as_slice() == [215u8] });
        assert!({ Some(131u8).as_slice() == [131u8] });
        assert!({ Some(214u8).as_slice() == [214u8] });
        assert!({ Some(126u8).as_slice() == [126u8] });
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`Vec`]
pub fn slice_get() {
    eprintln!(r#"Testing "Properties for [`Vec`]"... (1 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(vec![0u8] .get(0usize) == Some(& 0u8));
        assert!(vec![0u8] .get(0usize) == Some(& 0u8));
        assert!(vec![255u8] .get(0usize) == Some(& 255u8));
        assert!(vec![255u8] .get(0usize) == Some(& 255u8));
        assert!(vec![108u8] .get(0usize) == Some(& 108u8));
        assert!(vec![141u8, 220u8] .get(1usize) == Some(& 220u8));
        assert!(vec![240u8] .get(0usize) == Some(& 240u8));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Sub::<i8>::sub`]
pub fn core_ops_sub_i8_sub() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Sub::<i8>::sub`]"... (2 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(- 128i8 - - 128i8 == 0i8);
        assert!(- 128i8 - 0i8 == - 128i8);
        assert!(0i8 - 0i8 == 0i8);
        assert!(0i8 - 127i8 == - 127i8);
        assert!(127i8 - 0i8 == 127i8);
        assert!(127i8 - 127i8 == 0i8);
        assert!(27i8 - 3i8 == 24i8);
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(- 128i8 - 127i8));
        assert!(panics!(0i8 - - 128i8));
        assert!(panics!(127i8 - - 128i8));
        assert!(panics!(- 25i8 - 120i8));
        assert!(panics!(124i8 - - 122i8));
        assert!(panics!(43i8 - - 96i8));
        assert!(panics!(80i8 - - 84i8));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<i8>::add`]
pub fn core_ops_add_i8_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<i8>::add`]"... (2 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(- 128i8 + 0i8 == - 128i8);
        assert!(- 128i8 + 127i8 == - 1i8);
        assert!(0i8 + - 128i8 == - 128i8);
        assert!(0i8 + 0i8 == 0i8);
        assert!(0i8 + 127i8 == 127i8);
        assert!(127i8 + - 128i8 == - 1i8);
        assert!(127i8 + 0i8 == 127i8);
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(- 128i8 + - 128i8));
        assert!(panics!(127i8 + 127i8));
        assert!(panics!(123i8 + 74i8));
        assert!(panics!(- 111i8 + - 78i8));
        assert!(panics!(- 116i8 + - 41i8));
        assert!(panics!(- 46i8 + - 99i8));
        assert!(panics!(- 45i8 + - 99i8));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u8>::add`]
pub fn core_ops_add_u8_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<u8>::add`]"... (6 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(0u8 + 0u8 == 0u8);
        assert!(0u8 + 255u8 == 255u8);
        assert!(255u8 + 0u8 == 255u8);
        assert!(170u8 + 78u8 == 248u8);
        assert!(99u8 + 92u8 == 191u8);
        assert!(136u8 + 33u8 == 169u8);
        assert!(27u8 + 7u8 == 34u8);
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(255u8 + 255u8));
        assert!(panics!(139u8 + 135u8));
        assert!(panics!(246u8 + 94u8));
        assert!(panics!(139u8 + 164u8));
        assert!(panics!(241u8 + 52u8));
    }
    eprint!(r#"... "#);
    {
        assert!(0u8 + 0u8 == 0u8 + 0u8);
        assert!(0u8 + 255u8 == 255u8 + 0u8);
        assert!(255u8 + 0u8 == 0u8 + 255u8);
        assert!(93u8 + 15u8 == 15u8 + 93u8);
        assert!(239u8 + 14u8 == 14u8 + 239u8);
        assert!(131u8 + 18u8 == 18u8 + 131u8);
        assert!(47u8 + 89u8 == 89u8 + 47u8);
    }
    eprint!(r#"... "#);
    {
        assert!(0u8 + 0 == 0u8);
        assert!(255u8 + 0 == 255u8);
        assert!(229u8 + 0 == 229u8);
        assert!(151u8 + 0 == 151u8);
        assert!(106u8 + 0 == 106u8);
        assert!(29u8 + 0 == 29u8);
        assert!(193u8 + 0 == 193u8);
    }
    eprint!(r#"... "#);
    {
        assert!(0 + 0u8 == 0u8);
        assert!(0 + 255u8 == 255u8);
        assert!(0 + 14u8 == 14u8);
        assert!(0 + 94u8 == 94u8);
        assert!(0 + 180u8 == 180u8);
        assert!(0 + 46u8 == 46u8);
        assert!(0 + 126u8 == 126u8);
    }
    eprint!(r#"... "#);
    {
        assert!((0u8 + 0u8) + 0u8 == 0u8 + (0u8 + 0u8));
        assert!((0u8 + 0u8) + 255u8 == 0u8 + (0u8 + 255u8));
        assert!((0u8 + 255u8) + 0u8 == 0u8 + (255u8 + 0u8));
        assert!((255u8 + 0u8) + 0u8 == 255u8 + (0u8 + 0u8));
        assert!((107u8 + 46u8) + 10u8 == 107u8 + (46u8 + 10u8));
        assert!((115u8 + 97u8) + 29u8 == 115u8 + (97u8 + 29u8));
        assert!((98u8 + 8u8) + 23u8 == 98u8 + (8u8 + 23u8));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`u8::checked_add`]
pub fn core_ops_add_u8_checked_add() {
    eprintln!(r#"Testing "Properties for [`u8::checked_add`]"... (6 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(0u8.checked_add(255u8) == Some(255u8));
        assert!(255u8.checked_add(0u8) == Some(255u8));
        assert!(108u8.checked_add(27u8) == Some(135u8));
        assert!(22u8.checked_add(215u8) == Some(237u8));
        assert!(3u8.checked_add(112u8) == Some(115u8));
        assert!(42u8.checked_add(13u8) == Some(55u8));
    }
    eprint!(r#"... "#);
    {
        assert!(255u8.checked_add(255u8) == None);
        assert!(215u8.checked_add(47u8) == None);
        assert!(149u8.checked_add(158u8) == None);
        assert!(84u8.checked_add(201u8) == None);
        assert!(189u8.checked_add(83u8) == None);
    }
    eprint!(r#"... "#);
    {
        assert!(0u8.checked_add(0u8) == 0u8.checked_add(0u8));
        assert!(0u8.checked_add(255u8) == 255u8.checked_add(0u8));
        assert!(255u8.checked_add(0u8) == 0u8.checked_add(255u8));
        assert!(255u8.checked_add(255u8) == 255u8.checked_add(255u8));
        assert!(181u8.checked_add(99u8) == 99u8.checked_add(181u8));
        assert!(102u8.checked_add(224u8) == 224u8.checked_add(102u8));
        assert!(157u8.checked_add(50u8) == 50u8.checked_add(157u8));
    }
    eprint!(r#"... "#);
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(255u8.checked_add(0u8) == Some(255u8));
        assert!(230u8.checked_add(0u8) == Some(230u8));
        assert!(169u8.checked_add(0u8) == Some(169u8));
        assert!(76u8.checked_add(0u8) == Some(76u8));
        assert!(136u8.checked_add(0u8) == Some(136u8));
        assert!(235u8.checked_add(0u8) == Some(235u8));
    }
    eprint!(r#"... "#);
    {
        assert!(0u8.checked_add(0u8) == Some(0u8));
        assert!(0u8.checked_add(255u8) == Some(255u8));
        assert!(0u8.checked_add(23u8) == Some(23u8));
        assert!(0u8.checked_add(8u8) == Some(8u8));
        assert!(0u8.checked_add(235u8) == Some(235u8));
        assert!(0u8.checked_add(107u8) == Some(107u8));
        assert!(0u8.checked_add(54u8) == Some(54u8));
    }
    eprint!(r#"... "#);
    {
        assert!(
            0u8.checked_add(0u8).and_then(| iv | iv.checked_add(0u8)) == 0u8
            .checked_add(0u8).and_then(| iv | 0u8.checked_add(iv))
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
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u16>::add`]
pub fn core_ops_add_u16_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<u16>::add`]"... (6 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(0u16 + 0u16 == 0u16);
        assert!(0u16 + 65535u16 == 65535u16);
        assert!(65535u16 + 0u16 == 65535u16);
        assert!(11249u16 + 44979u16 == 56228u16);
        assert!(38420u16 + 18606u16 == 57026u16);
        assert!(24401u16 + 32548u16 == 56949u16);
        assert!(44143u16 + 11061u16 == 55204u16);
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(65535u16 + 65535u16));
        assert!(panics!(54913u16 + 59287u16));
        assert!(panics!(54159u16 + 48966u16));
        assert!(panics!(59892u16 + 56726u16));
        assert!(panics!(28870u16 + 38570u16));
    }
    eprint!(r#"... "#);
    {
        assert!(0u16 + 0u16 == 0u16 + 0u16);
        assert!(0u16 + 65535u16 == 65535u16 + 0u16);
        assert!(65535u16 + 0u16 == 0u16 + 65535u16);
        assert!(18292u16 + 10596u16 == 10596u16 + 18292u16);
        assert!(6244u16 + 19713u16 == 19713u16 + 6244u16);
        assert!(27998u16 + 8593u16 == 8593u16 + 27998u16);
        assert!(37207u16 + 23617u16 == 23617u16 + 37207u16);
    }
    eprint!(r#"... "#);
    {
        assert!(0u16 + 0 == 0u16);
        assert!(65535u16 + 0 == 65535u16);
        assert!(43255u16 + 0 == 43255u16);
        assert!(58948u16 + 0 == 58948u16);
        assert!(2746u16 + 0 == 2746u16);
        assert!(10924u16 + 0 == 10924u16);
        assert!(24256u16 + 0 == 24256u16);
    }
    eprint!(r#"... "#);
    {
        assert!(0 + 0u16 == 0u16);
        assert!(0 + 65535u16 == 65535u16);
        assert!(0 + 34108u16 == 34108u16);
        assert!(0 + 65455u16 == 65455u16);
        assert!(0 + 2967u16 == 2967u16);
        assert!(0 + 57210u16 == 57210u16);
        assert!(0 + 32188u16 == 32188u16);
    }
    eprint!(r#"... "#);
    {
        assert!((0u16 + 0u16) + 0u16 == 0u16 + (0u16 + 0u16));
        assert!((0u16 + 0u16) + 65535u16 == 0u16 + (0u16 + 65535u16));
        assert!((0u16 + 65535u16) + 0u16 == 0u16 + (65535u16 + 0u16));
        assert!((65535u16 + 0u16) + 0u16 == 65535u16 + (0u16 + 0u16));
        assert!((11502u16 + 24998u16) + 11595u16 == 11502u16 + (24998u16 + 11595u16));
        assert!((13712u16 + 22673u16) + 13104u16 == 13712u16 + (22673u16 + 13104u16));
        assert!((29822u16 + 16694u16) + 11801u16 == 29822u16 + (16694u16 + 11801u16));
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`u16::checked_add`]
pub fn core_ops_add_u16_checked_add() {
    eprintln!(r#"Testing "Properties for [`u16::checked_add`]"... (6 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(0u16.checked_add(65535u16) == Some(65535u16));
        assert!(65535u16.checked_add(0u16) == Some(65535u16));
        assert!(46062u16.checked_add(4337u16) == Some(50399u16));
        assert!(30541u16.checked_add(7503u16) == Some(38044u16));
        assert!(45782u16.checked_add(2438u16) == Some(48220u16));
        assert!(56181u16.checked_add(3999u16) == Some(60180u16));
    }
    eprint!(r#"... "#);
    {
        assert!(65535u16.checked_add(65535u16) == None);
        assert!(60042u16.checked_add(38540u16) == None);
        assert!(23616u16.checked_add(60010u16) == None);
        assert!(60924u16.checked_add(61552u16) == None);
        assert!(50130u16.checked_add(32230u16) == None);
    }
    eprint!(r#"... "#);
    {
        assert!(0u16.checked_add(0u16) == 0u16.checked_add(0u16));
        assert!(0u16.checked_add(65535u16) == 65535u16.checked_add(0u16));
        assert!(65535u16.checked_add(0u16) == 0u16.checked_add(65535u16));
        assert!(65535u16.checked_add(65535u16) == 65535u16.checked_add(65535u16));
        assert!(41206u16.checked_add(54723u16) == 54723u16.checked_add(41206u16));
        assert!(60357u16.checked_add(2650u16) == 2650u16.checked_add(60357u16));
        assert!(40472u16.checked_add(5601u16) == 5601u16.checked_add(40472u16));
    }
    eprint!(r#"... "#);
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(65535u16.checked_add(0u16) == Some(65535u16));
        assert!(25103u16.checked_add(0u16) == Some(25103u16));
        assert!(7811u16.checked_add(0u16) == Some(7811u16));
        assert!(49276u16.checked_add(0u16) == Some(49276u16));
        assert!(7199u16.checked_add(0u16) == Some(7199u16));
        assert!(14927u16.checked_add(0u16) == Some(14927u16));
    }
    eprint!(r#"... "#);
    {
        assert!(0u16.checked_add(0u16) == Some(0u16));
        assert!(0u16.checked_add(65535u16) == Some(65535u16));
        assert!(0u16.checked_add(23212u16) == Some(23212u16));
        assert!(0u16.checked_add(61322u16) == Some(61322u16));
        assert!(0u16.checked_add(11112u16) == Some(11112u16));
        assert!(0u16.checked_add(26638u16) == Some(26638u16));
        assert!(0u16.checked_add(30642u16) == Some(30642u16));
    }
    eprint!(r#"... "#);
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
            0u16.checked_add(65535u16).and_then(| iv | iv.checked_add(65535u16)) ==
            65535u16.checked_add(65535u16).and_then(| iv | 0u16.checked_add(iv))
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
            65535u16.checked_add(65535u16).and_then(| iv | iv.checked_add(0u16)) ==
            65535u16.checked_add(0u16).and_then(| iv | 65535u16.checked_add(iv))
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u32>::add`]
pub fn core_ops_add_u32_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<u32>::add`]"... (6 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(0u32 + 0u32 == 0u32);
        assert!(0u32 + 4294967295u32 == 4294967295u32);
        assert!(4294967295u32 + 0u32 == 4294967295u32);
        assert!(1113022419u32 + 1977629652u32 == 3090652071u32);
        assert!(847530100u32 + 2135667509u32 == 2983197609u32);
        assert!(22180556u32 + 2805604047u32 == 2827784603u32);
        assert!(823001358u32 + 735072366u32 == 1558073724u32);
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(4294967295u32 + 4294967295u32));
        assert!(panics!(1176698602u32 + 3901436441u32));
        assert!(panics!(4134731722u32 + 551787203u32));
        assert!(panics!(1497498534u32 + 2847339830u32));
        assert!(panics!(2347911205u32 + 3101046623u32));
    }
    eprint!(r#"... "#);
    {
        assert!(0u32 + 0u32 == 0u32 + 0u32);
        assert!(0u32 + 4294967295u32 == 4294967295u32 + 0u32);
        assert!(4294967295u32 + 0u32 == 0u32 + 4294967295u32);
        assert!(972661023u32 + 2680396401u32 == 2680396401u32 + 972661023u32);
        assert!(3748731811u32 + 406118275u32 == 406118275u32 + 3748731811u32);
        assert!(733129528u32 + 669569141u32 == 669569141u32 + 733129528u32);
        assert!(854882402u32 + 41686381u32 == 41686381u32 + 854882402u32);
    }
    eprint!(r#"... "#);
    {
        assert!(0u32 + 0 == 0u32);
        assert!(4294967295u32 + 0 == 4294967295u32);
        assert!(243378883u32 + 0 == 243378883u32);
        assert!(4073750699u32 + 0 == 4073750699u32);
        assert!(2270074857u32 + 0 == 2270074857u32);
        assert!(417256132u32 + 0 == 417256132u32);
        assert!(489013669u32 + 0 == 489013669u32);
    }
    eprint!(r#"... "#);
    {
        assert!(0 + 0u32 == 0u32);
        assert!(0 + 4294967295u32 == 4294967295u32);
        assert!(0 + 912853973u32 == 912853973u32);
        assert!(0 + 1806778014u32 == 1806778014u32);
        assert!(0 + 3136432970u32 == 3136432970u32);
        assert!(0 + 3746746262u32 == 3746746262u32);
        assert!(0 + 3141609637u32 == 3141609637u32);
    }
    eprint!(r#"... "#);
    {
        assert!((0u32 + 0u32) + 0u32 == 0u32 + (0u32 + 0u32));
        assert!((0u32 + 0u32) + 4294967295u32 == 0u32 + (0u32 + 4294967295u32));
        assert!((0u32 + 4294967295u32) + 0u32 == 0u32 + (4294967295u32 + 0u32));
        assert!((4294967295u32 + 0u32) + 0u32 == 4294967295u32 + (0u32 + 0u32));
        assert!(
            (1159101526u32 + 1970943638u32) + 745456067u32 == 1159101526u32 +
            (1970943638u32 + 745456067u32)
        );
        assert!(
            (441662636u32 + 1485688977u32) + 137806018u32 == 441662636u32 +
            (1485688977u32 + 137806018u32)
        );
        assert!(
            (123644037u32 + 268912338u32) + 3619402573u32 == 123644037u32 + (268912338u32
            + 3619402573u32)
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`u32::checked_add`]
pub fn core_ops_add_u32_checked_add() {
    eprintln!(r#"Testing "Properties for [`u32::checked_add`]"... (6 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
        assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
        assert!(733210028u32.checked_add(1816083133u32) == Some(2549293161u32));
        assert!(117219081u32.checked_add(2949497195u32) == Some(3066716276u32));
        assert!(305240092u32.checked_add(1664994344u32) == Some(1970234436u32));
        assert!(423292212u32.checked_add(1955258074u32) == Some(2378550286u32));
    }
    eprint!(r#"... "#);
    {
        assert!(4294967295u32.checked_add(4294967295u32) == None);
        assert!(1890522264u32.checked_add(2649357131u32) == None);
        assert!(2219304216u32.checked_add(2760456207u32) == None);
        assert!(1748014252u32.checked_add(2701875080u32) == None);
        assert!(4269971189u32.checked_add(1140412535u32) == None);
    }
    eprint!(r#"... "#);
    {
        assert!(0u32.checked_add(0u32) == 0u32.checked_add(0u32));
        assert!(0u32.checked_add(4294967295u32) == 4294967295u32.checked_add(0u32));
        assert!(4294967295u32.checked_add(0u32) == 0u32.checked_add(4294967295u32));
        assert!(
            4294967295u32.checked_add(4294967295u32) == 4294967295u32
            .checked_add(4294967295u32)
        );
        assert!(
            2978925542u32.checked_add(2542189118u32) == 2542189118u32
            .checked_add(2978925542u32)
        );
        assert!(
            648804476u32.checked_add(2190182634u32) == 2190182634u32
            .checked_add(648804476u32)
        );
        assert!(
            4092167640u32.checked_add(2041777482u32) == 2041777482u32
            .checked_add(4092167640u32)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
        assert!(2743979790u32.checked_add(0u32) == Some(2743979790u32));
        assert!(2101447684u32.checked_add(0u32) == Some(2101447684u32));
        assert!(2703516016u32.checked_add(0u32) == Some(2703516016u32));
        assert!(1418619811u32.checked_add(0u32) == Some(1418619811u32));
        assert!(1439404781u32.checked_add(0u32) == Some(1439404781u32));
    }
    eprint!(r#"... "#);
    {
        assert!(0u32.checked_add(0u32) == Some(0u32));
        assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
        assert!(0u32.checked_add(3060924575u32) == Some(3060924575u32));
        assert!(0u32.checked_add(87364452u32) == Some(87364452u32));
        assert!(0u32.checked_add(4232571359u32) == Some(4232571359u32));
        assert!(0u32.checked_add(2536057809u32) == Some(2536057809u32));
        assert!(0u32.checked_add(3685172592u32) == Some(3685172592u32));
    }
    eprint!(r#"... "#);
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
            0u32.checked_add(4294967295u32).and_then(| iv | iv
            .checked_add(4294967295u32)) == 4294967295u32.checked_add(4294967295u32)
            .and_then(| iv | 0u32.checked_add(iv))
        );
        assert!(
            4294967295u32.checked_add(0u32).and_then(| iv | iv.checked_add(0u32)) == 0u32
            .checked_add(0u32).and_then(| iv | 4294967295u32.checked_add(iv))
        );
        assert!(
            4294967295u32.checked_add(0u32).and_then(| iv | iv
            .checked_add(4294967295u32)) == 0u32.checked_add(4294967295u32).and_then(| iv
            | 4294967295u32.checked_add(iv))
        );
        assert!(
            4294967295u32.checked_add(4294967295u32).and_then(| iv | iv
            .checked_add(0u32)) == 4294967295u32.checked_add(0u32).and_then(| iv |
            4294967295u32.checked_add(iv))
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u64>::add`]
pub fn core_ops_add_u64_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<u64>::add`]"... (6 contracts)"#
    );
    eprint!(r#"  "#);
    {
        assert!(0u64 + 0u64 == 0u64);
        assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64);
        assert!(18446744073709551615u64 + 0u64 == 18446744073709551615u64);
        assert!(
            11871368544704088055u64 + 888011700992038935u64 == 12759380245696126990u64
        );
        assert!(
            11350891116549680892u64 + 4242852414073054584u64 == 15593743530622735476u64
        );
        assert!(
            497512941569413017u64 + 7372574157137864387u64 == 7870087098707277404u64
        );
        assert!(
            9520668372638770033u64 + 1961869264817343129u64 == 11482537637456113162u64
        );
    }
    eprint!(r#"... "#);
    {
        assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
        assert!(panics!(17408940985326541082u64 + 5572807933174964009u64));
        assert!(panics!(16403860446215605306u64 + 7136513698375779943u64));
        assert!(panics!(15660804276946993070u64 + 13856670767958437915u64));
        assert!(panics!(18408376184370624417u64 + 12632284705455054404u64));
    }
    eprint!(r#"... "#);
    {
        assert!(0u64 + 0u64 == 0u64 + 0u64);
        assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64 + 0u64);
        assert!(18446744073709551615u64 + 0u64 == 0u64 + 18446744073709551615u64);
        assert!(
            10758124627942853055u64 + 4307736027674636938u64 == 4307736027674636938u64 +
            10758124627942853055u64
        );
        assert!(
            10465776571036549980u64 + 7667941233865951239u64 == 7667941233865951239u64 +
            10465776571036549980u64
        );
        assert!(
            248808288612872646u64 + 12266172561417304657u64 == 12266172561417304657u64 +
            248808288612872646u64
        );
        assert!(
            1801422218836205355u64 + 380912020150770053u64 == 380912020150770053u64 +
            1801422218836205355u64
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u64 + 0 == 0u64);
        assert!(18446744073709551615u64 + 0 == 18446744073709551615u64);
        assert!(618574018569978074u64 + 0 == 618574018569978074u64);
        assert!(18118863400495534654u64 + 0 == 18118863400495534654u64);
        assert!(11028282462458603231u64 + 0 == 11028282462458603231u64);
        assert!(9615630270236151408u64 + 0 == 9615630270236151408u64);
        assert!(1897460097634401732u64 + 0 == 1897460097634401732u64);
    }
    eprint!(r#"... "#);
    {
        assert!(0 + 0u64 == 0u64);
        assert!(0 + 18446744073709551615u64 == 18446744073709551615u64);
        assert!(0 + 14590073470324539901u64 == 14590073470324539901u64);
        assert!(0 + 8167365366061881235u64 == 8167365366061881235u64);
        assert!(0 + 13575968443785167087u64 == 13575968443785167087u64);
        assert!(0 + 17156924991387194427u64 == 17156924991387194427u64);
        assert!(0 + 5631214915770259577u64 == 5631214915770259577u64);
    }
    eprint!(r#"... "#);
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
            (4831166760179866709u64 + 4381723276812744171u64) + 9063414266465401720u64 ==
            4831166760179866709u64 + (4381723276812744171u64 + 9063414266465401720u64)
        );
        assert!(
            (3283240233562121164u64 + 10923207668211229844u64) + 1376106991170207504u64
            == 3283240233562121164u64 + (10923207668211229844u64 +
            1376106991170207504u64)
        );
        assert!(
            (3457201212874433870u64 + 7272927831482578559u64) + 1464277534800466864u64 ==
            3457201212874433870u64 + (7272927831482578559u64 + 1464277534800466864u64)
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`u64::checked_add`]
pub fn core_ops_add_u64_checked_add() {
    eprintln!(r#"Testing "Properties for [`u64::checked_add`]"... (6 contracts)"#);
    eprint!(r#"  "#);
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(
            0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64)
        );
        assert!(
            18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64)
        );
        assert!(
            6421644081636274296u64.checked_add(1975814712575291192u64) ==
            Some(8397458794211565488u64)
        );
        assert!(
            5587963983248741622u64.checked_add(6260766345293237330u64) ==
            Some(11848730328541978952u64)
        );
        assert!(
            9994728396847391065u64.checked_add(418551447663990640u64) ==
            Some(10413279844511381705u64)
        );
        assert!(
            4170087122906035120u64.checked_add(8233713955671513950u64) ==
            Some(12403801078577549070u64)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
        assert!(16738380969218209391u64.checked_add(1734452118507359949u64) == None);
        assert!(13245733445337050152u64.checked_add(15971657778455176311u64) == None);
        assert!(9229198604863180362u64.checked_add(12381578734394965130u64) == None);
        assert!(9544075793982093323u64.checked_add(15518938423964776672u64) == None);
    }
    eprint!(r#"... "#);
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
            2414861540101794906u64.checked_add(4773178533036005721u64) ==
            4773178533036005721u64.checked_add(2414861540101794906u64)
        );
        assert!(
            13310688041786787170u64.checked_add(12004327392646337751u64) ==
            12004327392646337751u64.checked_add(13310688041786787170u64)
        );
        assert!(
            17221613034456464262u64.checked_add(7479145416433949492u64) ==
            7479145416433949492u64.checked_add(17221613034456464262u64)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(
            18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64)
        );
        assert!(
            2797366007693812659u64.checked_add(0u64) == Some(2797366007693812659u64)
        );
        assert!(
            5554164456954812873u64.checked_add(0u64) == Some(5554164456954812873u64)
        );
        assert!(
            6065242237336584507u64.checked_add(0u64) == Some(6065242237336584507u64)
        );
        assert!(
            6284473443622061021u64.checked_add(0u64) == Some(6284473443622061021u64)
        );
        assert!(
            16204980333079000017u64.checked_add(0u64) == Some(16204980333079000017u64)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u64.checked_add(0u64) == Some(0u64));
        assert!(
            0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64)
        );
        assert!(0u64.checked_add(372627513004441261u64) == Some(372627513004441261u64));
        assert!(
            0u64.checked_add(5225006725455295128u64) == Some(5225006725455295128u64)
        );
        assert!(0u64.checked_add(245831050827445708u64) == Some(245831050827445708u64));
        assert!(
            0u64.checked_add(2502366769960902339u64) == Some(2502366769960902339u64)
        );
        assert!(
            0u64.checked_add(3305599750053080643u64) == Some(3305599750053080643u64)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(
            0u64.checked_add(0u64).and_then(| iv | iv.checked_add(0u64)) == 0u64
            .checked_add(0u64).and_then(| iv | 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(0u64).and_then(| iv | iv
            .checked_add(18446744073709551615u64)) == 0u64
            .checked_add(18446744073709551615u64).and_then(| iv | 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(18446744073709551615u64).and_then(| iv | iv
            .checked_add(0u64)) == 18446744073709551615u64.checked_add(0u64).and_then(|
            iv | 0u64.checked_add(iv))
        );
        assert!(
            0u64.checked_add(18446744073709551615u64).and_then(| iv | iv
            .checked_add(18446744073709551615u64)) == 18446744073709551615u64
            .checked_add(18446744073709551615u64).and_then(| iv | 0u64.checked_add(iv))
        );
        assert!(
            18446744073709551615u64.checked_add(0u64).and_then(| iv | iv
            .checked_add(0u64)) == 0u64.checked_add(0u64).and_then(| iv |
            18446744073709551615u64.checked_add(iv))
        );
        assert!(
            18446744073709551615u64.checked_add(0u64).and_then(| iv | iv
            .checked_add(18446744073709551615u64)) == 0u64
            .checked_add(18446744073709551615u64).and_then(| iv | 18446744073709551615u64
            .checked_add(iv))
        );
        assert!(
            18446744073709551615u64.checked_add(18446744073709551615u64).and_then(| iv |
            iv.checked_add(0u64)) == 18446744073709551615u64.checked_add(0u64).and_then(|
            iv | 18446744073709551615u64.checked_add(iv))
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`core::ops::Add::<u128>::add`]
pub fn core_ops_add_u128_add() {
    eprintln!(
        r#"Testing "Properties for [`core::ops::Add::<u128>::add`]"... (6 contracts)"#
    );
    eprint!(r#"  "#);
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
            52118741192020004367119803800001461165u128 +
            32394825483565375382071303074018972767u128 ==
            84513566675585379749191106874020433932u128
        );
        assert!(
            30857817412627131257419110161216680934u128 +
            111607922393433224012358613843813532896u128 ==
            142465739806060355269777724005030213830u128
        );
        assert!(
            62531181654939346226968474067026981219u128 +
            82976781304335242217982221804724507236u128 ==
            145507962959274588444950695871751488455u128
        );
        assert!(
            57480491386376494270453632295995493336u128 +
            67438813556729444904213322755875497744u128 ==
            124919304943105939174666955051870991080u128
        );
    }
    eprint!(r#"... "#);
    {
        assert!(
            panics!(340282366920938463463374607431768211455u128 +
            340282366920938463463374607431768211455u128)
        );
        assert!(
            panics!(304729255982784698050401176630358565864u128 +
            246092185862351841961398637806498643314u128)
        );
        assert!(
            panics!(231583563439984710731710747125776287866u128 +
            201167471374747631758258845457373798598u128)
        );
        assert!(
            panics!(289520500223168794718019099630616242398u128 +
            250219856797968508128097317497906499561u128)
        );
        assert!(
            panics!(158480307502663174219261828881678548762u128 +
            303036418102004821058017974648419110404u128)
        );
    }
    eprint!(r#"... "#);
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
            104113893909807863542821623763323396578u128 +
            113359918103923353212891714347505499989u128 ==
            113359918103923353212891714347505499989u128 +
            104113893909807863542821623763323396578u128
        );
        assert!(
            153189853854053165956622513180694671298u128 +
            180765415661066254848562026618010297731u128 ==
            180765415661066254848562026618010297731u128 +
            153189853854053165956622513180694671298u128
        );
        assert!(
            242404325068749086077326329968872505516u128 +
            48830838652318518924674314319500674373u128 ==
            48830838652318518924674314319500674373u128 +
            242404325068749086077326329968872505516u128
        );
        assert!(
            7716550941286034310901884393064555039u128 +
            214265892041566403893226896633770875923u128 ==
            214265892041566403893226896633770875923u128 +
            7716550941286034310901884393064555039u128
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u128 + 0 == 0u128);
        assert!(
            340282366920938463463374607431768211455u128 + 0 ==
            340282366920938463463374607431768211455u128
        );
        assert!(
            208679760187711222092234600280620218905u128 + 0 ==
            208679760187711222092234600280620218905u128
        );
        assert!(
            254515581157111815171908781040291506688u128 + 0 ==
            254515581157111815171908781040291506688u128
        );
        assert!(
            68249478854860180228768214461596920639u128 + 0 ==
            68249478854860180228768214461596920639u128
        );
        assert!(
            303114730350188113105657230788108688814u128 + 0 ==
            303114730350188113105657230788108688814u128
        );
        assert!(
            24889589056419785914457194540251549286u128 + 0 ==
            24889589056419785914457194540251549286u128
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0 + 0u128 == 0u128);
        assert!(
            0 + 340282366920938463463374607431768211455u128 ==
            340282366920938463463374607431768211455u128
        );
        assert!(
            0 + 118161551329733355621697751123271393725u128 ==
            118161551329733355621697751123271393725u128
        );
        assert!(
            0 + 236517319423271216909496246633447694678u128 ==
            236517319423271216909496246633447694678u128
        );
        assert!(
            0 + 262775740016103845022162152598166628851u128 ==
            262775740016103845022162152598166628851u128
        );
        assert!(
            0 + 127752164075952016463164275435037271429u128 ==
            127752164075952016463164275435037271429u128
        );
        assert!(
            0 + 174891994191968848750246435175535156536u128 ==
            174891994191968848750246435175535156536u128
        );
    }
    eprint!(r#"... "#);
    {
        assert!((0u128 + 0u128) + 0u128 == 0u128 + (0u128 + 0u128));
        assert!(
            (0u128 + 0u128) + 340282366920938463463374607431768211455u128 == 0u128 +
            (0u128 + 340282366920938463463374607431768211455u128)
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
            (26722379428880807347411130526663864129u128 +
            247082526874420784304601804380473055041u128) +
            12857151542568655906927240522699745055u128 ==
            26722379428880807347411130526663864129u128 +
            (247082526874420784304601804380473055041u128 +
            12857151542568655906927240522699745055u128)
        );
        assert!(
            (50206869902125921366118396906901670084u128 +
            82888956811223442925349255869331597429u128) +
            155875025336365988923758100077473903601u128 ==
            50206869902125921366118396906901670084u128 +
            (82888956811223442925349255869331597429u128 +
            155875025336365988923758100077473903601u128)
        );
        assert!(
            (17346417100613710069605018832947627826u128 +
            157007940838945566328333423530779625566u128) +
            20018532810973968440570735001516839424u128 ==
            17346417100613710069605018832947627826u128 +
            (157007940838945566328333423530779625566u128 +
            20018532810973968440570735001516839424u128)
        );
    }
    eprint!(r#"... "#);
    eprintln!("✓\n");
}
/// Properties for [`u128::checked_add`]
pub fn core_ops_add_u128_checked_add() {
    eprintln!(r#"Testing "Properties for [`u128::checked_add`]"... (6 contracts)"#);
    eprint!(r#"  "#);
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
            105536665146264506134698334369477351128u128
            .checked_add(124494546673903300121595340929144936140u128) ==
            Some(230031211820167806256293675298622287268u128)
        );
        assert!(
            10591899265453305966498696797906544171u128
            .checked_add(111039354807395278282575820159701164279u128) ==
            Some(121631254072848584249074516957607708450u128)
        );
        assert!(
            30543869825606394322204668012140998684u128
            .checked_add(284255106985365315627123663071950891158u128) ==
            Some(314798976810971709949328331084091889842u128)
        );
        assert!(
            320127786971442254908830862512111624706u128
            .checked_add(619079596870004294905005359398987852u128) ==
            Some(320746866568312259203735867871510612558u128)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(
            340282366920938463463374607431768211455u128
            .checked_add(340282366920938463463374607431768211455u128) == None
        );
        assert!(
            270467049828537614618764599626770980812u128
            .checked_add(288126481526285981805704370192334052987u128) == None
        );
        assert!(
            36045971748951525716554327082115614094u128
            .checked_add(334381050094727626233590565272250941002u128) == None
        );
        assert!(
            111986577799617418025916371268163844122u128
            .checked_add(243584389146707846720913375133312066999u128) == None
        );
        assert!(
            56403673948442921958739413523681560011u128
            .checked_add(303894913264340237149562877572384391927u128) == None
        );
    }
    eprint!(r#"... "#);
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
            145751601124010921353479323471917357878u128
            .checked_add(6735561913568735039241032532044168646u128) ==
            6735561913568735039241032532044168646u128
            .checked_add(145751601124010921353479323471917357878u128)
        );
        assert!(
            282705233823680694728756139015676226473u128
            .checked_add(323948863570888356929573058758294145279u128) ==
            323948863570888356929573058758294145279u128
            .checked_add(282705233823680694728756139015676226473u128)
        );
        assert!(
            135837792909016589480713516885864963892u128
            .checked_add(214385952061470388115965833463029563423u128) ==
            214385952061470388115965833463029563423u128
            .checked_add(135837792909016589480713516885864963892u128)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u128.checked_add(0u128) == Some(0u128));
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128) ==
            Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            131549467034166332901059641356837276926u128.checked_add(0u128) ==
            Some(131549467034166332901059641356837276926u128)
        );
        assert!(
            53550988151919353031835073824315121708u128.checked_add(0u128) ==
            Some(53550988151919353031835073824315121708u128)
        );
        assert!(
            5762964146128519402181475002220796759u128.checked_add(0u128) ==
            Some(5762964146128519402181475002220796759u128)
        );
        assert!(
            175344093148714898200802848363235616513u128.checked_add(0u128) ==
            Some(175344093148714898200802848363235616513u128)
        );
        assert!(
            24494802265958941267650736120226515915u128.checked_add(0u128) ==
            Some(24494802265958941267650736120226515915u128)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(0u128.checked_add(0u128) == Some(0u128));
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128) ==
            Some(340282366920938463463374607431768211455u128)
        );
        assert!(
            0u128.checked_add(223393970518338524840982936410025334088u128) ==
            Some(223393970518338524840982936410025334088u128)
        );
        assert!(
            0u128.checked_add(205406436429678396900677614243975148419u128) ==
            Some(205406436429678396900677614243975148419u128)
        );
        assert!(
            0u128.checked_add(232819812947974530198024205277468994823u128) ==
            Some(232819812947974530198024205277468994823u128)
        );
        assert!(
            0u128.checked_add(2555104023491319561175488322427128910u128) ==
            Some(2555104023491319561175488322427128910u128)
        );
        assert!(
            0u128.checked_add(127463225468834824516058818947122317193u128) ==
            Some(127463225468834824516058818947122317193u128)
        );
    }
    eprint!(r#"... "#);
    {
        assert!(
            0u128.checked_add(0u128).and_then(| iv | iv.checked_add(0u128)) == 0u128
            .checked_add(0u128).and_then(| iv | 0u128.checked_add(iv))
        );
        assert!(
            0u128.checked_add(0u128).and_then(| iv | iv
            .checked_add(340282366920938463463374607431768211455u128)) == 0u128
            .checked_add(340282366920938463463374607431768211455u128).and_then(| iv |
            0u128.checked_add(iv))
        );
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128).and_then(| iv
            | iv.checked_add(0u128)) == 340282366920938463463374607431768211455u128
            .checked_add(0u128).and_then(| iv | 0u128.checked_add(iv))
        );
        assert!(
            0u128.checked_add(340282366920938463463374607431768211455u128).and_then(| iv
            | iv.checked_add(340282366920938463463374607431768211455u128)) ==
            340282366920938463463374607431768211455u128
            .checked_add(340282366920938463463374607431768211455u128).and_then(| iv |
            0u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128).and_then(| iv
            | iv.checked_add(0u128)) == 0u128.checked_add(0u128).and_then(| iv |
            340282366920938463463374607431768211455u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128.checked_add(0u128).and_then(| iv
            | iv.checked_add(340282366920938463463374607431768211455u128)) == 0u128
            .checked_add(340282366920938463463374607431768211455u128).and_then(| iv |
            340282366920938463463374607431768211455u128.checked_add(iv))
        );
        assert!(
            340282366920938463463374607431768211455u128
            .checked_add(340282366920938463463374607431768211455u128).and_then(| iv | iv
            .checked_add(0u128)) == 340282366920938463463374607431768211455u128
            .checked_add(0u128).and_then(| iv |
            340282366920938463463374607431768211455u128.checked_add(iv))
        );
    }
    eprint!(r#"... "#);
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
