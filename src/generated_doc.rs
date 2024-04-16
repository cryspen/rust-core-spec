//! This module contains placeholder functions decorated with contracts and concrete tests. There are 533 tests.
/// # Properties for [`Option::is_some`]
/// ## `is_some` is a shorthand to pattern matching
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `v.is_some()
///         == (match v {
///             Some(_) => true,
///             None => false,
///         })`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.is_some()
///         == (match Option::<u8>::None {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(0u8).is_some()
///         == (match Some(0u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(255u8).is_some()
///         == (match Some(255u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(31u8).is_some()
///         == (match Some(31u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(119u8).is_some()
///         == (match Some(119u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(27u8).is_some()
///         == (match Some(27u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(38u8).is_some()
///         == (match Some(38u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// # }
/// ```
pub fn option_is_some(){}
/// # Properties for [`Option::is_none`]
/// ## `is_none` is a shorthand to pattern matching
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `v.is_none()
///         == (match v {
///             Some(_) => false,
///             None => true,
///         })`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.is_none()
///         == (match Option::<u8>::None {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(0u8).is_none()
///         == (match Some(0u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(255u8).is_none()
///         == (match Some(255u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(252u8).is_none()
///         == (match Some(252u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(77u8).is_none()
///         == (match Some(77u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(89u8).is_none()
///         == (match Some(89u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(83u8).is_none()
///         == (match Some(83u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// # }
/// ```
pub fn option_is_none(){}
/// # Properties for [`Option::expect`]
/// ## Unwrapping a [`None`] with `expect` always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.expect("message"))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(Option::< u8 >::None.expect("message")));
/// # }
/// ```
/// ## Unwrapping a [`Some(_)`] with `expect` always succeeds
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_some()`  
/// __Postcondition:__ `doesn_t_panic!(v.expect("message"))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).expect("message")));
/// assert!(doesn_t_panic!(Some(255u8).expect("message")));
/// assert!(doesn_t_panic!(Some(81u8).expect("message")));
/// assert!(doesn_t_panic!(Some(21u8).expect("message")));
/// assert!(doesn_t_panic!(Some(29u8).expect("message")));
/// assert!(doesn_t_panic!(Some(156u8).expect("message")));
/// assert!(doesn_t_panic!(Some(107u8).expect("message")));
/// # }
/// ```
/// ## Wrapping a value in a `Some` and unwrapping is identity
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).unwrap() == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).unwrap() == 0u8);
/// assert!(Some(255u8).unwrap() == 255u8);
/// assert!(Some(155u8).unwrap() == 155u8);
/// assert!(Some(199u8).unwrap() == 199u8);
/// assert!(Some(119u8).unwrap() == 119u8);
/// assert!(Some(139u8).unwrap() == 139u8);
/// assert!(Some(78u8).unwrap() == 78u8);
/// # }
/// ```
pub fn option_expect(){}
/// # Properties for [`Option::map`]
/// ## Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`
/// __Inputs:__ `v : T, f : Fn1<T, T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).map(f) == Some((f)(v))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(x)))
///         == Some(((|x: u8| x.wrapping_add(x)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(0u8)))
///         == Some(((|x: u8| x.wrapping_add(0u8)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(255u8)))
///         == Some(((|x: u8| x.wrapping_add(255u8)))(0u8)));
/// assert!(Some(255u8).map((|x: u8| x.wrapping_add(x)))
///         == Some(((|x: u8| x.wrapping_add(x)))(255u8)));
/// assert!(Some(255u8).map((|x: u8| x.wrapping_add(0u8)))
///         == Some(((|x: u8| x.wrapping_add(0u8)))(255u8)));
/// assert!(Some(255u8).map((|x: u8| x.wrapping_add(255u8)))
///         == Some(((|x: u8| x.wrapping_add(255u8)))(255u8)));
/// assert!(Some(26u8).map((|x: u8| x)) == Some(((|x: u8| x))(26u8)));
/// # }
/// ```
/// ## Mapping a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : Fn1<T, T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.map(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.map((|x: u8| x.wrapping_add(x))) == Option::<u8>::None);
/// # }
/// ```
pub fn option_map(){}
/// # Properties for [`Option::filter`]
/// ## The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`
/// __Inputs:__ `v : T, f : FnR1<T, bool>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).filter(f).is_some() == f(&v)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&0u8));
/// assert!(Some(0u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&0u8));
/// assert!(Some(255u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&255u8));
/// assert!(Some(255u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&255u8));
/// assert!(Some(102u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&102u8));
/// assert!(Some(128u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&128u8));
/// assert!(Some(32u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&32u8));
/// # }
/// ```
/// ## Filtering a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : FnR1<T, bool>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.filter(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.filter((|x: &u8| *x < 0u8)) == Option::<u8>::None);
/// # }
/// ```
pub fn option_filter(){}
/// # Properties for [`Option::flatten`]
/// ## Nested `Some`s
/// __Inputs:__ `x : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(Some(x)).flatten() == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(Some(0u8)).flatten() == Some(0u8));
/// assert!(Some(Some(255u8)).flatten() == Some(255u8));
/// assert!(Some(Some(172u8)).flatten() == Some(172u8));
/// assert!(Some(Some(158u8)).flatten() == Some(158u8));
/// assert!(Some(Some(67u8)).flatten() == Some(67u8));
/// assert!(Some(Some(35u8)).flatten() == Some(35u8));
/// assert!(Some(Some(225u8)).flatten() == Some(225u8));
/// # }
/// ```
/// ## Nested or direct `None` flattens to None
/// __Inputs:__ `x : Option<Option<T>>`  
/// __Precondition:__ `x.is_none() || x.unwrap().is_none()`  
/// __Postcondition:__ `x.flatten() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<Option<u8>>::None.flatten() == None);
/// assert!(Some(Option::<u8>::None).flatten() == None);
/// # }
/// ```
pub fn option_flatten(){}
/// # Properties for [`Option::take`]
/// ## Take steals a value
/// __Inputs:__ `x : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let mut y = x.clone();
///         y.take() == x && y.is_none()
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let mut y = Option::<u8>::None.clone();
///         y.take() == Option::<u8>::None && y.is_none()
///     });
/// assert!({
///         let mut y = Some(0u8).clone();
///         y.take() == Some(0u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(255u8).clone();
///         y.take() == Some(255u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(5u8).clone();
///         y.take() == Some(5u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(151u8).clone();
///         y.take() == Some(151u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(148u8).clone();
///         y.take() == Some(148u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(80u8).clone();
///         y.take() == Some(80u8) && y.is_none()
///     });
/// # }
/// ```
pub fn option_take(){}
/// # Properties for [`Option::zip`]
/// ## Zipping two non-empty options
/// __Inputs:__ `x : T, y : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(x).zip(Some(y)) == Some((x, y))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).zip(Some(0u8)) == Some((0u8, 0u8)));
/// assert!(Some(0u8).zip(Some(255u8)) == Some((0u8, 255u8)));
/// assert!(Some(255u8).zip(Some(0u8)) == Some((255u8, 0u8)));
/// assert!(Some(255u8).zip(Some(255u8)) == Some((255u8, 255u8)));
/// assert!(Some(97u8).zip(Some(121u8)) == Some((97u8, 121u8)));
/// assert!(Some(4u8).zip(Some(195u8)) == Some((4u8, 195u8)));
/// assert!(Some(246u8).zip(Some(8u8)) == Some((246u8, 8u8)));
/// # }
/// ```
/// ## Zipping two options when one is a `None` makes `None`
/// __Inputs:__ `x : Option<T>, y : Option<T>`  
/// __Precondition:__ `x.is_none() || y.is_none()`  
/// __Postcondition:__ `x.zip(y).is_none()`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Some(0u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(255u8)).is_none());
/// assert!(Some(0u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(255u8).zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Some(61u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(101u8)).is_none());
/// # }
/// ```
pub fn option_zip(){}
/// # Properties for [`Option::unwrap`]
/// ## Unwrapping a [`None`] always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(Option::< u8 >::None.unwrap()));
/// # }
/// ```
/// ## Unwrapping a [`Some(_)`] always succeeds
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_some()`  
/// __Postcondition:__ `doesn_t_panic!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).unwrap()));
/// assert!(doesn_t_panic!(Some(255u8).unwrap()));
/// assert!(doesn_t_panic!(Some(114u8).unwrap()));
/// assert!(doesn_t_panic!(Some(117u8).unwrap()));
/// assert!(doesn_t_panic!(Some(100u8).unwrap()));
/// assert!(doesn_t_panic!(Some(181u8).unwrap()));
/// assert!(doesn_t_panic!(Some(220u8).unwrap()));
/// # }
/// ```
pub fn option_unwrap(){}
/// # Properties for [`Option::as_mut`]
/// ## In place update via `as_mut` is equivalent to functional update
/// __Inputs:__ `v : Option<u8>`  
/// __Precondition:__ `v.is_some() && v.unwrap() < 50`  
/// __Postcondition:__ `{
///         let (v_unwrapped, mut v_mut) = (v.unwrap().clone(), v);
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(18u8).unwrap().clone(), Some(18u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(1u8).unwrap().clone(), Some(1u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(28u8).unwrap().clone(), Some(28u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(48u8).unwrap().clone(), Some(48u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(46u8).unwrap().clone(), Some(46u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(35u8).unwrap().clone(), Some(35u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// # }
/// ```
pub fn option_as_mut(){}
/// # Properties for [`Option::as_slice`]
/// ## [`None.as_slice()`] should always result in an empty slice
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `{ v.as_slice().is_empty() }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Option::<u8>::None.as_slice().is_empty() });
/// # }
/// ```
/// ## [`Some(v).as_slice()`] should always result in a slice containing exactly `v`
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ Some(v).as_slice() == [v] }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Some(0u8).as_slice() == [0u8] });
/// assert!({ Some(255u8).as_slice() == [255u8] });
/// assert!({ Some(51u8).as_slice() == [51u8] });
/// assert!({ Some(109u8).as_slice() == [109u8] });
/// assert!({ Some(20u8).as_slice() == [20u8] });
/// assert!({ Some(247u8).as_slice() == [247u8] });
/// assert!({ Some(100u8).as_slice() == [100u8] });
/// # }
/// ```
pub fn option_as_slice(){}
/// # Properties for [`Vec`]
/// ## Indexing
/// __Inputs:__ `v : Vec<u8>, i : usize`  
/// __Precondition:__ `v.len() > 0`  
/// __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![255u8].get(0usize) == Some(&255u8));
/// assert!(vec![255u8].get(0usize) == Some(&255u8));
/// assert!(vec![204u8, 74u8].get(1usize) == Some(&74u8));
/// assert!(vec![195u8].get(0usize) == Some(&195u8));
/// assert!(vec![130u8, 238u8, 166u8].get(0usize) == Some(&130u8));
/// # }
/// ```
pub fn slice_get(){}
/// # Properties for [`core::ops::Sub::<i8>::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() < 128.up() && x.up() - y.up() >= -128i32.up()`  
/// __Postcondition:__ `x - y == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(-128i8 - -128i8 == 0i8);
/// assert!(-128i8 - 0i8 == -128i8);
/// assert!(0i8 - 0i8 == 0i8);
/// assert!(0i8 - 127i8 == -127i8);
/// assert!(127i8 - 0i8 == 127i8);
/// assert!(127i8 - 127i8 == 0i8);
/// assert!(-48i8 - -110i8 == 62i8);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() >= 128.up() || x.up() - y.up() < -128i32.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(- 128i8 - 127i8));
/// assert!(panics!(0i8 - - 128i8));
/// assert!(panics!(127i8 - - 128i8));
/// assert!(panics!(- 125i8 - 53i8));
/// assert!(panics!(- 107i8 - 100i8));
/// assert!(panics!(127i8 - - 87i8));
/// assert!(panics!(- 48i8 - 122i8));
/// # }
/// ```
pub fn core_ops_sub_i8_sub(){}
/// # Properties for [`core::ops::Add::<i8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() < 128.up() && x.up() + y.up() >= -128i32.up()`  
/// __Postcondition:__ `x + y == i8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(-128i8 + 0i8 == -128i8);
/// assert!(-128i8 + 127i8 == -1i8);
/// assert!(0i8 + -128i8 == -128i8);
/// assert!(0i8 + 0i8 == 0i8);
/// assert!(0i8 + 127i8 == 127i8);
/// assert!(127i8 + -128i8 == -1i8);
/// assert!(127i8 + 0i8 == 127i8);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() >= 128.up() || x.up() + y.up() < -128i32.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(- 128i8 + - 128i8));
/// assert!(panics!(127i8 + 127i8));
/// assert!(panics!(118i8 + 68i8));
/// assert!(panics!(- 50i8 + - 79i8));
/// assert!(panics!(94i8 + 109i8));
/// assert!(panics!(121i8 + 65i8));
/// assert!(panics!(107i8 + 75i8));
/// # }
/// ```
pub fn core_ops_add_i8_add(){}
/// # Properties for [`core::ops::Add::<u8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x + y == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8);
/// assert!(0u8 + 255u8 == 255u8);
/// assert!(255u8 + 0u8 == 255u8);
/// assert!(97u8 + 35u8 == 132u8);
/// assert!(133u8 + 32u8 == 165u8);
/// assert!(62u8 + 116u8 == 178u8);
/// assert!(106u8 + 94u8 == 200u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(255u8 + 255u8));
/// assert!(panics!(244u8 + 72u8));
/// assert!(panics!(82u8 + 214u8));
/// assert!(panics!(210u8 + 191u8));
/// assert!(panics!(214u8 + 144u8));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8 + 0u8);
/// assert!(0u8 + 255u8 == 255u8 + 0u8);
/// assert!(255u8 + 0u8 == 0u8 + 255u8);
/// assert!(71u8 + 75u8 == 75u8 + 71u8);
/// assert!(83u8 + 71u8 == 71u8 + 83u8);
/// assert!(88u8 + 35u8 == 35u8 + 88u8);
/// assert!(25u8 + 144u8 == 144u8 + 25u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0 == 0u8);
/// assert!(255u8 + 0 == 255u8);
/// assert!(133u8 + 0 == 133u8);
/// assert!(94u8 + 0 == 94u8);
/// assert!(156u8 + 0 == 156u8);
/// assert!(3u8 + 0 == 3u8);
/// assert!(84u8 + 0 == 84u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u8 == 0u8);
/// assert!(0 + 255u8 == 255u8);
/// assert!(0 + 148u8 == 148u8);
/// assert!(0 + 160u8 == 160u8);
/// assert!(0 + 198u8 == 198u8);
/// assert!(0 + 78u8 == 78u8);
/// assert!(0 + 12u8 == 12u8);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u8::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u8 + 0u8) + 0u8 == 0u8 + (0u8 + 0u8));
/// assert!((0u8 + 0u8) + 255u8 == 0u8 + (0u8 + 255u8));
/// assert!((0u8 + 255u8) + 0u8 == 0u8 + (255u8 + 0u8));
/// assert!((255u8 + 0u8) + 0u8 == 255u8 + (0u8 + 0u8));
/// assert!((160u8 + 0u8) + 9u8 == 160u8 + (0u8 + 9u8));
/// assert!((60u8 + 5u8) + 120u8 == 60u8 + (5u8 + 120u8));
/// assert!((109u8 + 19u8) + 61u8 == 109u8 + (19u8 + 61u8));
/// # }
/// ```
pub fn core_ops_add_u8_add(){}
/// # Properties for [`u8::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u8::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(0u8.checked_add(255u8) == Some(255u8));
/// assert!(255u8.checked_add(0u8) == Some(255u8));
/// assert!(142u8.checked_add(31u8) == Some(173u8));
/// assert!(3u8.checked_add(178u8) == Some(181u8));
/// assert!(35u8.checked_add(220u8) == Some(255u8));
/// assert!(153u8.checked_add(86u8) == Some(239u8));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_add(255u8) == None);
/// assert!(167u8.checked_add(126u8) == None);
/// assert!(132u8.checked_add(191u8) == None);
/// assert!(139u8.checked_add(202u8) == None);
/// assert!(129u8.checked_add(233u8) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == 0u8.checked_add(0u8));
/// assert!(0u8.checked_add(255u8) == 255u8.checked_add(0u8));
/// assert!(255u8.checked_add(0u8) == 0u8.checked_add(255u8));
/// assert!(255u8.checked_add(255u8) == 255u8.checked_add(255u8));
/// assert!(187u8.checked_add(60u8) == 60u8.checked_add(187u8));
/// assert!(240u8.checked_add(135u8) == 135u8.checked_add(240u8));
/// assert!(131u8.checked_add(21u8) == 21u8.checked_add(131u8));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u8) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(255u8.checked_add(0u8) == Some(255u8));
/// assert!(53u8.checked_add(0u8) == Some(53u8));
/// assert!(28u8.checked_add(0u8) == Some(28u8));
/// assert!(203u8.checked_add(0u8) == Some(203u8));
/// assert!(23u8.checked_add(0u8) == Some(23u8));
/// assert!(235u8.checked_add(0u8) == Some(235u8));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u8.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(0u8.checked_add(255u8) == Some(255u8));
/// assert!(0u8.checked_add(98u8) == Some(98u8));
/// assert!(0u8.checked_add(69u8) == Some(69u8));
/// assert!(0u8.checked_add(72u8) == Some(72u8));
/// assert!(0u8.checked_add(37u8) == Some(37u8));
/// assert!(0u8.checked_add(233u8) == Some(233u8));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8).and_then(|iv| iv.checked_add(0u8))
///         == 0u8.checked_add(0u8).and_then(|iv| 0u8.checked_add(iv)));
/// assert!(0u8.checked_add(0u8).and_then(|iv| iv.checked_add(255u8))
///         == 0u8.checked_add(255u8).and_then(|iv| 0u8.checked_add(iv)));
/// assert!(0u8.checked_add(255u8).and_then(|iv| iv.checked_add(0u8))
///         == 255u8.checked_add(0u8).and_then(|iv| 0u8.checked_add(iv)));
/// assert!(0u8.checked_add(255u8).and_then(|iv| iv.checked_add(255u8))
///         == 255u8.checked_add(255u8).and_then(|iv| 0u8.checked_add(iv)));
/// assert!(255u8.checked_add(0u8).and_then(|iv| iv.checked_add(0u8))
///         == 0u8.checked_add(0u8).and_then(|iv| 255u8.checked_add(iv)));
/// assert!(255u8.checked_add(0u8).and_then(|iv| iv.checked_add(255u8))
///         == 0u8.checked_add(255u8).and_then(|iv| 255u8.checked_add(iv)));
/// assert!(255u8.checked_add(255u8).and_then(|iv| iv.checked_add(0u8))
///         == 255u8.checked_add(0u8).and_then(|iv| 255u8.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u8_checked_add(){}
/// # Properties for [`core::ops::Add::<u16>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x + y == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 + 0u16 == 0u16);
/// assert!(0u16 + 65535u16 == 65535u16);
/// assert!(65535u16 + 0u16 == 65535u16);
/// assert!(13595u16 + 4718u16 == 18313u16);
/// assert!(22118u16 + 25876u16 == 47994u16);
/// assert!(5887u16 + 17205u16 == 23092u16);
/// assert!(48785u16 + 12775u16 == 61560u16);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65535u16 + 65535u16));
/// assert!(panics!(56401u16 + 12362u16));
/// assert!(panics!(45218u16 + 43577u16));
/// assert!(panics!(65146u16 + 6104u16));
/// assert!(panics!(65131u16 + 25049u16));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 + 0u16 == 0u16 + 0u16);
/// assert!(0u16 + 65535u16 == 65535u16 + 0u16);
/// assert!(65535u16 + 0u16 == 0u16 + 65535u16);
/// assert!(20635u16 + 10417u16 == 10417u16 + 20635u16);
/// assert!(21678u16 + 8090u16 == 8090u16 + 21678u16);
/// assert!(55475u16 + 73u16 == 73u16 + 55475u16);
/// assert!(30424u16 + 23890u16 == 23890u16 + 30424u16);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 + 0 == 0u16);
/// assert!(65535u16 + 0 == 65535u16);
/// assert!(31819u16 + 0 == 31819u16);
/// assert!(10938u16 + 0 == 10938u16);
/// assert!(36566u16 + 0 == 36566u16);
/// assert!(24467u16 + 0 == 24467u16);
/// assert!(20318u16 + 0 == 20318u16);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u16 == 0u16);
/// assert!(0 + 65535u16 == 65535u16);
/// assert!(0 + 27102u16 == 27102u16);
/// assert!(0 + 21191u16 == 21191u16);
/// assert!(0 + 200u16 == 200u16);
/// assert!(0 + 26367u16 == 26367u16);
/// assert!(0 + 7699u16 == 7699u16);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u16::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u16 + 0u16) + 0u16 == 0u16 + (0u16 + 0u16));
/// assert!((0u16 + 0u16) + 65535u16 == 0u16 + (0u16 + 65535u16));
/// assert!((0u16 + 65535u16) + 0u16 == 0u16 + (65535u16 + 0u16));
/// assert!((65535u16 + 0u16) + 0u16 == 65535u16 + (0u16 + 0u16));
/// assert!((4527u16 + 16733u16) + 36066u16 == 4527u16 + (16733u16 + 36066u16));
/// assert!((19027u16 + 17156u16) + 10932u16 == 19027u16 + (17156u16 + 10932u16));
/// assert!((34444u16 + 3095u16) + 7028u16 == 34444u16 + (3095u16 + 7028u16));
/// # }
/// ```
pub fn core_ops_add_u16_add(){}
/// # Properties for [`u16::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u16::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(0u16.checked_add(65535u16) == Some(65535u16));
/// assert!(65535u16.checked_add(0u16) == Some(65535u16));
/// assert!(38504u16.checked_add(9099u16) == Some(47603u16));
/// assert!(12696u16.checked_add(12515u16) == Some(25211u16));
/// assert!(12578u16.checked_add(33993u16) == Some(46571u16));
/// assert!(10348u16.checked_add(32300u16) == Some(42648u16));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.checked_add(65535u16) == None);
/// assert!(63412u16.checked_add(53452u16) == None);
/// assert!(35459u16.checked_add(49939u16) == None);
/// assert!(60607u16.checked_add(50104u16) == None);
/// assert!(64237u16.checked_add(46366u16) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16) == 0u16.checked_add(0u16));
/// assert!(0u16.checked_add(65535u16) == 65535u16.checked_add(0u16));
/// assert!(65535u16.checked_add(0u16) == 0u16.checked_add(65535u16));
/// assert!(65535u16.checked_add(65535u16) == 65535u16.checked_add(65535u16));
/// assert!(44898u16.checked_add(13373u16) == 13373u16.checked_add(44898u16));
/// assert!(45444u16.checked_add(49738u16) == 49738u16.checked_add(45444u16));
/// assert!(48665u16.checked_add(17078u16) == 17078u16.checked_add(48665u16));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u16) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(65535u16.checked_add(0u16) == Some(65535u16));
/// assert!(48140u16.checked_add(0u16) == Some(48140u16));
/// assert!(23658u16.checked_add(0u16) == Some(23658u16));
/// assert!(4584u16.checked_add(0u16) == Some(4584u16));
/// assert!(20873u16.checked_add(0u16) == Some(20873u16));
/// assert!(24872u16.checked_add(0u16) == Some(24872u16));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u16.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(0u16.checked_add(65535u16) == Some(65535u16));
/// assert!(0u16.checked_add(53138u16) == Some(53138u16));
/// assert!(0u16.checked_add(46444u16) == Some(46444u16));
/// assert!(0u16.checked_add(59935u16) == Some(59935u16));
/// assert!(0u16.checked_add(39800u16) == Some(39800u16));
/// assert!(0u16.checked_add(30992u16) == Some(30992u16));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16).and_then(|iv| iv.checked_add(0u16))
///         == 0u16.checked_add(0u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(0u16.checked_add(0u16).and_then(|iv| iv.checked_add(65535u16))
///         == 0u16.checked_add(65535u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(0u16.checked_add(65535u16).and_then(|iv| iv.checked_add(0u16))
///         == 65535u16.checked_add(0u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(0u16.checked_add(65535u16).and_then(|iv| iv.checked_add(65535u16))
///         == 65535u16.checked_add(65535u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(65535u16.checked_add(0u16).and_then(|iv| iv.checked_add(0u16))
///         == 0u16.checked_add(0u16).and_then(|iv| 65535u16.checked_add(iv)));
/// assert!(65535u16.checked_add(0u16).and_then(|iv| iv.checked_add(65535u16))
///         == 0u16.checked_add(65535u16).and_then(|iv| 65535u16.checked_add(iv)));
/// assert!(65535u16.checked_add(65535u16).and_then(|iv| iv.checked_add(0u16))
///         == 65535u16.checked_add(0u16).and_then(|iv| 65535u16.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u16_checked_add(){}
/// # Properties for [`core::ops::Add::<u32>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x + y == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 + 0u32 == 0u32);
/// assert!(0u32 + 4294967295u32 == 4294967295u32);
/// assert!(4294967295u32 + 0u32 == 4294967295u32);
/// assert!(2428432152u32 + 1653881166u32 == 4082313318u32);
/// assert!(2309269632u32 + 577764189u32 == 2887033821u32);
/// assert!(3688919628u32 + 529754662u32 == 4218674290u32);
/// assert!(669370967u32 + 1164699285u32 == 1834070252u32);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(4294967295u32 + 4294967295u32));
/// assert!(panics!(1752112523u32 + 3761242419u32));
/// assert!(panics!(2096203312u32 + 3085097801u32));
/// assert!(panics!(2233599276u32 + 2495597598u32));
/// assert!(panics!(4024406167u32 + 3775900451u32));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 + 0u32 == 0u32 + 0u32);
/// assert!(0u32 + 4294967295u32 == 4294967295u32 + 0u32);
/// assert!(4294967295u32 + 0u32 == 0u32 + 4294967295u32);
/// assert!(1489649489u32 + 2111464312u32 == 2111464312u32 + 1489649489u32);
/// assert!(430027375u32 + 860233823u32 == 860233823u32 + 430027375u32);
/// assert!(1204458014u32 + 393433141u32 == 393433141u32 + 1204458014u32);
/// assert!(772262638u32 + 1865070713u32 == 1865070713u32 + 772262638u32);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 + 0 == 0u32);
/// assert!(4294967295u32 + 0 == 4294967295u32);
/// assert!(4162883769u32 + 0 == 4162883769u32);
/// assert!(2660758149u32 + 0 == 2660758149u32);
/// assert!(4181323164u32 + 0 == 4181323164u32);
/// assert!(4051827317u32 + 0 == 4051827317u32);
/// assert!(4217115865u32 + 0 == 4217115865u32);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u32 == 0u32);
/// assert!(0 + 4294967295u32 == 4294967295u32);
/// assert!(0 + 541679851u32 == 541679851u32);
/// assert!(0 + 3313911122u32 == 3313911122u32);
/// assert!(0 + 3066975833u32 == 3066975833u32);
/// assert!(0 + 1116312417u32 == 1116312417u32);
/// assert!(0 + 1564815180u32 == 1564815180u32);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u32::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u32 + 0u32) + 0u32 == 0u32 + (0u32 + 0u32));
/// assert!((0u32 + 0u32) + 4294967295u32 == 0u32 + (0u32 + 4294967295u32));
/// assert!((0u32 + 4294967295u32) + 0u32 == 0u32 + (4294967295u32 + 0u32));
/// assert!((4294967295u32 + 0u32) + 0u32 == 4294967295u32 + (0u32 + 0u32));
/// assert!((1042550799u32 + 782482406u32) + 1561262534u32
///         == 1042550799u32 + (782482406u32 + 1561262534u32));
/// assert!((85693732u32 + 3249341552u32) + 233414786u32
///         == 85693732u32 + (3249341552u32 + 233414786u32));
/// assert!((1303862261u32 + 1982577543u32) + 923700650u32
///         == 1303862261u32 + (1982577543u32 + 923700650u32));
/// # }
/// ```
pub fn core_ops_add_u32_add(){}
/// # Properties for [`u32::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u32::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
/// assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
/// assert!(1510896598u32.checked_add(739546027u32) == Some(2250442625u32));
/// assert!(34969989u32.checked_add(1503012627u32) == Some(1537982616u32));
/// assert!(2051258597u32.checked_add(876194936u32) == Some(2927453533u32));
/// assert!(1979875804u32.checked_add(766297611u32) == Some(2746173415u32));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.checked_add(4294967295u32) == None);
/// assert!(2764779653u32.checked_add(4015205926u32) == None);
/// assert!(1691089602u32.checked_add(3923515427u32) == None);
/// assert!(2529952733u32.checked_add(3755503893u32) == None);
/// assert!(4049832349u32.checked_add(385872821u32) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32) == 0u32.checked_add(0u32));
/// assert!(0u32.checked_add(4294967295u32) == 4294967295u32.checked_add(0u32));
/// assert!(4294967295u32.checked_add(0u32) == 0u32.checked_add(4294967295u32));
/// assert!(4294967295u32.checked_add(4294967295u32) == 4294967295u32.checked_add(4294967295u32));
/// assert!(1832355262u32.checked_add(2178268854u32) == 2178268854u32.checked_add(1832355262u32));
/// assert!(2102922986u32.checked_add(3056301062u32) == 3056301062u32.checked_add(2102922986u32));
/// assert!(1156900450u32.checked_add(3261964514u32) == 3261964514u32.checked_add(1156900450u32));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u32) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
/// assert!(3249650077u32.checked_add(0u32) == Some(3249650077u32));
/// assert!(2130050197u32.checked_add(0u32) == Some(2130050197u32));
/// assert!(2117031684u32.checked_add(0u32) == Some(2117031684u32));
/// assert!(2019705704u32.checked_add(0u32) == Some(2019705704u32));
/// assert!(1209308376u32.checked_add(0u32) == Some(1209308376u32));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u32.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
/// assert!(0u32.checked_add(17725074u32) == Some(17725074u32));
/// assert!(0u32.checked_add(272488299u32) == Some(272488299u32));
/// assert!(0u32.checked_add(511541055u32) == Some(511541055u32));
/// assert!(0u32.checked_add(86331823u32) == Some(86331823u32));
/// assert!(0u32.checked_add(2174904342u32) == Some(2174904342u32));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32).and_then(|iv| iv.checked_add(0u32))
///         == 0u32.checked_add(0u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(0u32.checked_add(0u32).and_then(|iv| iv.checked_add(4294967295u32))
///         == 0u32.checked_add(4294967295u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(0u32.checked_add(4294967295u32).and_then(|iv| iv.checked_add(0u32))
///         == 4294967295u32.checked_add(0u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(0u32.checked_add(4294967295u32).and_then(|iv| iv.checked_add(4294967295u32))
///         == 4294967295u32.checked_add(4294967295u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(4294967295u32.checked_add(0u32).and_then(|iv| iv.checked_add(0u32))
///         == 0u32.checked_add(0u32).and_then(|iv| 4294967295u32.checked_add(iv)));
/// assert!(4294967295u32.checked_add(0u32).and_then(|iv| iv.checked_add(4294967295u32))
///         == 0u32.checked_add(4294967295u32).and_then(|iv| 4294967295u32.checked_add(iv)));
/// assert!(4294967295u32.checked_add(4294967295u32).and_then(|iv| iv.checked_add(0u32))
///         == 4294967295u32.checked_add(0u32).and_then(|iv| 4294967295u32.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u32_checked_add(){}
/// # Properties for [`core::ops::Add::<u64>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x + y == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 + 0u64 == 0u64);
/// assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 + 0u64 == 18446744073709551615u64);
/// assert!(4451248067406516904u64 + 6137155643965851413u64 == 10588403711372368317u64);
/// assert!(4759217687863202988u64 + 3712035701164232720u64 == 8471253389027435708u64);
/// assert!(13153755130886044289u64 + 5288076401089458045u64 == 18441831531975502334u64);
/// assert!(12207833651056998531u64 + 3999932359202734935u64 == 16207766010259733466u64);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
/// assert!(panics!(13177000629990521530u64 + 13559696913066999837u64));
/// assert!(panics!(11314422851191365257u64 + 11197552383355996075u64));
/// assert!(panics!(6074131584535470248u64 + 18404918094397467774u64));
/// assert!(panics!(17908097402294852684u64 + 15021816312840762502u64));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 + 0u64 == 0u64 + 0u64);
/// assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64 + 0u64);
/// assert!(18446744073709551615u64 + 0u64 == 0u64 + 18446744073709551615u64);
/// assert!(5687026623193332530u64 + 820890411863974613u64
///         == 820890411863974613u64 + 5687026623193332530u64);
/// assert!(10712410455245545096u64 + 7222348910621825969u64
///         == 7222348910621825969u64 + 10712410455245545096u64);
/// assert!(862407298443626099u64 + 12281928211865385330u64
///         == 12281928211865385330u64 + 862407298443626099u64);
/// assert!(5073377454086887523u64 + 9391899965784426052u64
///         == 9391899965784426052u64 + 5073377454086887523u64);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 + 0 == 0u64);
/// assert!(18446744073709551615u64 + 0 == 18446744073709551615u64);
/// assert!(11225437658332914997u64 + 0 == 11225437658332914997u64);
/// assert!(3435954796847335228u64 + 0 == 3435954796847335228u64);
/// assert!(5138817863390466403u64 + 0 == 5138817863390466403u64);
/// assert!(6021077595623713517u64 + 0 == 6021077595623713517u64);
/// assert!(5996584602221374917u64 + 0 == 5996584602221374917u64);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u64 == 0u64);
/// assert!(0 + 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(0 + 14124304551593770404u64 == 14124304551593770404u64);
/// assert!(0 + 14764796059550823016u64 == 14764796059550823016u64);
/// assert!(0 + 9814677291571155236u64 == 9814677291571155236u64);
/// assert!(0 + 4592244130015484588u64 == 4592244130015484588u64);
/// assert!(0 + 8507402111284511317u64 == 8507402111284511317u64);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u64::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u64 + 0u64) + 0u64 == 0u64 + (0u64 + 0u64));
/// assert!((0u64 + 0u64) + 18446744073709551615u64 == 0u64 + (0u64 + 18446744073709551615u64));
/// assert!((0u64 + 18446744073709551615u64) + 0u64 == 0u64 + (18446744073709551615u64 + 0u64));
/// assert!((18446744073709551615u64 + 0u64) + 0u64 == 18446744073709551615u64 + (0u64 + 0u64));
/// assert!((2913720325477643315u64 + 309511373321093801u64) + 3754264632700635599u64
///         == 2913720325477643315u64 + (309511373321093801u64 + 3754264632700635599u64));
/// assert!((1025934828906614216u64 + 5266979172070452900u64) + 11021889598623830417u64
///         == 1025934828906614216u64 + (5266979172070452900u64 + 11021889598623830417u64));
/// assert!((4596738939073386562u64 + 6254905452981452537u64) + 6005791977405042189u64
///         == 4596738939073386562u64 + (6254905452981452537u64 + 6005791977405042189u64));
/// # }
/// ```
pub fn core_ops_add_u64_add(){}
/// # Properties for [`u64::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u64::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
/// assert!(5430976214486531257u64.checked_add(4452159472817542807u64)
///         == Some(9883135687304074064u64));
/// assert!(3714166765535238078u64.checked_add(5568796798007876915u64)
///         == Some(9282963563543114993u64));
/// assert!(7018291680783791766u64.checked_add(3801163595538201440u64)
///         == Some(10819455276321993206u64));
/// assert!(759096659125148862u64.checked_add(13872589988617603284u64)
///         == Some(14631686647742752146u64));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
/// assert!(17956522190422476652u64.checked_add(12124157300887935670u64) == None);
/// assert!(10044213781095970871u64.checked_add(12585486152016111974u64) == None);
/// assert!(15836385237374528562u64.checked_add(12377569923601906209u64) == None);
/// assert!(4505638310268533491u64.checked_add(15245720112511460449u64) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == 0u64.checked_add(0u64));
/// assert!(0u64.checked_add(18446744073709551615u64)
///         == 18446744073709551615u64.checked_add(0u64));
/// assert!(18446744073709551615u64.checked_add(0u64)
///         == 0u64.checked_add(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_add(18446744073709551615u64)
///         == 18446744073709551615u64.checked_add(18446744073709551615u64));
/// assert!(6610704845120514490u64.checked_add(5641838204242065259u64)
///         == 5641838204242065259u64.checked_add(6610704845120514490u64));
/// assert!(11525573352864736181u64.checked_add(7339786106222254945u64)
///         == 7339786106222254945u64.checked_add(11525573352864736181u64));
/// assert!(9419931050420023483u64.checked_add(7711411145921816272u64)
///         == 7711411145921816272u64.checked_add(9419931050420023483u64));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u64) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
/// assert!(17715094714652803987u64.checked_add(0u64) == Some(17715094714652803987u64));
/// assert!(8904946019291964715u64.checked_add(0u64) == Some(8904946019291964715u64));
/// assert!(748583592113745896u64.checked_add(0u64) == Some(748583592113745896u64));
/// assert!(4401535664202265188u64.checked_add(0u64) == Some(4401535664202265188u64));
/// assert!(5521650004380733846u64.checked_add(0u64) == Some(5521650004380733846u64));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u64.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(0u64.checked_add(11935624364662888400u64) == Some(11935624364662888400u64));
/// assert!(0u64.checked_add(12239669834450739386u64) == Some(12239669834450739386u64));
/// assert!(0u64.checked_add(124541409723598328u64) == Some(124541409723598328u64));
/// assert!(0u64.checked_add(15146935268267261830u64) == Some(15146935268267261830u64));
/// assert!(0u64.checked_add(12504683014550259814u64) == Some(12504683014550259814u64));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64).and_then(|iv| iv.checked_add(0u64))
///         == 0u64.checked_add(0u64).and_then(|iv| 0u64.checked_add(iv)));
/// assert!(0u64.checked_add(0u64).and_then(|iv| iv.checked_add(18446744073709551615u64))
///         == 0u64.checked_add(18446744073709551615u64).and_then(|iv| 0u64.checked_add(iv)));
/// assert!(0u64.checked_add(18446744073709551615u64).and_then(|iv| iv.checked_add(0u64))
///         == 18446744073709551615u64.checked_add(0u64).and_then(|iv| 0u64.checked_add(iv)));
/// assert!(0u64
///         .checked_add(18446744073709551615u64)
///         .and_then(|iv| iv.checked_add(18446744073709551615u64))
///         == 18446744073709551615u64
///             .checked_add(18446744073709551615u64)
///             .and_then(|iv| 0u64.checked_add(iv)));
/// assert!(18446744073709551615u64.checked_add(0u64).and_then(|iv| iv.checked_add(0u64))
///         == 0u64.checked_add(0u64).and_then(|iv| 18446744073709551615u64.checked_add(iv)));
/// assert!(18446744073709551615u64
///         .checked_add(0u64)
///         .and_then(|iv| iv.checked_add(18446744073709551615u64))
///         == 0u64
///             .checked_add(18446744073709551615u64)
///             .and_then(|iv| 18446744073709551615u64.checked_add(iv)));
/// assert!(18446744073709551615u64
///         .checked_add(18446744073709551615u64)
///         .and_then(|iv| iv.checked_add(0u64))
///         == 18446744073709551615u64
///             .checked_add(0u64)
///             .and_then(|iv| 18446744073709551615u64.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u64_checked_add(){}
/// # Properties for [`core::ops::Add::<u128>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x + y == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0u128 == 0u128);
/// assert!(0u128 + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 + 0u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(76988045222912797819913230140414667152u128
///         + 157884682124212412824168568043823352980u128
///         == 234872727347125210644081798184238020132u128);
/// assert!(67795153207629498911801914561204055627u128
///         + 20980839665622989303264794747867557956u128
///         == 88775992873252488215066709309071613583u128);
/// assert!(153016411621375441846607006017390144298u128
///         + 148995609526074464804427841357562782455u128
///         == 302012021147449906651034847374952926753u128);
/// assert!(95034978183316842509114773162706502186u128
///         + 53695433238845553103150299267760889656u128
///         == 148730411422162395612265072430467391842u128);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 +
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         184425210232649071852582189860066537521u128 +
///         159469729460936646573622456627906172987u128
///     ));
/// assert!(panics!(
///         330055022628157842902120943159899804011u128 +
///         56732038160401495637301816482831681653u128
///     ));
/// assert!(panics!(
///         336332491792630363572829843219173405916u128 +
///         237816585063136717276706582935827735024u128
///     ));
/// assert!(panics!(
///         109305549229489408126476677402562735799u128 +
///         276366197622042656484248505639452640090u128
///     ));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0u128 == 0u128 + 0u128);
/// assert!(0u128 + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128 + 0u128);
/// assert!(340282366920938463463374607431768211455u128 + 0u128
///         == 0u128 + 340282366920938463463374607431768211455u128);
/// assert!(98272436117253022590889551109077826123u128
///         + 4133783274923621524649499630261771071u128
///         == 4133783274923621524649499630261771071u128
///             + 98272436117253022590889551109077826123u128);
/// assert!(334545049351844727957359547823211547851u128
///         + 3992631683673841075561309196590390204u128
///         == 3992631683673841075561309196590390204u128
///             + 334545049351844727957359547823211547851u128);
/// assert!(54878166746167883288818290744274364091u128
///         + 27586769450578807133970881155185267186u128
///         == 27586769450578807133970881155185267186u128
///             + 54878166746167883288818290744274364091u128);
/// assert!(68298300713037386585251937934160213142u128
///         + 120276816929747273769657386808809081961u128
///         == 120276816929747273769657386808809081961u128
///             + 68298300713037386585251937934160213142u128);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 + 0
///         == 340282366920938463463374607431768211455u128);
/// assert!(146899066460309722320481839646626579616u128 + 0
///         == 146899066460309722320481839646626579616u128);
/// assert!(285707762915744874666619329362836493975u128 + 0
///         == 285707762915744874666619329362836493975u128);
/// assert!(311661337007061384407241621334560279141u128 + 0
///         == 311661337007061384407241621334560279141u128);
/// assert!(214165971470013110037714762279066109657u128 + 0
///         == 214165971470013110037714762279066109657u128);
/// assert!(311131795382448448020321925594324656476u128 + 0
///         == 311131795382448448020321925594324656476u128);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u128 == 0u128);
/// assert!(0 + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(0 + 186272985158529778037870003626382741622u128
///         == 186272985158529778037870003626382741622u128);
/// assert!(0 + 304548722077211209366187599494906146860u128
///         == 304548722077211209366187599494906146860u128);
/// assert!(0 + 13375174828960519415938220668212393962u128
///         == 13375174828960519415938220668212393962u128);
/// assert!(0 + 214312118478724164359959175253743035993u128
///         == 214312118478724164359959175253743035993u128);
/// assert!(0 + 43290313256206281229707599714978280658u128
///         == 43290313256206281229707599714978280658u128);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u128::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u128 + 0u128) + 0u128 == 0u128 + (0u128 + 0u128));
/// assert!((0u128 + 0u128) + 340282366920938463463374607431768211455u128
///         == 0u128 + (0u128 + 340282366920938463463374607431768211455u128));
/// assert!((0u128 + 340282366920938463463374607431768211455u128) + 0u128
///         == 0u128 + (340282366920938463463374607431768211455u128 + 0u128));
/// assert!((340282366920938463463374607431768211455u128 + 0u128) + 0u128
///         == 340282366920938463463374607431768211455u128 + (0u128 + 0u128));
/// assert!((47542113992051886557613169266663159960u128
///         + 18358924576283285282403235562461441033u128)
///         + 199996202377562266571128274811415623741u128
///         == 47542113992051886557613169266663159960u128
///             + (18358924576283285282403235562461441033u128
///                 + 199996202377562266571128274811415623741u128));
/// assert!((150694014431239939377259043859006614611u128
///         + 2522201093242469629019467207116846795u128)
///         + 20514400101465373097931618083101802139u128
///         == 150694014431239939377259043859006614611u128
///             + (2522201093242469629019467207116846795u128
///                 + 20514400101465373097931618083101802139u128));
/// assert!((185127814117464554267374316208993215150u128
///         + 11541701926897877198840461646028786745u128)
///         + 117726485425294229999190613638903333192u128
///         == 185127814117464554267374316208993215150u128
///             + (11541701926897877198840461646028786745u128
///                 + 117726485425294229999190613638903333192u128));
/// # }
/// ```
pub fn core_ops_add_u128_add(){}
/// # Properties for [`u128::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u128::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(124429733625595774826057824204290756414u128
///         .checked_add(753869259891939005704978176557431790u128)
///         == Some(125183602885487713831762802380848188204u128));
/// assert!(35649817079931564808622326409779656118u128
///         .checked_add(50110795600880266500328561204994868311u128)
///         == Some(85760612680811831308950887614774524429u128));
/// assert!(26293038726565610251561079820392329036u128
///         .checked_add(93111726590833087029890422164420757113u128)
///         == Some(119404765317398697281451501984813086149u128));
/// assert!(94237444331017503599521382607311066365u128
///         .checked_add(36631734981315473446586363361523328411u128)
///         == Some(130869179312332977046107745968834394776u128));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(72458187626359767538881667546874434552u128
///         .checked_add(297346540425438035971591137457768079884u128) == None);
/// assert!(238839560601171347832399587309251201746u128
///         .checked_add(242579218632976152146903182551173496368u128) == None);
/// assert!(225397974957210198494564952452362732921u128
///         .checked_add(154164508028863701201042980473917385939u128) == None);
/// assert!(294566236161675223872576442910457868935u128
///         .checked_add(329306771496141303372377619188235007973u128) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == 0u128.checked_add(0u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128.checked_add(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == 0u128.checked_add(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128
///             .checked_add(340282366920938463463374607431768211455u128));
/// assert!(199433924173343872839465418920654472541u128
///         .checked_add(59163026947700566482286024598479722460u128)
///         == 59163026947700566482286024598479722460u128
///             .checked_add(199433924173343872839465418920654472541u128));
/// assert!(289212290630215333669252338697091235710u128
///         .checked_add(236075549949753618916452205453728983327u128)
///         == 236075549949753618916452205453728983327u128
///             .checked_add(289212290630215333669252338697091235710u128));
/// assert!(284518684882817033394032554823890700836u128
///         .checked_add(19378099773157293356198087970618110082u128)
///         == 19378099773157293356198087970618110082u128
///             .checked_add(284518684882817033394032554823890700836u128));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u128) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(335301091005109133454616108549665587307u128.checked_add(0u128)
///         == Some(335301091005109133454616108549665587307u128));
/// assert!(207586692813909776091453831710488124798u128.checked_add(0u128)
///         == Some(207586692813909776091453831710488124798u128));
/// assert!(140451055432199402458350061288231770804u128.checked_add(0u128)
///         == Some(140451055432199402458350061288231770804u128));
/// assert!(313066521221137356429217935298879591721u128.checked_add(0u128)
///         == Some(313066521221137356429217935298879591721u128));
/// assert!(77936528754177969417578059569092989353u128.checked_add(0u128)
///         == Some(77936528754177969417578059569092989353u128));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u128.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(0u128.checked_add(113891871890069189995663307045975395387u128)
///         == Some(113891871890069189995663307045975395387u128));
/// assert!(0u128.checked_add(279706794151156716772881565568633583013u128)
///         == Some(279706794151156716772881565568633583013u128));
/// assert!(0u128.checked_add(18250993685945966765121698267293579222u128)
///         == Some(18250993685945966765121698267293579222u128));
/// assert!(0u128.checked_add(147603520736975998041042811953217031511u128)
///         == Some(147603520736975998041042811953217031511u128));
/// assert!(0u128.checked_add(247621885866340685968408890283294647766u128)
///         == Some(247621885866340685968408890283294647766u128));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128).and_then(|iv| iv.checked_add(0u128))
///         == 0u128.checked_add(0u128).and_then(|iv| 0u128.checked_add(iv)));
/// assert!(0u128
///         .checked_add(0u128)
///         .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
///         == 0u128
///             .checked_add(340282366920938463463374607431768211455u128)
///             .and_then(|iv| 0u128.checked_add(iv)));
/// assert!(0u128
///         .checked_add(340282366920938463463374607431768211455u128)
///         .and_then(|iv| iv.checked_add(0u128))
///         == 340282366920938463463374607431768211455u128
///             .checked_add(0u128)
///             .and_then(|iv| 0u128.checked_add(iv)));
/// assert!(0u128
///         .checked_add(340282366920938463463374607431768211455u128)
///         .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
///         == 340282366920938463463374607431768211455u128
///             .checked_add(340282366920938463463374607431768211455u128)
///             .and_then(|iv| 0u128.checked_add(iv)));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(0u128)
///         .and_then(|iv| iv.checked_add(0u128))
///         == 0u128
///             .checked_add(0u128)
///             .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv)));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(0u128)
///         .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
///         == 0u128
///             .checked_add(340282366920938463463374607431768211455u128)
///             .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv)));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211455u128)
///         .and_then(|iv| iv.checked_add(0u128))
///         == 340282366920938463463374607431768211455u128
///             .checked_add(0u128)
///             .and_then(|iv| 340282366920938463463374607431768211455u128.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u128_checked_add(){}