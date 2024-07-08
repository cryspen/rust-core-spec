//! This module contains placeholder functions decorated with contracts and concrete tests. There are 6843 tests.
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).is_some()
///         == (match Some(0u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Option::<u8>::None.is_some()
///         == (match Option::<u8>::None {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(254u8).is_some()
///         == (match Some(254u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(1u8).is_some()
///         == (match Some(1u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(255u8).is_some()
///         == (match Some(255u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(198u8).is_some()
///         == (match Some(198u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(75u8).is_some()
///         == (match Some(75u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(144u8).is_some()
///         == (match Some(144u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(125u8).is_some()
///         == (match Some(125u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(224u8).is_some()
///         == (match Some(224u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// # }
/// ```
pub fn option_is_some() {}
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(1u8).is_none()
///         == (match Some(1u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Option::<u8>::None.is_none()
///         == (match Option::<u8>::None {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(255u8).is_none()
///         == (match Some(255u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(0u8).is_none()
///         == (match Some(0u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(254u8).is_none()
///         == (match Some(254u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(10u8).is_none()
///         == (match Some(10u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(164u8).is_none()
///         == (match Some(164u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(97u8).is_none()
///         == (match Some(97u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(2u8).is_none()
///         == (match Some(2u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(47u8).is_none()
///         == (match Some(47u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// # }
/// ```
pub fn option_is_none() {}
/// # Properties for [`Option::expect`]
/// ## Unwrapping a [`None`] with `expect` always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.expect("message"))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(1u8).expect("message")));
/// assert!(doesn_t_panic!(Some(0u8).expect("message")));
/// assert!(doesn_t_panic!(Some(254u8).expect("message")));
/// assert!(doesn_t_panic!(Some(255u8).expect("message")));
/// assert!(doesn_t_panic!(Some(177u8).expect("message")));
/// assert!(doesn_t_panic!(Some(197u8).expect("message")));
/// assert!(doesn_t_panic!(Some(6u8).expect("message")));
/// assert!(doesn_t_panic!(Some(245u8).expect("message")));
/// assert!(doesn_t_panic!(Some(227u8).expect("message")));
/// assert!(doesn_t_panic!(Some(72u8).expect("message")));
/// # }
/// ```
/// ## Wrapping a value in a `Some` and unwrapping is identity
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).unwrap() == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(255u8).unwrap() == 255u8);
/// assert!(Some(0u8).unwrap() == 0u8);
/// assert!(Some(1u8).unwrap() == 1u8);
/// assert!(Some(254u8).unwrap() == 254u8);
/// assert!(Some(221u8).unwrap() == 221u8);
/// assert!(Some(106u8).unwrap() == 106u8);
/// assert!(Some(134u8).unwrap() == 134u8);
/// assert!(Some(178u8).unwrap() == 178u8);
/// assert!(Some(145u8).unwrap() == 145u8);
/// assert!(Some(115u8).unwrap() == 115u8);
/// # }
/// ```
pub fn option_expect() {}
/// # Properties for [`Option::map`]
/// ## Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`
/// __Inputs:__ `v : T, f : Fn1<T, T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).map(f) == Some((f)(v))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(254u8).map((|x: u8| x.wrapping_add(1u8)))
///         == Some(((|x: u8| x.wrapping_add(1u8)))(254u8)));
/// assert!(Some(255u8).map((|x: u8| x.wrapping_add(0u8)))
///         == Some(((|x: u8| x.wrapping_add(0u8)))(255u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(255u8)))
///         == Some(((|x: u8| x.wrapping_add(255u8)))(0u8)));
/// assert!(Some(254u8).map((|x: u8| x.wrapping_add(254u8)))
///         == Some(((|x: u8| x.wrapping_add(254u8)))(254u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(254u8)))
///         == Some(((|x: u8| x.wrapping_add(254u8)))(0u8)));
/// assert!(Some(255u8).map((|x: u8| x.wrapping_add(x)))
///         == Some(((|x: u8| x.wrapping_add(x)))(255u8)));
/// assert!(Some(243u8).map((|x: u8| x)) == Some(((|x: u8| x))(243u8)));
/// assert!(Some(112u8).map((|x: u8| x)) == Some(((|x: u8| x))(112u8)));
/// assert!(Some(84u8).map((|x: u8| x)) == Some(((|x: u8| x))(84u8)));
/// assert!(Some(110u8).map((|x: u8| x)) == Some(((|x: u8| x))(110u8)));
/// # }
/// ```
/// ## Mapping a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : Fn1<T, T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.map(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.map((|x: u8| x.wrapping_add(x))) == Option::<u8>::None);
/// # }
/// ```
pub fn option_map() {}
/// # Properties for [`Option::filter`]
/// ## The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`
/// __Inputs:__ `v : T, f : FnR1<T, bool>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).filter(f).is_some() == f(&v)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(255u8).filter((|x: &u8| *x < 254u8)).is_some() == (|x: &u8| *x < 254u8)(&255u8));
/// assert!(Some(255u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&255u8));
/// assert!(Some(255u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&255u8));
/// assert!(Some(254u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&254u8));
/// assert!(Some(0u8).filter((|x: &u8| *x < 254u8)).is_some() == (|x: &u8| *x < 254u8)(&0u8));
/// assert!(Some(254u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&254u8));
/// assert!(Some(131u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&131u8));
/// assert!(Some(109u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&109u8));
/// assert!(Some(8u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&8u8));
/// assert!(Some(22u8).filter((|x: &u8| *x > 128)).is_some() == (|x: &u8| *x > 128)(&22u8));
/// # }
/// ```
/// ## Filtering a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : FnR1<T, bool>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.filter(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.filter((|x: &u8| *x < 0u8)) == Option::<u8>::None);
/// # }
/// ```
pub fn option_filter() {}
/// # Properties for [`Option::flatten`]
/// ## Nested `Some`s
/// __Inputs:__ `x : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(Some(x)).flatten() == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(Some(0u8)).flatten() == Some(0u8));
/// assert!(Some(Some(254u8)).flatten() == Some(254u8));
/// assert!(Some(Some(1u8)).flatten() == Some(1u8));
/// assert!(Some(Some(255u8)).flatten() == Some(255u8));
/// assert!(Some(Some(113u8)).flatten() == Some(113u8));
/// assert!(Some(Some(217u8)).flatten() == Some(217u8));
/// assert!(Some(Some(72u8)).flatten() == Some(72u8));
/// assert!(Some(Some(168u8)).flatten() == Some(168u8));
/// assert!(Some(Some(19u8)).flatten() == Some(19u8));
/// assert!(Some(Some(50u8)).flatten() == Some(50u8));
/// # }
/// ```
/// ## Nested or direct `None` flattens to None
/// __Inputs:__ `x : Option<Option<T>>`  
/// __Precondition:__ `x.is_none() || x.unwrap().is_none()`  
/// __Postcondition:__ `x.flatten() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<Option<u8>>::None.flatten() == None);
/// assert!(Some(Option::<u8>::None).flatten() == None);
/// # }
/// ```
pub fn option_flatten() {}
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let mut y = Some(0u8).clone();
///         y.take() == Some(0u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(255u8).clone();
///         y.take() == Some(255u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(254u8).clone();
///         y.take() == Some(254u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Option::<u8>::None.clone();
///         y.take() == Option::<u8>::None && y.is_none()
///     });
/// assert!({
///         let mut y = Some(1u8).clone();
///         y.take() == Some(1u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(249u8).clone();
///         y.take() == Some(249u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(173u8).clone();
///         y.take() == Some(173u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(154u8).clone();
///         y.take() == Some(154u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(12u8).clone();
///         y.take() == Some(12u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(172u8).clone();
///         y.take() == Some(172u8) && y.is_none()
///     });
/// # }
/// ```
pub fn option_take() {}
/// # Properties for [`Option::zip`]
/// ## Zipping two non-empty options
/// __Inputs:__ `x : T, y : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(x).zip(Some(y)) == Some((x, y))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).zip(Some(255u8)) == Some((0u8, 255u8)));
/// assert!(Some(0u8).zip(Some(254u8)) == Some((0u8, 254u8)));
/// assert!(Some(1u8).zip(Some(1u8)) == Some((1u8, 1u8)));
/// assert!(Some(254u8).zip(Some(254u8)) == Some((254u8, 254u8)));
/// assert!(Some(0u8).zip(Some(0u8)) == Some((0u8, 0u8)));
/// assert!(Some(173u8).zip(Some(15u8)) == Some((173u8, 15u8)));
/// assert!(Some(5u8).zip(Some(174u8)) == Some((5u8, 174u8)));
/// assert!(Some(131u8).zip(Some(97u8)) == Some((131u8, 97u8)));
/// assert!(Some(190u8).zip(Some(30u8)) == Some((190u8, 30u8)));
/// assert!(Some(192u8).zip(Some(230u8)) == Some((192u8, 230u8)));
/// # }
/// ```
/// ## Zipping two options when one is a `None` makes `None`
/// __Inputs:__ `x : Option<T>, y : Option<T>`  
/// __Precondition:__ `x.is_none() || y.is_none()`  
/// __Postcondition:__ `x.zip(y).is_none()`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.zip(Some(0u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(1u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(254u8)).is_none());
/// assert!(Some(254u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(1u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(255u8).zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Some(10u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(39u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(211u8)).is_none());
/// # }
/// ```
pub fn option_zip() {}
/// # Properties for [`Option::unwrap`]
/// ## Unwrapping a [`None`] always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).unwrap()));
/// assert!(doesn_t_panic!(Some(254u8).unwrap()));
/// assert!(doesn_t_panic!(Some(1u8).unwrap()));
/// assert!(doesn_t_panic!(Some(255u8).unwrap()));
/// assert!(doesn_t_panic!(Some(50u8).unwrap()));
/// assert!(doesn_t_panic!(Some(134u8).unwrap()));
/// assert!(doesn_t_panic!(Some(118u8).unwrap()));
/// assert!(doesn_t_panic!(Some(99u8).unwrap()));
/// assert!(doesn_t_panic!(Some(197u8).unwrap()));
/// assert!(doesn_t_panic!(Some(7u8).unwrap()));
/// # }
/// ```
pub fn option_unwrap() {}
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(1u8).unwrap().clone(), Some(1u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(20u8).unwrap().clone(), Some(20u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(11u8).unwrap().clone(), Some(11u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(34u8).unwrap().clone(), Some(34u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(37u8).unwrap().clone(), Some(37u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(42u8).unwrap().clone(), Some(42u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(3u8).unwrap().clone(), Some(3u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(26u8).unwrap().clone(), Some(26u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(47u8).unwrap().clone(), Some(47u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// # }
/// ```
pub fn option_as_mut() {}
/// # Properties for [`Option::as_slice`]
/// ## [`None.as_slice()`] should always result in an empty slice
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `{ v.as_slice().is_empty() }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Some(254u8).as_slice() == [254u8] });
/// assert!({ Some(0u8).as_slice() == [0u8] });
/// assert!({ Some(1u8).as_slice() == [1u8] });
/// assert!({ Some(255u8).as_slice() == [255u8] });
/// assert!({ Some(144u8).as_slice() == [144u8] });
/// assert!({ Some(11u8).as_slice() == [11u8] });
/// assert!({ Some(94u8).as_slice() == [94u8] });
/// assert!({ Some(246u8).as_slice() == [246u8] });
/// assert!({ Some(177u8).as_slice() == [177u8] });
/// assert!({ Some(174u8).as_slice() == [174u8] });
/// # }
/// ```
pub fn option_as_slice() {}
/// # Properties for [`Vec`]
/// ## Indexing
/// __Inputs:__ `v : Vec<u8>, i : usize`  
/// __Precondition:__ `v.len() > 0`  
/// __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![255u8].get(0usize) == Some(&255u8));
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![254u8].get(0usize) == Some(&254u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![206u8].get(0usize) == Some(&206u8));
/// assert!(vec![72u8, 252u8].get(0usize) == Some(&72u8));
/// assert!(vec![76u8].get(0usize) == Some(&76u8));
/// assert!(vec![182u8, 228u8, 161u8, 0u8, 39u8].get(1usize) == Some(&228u8));
/// assert!(vec![218u8].get(0usize) == Some(&218u8));
/// # }
/// ```
pub fn slice_get() {}
/// # Properties for [`core::ops::Rem::<u8>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u8::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8 % 255u8 == 0u8);
/// assert!(255u8 % 254u8 == 1u8);
/// assert!(0u8 % 255u8 == 0u8);
/// assert!(0u8 % 254u8 == 0u8);
/// assert!(254u8 % 1u8 == 0u8);
/// assert!(254u8 % 254u8 == 0u8);
/// assert!(153u8 % 247u8 == 153u8);
/// assert!(221u8 % 176u8 == 45u8);
/// assert!(47u8 % 70u8 == 47u8);
/// assert!(148u8 % 147u8 == 1u8);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u8_rem() {}
/// # Properties for [`u8::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u8::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_rem(1u8) == Some(0u8));
/// assert!(0u8.checked_rem(255u8) == Some(0u8));
/// assert!(255u8.checked_rem(255u8) == Some(0u8));
/// assert!(0u8.checked_rem(254u8) == Some(0u8));
/// assert!(254u8.checked_rem(254u8) == Some(0u8));
/// assert!(200u8.checked_rem(199u8) == Some(1u8));
/// assert!(179u8.checked_rem(224u8) == Some(179u8));
/// assert!(161u8.checked_rem(99u8) == Some(62u8));
/// assert!(158u8.checked_rem(169u8) == Some(158u8));
/// assert!(22u8.checked_rem(20u8) == Some(2u8));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u8.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u8_checked_rem() {}
/// # Properties for [`u8::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `x == u8::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u8::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_neg() == Some(0u8));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `x != u8::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_neg() == None);
/// assert!(1u8.checked_neg() == None);
/// assert!(254u8.checked_neg() == None);
/// assert!(103u8.checked_neg() == None);
/// assert!(119u8.checked_neg() == None);
/// assert!(211u8.checked_neg() == None);
/// assert!(91u8.checked_neg() == None);
/// assert!(205u8.checked_neg() == None);
/// assert!(58u8.checked_neg() == None);
/// assert!(62u8.checked_neg() == None);
/// # }
/// ```
pub fn t_u8_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u8>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x << y == u8::down((x.up() << y) & u8::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8 << 0u32 == 254u8);
/// assert!(0u8 << 1u32 == 0u8);
/// assert!(255u8 << 1u32 == 254u8);
/// assert!(0u8 << 0u32 == 0u8);
/// assert!(1u8 << 0u32 == 1u8);
/// assert!(32u8 << 2u32 == 128u8);
/// assert!(223u8 << 2u32 == 124u8);
/// assert!(242u8 << 7u32 == 0u8);
/// assert!(165u8 << 2u32 == 148u8);
/// assert!(132u8 << 4u32 == 64u8);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u8 << 4294967295u32));
/// assert!(panics!(254u8 << 4294967295u32));
/// assert!(panics!(254u8 << 4294967294u32));
/// assert!(panics!(255u8 << 4294967295u32));
/// assert!(panics!(0u8 << 4294967294u32));
/// assert!(panics!(255u8 << 4294967294u32));
/// assert!(panics!(236u8 << 130u32));
/// assert!(panics!(27u8 << 47u32));
/// assert!(panics!(246u8 << 28u32));
/// assert!(panics!(119u8 << 76u32));
/// # }
/// ```
pub fn core_ops_shl_u8_shl() {}
/// # Properties for [`core::ops::Shl::<u8>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u8::down((x.up() << y) & u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shl(0u32) == Some(0u8));
/// assert!(1u8.checked_shl(0u32) == Some(1u8));
/// assert!(255u8.checked_shl(0u32) == Some(255u8));
/// assert!(254u8.checked_shl(0u32) == Some(254u8));
/// assert!(32u8.checked_shl(3u32) == Some(0u8));
/// assert!(233u8.checked_shl(5u32) == Some(32u8));
/// assert!(26u8.checked_shl(0u32) == Some(26u8));
/// assert!(70u8.checked_shl(0u32) == Some(70u8));
/// assert!(1u8.checked_shl(5u32) == Some(32u8));
/// assert!(179u8.checked_shl(6u32) == Some(192u8));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_shl(4294967295u32) == None);
/// assert!(0u8.checked_shl(4294967295u32) == None);
/// assert!(254u8.checked_shl(4294967294u32) == None);
/// assert!(254u8.checked_shl(4294967295u32) == None);
/// assert!(0u8.checked_shl(4294967294u32) == None);
/// assert!(255u8.checked_shl(106u32) == None);
/// assert!(160u8.checked_shl(72u32) == None);
/// assert!(76u8.checked_shl(11u32) == None);
/// assert!(66u8.checked_shl(20u32) == None);
/// assert!(102u8.checked_shl(78u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u8_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u8>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u8::down((x.up() << y) & u8::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shl(0u32) == (0u8, false));
/// assert!(0u8.overflowing_shl(1u32) == (0u8, false));
/// assert!(1u8.overflowing_shl(0u32) == (1u8, false));
/// assert!(254u8.overflowing_shl(0u32) == (254u8, false));
/// assert!(155u8.overflowing_shl(4u32) == (176u8, false));
/// assert!(151u8.overflowing_shl(3u32) == (184u8, false));
/// assert!(155u8.overflowing_shl(6u32) == (192u8, false));
/// assert!(71u8.overflowing_shl(0u32) == (71u8, false));
/// assert!(59u8.overflowing_shl(5u32) == (96u8, false));
/// assert!(204u8.overflowing_shl(2u32) == (48u8, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u8::down((x.up() << (y & (u8::BITS - 1)) & u8::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.overflowing_shl(4294967295u32) == (128u8, true));
/// assert!(254u8.overflowing_shl(4294967294u32) == (128u8, true));
/// assert!(0u8.overflowing_shl(4294967294u32) == (0u8, true));
/// assert!(1u8.overflowing_shl(4294967295u32) == (128u8, true));
/// assert!(0u8.overflowing_shl(4294967295u32) == (0u8, true));
/// assert!(100u8.overflowing_shl(55u32) == (0u8, true));
/// assert!(216u8.overflowing_shl(35u32) == (192u8, true));
/// assert!(87u8.overflowing_shl(95u32) == (128u8, true));
/// assert!(63u8.overflowing_shl(88u32) == (63u8, true));
/// assert!(224u8.overflowing_shl(112u32) == (224u8, true));
/// # }
/// ```
pub fn core_ops_shl_u8_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u8>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x >> y == u8::down((x.up() >> y) & u8::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 >> 0u32 == 0u8);
/// assert!(0u8 >> 1u32 == 0u8);
/// assert!(255u8 >> 0u32 == 255u8);
/// assert!(1u8 >> 0u32 == 1u8);
/// assert!(139u8 >> 4u32 == 8u8);
/// assert!(86u8 >> 7u32 == 0u8);
/// assert!(225u8 >> 0u32 == 225u8);
/// assert!(129u8 >> 6u32 == 2u8);
/// assert!(18u8 >> 6u32 == 0u8);
/// assert!(141u8 >> 4u32 == 8u8);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(255u8 >> 4294967295u32));
/// assert!(panics!(0u8 >> 4294967294u32));
/// assert!(panics!(255u8 >> 4294967294u32));
/// assert!(panics!(0u8 >> 4294967295u32));
/// assert!(panics!(1u8 >> 4294967295u32));
/// assert!(panics!(1u8 >> 4294967294u32));
/// assert!(panics!(127u8 >> 94u32));
/// assert!(panics!(158u8 >> 23u32));
/// assert!(panics!(204u8 >> 35u32));
/// assert!(panics!(132u8 >> 68u32));
/// # }
/// ```
pub fn core_ops_shr_u8_shr() {}
/// # Properties for [`core::ops::Shr::<u8>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u8::down((x.up() >> y) & u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.checked_shr(1u32) == Some(127u8));
/// assert!(254u8.checked_shr(0u32) == Some(254u8));
/// assert!(255u8.checked_shr(0u32) == Some(255u8));
/// assert!(1u8.checked_shr(1u32) == Some(0u8));
/// assert!(0u8.checked_shr(0u32) == Some(0u8));
/// assert!(252u8.checked_shr(7u32) == Some(1u8));
/// assert!(214u8.checked_shr(0u32) == Some(214u8));
/// assert!(149u8.checked_shr(2u32) == Some(37u8));
/// assert!(99u8.checked_shr(6u32) == Some(1u8));
/// assert!(246u8.checked_shr(6u32) == Some(3u8));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shr(4294967295u32) == None);
/// assert!(254u8.checked_shr(4294967295u32) == None);
/// assert!(0u8.checked_shr(4294967294u32) == None);
/// assert!(255u8.checked_shr(4294967294u32) == None);
/// assert!(255u8.checked_shr(4294967295u32) == None);
/// assert!(69u8.checked_shr(83u32) == None);
/// assert!(155u8.checked_shr(47u32) == None);
/// assert!(168u8.checked_shr(54u32) == None);
/// assert!(192u8.checked_shr(35u32) == None);
/// assert!(11u8.checked_shr(28u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u8_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u8>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u8::down((x.up() >> y) & u8::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shr(0u32) == (0u8, false));
/// assert!(255u8.overflowing_shr(0u32) == (255u8, false));
/// assert!(254u8.overflowing_shr(0u32) == (254u8, false));
/// assert!(1u8.overflowing_shr(0u32) == (1u8, false));
/// assert!(255u8.overflowing_shr(1u32) == (127u8, false));
/// assert!(185u8.overflowing_shr(6u32) == (2u8, false));
/// assert!(97u8.overflowing_shr(1u32) == (48u8, false));
/// assert!(213u8.overflowing_shr(2u32) == (53u8, false));
/// assert!(67u8.overflowing_shr(6u32) == (1u8, false));
/// assert!(251u8.overflowing_shr(0u32) == (251u8, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u8::down((x.up() >> (y & (u8::BITS - 1)) & u8::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.overflowing_shr(4294967295u32) == (1u8, true));
/// assert!(255u8.overflowing_shr(4294967294u32) == (3u8, true));
/// assert!(255u8.overflowing_shr(4294967295u32) == (1u8, true));
/// assert!(0u8.overflowing_shr(4294967295u32) == (0u8, true));
/// assert!(1u8.overflowing_shr(4294967295u32) == (0u8, true));
/// assert!(254u8.overflowing_shr(4294967294u32) == (3u8, true));
/// assert!(47u8.overflowing_shr(114u32) == (11u8, true));
/// assert!(163u8.overflowing_shr(46u32) == (2u8, true));
/// assert!(14u8.overflowing_shr(65u32) == (7u8, true));
/// assert!(127u8.overflowing_shr(125u32) == (3u8, true));
/// # }
/// ```
pub fn core_ops_shr_u8_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u8>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u8::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8 / 255u8 == 0u8);
/// assert!(255u8 / 255u8 == 1u8);
/// assert!(255u8 / 1u8 == 255u8);
/// assert!(0u8 / 255u8 == 0u8);
/// assert!(1u8 / 1u8 == 1u8);
/// assert!(7u8 / 3u8 == 2u8);
/// assert!(13u8 / 8u8 == 1u8);
/// assert!(15u8 / 15u8 == 1u8);
/// assert!(14u8 / 11u8 == 1u8);
/// assert!(10u8 / 15u8 == 0u8);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(255u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(254u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(219u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(189u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(141u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(146u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(42u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(208u8 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u8_div() {}
/// # Properties for [`u8::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u8::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_div(254u8) == 0u8);
/// assert!(255u8.saturating_div(1u8) == 255u8);
/// assert!(254u8.saturating_div(1u8) == 254u8);
/// assert!(1u8.saturating_div(255u8) == 0u8);
/// assert!(254u8.saturating_div(255u8) == 0u8);
/// assert!(15u8.saturating_div(10u8) == 1u8);
/// assert!(2u8.saturating_div(13u8) == 0u8);
/// assert!(15u8.saturating_div(12u8) == 1u8);
/// assert!(15u8.saturating_div(15u8) == 1u8);
/// assert!(11u8.saturating_div(4u8) == 2u8);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(254u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(255u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(59u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(119u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(19u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(134u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(144u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(115u8.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u8_saturating_div() {}
/// # Properties for [`u8::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u8::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.checked_div(1u8) == Some(1u8));
/// assert!(254u8.checked_div(255u8) == Some(0u8));
/// assert!(0u8.checked_div(255u8) == Some(0u8));
/// assert!(0u8.checked_div(254u8) == Some(0u8));
/// assert!(0u8.checked_div(1u8) == Some(0u8));
/// assert!(254u8.checked_div(1u8) == Some(254u8));
/// assert!(11u8.checked_div(15u8) == Some(0u8));
/// assert!(7u8.checked_div(8u8) == Some(0u8));
/// assert!(14u8.checked_div(13u8) == Some(1u8));
/// assert!(15u8.checked_div(8u8) == Some(1u8));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_div(0) == None);
/// assert!(255u8.checked_div(0) == None);
/// assert!(254u8.checked_div(0) == None);
/// assert!(1u8.checked_div(0) == None);
/// assert!(236u8.checked_div(0) == None);
/// assert!(223u8.checked_div(0) == None);
/// assert!(91u8.checked_div(0) == None);
/// assert!(246u8.checked_div(0) == None);
/// assert!(125u8.checked_div(0) == None);
/// assert!(124u8.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u8_checked_div() {}
/// # Properties for [`core::ops::Mul::<u8>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x * y == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 0u8 == 0u8);
/// assert!(1u8 * 0u8 == 0u8);
/// assert!(0u8 * 254u8 == 0u8);
/// assert!(0u8 * 255u8 == 0u8);
/// assert!(10u8 * 15u8 == 150u8);
/// assert!(4u8 * 1u8 == 4u8);
/// assert!(9u8 * 6u8 == 54u8);
/// assert!(7u8 * 6u8 == 42u8);
/// assert!(9u8 * 7u8 == 63u8);
/// assert!(14u8 * 4u8 == 56u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(255u8 * 254u8));
/// assert!(panics!(254u8 * 255u8));
/// assert!(panics!(255u8 * 255u8));
/// assert!(panics!(254u8 * 254u8));
/// assert!(panics!(46u8 * 16u8));
/// assert!(panics!(85u8 * 183u8));
/// assert!(panics!(251u8 * 193u8));
/// assert!(panics!(88u8 * 38u8));
/// assert!(panics!(65u8 * 222u8));
/// assert!(panics!(44u8 * 180u8));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 1 == 0u8);
/// assert!(255u8 * 1 == 255u8);
/// assert!(254u8 * 1 == 254u8);
/// assert!(1u8 * 1 == 1u8);
/// assert!(7u8 * 1 == 7u8);
/// assert!(67u8 * 1 == 67u8);
/// assert!(132u8 * 1 == 132u8);
/// assert!(234u8 * 1 == 234u8);
/// assert!(38u8 * 1 == 38u8);
/// assert!(50u8 * 1 == 50u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 1u8 == 1u8);
/// assert!(1 * 0u8 == 0u8);
/// assert!(1 * 254u8 == 254u8);
/// assert!(1 * 255u8 == 255u8);
/// assert!(1 * 176u8 == 176u8);
/// assert!(1 * 100u8 == 100u8);
/// assert!(1 * 59u8 == 59u8);
/// assert!(1 * 149u8 == 149u8);
/// assert!(1 * 118u8 == 118u8);
/// assert!(1 * 18u8 == 18u8);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8 * 0u8 == 0u8 * 1u8);
/// assert!(0u8 * 0u8 == 0u8 * 0u8);
/// assert!(0u8 * 254u8 == 254u8 * 0u8);
/// assert!(0u8 * 255u8 == 255u8 * 0u8);
/// assert!(254u8 * 0u8 == 0u8 * 254u8);
/// assert!(6u8 * 12u8 == 12u8 * 6u8);
/// assert!(8u8 * 8u8 == 8u8 * 8u8);
/// assert!(9u8 * 5u8 == 5u8 * 9u8);
/// assert!(14u8 * 6u8 == 6u8 * 14u8);
/// assert!(10u8 * 15u8 == 15u8 * 10u8);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u8::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u8 * 1u8) * 254u8 == 1u8 * (1u8 * 254u8));
/// assert!((1u8 * 254u8) * 1u8 == 1u8 * (254u8 * 1u8));
/// assert!((254u8 * 1u8) * 1u8 == 254u8 * (1u8 * 1u8));
/// assert!((1u8 * 1u8) * 1u8 == 1u8 * (1u8 * 1u8));
/// assert!((1u8 * 1u8) * 255u8 == 1u8 * (1u8 * 255u8));
/// assert!((255u8 * 1u8) * 1u8 == 255u8 * (1u8 * 1u8));
/// assert!((4u8 * 5u8) * 4u8 == 4u8 * (5u8 * 4u8));
/// assert!((3u8 * 3u8) * 4u8 == 3u8 * (3u8 * 4u8));
/// assert!((5u8 * 3u8) * 5u8 == 5u8 * (3u8 * 5u8));
/// assert!((6u8 * 3u8) * 6u8 == 6u8 * (3u8 * 6u8));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u8::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8 * (0u8 + 0u8) == 254u8 * 0u8 + 254u8 * 0u8);
/// assert!(1u8 * (0u8 + 254u8) == 1u8 * 0u8 + 1u8 * 254u8);
/// assert!(1u8 * (0u8 + 255u8) == 1u8 * 0u8 + 1u8 * 255u8);
/// assert!(1u8 * (254u8 + 0u8) == 1u8 * 254u8 + 1u8 * 0u8);
/// assert!(1u8 * (1u8 + 0u8) == 1u8 * 1u8 + 1u8 * 0u8);
/// assert!(6u8 * (12u8 + 15u8) == 6u8 * 12u8 + 6u8 * 15u8);
/// assert!(11u8 * (14u8 + 2u8) == 11u8 * 14u8 + 11u8 * 2u8);
/// assert!(9u8 * (14u8 + 5u8) == 9u8 * 14u8 + 9u8 * 5u8);
/// assert!(8u8 * (14u8 + 15u8) == 8u8 * 14u8 + 8u8 * 15u8);
/// assert!(15u8 * (4u8 + 13u8) == 15u8 * 4u8 + 15u8 * 13u8);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8 * 0 == 0);
/// assert!(0u8 * 0 == 0);
/// assert!(255u8 * 0 == 0);
/// assert!(254u8 * 0 == 0);
/// assert!(5u8 * 0 == 0);
/// assert!(15u8 * 0 == 0);
/// assert!(14u8 * 0 == 0);
/// assert!(10u8 * 0 == 0);
/// assert!(11u8 * 0 == 0);
/// assert!(12u8 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u8_mul() {}
/// # Properties for [`u8::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u8::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_mul(1u8) == Some(255u8));
/// assert!(0u8.checked_mul(254u8) == Some(0u8));
/// assert!(255u8.checked_mul(0u8) == Some(0u8));
/// assert!(0u8.checked_mul(1u8) == Some(0u8));
/// assert!(1u8.checked_mul(0u8) == Some(0u8));
/// assert!(6u8.checked_mul(12u8) == Some(72u8));
/// assert!(15u8.checked_mul(15u8) == Some(225u8));
/// assert!(8u8.checked_mul(14u8) == Some(112u8));
/// assert!(10u8.checked_mul(14u8) == Some(140u8));
/// assert!(11u8.checked_mul(15u8) == Some(165u8));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_mul(255u8) == None);
/// assert!(254u8.checked_mul(254u8) == None);
/// assert!(254u8.checked_mul(255u8) == None);
/// assert!(255u8.checked_mul(254u8) == None);
/// assert!(176u8.checked_mul(162u8) == None);
/// assert!(40u8.checked_mul(221u8) == None);
/// assert!(190u8.checked_mul(81u8) == None);
/// assert!(29u8.checked_mul(232u8) == None);
/// assert!(135u8.checked_mul(22u8) == None);
/// assert!(112u8.checked_mul(139u8) == None);
/// # }
/// ```
pub fn core_ops_mul_u8_checked_mul() {}
/// # Properties for [`u8::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u8::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_mul(255u8) == (0u8, false));
/// assert!(255u8.overflowing_mul(1u8) == (255u8, false));
/// assert!(1u8.overflowing_mul(1u8) == (1u8, false));
/// assert!(1u8.overflowing_mul(0u8) == (0u8, false));
/// assert!(0u8.overflowing_mul(1u8) == (0u8, false));
/// assert!(10u8.overflowing_mul(11u8) == (110u8, false));
/// assert!(15u8.overflowing_mul(15u8) == (225u8, false));
/// assert!(5u8.overflowing_mul(11u8) == (55u8, false));
/// assert!(13u8.overflowing_mul(14u8) == (182u8, false));
/// assert!(10u8.overflowing_mul(4u8) == (40u8, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.overflowing_mul(254u8) == (4u8, true));
/// assert!(254u8.overflowing_mul(255u8) == (2u8, true));
/// assert!(255u8.overflowing_mul(255u8) == (1u8, true));
/// assert!(255u8.overflowing_mul(254u8) == (2u8, true));
/// assert!(46u8.overflowing_mul(55u8) == (226u8, true));
/// assert!(15u8.overflowing_mul(108u8) == (84u8, true));
/// assert!(213u8.overflowing_mul(77u8) == (17u8, true));
/// assert!(249u8.overflowing_mul(193u8) == (185u8, true));
/// assert!(31u8.overflowing_mul(205u8) == (211u8, true));
/// assert!(19u8.overflowing_mul(23u8) == (181u8, true));
/// # }
/// ```
pub fn core_ops_mul_u8_overflowing_mul() {}
/// # Properties for [`u8::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::down((x.up() * y.up()).min(u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.saturating_mul(255u8) == 255u8);
/// assert!(1u8.saturating_mul(254u8) == 254u8);
/// assert!(1u8.saturating_mul(0u8) == 0u8);
/// assert!(1u8.saturating_mul(1u8) == 1u8);
/// assert!(0u8.saturating_mul(254u8) == 0u8);
/// assert!(0u8.saturating_mul(0u8) == 0u8);
/// assert!(171u8.saturating_mul(143u8) == 255u8);
/// assert!(193u8.saturating_mul(175u8) == 255u8);
/// assert!(175u8.saturating_mul(79u8) == 255u8);
/// assert!(167u8.saturating_mul(220u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_mul(1u8) == 0u8);
/// assert!(1u8.saturating_mul(0u8) == 0u8);
/// assert!(0u8.saturating_mul(0u8) == 0u8);
/// assert!(0u8.saturating_mul(255u8) == 0u8);
/// assert!(254u8.saturating_mul(0u8) == 0u8);
/// assert!(1u8.saturating_mul(255u8) == 255u8);
/// assert!(11u8.saturating_mul(11u8) == 121u8);
/// assert!(6u8.saturating_mul(10u8) == 60u8);
/// assert!(8u8.saturating_mul(1u8) == 8u8);
/// assert!(13u8.saturating_mul(9u8) == 117u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.saturating_mul(254u8) == u8::MAX);
/// assert!(255u8.saturating_mul(255u8) == u8::MAX);
/// assert!(254u8.saturating_mul(255u8) == u8::MAX);
/// assert!(254u8.saturating_mul(254u8) == u8::MAX);
/// assert!(74u8.saturating_mul(63u8) == u8::MAX);
/// assert!(164u8.saturating_mul(131u8) == u8::MAX);
/// assert!(6u8.saturating_mul(182u8) == u8::MAX);
/// assert!(216u8.saturating_mul(85u8) == u8::MAX);
/// assert!(216u8.saturating_mul(189u8) == u8::MAX);
/// assert!(2u8.saturating_mul(128u8) == u8::MAX);
/// # }
/// ```
pub fn core_ops_mul_u8_saturating_mul() {}
/// # Properties for [`u8::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u8::down((x.up() * y.up()) % (u8::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.wrapping_mul(255u8) == 255u8);
/// assert!(0u8.wrapping_mul(254u8) == 0u8);
/// assert!(0u8.wrapping_mul(0u8) == 0u8);
/// assert!(255u8.wrapping_mul(0u8) == 0u8);
/// assert!(254u8.wrapping_mul(1u8) == 254u8);
/// assert!(229u8.wrapping_mul(22u8) == 174u8);
/// assert!(191u8.wrapping_mul(151u8) == 169u8);
/// assert!(123u8.wrapping_mul(63u8) == 69u8);
/// assert!(92u8.wrapping_mul(148u8) == 48u8);
/// assert!(184u8.wrapping_mul(149u8) == 24u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.wrapping_mul(1u8) == 254u8);
/// assert!(0u8.wrapping_mul(1u8) == 0u8);
/// assert!(0u8.wrapping_mul(255u8) == 0u8);
/// assert!(1u8.wrapping_mul(1u8) == 1u8);
/// assert!(1u8.wrapping_mul(254u8) == 254u8);
/// assert!(9u8.wrapping_mul(15u8) == 135u8);
/// assert!(5u8.wrapping_mul(9u8) == 45u8);
/// assert!(15u8.wrapping_mul(11u8) == 165u8);
/// assert!(14u8.wrapping_mul(13u8) == 182u8);
/// assert!(9u8.wrapping_mul(4u8) == 36u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 1 == 0u8);
/// assert!(254u8 * 1 == 254u8);
/// assert!(1u8 * 1 == 1u8);
/// assert!(255u8 * 1 == 255u8);
/// assert!(92u8 * 1 == 92u8);
/// assert!(106u8 * 1 == 106u8);
/// assert!(36u8 * 1 == 36u8);
/// assert!(197u8 * 1 == 197u8);
/// assert!(113u8 * 1 == 113u8);
/// assert!(46u8 * 1 == 46u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 255u8 == 255u8);
/// assert!(1 * 0u8 == 0u8);
/// assert!(1 * 1u8 == 1u8);
/// assert!(1 * 254u8 == 254u8);
/// assert!(1 * 109u8 == 109u8);
/// assert!(1 * 87u8 == 87u8);
/// assert!(1 * 183u8 == 183u8);
/// assert!(1 * 4u8 == 4u8);
/// assert!(1 * 141u8 == 141u8);
/// assert!(1 * 172u8 == 172u8);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 0u8 == 0u8 * 0u8);
/// assert!(254u8 * 0u8 == 0u8 * 254u8);
/// assert!(0u8 * 255u8 == 255u8 * 0u8);
/// assert!(1u8 * 0u8 == 0u8 * 1u8);
/// assert!(1u8 * 1u8 == 1u8 * 1u8);
/// assert!(255u8 * 1u8 == 1u8 * 255u8);
/// assert!(15u8 * 2u8 == 2u8 * 15u8);
/// assert!(7u8 * 11u8 == 11u8 * 7u8);
/// assert!(11u8 * 15u8 == 15u8 * 11u8);
/// assert!(2u8 * 13u8 == 13u8 * 2u8);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u8::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u8 * 1u8) * 254u8 == 1u8 * (1u8 * 254u8));
/// assert!((254u8 * 1u8) * 1u8 == 254u8 * (1u8 * 1u8));
/// assert!((1u8 * 1u8) * 1u8 == 1u8 * (1u8 * 1u8));
/// assert!((1u8 * 1u8) * 255u8 == 1u8 * (1u8 * 255u8));
/// assert!((1u8 * 255u8) * 1u8 == 1u8 * (255u8 * 1u8));
/// assert!((255u8 * 1u8) * 1u8 == 255u8 * (1u8 * 1u8));
/// assert!((4u8 * 2u8) * 5u8 == 4u8 * (2u8 * 5u8));
/// assert!((2u8 * 3u8) * 4u8 == 2u8 * (3u8 * 4u8));
/// assert!((5u8 * 5u8) * 4u8 == 5u8 * (5u8 * 4u8));
/// assert!((3u8 * 4u8) * 3u8 == 3u8 * (4u8 * 3u8));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u8::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8 * (0u8 + 1u8) == 255u8 * 0u8 + 255u8 * 1u8);
/// assert!(255u8 * (0u8 + 0u8) == 255u8 * 0u8 + 255u8 * 0u8);
/// assert!(1u8 * (0u8 + 1u8) == 1u8 * 0u8 + 1u8 * 1u8);
/// assert!(1u8 * (0u8 + 255u8) == 1u8 * 0u8 + 1u8 * 255u8);
/// assert!(254u8 * (0u8 + 0u8) == 254u8 * 0u8 + 254u8 * 0u8);
/// assert!(6u8 * (11u8 + 15u8) == 6u8 * 11u8 + 6u8 * 15u8);
/// assert!(10u8 * (11u8 + 2u8) == 10u8 * 11u8 + 10u8 * 2u8);
/// assert!(9u8 * (11u8 + 8u8) == 9u8 * 11u8 + 9u8 * 8u8);
/// assert!(11u8 * (8u8 + 10u8) == 11u8 * 8u8 + 11u8 * 10u8);
/// assert!(1u8 * (8u8 + 10u8) == 1u8 * 8u8 + 1u8 * 10u8);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 0 == 0);
/// assert!(255u8 * 0 == 0);
/// assert!(254u8 * 0 == 0);
/// assert!(1u8 * 0 == 0);
/// assert!(11u8 * 0 == 0);
/// assert!(14u8 * 0 == 0);
/// assert!(15u8 * 0 == 0);
/// assert!(12u8 * 0 == 0);
/// assert!(4u8 * 0 == 0);
/// assert!(10u8 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u8_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u8>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8 - 0u8 == 1u8);
/// assert!(255u8 - 255u8 == 0u8);
/// assert!(0u8 - 0u8 == 0u8);
/// assert!(255u8 - 0u8 == 255u8);
/// assert!(130u8 - 115u8 == 15u8);
/// assert!(77u8 - 1u8 == 76u8);
/// assert!(235u8 - 123u8 == 112u8);
/// assert!(222u8 - 22u8 == 200u8);
/// assert!(208u8 - 1u8 == 207u8);
/// assert!(104u8 - 87u8 == 17u8);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u8 - 1u8));
/// assert!(panics!(1u8 - 255u8));
/// assert!(panics!(0u8 - 254u8));
/// assert!(panics!(0u8 - 255u8));
/// assert!(panics!(173u8 - 253u8));
/// assert!(panics!(39u8 - 122u8));
/// assert!(panics!(28u8 - 226u8));
/// assert!(panics!(3u8 - 207u8));
/// assert!(panics!(55u8 - 231u8));
/// assert!(panics!(150u8 - 221u8));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u8 - 0u8) + 0u8 == 1u8);
/// assert!((255u8 - 1u8) + 1u8 == 255u8);
/// assert!((0u8 - 0u8) + 0u8 == 0u8);
/// assert!((254u8 - 0u8) + 0u8 == 254u8);
/// assert!((255u8 - 254u8) + 254u8 == 255u8);
/// assert!((124u8 - 10u8) + 10u8 == 124u8);
/// assert!((156u8 - 46u8) + 46u8 == 156u8);
/// assert!((166u8 - 117u8) + 117u8 == 166u8);
/// assert!((170u8 - 104u8) + 104u8 == 170u8);
/// assert!((164u8 - 137u8) + 137u8 == 164u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 - 0 == 0u8);
/// assert!(255u8 - 0 == 255u8);
/// assert!(1u8 - 0 == 1u8);
/// assert!(254u8 - 0 == 254u8);
/// assert!(112u8 - 0 == 112u8);
/// assert!(214u8 - 0 == 214u8);
/// assert!(244u8 - 0 == 244u8);
/// assert!(172u8 - 0 == 172u8);
/// assert!(161u8 - 0 == 161u8);
/// assert!(67u8 - 0 == 67u8);
/// # }
/// ```
pub fn core_ops_add_u8_sub() {}
/// # Properties for [`u8::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.wrapping_sub(0u8) == 255u8);
/// assert!(1u8.wrapping_sub(0u8) == 1u8);
/// assert!(254u8.wrapping_sub(0u8) == 254u8);
/// assert!(0u8.wrapping_sub(0u8) == 0u8);
/// assert!(255u8.wrapping_sub(255u8) == 0u8);
/// assert!(210u8.wrapping_sub(115u8) == 95u8);
/// assert!(167u8.wrapping_sub(3u8) == 164u8);
/// assert!(211u8.wrapping_sub(24u8) == 187u8);
/// assert!(154u8.wrapping_sub(92u8) == 62u8);
/// assert!(189u8.wrapping_sub(183u8) == 6u8);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down(x.up() - y.up() + u8::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(254u8) == 2u8);
/// assert!(254u8.wrapping_sub(255u8) == 255u8);
/// assert!(0u8.wrapping_sub(255u8) == 1u8);
/// assert!(1u8.wrapping_sub(255u8) == 2u8);
/// assert!(167u8.wrapping_sub(240u8) == 183u8);
/// assert!(8u8.wrapping_sub(52u8) == 212u8);
/// assert!(12u8.wrapping_sub(211u8) == 57u8);
/// assert!(100u8.wrapping_sub(139u8) == 217u8);
/// assert!(51u8.wrapping_sub(175u8) == 132u8);
/// assert!(210u8.wrapping_sub(214u8) == 252u8);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down((x.up() - y.up()).rem_euclid(&(u8::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(1u8) == 255u8);
/// assert!(1u8.wrapping_sub(255u8) == 2u8);
/// assert!(1u8.wrapping_sub(1u8) == 0u8);
/// assert!(255u8.wrapping_sub(0u8) == 255u8);
/// assert!(255u8.wrapping_sub(255u8) == 0u8);
/// assert!(254u8.wrapping_sub(0u8) == 254u8);
/// assert!(159u8.wrapping_sub(44u8) == 115u8);
/// assert!(220u8.wrapping_sub(168u8) == 52u8);
/// assert!(138u8.wrapping_sub(121u8) == 17u8);
/// assert!(131u8.wrapping_sub(113u8) == 18u8);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((254u8.wrapping_sub(1u8)).wrapping_add(1u8) == 254u8);
/// assert!((1u8.wrapping_sub(255u8)).wrapping_add(255u8) == 1u8);
/// assert!((0u8.wrapping_sub(0u8)).wrapping_add(0u8) == 0u8);
/// assert!((254u8.wrapping_sub(0u8)).wrapping_add(0u8) == 254u8);
/// assert!((0u8.wrapping_sub(254u8)).wrapping_add(254u8) == 0u8);
/// assert!((255u8.wrapping_sub(254u8)).wrapping_add(254u8) == 255u8);
/// assert!((2u8.wrapping_sub(211u8)).wrapping_add(211u8) == 2u8);
/// assert!((214u8.wrapping_sub(34u8)).wrapping_add(34u8) == 214u8);
/// assert!((231u8.wrapping_sub(4u8)).wrapping_add(4u8) == 231u8);
/// assert!((154u8.wrapping_sub(140u8)).wrapping_add(140u8) == 154u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(0) == 0u8);
/// assert!(255u8.wrapping_sub(0) == 255u8);
/// assert!(254u8.wrapping_sub(0) == 254u8);
/// assert!(1u8.wrapping_sub(0) == 1u8);
/// assert!(187u8.wrapping_sub(0) == 187u8);
/// assert!(183u8.wrapping_sub(0) == 183u8);
/// assert!(103u8.wrapping_sub(0) == 103u8);
/// assert!(38u8.wrapping_sub(0) == 38u8);
/// assert!(129u8.wrapping_sub(0) == 129u8);
/// assert!(109u8.wrapping_sub(0) == 109u8);
/// # }
/// ```
pub fn core_ops_add_u8_wrapping_sub() {}
/// # Properties for [`u8::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u8::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_sub(0u8) == Some(0u8));
/// assert!(255u8.checked_sub(1u8) == Some(254u8));
/// assert!(1u8.checked_sub(0u8) == Some(1u8));
/// assert!(254u8.checked_sub(254u8) == Some(0u8));
/// assert!(255u8.checked_sub(0u8) == Some(255u8));
/// assert!(166u8.checked_sub(93u8) == Some(73u8));
/// assert!(146u8.checked_sub(101u8) == Some(45u8));
/// assert!(228u8.checked_sub(70u8) == Some(158u8));
/// assert!(222u8.checked_sub(59u8) == Some(163u8));
/// assert!(205u8.checked_sub(176u8) == Some(29u8));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.checked_sub(255u8) == None);
/// assert!(0u8.checked_sub(254u8) == None);
/// assert!(1u8.checked_sub(255u8) == None);
/// assert!(0u8.checked_sub(255u8) == None);
/// assert!(0u8.checked_sub(1u8) == None);
/// assert!(182u8.checked_sub(223u8) == None);
/// assert!(154u8.checked_sub(199u8) == None);
/// assert!(110u8.checked_sub(158u8) == None);
/// assert!(106u8.checked_sub(140u8) == None);
/// assert!(58u8.checked_sub(80u8) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((255u8.checked_sub(0u8)).and_then(|r| r.checked_add(0u8)) == Some(255u8));
/// assert!((1u8.checked_sub(1u8)).and_then(|r| r.checked_add(1u8)) == Some(1u8));
/// assert!((255u8.checked_sub(1u8)).and_then(|r| r.checked_add(1u8)) == Some(255u8));
/// assert!((0u8.checked_sub(0u8)).and_then(|r| r.checked_add(0u8)) == Some(0u8));
/// assert!((1u8.checked_sub(0u8)).and_then(|r| r.checked_add(0u8)) == Some(1u8));
/// assert!((148u8.checked_sub(92u8)).and_then(|r| r.checked_add(92u8)) == Some(148u8));
/// assert!((28u8.checked_sub(18u8)).and_then(|r| r.checked_add(18u8)) == Some(28u8));
/// assert!((182u8.checked_sub(148u8)).and_then(|r| r.checked_add(148u8)) == Some(182u8));
/// assert!((149u8.checked_sub(74u8)).and_then(|r| r.checked_add(74u8)) == Some(149u8));
/// assert!((210u8.checked_sub(130u8)).and_then(|r| r.checked_add(130u8)) == Some(210u8));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.checked_sub(0) == Some(1u8));
/// assert!(254u8.checked_sub(0) == Some(254u8));
/// assert!(255u8.checked_sub(0) == Some(255u8));
/// assert!(0u8.checked_sub(0) == Some(0u8));
/// assert!(74u8.checked_sub(0) == Some(74u8));
/// assert!(87u8.checked_sub(0) == Some(87u8));
/// assert!(107u8.checked_sub(0) == Some(107u8));
/// assert!(220u8.checked_sub(0) == Some(220u8));
/// assert!(142u8.checked_sub(0) == Some(142u8));
/// assert!(154u8.checked_sub(0) == Some(154u8));
/// # }
/// ```
pub fn core_ops_add_u8_checked_sub() {}
/// # Properties for [`u8::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u8::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_sub(1u8) == 0u8);
/// assert!(1u8.saturating_sub(255u8) == 0u8);
/// assert!(0u8.saturating_sub(255u8) == 0u8);
/// assert!(1u8.saturating_sub(1u8) == 0u8);
/// assert!(0u8.saturating_sub(254u8) == 0u8);
/// assert!(231u8.saturating_sub(25u8) == 206u8);
/// assert!(162u8.saturating_sub(224u8) == 0u8);
/// assert!(86u8.saturating_sub(173u8) == 0u8);
/// assert!(229u8.saturating_sub(102u8) == 127u8);
/// assert!(175u8.saturating_sub(203u8) == 0u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.saturating_sub(0u8) == 1u8);
/// assert!(1u8.saturating_sub(1u8) == 0u8);
/// assert!(254u8.saturating_sub(1u8) == 253u8);
/// assert!(254u8.saturating_sub(254u8) == 0u8);
/// assert!(255u8.saturating_sub(254u8) == 1u8);
/// assert!(254u8.saturating_sub(0u8) == 254u8);
/// assert!(247u8.saturating_sub(120u8) == 127u8);
/// assert!(148u8.saturating_sub(135u8) == 13u8);
/// assert!(112u8.saturating_sub(110u8) == 2u8);
/// assert!(103u8.saturating_sub(57u8) == 46u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_sub(255u8) == 0);
/// assert!(0u8.saturating_sub(254u8) == 0);
/// assert!(254u8.saturating_sub(255u8) == 0);
/// assert!(1u8.saturating_sub(255u8) == 0);
/// assert!(1u8.saturating_sub(254u8) == 0);
/// assert!(167u8.saturating_sub(172u8) == 0);
/// assert!(89u8.saturating_sub(110u8) == 0);
/// assert!(34u8.saturating_sub(235u8) == 0);
/// assert!(63u8.saturating_sub(126u8) == 0);
/// assert!(83u8.saturating_sub(245u8) == 0);
/// # }
/// ```
pub fn core_ops_sub_u8_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x + y == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8);
/// assert!(1u8 + 0u8 == 1u8);
/// assert!(0u8 + 255u8 == 255u8);
/// assert!(1u8 + 254u8 == 255u8);
/// assert!(254u8 + 0u8 == 254u8);
/// assert!(150u8 + 79u8 == 229u8);
/// assert!(31u8 + 161u8 == 192u8);
/// assert!(31u8 + 176u8 == 207u8);
/// assert!(102u8 + 25u8 == 127u8);
/// assert!(175u8 + 24u8 == 199u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(254u8 + 254u8));
/// assert!(panics!(255u8 + 1u8));
/// assert!(panics!(255u8 + 255u8));
/// assert!(panics!(254u8 + 255u8));
/// assert!(panics!(1u8 + 255u8));
/// assert!(panics!(255u8 + 254u8));
/// assert!(panics!(251u8 + 24u8));
/// assert!(panics!(228u8 + 113u8));
/// assert!(panics!(144u8 + 194u8));
/// assert!(panics!(120u8 + 194u8));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8 + 0u8 == 0u8 + 255u8);
/// assert!(1u8 + 0u8 == 0u8 + 1u8);
/// assert!(0u8 + 0u8 == 0u8 + 0u8);
/// assert!(0u8 + 1u8 == 1u8 + 0u8);
/// assert!(254u8 + 0u8 == 0u8 + 254u8);
/// assert!(74u8 + 170u8 == 170u8 + 74u8);
/// assert!(14u8 + 99u8 == 99u8 + 14u8);
/// assert!(106u8 + 95u8 == 95u8 + 106u8);
/// assert!(17u8 + 228u8 == 228u8 + 17u8);
/// assert!(3u8 + 216u8 == 216u8 + 3u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0 == 0u8);
/// assert!(255u8 + 0 == 255u8);
/// assert!(254u8 + 0 == 254u8);
/// assert!(1u8 + 0 == 1u8);
/// assert!(151u8 + 0 == 151u8);
/// assert!(113u8 + 0 == 113u8);
/// assert!(181u8 + 0 == 181u8);
/// assert!(166u8 + 0 == 166u8);
/// assert!(12u8 + 0 == 12u8);
/// assert!(107u8 + 0 == 107u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 254u8 == 254u8);
/// assert!(0 + 0u8 == 0u8);
/// assert!(0 + 1u8 == 1u8);
/// assert!(0 + 255u8 == 255u8);
/// assert!(0 + 209u8 == 209u8);
/// assert!(0 + 134u8 == 134u8);
/// assert!(0 + 146u8 == 146u8);
/// assert!(0 + 248u8 == 248u8);
/// assert!(0 + 70u8 == 70u8);
/// assert!(0 + 103u8 == 103u8);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u8::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u8 + 255u8) + 0u8 == 0u8 + (255u8 + 0u8));
/// assert!((254u8 + 0u8) + 1u8 == 254u8 + (0u8 + 1u8));
/// assert!((1u8 + 254u8) + 0u8 == 1u8 + (254u8 + 0u8));
/// assert!((1u8 + 0u8) + 254u8 == 1u8 + (0u8 + 254u8));
/// assert!((0u8 + 1u8) + 0u8 == 0u8 + (1u8 + 0u8));
/// assert!((0u8 + 1u8) + 254u8 == 0u8 + (1u8 + 254u8));
/// assert!((116u8 + 25u8) + 97u8 == 116u8 + (25u8 + 97u8));
/// assert!((5u8 + 26u8) + 167u8 == 5u8 + (26u8 + 167u8));
/// assert!((43u8 + 190u8) + 14u8 == 43u8 + (190u8 + 14u8));
/// assert!((3u8 + 3u8) + 205u8 == 3u8 + (3u8 + 205u8));
/// # }
/// ```
pub fn core_ops_add_u8_add() {}
/// # Properties for [`u8::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down((x.up() + y.up()) % (u8::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.wrapping_add(1u8) == 2u8);
/// assert!(254u8.wrapping_add(0u8) == 254u8);
/// assert!(255u8.wrapping_add(254u8) == 253u8);
/// assert!(1u8.wrapping_add(254u8) == 255u8);
/// assert!(1u8.wrapping_add(0u8) == 1u8);
/// assert!(254u8.wrapping_add(1u8) == 255u8);
/// assert!(134u8.wrapping_add(3u8) == 137u8);
/// assert!(169u8.wrapping_add(26u8) == 195u8);
/// assert!(43u8.wrapping_add(144u8) == 187u8);
/// assert!(53u8.wrapping_add(238u8) == 35u8);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_add(0u8) == 0u8);
/// assert!(0u8.wrapping_add(254u8) == 254u8);
/// assert!(254u8.wrapping_add(0u8) == 254u8);
/// assert!(1u8.wrapping_add(0u8) == 1u8);
/// assert!(1u8.wrapping_add(254u8) == 255u8);
/// assert!(2u8.wrapping_add(88u8) == 90u8);
/// assert!(72u8.wrapping_add(46u8) == 118u8);
/// assert!(95u8.wrapping_add(128u8) == 223u8);
/// assert!(138u8.wrapping_add(12u8) == 150u8);
/// assert!(184u8.wrapping_add(40u8) == 224u8);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down(x.up() + y.up() - u8::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.wrapping_add(255u8) == 0u8);
/// assert!(255u8.wrapping_add(254u8) == 253u8);
/// assert!(254u8.wrapping_add(254u8) == 252u8);
/// assert!(255u8.wrapping_add(1u8) == 0u8);
/// assert!(255u8.wrapping_add(255u8) == 254u8);
/// assert!(254u8.wrapping_add(255u8) == 253u8);
/// assert!(115u8.wrapping_add(201u8) == 60u8);
/// assert!(186u8.wrapping_add(88u8) == 18u8);
/// assert!(174u8.wrapping_add(131u8) == 49u8);
/// assert!(242u8.wrapping_add(166u8) == 152u8);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.wrapping_add(1u8) == 1u8.wrapping_add(1u8));
/// assert!(255u8.wrapping_add(0u8) == 0u8.wrapping_add(255u8));
/// assert!(0u8.wrapping_add(255u8) == 255u8.wrapping_add(0u8));
/// assert!(254u8.wrapping_add(254u8) == 254u8.wrapping_add(254u8));
/// assert!(255u8.wrapping_add(255u8) == 255u8.wrapping_add(255u8));
/// assert!(0u8.wrapping_add(1u8) == 1u8.wrapping_add(0u8));
/// assert!(41u8.wrapping_add(203u8) == 203u8.wrapping_add(41u8));
/// assert!(205u8.wrapping_add(13u8) == 13u8.wrapping_add(205u8));
/// assert!(94u8.wrapping_add(218u8) == 218u8.wrapping_add(94u8));
/// assert!(238u8.wrapping_add(169u8) == 169u8.wrapping_add(238u8));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.wrapping_add(0) == 254u8);
/// assert!(0u8.wrapping_add(0) == 0u8);
/// assert!(1u8.wrapping_add(0) == 1u8);
/// assert!(255u8.wrapping_add(0) == 255u8);
/// assert!(49u8.wrapping_add(0) == 49u8);
/// assert!(227u8.wrapping_add(0) == 227u8);
/// assert!(108u8.wrapping_add(0) == 108u8);
/// assert!(92u8.wrapping_add(0) == 92u8);
/// assert!(207u8.wrapping_add(0) == 207u8);
/// assert!(218u8.wrapping_add(0) == 218u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: u8 = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(0u8) == 0u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(255u8) == 255u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(1u8) == 1u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(254u8) == 254u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(55u8) == 55u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(109u8) == 109u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(7u8) == 7u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(183u8) == 183u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(25u8) == 25u8
///     });
/// assert!({
///         let zero: u8 = 0;
///         zero.wrapping_add(63u8) == 63u8
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u8::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u8.wrapping_add(0u8)).wrapping_add(0u8) == 0u8.wrapping_add(0u8.wrapping_add(0u8)));
/// assert!((254u8.wrapping_add(0u8)).wrapping_add(1u8)
///         == 254u8.wrapping_add(0u8.wrapping_add(1u8)));
/// assert!((0u8.wrapping_add(0u8)).wrapping_add(255u8)
///         == 0u8.wrapping_add(0u8.wrapping_add(255u8)));
/// assert!((255u8.wrapping_add(0u8)).wrapping_add(0u8)
///         == 255u8.wrapping_add(0u8.wrapping_add(0u8)));
/// assert!((1u8.wrapping_add(254u8)).wrapping_add(0u8)
///         == 1u8.wrapping_add(254u8.wrapping_add(0u8)));
/// assert!((1u8.wrapping_add(0u8)).wrapping_add(0u8) == 1u8.wrapping_add(0u8.wrapping_add(0u8)));
/// assert!((147u8.wrapping_add(22u8)).wrapping_add(29u8)
///         == 147u8.wrapping_add(22u8.wrapping_add(29u8)));
/// assert!((22u8.wrapping_add(10u8)).wrapping_add(163u8)
///         == 22u8.wrapping_add(10u8.wrapping_add(163u8)));
/// assert!((141u8.wrapping_add(27u8)).wrapping_add(74u8)
///         == 141u8.wrapping_add(27u8.wrapping_add(74u8)));
/// assert!((143u8.wrapping_add(85u8)).wrapping_add(26u8)
///         == 143u8.wrapping_add(85u8.wrapping_add(26u8)));
/// # }
/// ```
pub fn core_ops_add_u8_wrapping_add() {}
/// # Properties for [`u8::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u8::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(0u8.checked_add(255u8) == Some(255u8));
/// assert!(1u8.checked_add(0u8) == Some(1u8));
/// assert!(254u8.checked_add(0u8) == Some(254u8));
/// assert!(1u8.checked_add(1u8) == Some(2u8));
/// assert!(1u8.checked_add(254u8) == Some(255u8));
/// assert!(49u8.checked_add(202u8) == Some(251u8));
/// assert!(104u8.checked_add(11u8) == Some(115u8));
/// assert!(11u8.checked_add(116u8) == Some(127u8));
/// assert!(29u8.checked_add(198u8) == Some(227u8));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.checked_add(255u8) == None);
/// assert!(255u8.checked_add(254u8) == None);
/// assert!(255u8.checked_add(1u8) == None);
/// assert!(254u8.checked_add(255u8) == None);
/// assert!(254u8.checked_add(254u8) == None);
/// assert!(1u8.checked_add(255u8) == None);
/// assert!(219u8.checked_add(188u8) == None);
/// assert!(244u8.checked_add(32u8) == None);
/// assert!(205u8.checked_add(51u8) == None);
/// assert!(226u8.checked_add(199u8) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.checked_add(0u8) == 0u8.checked_add(1u8));
/// assert!(254u8.checked_add(1u8) == 1u8.checked_add(254u8));
/// assert!(0u8.checked_add(255u8) == 255u8.checked_add(0u8));
/// assert!(254u8.checked_add(255u8) == 255u8.checked_add(254u8));
/// assert!(255u8.checked_add(1u8) == 1u8.checked_add(255u8));
/// assert!(1u8.checked_add(255u8) == 255u8.checked_add(1u8));
/// assert!(229u8.checked_add(26u8) == 26u8.checked_add(229u8));
/// assert!(214u8.checked_add(36u8) == 36u8.checked_add(214u8));
/// assert!(86u8.checked_add(68u8) == 68u8.checked_add(86u8));
/// assert!(73u8.checked_add(196u8) == 196u8.checked_add(73u8));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u8) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(1u8.checked_add(0u8) == Some(1u8));
/// assert!(255u8.checked_add(0u8) == Some(255u8));
/// assert!(254u8.checked_add(0u8) == Some(254u8));
/// assert!(230u8.checked_add(0u8) == Some(230u8));
/// assert!(194u8.checked_add(0u8) == Some(194u8));
/// assert!(152u8.checked_add(0u8) == Some(152u8));
/// assert!(56u8.checked_add(0u8) == Some(56u8));
/// assert!(244u8.checked_add(0u8) == Some(244u8));
/// assert!(202u8.checked_add(0u8) == Some(202u8));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u8.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(0u8.checked_add(254u8) == Some(254u8));
/// assert!(0u8.checked_add(1u8) == Some(1u8));
/// assert!(0u8.checked_add(255u8) == Some(255u8));
/// assert!(0u8.checked_add(14u8) == Some(14u8));
/// assert!(0u8.checked_add(117u8) == Some(117u8));
/// assert!(0u8.checked_add(43u8) == Some(43u8));
/// assert!(0u8.checked_add(182u8) == Some(182u8));
/// assert!(0u8.checked_add(181u8) == Some(181u8));
/// assert!(0u8.checked_add(158u8) == Some(158u8));
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.checked_add(0u8).and_then(|iv| iv.checked_add(255u8))
///         == 0u8.checked_add(255u8).and_then(|iv| 254u8.checked_add(iv)));
/// assert!(0u8.checked_add(0u8).and_then(|iv| iv.checked_add(254u8))
///         == 0u8.checked_add(254u8).and_then(|iv| 0u8.checked_add(iv)));
/// assert!(255u8.checked_add(254u8).and_then(|iv| iv.checked_add(254u8))
///         == 254u8.checked_add(254u8).and_then(|iv| 255u8.checked_add(iv)));
/// assert!(254u8.checked_add(255u8).and_then(|iv| iv.checked_add(254u8))
///         == 255u8.checked_add(254u8).and_then(|iv| 254u8.checked_add(iv)));
/// assert!(1u8.checked_add(1u8).and_then(|iv| iv.checked_add(0u8))
///         == 1u8.checked_add(0u8).and_then(|iv| 1u8.checked_add(iv)));
/// assert!(255u8.checked_add(0u8).and_then(|iv| iv.checked_add(0u8))
///         == 0u8.checked_add(0u8).and_then(|iv| 255u8.checked_add(iv)));
/// assert!(51u8.checked_add(245u8).and_then(|iv| iv.checked_add(213u8))
///         == 245u8.checked_add(213u8).and_then(|iv| 51u8.checked_add(iv)));
/// assert!(79u8.checked_add(56u8).and_then(|iv| iv.checked_add(66u8))
///         == 56u8.checked_add(66u8).and_then(|iv| 79u8.checked_add(iv)));
/// assert!(167u8.checked_add(72u8).and_then(|iv| iv.checked_add(149u8))
///         == 72u8.checked_add(149u8).and_then(|iv| 167u8.checked_add(iv)));
/// assert!(7u8.checked_add(124u8).and_then(|iv| iv.checked_add(197u8))
///         == 124u8.checked_add(197u8).and_then(|iv| 7u8.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u8_checked_add() {}
/// # Properties for [`u8::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u8::down((x.up() + y.up()).min(u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_add(255u8) == 255u8);
/// assert!(0u8.saturating_add(0u8) == 0u8);
/// assert!(255u8.saturating_add(1u8) == 255u8);
/// assert!(0u8.saturating_add(254u8) == 254u8);
/// assert!(1u8.saturating_add(255u8) == 255u8);
/// assert!(226u8.saturating_add(189u8) == 255u8);
/// assert!(149u8.saturating_add(171u8) == 255u8);
/// assert!(140u8.saturating_add(90u8) == 230u8);
/// assert!(57u8.saturating_add(189u8) == 246u8);
/// assert!(234u8.saturating_add(45u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8.saturating_add(0u8) == 255u8);
/// assert!(0u8.saturating_add(0u8) == 0u8);
/// assert!(0u8.saturating_add(1u8) == 1u8);
/// assert!(254u8.saturating_add(0u8) == 254u8);
/// assert!(1u8.saturating_add(1u8) == 2u8);
/// assert!(179u8.saturating_add(48u8) == 227u8);
/// assert!(37u8.saturating_add(189u8) == 226u8);
/// assert!(32u8.saturating_add(156u8) == 188u8);
/// assert!(9u8.saturating_add(177u8) == 186u8);
/// assert!(100u8.saturating_add(141u8) == 241u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u8::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.saturating_add(254u8) == u8::MAX);
/// assert!(255u8.saturating_add(254u8) == u8::MAX);
/// assert!(254u8.saturating_add(255u8) == u8::MAX);
/// assert!(255u8.saturating_add(255u8) == u8::MAX);
/// assert!(255u8.saturating_add(1u8) == u8::MAX);
/// assert!(1u8.saturating_add(255u8) == u8::MAX);
/// assert!(184u8.saturating_add(101u8) == u8::MAX);
/// assert!(119u8.saturating_add(188u8) == u8::MAX);
/// assert!(243u8.saturating_add(34u8) == u8::MAX);
/// assert!(251u8.saturating_add(137u8) == u8::MAX);
/// # }
/// ```
pub fn core_ops_add_u8_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u16>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u16::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16 % 65535u16 == 1u16);
/// assert!(65534u16 % 65534u16 == 0u16);
/// assert!(0u16 % 65534u16 == 0u16);
/// assert!(0u16 % 65535u16 == 0u16);
/// assert!(1u16 % 65534u16 == 1u16);
/// assert!(65535u16 % 1u16 == 0u16);
/// assert!(52556u16 % 44500u16 == 8056u16);
/// assert!(58755u16 % 8799u16 == 5961u16);
/// assert!(4669u16 % 46768u16 == 4669u16);
/// assert!(49602u16 % 53375u16 == 49602u16);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u16_rem() {}
/// # Properties for [`u16::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u16::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_rem(65535u16) == Some(1u16));
/// assert!(0u16.checked_rem(65534u16) == Some(0u16));
/// assert!(65534u16.checked_rem(65534u16) == Some(0u16));
/// assert!(0u16.checked_rem(1u16) == Some(0u16));
/// assert!(65535u16.checked_rem(65535u16) == Some(0u16));
/// assert!(65535u16.checked_rem(65534u16) == Some(1u16));
/// assert!(34668u16.checked_rem(28636u16) == Some(6032u16));
/// assert!(63447u16.checked_rem(51830u16) == Some(11617u16));
/// assert!(50563u16.checked_rem(57953u16) == Some(50563u16));
/// assert!(48012u16.checked_rem(54314u16) == Some(48012u16));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u16.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u16_checked_rem() {}
/// # Properties for [`u16::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `x == u16::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u16::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_neg() == Some(0u16));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `x != u16::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_neg() == None);
/// assert!(65535u16.checked_neg() == None);
/// assert!(65534u16.checked_neg() == None);
/// assert!(5588u16.checked_neg() == None);
/// assert!(55151u16.checked_neg() == None);
/// assert!(32649u16.checked_neg() == None);
/// assert!(18193u16.checked_neg() == None);
/// assert!(63989u16.checked_neg() == None);
/// assert!(39155u16.checked_neg() == None);
/// assert!(29483u16.checked_neg() == None);
/// # }
/// ```
pub fn t_u16_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u16>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x << y == u16::down((x.up() << y) & u16::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 << 0u32 == 0u16);
/// assert!(65535u16 << 0u32 == 65535u16);
/// assert!(65534u16 << 0u32 == 65534u16);
/// assert!(0u16 << 1u32 == 0u16);
/// assert!(14842u16 << 8u32 == 64000u16);
/// assert!(20905u16 << 8u32 == 43264u16);
/// assert!(35015u16 << 4u32 == 35952u16);
/// assert!(29282u16 << 7u32 == 12544u16);
/// assert!(57812u16 << 3u32 == 3744u16);
/// assert!(38891u16 << 7u32 == 62848u16);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65535u16 << 4294967295u32));
/// assert!(panics!(0u16 << 4294967295u32));
/// assert!(panics!(1u16 << 4294967295u32));
/// assert!(panics!(0u16 << 4294967294u32));
/// assert!(panics!(65534u16 << 4294967294u32));
/// assert!(panics!(1u16 << 4294967294u32));
/// assert!(panics!(51827u16 << 44u32));
/// assert!(panics!(10740u16 << 102u32));
/// assert!(panics!(40306u16 << 30u32));
/// assert!(panics!(9931u16 << 103u32));
/// # }
/// ```
pub fn core_ops_shl_u16_shl() {}
/// # Properties for [`core::ops::Shl::<u16>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u16::down((x.up() << y) & u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_shl(1u32) == Some(2u16));
/// assert!(65535u16.checked_shl(0u32) == Some(65535u16));
/// assert!(1u16.checked_shl(0u32) == Some(1u16));
/// assert!(65534u16.checked_shl(0u32) == Some(65534u16));
/// assert!(0u16.checked_shl(0u32) == Some(0u16));
/// assert!(65535u16.checked_shl(1u32) == Some(65534u16));
/// assert!(12046u16.checked_shl(14u32) == Some(32768u16));
/// assert!(12868u16.checked_shl(1u32) == Some(25736u16));
/// assert!(29529u16.checked_shl(8u32) == Some(22784u16));
/// assert!(9556u16.checked_shl(11u32) == Some(40960u16));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_shl(4294967294u32) == None);
/// assert!(1u16.checked_shl(4294967295u32) == None);
/// assert!(0u16.checked_shl(4294967295u32) == None);
/// assert!(65534u16.checked_shl(4294967294u32) == None);
/// assert!(1u16.checked_shl(4294967294u32) == None);
/// assert!(65535u16.checked_shl(4294967295u32) == None);
/// assert!(8640u16.checked_shl(48u32) == None);
/// assert!(23672u16.checked_shl(126u32) == None);
/// assert!(10116u16.checked_shl(39u32) == None);
/// assert!(12420u16.checked_shl(61u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u16_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u16>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u16::down((x.up() << y) & u16::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.overflowing_shl(0u32) == (65534u16, false));
/// assert!(1u16.overflowing_shl(1u32) == (2u16, false));
/// assert!(0u16.overflowing_shl(1u32) == (0u16, false));
/// assert!(65535u16.overflowing_shl(0u32) == (65535u16, false));
/// assert!(0u16.overflowing_shl(0u32) == (0u16, false));
/// assert!(12607u16.overflowing_shl(4u32) == (5104u16, false));
/// assert!(28599u16.overflowing_shl(12u32) == (28672u16, false));
/// assert!(53953u16.overflowing_shl(8u32) == (49408u16, false));
/// assert!(41025u16.overflowing_shl(0u32) == (41025u16, false));
/// assert!(41086u16.overflowing_shl(0u32) == (41086u16, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u16::down((x.up() << (y & (u16::BITS - 1)) & u16::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shl(4294967295u32) == (0u16, true));
/// assert!(1u16.overflowing_shl(4294967295u32) == (32768u16, true));
/// assert!(65535u16.overflowing_shl(4294967295u32) == (32768u16, true));
/// assert!(65535u16.overflowing_shl(4294967294u32) == (49152u16, true));
/// assert!(65534u16.overflowing_shl(4294967294u32) == (32768u16, true));
/// assert!(1496u16.overflowing_shl(119u32) == (60416u16, true));
/// assert!(9689u16.overflowing_shl(76u32) == (36864u16, true));
/// assert!(3957u16.overflowing_shl(134u32) == (56640u16, true));
/// assert!(9106u16.overflowing_shl(26u32) == (18432u16, true));
/// assert!(14235u16.overflowing_shl(40u32) == (39680u16, true));
/// # }
/// ```
pub fn core_ops_shl_u16_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u16>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x >> y == u16::down((x.up() >> y) & u16::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16 >> 0u32 == 1u16);
/// assert!(0u16 >> 0u32 == 0u16);
/// assert!(65535u16 >> 0u32 == 65535u16);
/// assert!(65534u16 >> 0u32 == 65534u16);
/// assert!(65534u16 >> 1u32 == 32767u16);
/// assert!(23829u16 >> 4u32 == 1489u16);
/// assert!(7311u16 >> 2u32 == 1827u16);
/// assert!(1568u16 >> 13u32 == 0u16);
/// assert!(46392u16 >> 15u32 == 1u16);
/// assert!(27735u16 >> 15u32 == 0u16);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65535u16 >> 4294967294u32));
/// assert!(panics!(0u16 >> 4294967294u32));
/// assert!(panics!(0u16 >> 4294967295u32));
/// assert!(panics!(1u16 >> 4294967295u32));
/// assert!(panics!(65534u16 >> 4294967295u32));
/// assert!(panics!(56745u16 >> 74u32));
/// assert!(panics!(49531u16 >> 87u32));
/// assert!(panics!(16068u16 >> 138u32));
/// assert!(panics!(54646u16 >> 22u32));
/// assert!(panics!(57439u16 >> 124u32));
/// # }
/// ```
pub fn core_ops_shr_u16_shr() {}
/// # Properties for [`core::ops::Shr::<u16>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u16::down((x.up() >> y) & u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_shr(0u32) == Some(1u16));
/// assert!(65534u16.checked_shr(0u32) == Some(65534u16));
/// assert!(1u16.checked_shr(1u32) == Some(0u16));
/// assert!(0u16.checked_shr(0u32) == Some(0u16));
/// assert!(0u16.checked_shr(1u32) == Some(0u16));
/// assert!(12015u16.checked_shr(12u32) == Some(2u16));
/// assert!(14778u16.checked_shr(2u32) == Some(3694u16));
/// assert!(44728u16.checked_shr(12u32) == Some(10u16));
/// assert!(1806u16.checked_shr(2u32) == Some(451u16));
/// assert!(47504u16.checked_shr(3u32) == Some(5938u16));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.checked_shr(4294967295u32) == None);
/// assert!(0u16.checked_shr(4294967294u32) == None);
/// assert!(0u16.checked_shr(4294967295u32) == None);
/// assert!(1u16.checked_shr(4294967294u32) == None);
/// assert!(65535u16.checked_shr(4294967294u32) == None);
/// assert!(6572u16.checked_shr(48u32) == None);
/// assert!(59152u16.checked_shr(129u32) == None);
/// assert!(43323u16.checked_shr(23u32) == None);
/// assert!(13710u16.checked_shr(91u32) == None);
/// assert!(57544u16.checked_shr(71u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u16_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u16>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u16::down((x.up() >> y) & u16::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.overflowing_shr(1u32) == (32767u16, false));
/// assert!(0u16.overflowing_shr(0u32) == (0u16, false));
/// assert!(1u16.overflowing_shr(0u32) == (1u16, false));
/// assert!(0u16.overflowing_shr(1u32) == (0u16, false));
/// assert!(63918u16.overflowing_shr(12u32) == (15u16, false));
/// assert!(20019u16.overflowing_shr(13u32) == (2u16, false));
/// assert!(17564u16.overflowing_shr(15u32) == (0u16, false));
/// assert!(2164u16.overflowing_shr(11u32) == (1u16, false));
/// assert!(45887u16.overflowing_shr(12u32) == (11u16, false));
/// assert!(12656u16.overflowing_shr(11u32) == (6u16, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u16::down((x.up() >> (y & (u16::BITS - 1)) & u16::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shr(4294967294u32) == (0u16, true));
/// assert!(0u16.overflowing_shr(4294967295u32) == (0u16, true));
/// assert!(65535u16.overflowing_shr(4294967295u32) == (1u16, true));
/// assert!(65534u16.overflowing_shr(4294967294u32) == (3u16, true));
/// assert!(65534u16.overflowing_shr(4294967295u32) == (1u16, true));
/// assert!(65535u16.overflowing_shr(4294967294u32) == (3u16, true));
/// assert!(16149u16.overflowing_shr(36u32) == (1009u16, true));
/// assert!(27855u16.overflowing_shr(119u32) == (217u16, true));
/// assert!(37840u16.overflowing_shr(45u32) == (4u16, true));
/// assert!(46435u16.overflowing_shr(126u32) == (2u16, true));
/// # }
/// ```
pub fn core_ops_shr_u16_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u16>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u16::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16 / 65535u16 == 1u16);
/// assert!(65534u16 / 65535u16 == 0u16);
/// assert!(1u16 / 1u16 == 1u16);
/// assert!(65534u16 / 65534u16 == 1u16);
/// assert!(65534u16 / 1u16 == 65534u16);
/// assert!(0u16 / 1u16 == 0u16);
/// assert!(147u16 / 68u16 == 2u16);
/// assert!(81u16 / 192u16 == 0u16);
/// assert!(235u16 / 176u16 == 1u16);
/// assert!(56u16 / 158u16 == 0u16);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65534u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65535u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(20923u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(789u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(45287u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(11081u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(22525u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(49310u16 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u16_div() {}
/// # Properties for [`u16::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u16::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.saturating_div(65535u16) == 0u16);
/// assert!(65535u16.saturating_div(1u16) == 65535u16);
/// assert!(0u16.saturating_div(1u16) == 0u16);
/// assert!(65534u16.saturating_div(65534u16) == 1u16);
/// assert!(65535u16.saturating_div(65535u16) == 1u16);
/// assert!(65534u16.saturating_div(1u16) == 65534u16);
/// assert!(147u16.saturating_div(80u16) == 1u16);
/// assert!(218u16.saturating_div(175u16) == 1u16);
/// assert!(193u16.saturating_div(103u16) == 1u16);
/// assert!(238u16.saturating_div(61u16) == 3u16);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65535u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65534u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(26678u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(57542u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(34219u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(32349u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(45301u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(33050u16.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u16_saturating_div() {}
/// # Properties for [`u16::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u16::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_div(65535u16) == Some(0u16));
/// assert!(65534u16.checked_div(65535u16) == Some(0u16));
/// assert!(65534u16.checked_div(65534u16) == Some(1u16));
/// assert!(1u16.checked_div(65534u16) == Some(0u16));
/// assert!(65534u16.checked_div(1u16) == Some(65534u16));
/// assert!(158u16.checked_div(159u16) == Some(0u16));
/// assert!(164u16.checked_div(178u16) == Some(0u16));
/// assert!(145u16.checked_div(174u16) == Some(0u16));
/// assert!(114u16.checked_div(235u16) == Some(0u16));
/// assert!(156u16.checked_div(235u16) == Some(0u16));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_div(0) == None);
/// assert!(0u16.checked_div(0) == None);
/// assert!(65534u16.checked_div(0) == None);
/// assert!(65535u16.checked_div(0) == None);
/// assert!(6467u16.checked_div(0) == None);
/// assert!(37164u16.checked_div(0) == None);
/// assert!(62081u16.checked_div(0) == None);
/// assert!(6563u16.checked_div(0) == None);
/// assert!(34154u16.checked_div(0) == None);
/// assert!(41885u16.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u16_checked_div() {}
/// # Properties for [`core::ops::Mul::<u16>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x * y == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 65534u16 == 0u16);
/// assert!(65535u16 * 0u16 == 0u16);
/// assert!(65534u16 * 0u16 == 0u16);
/// assert!(1u16 * 65535u16 == 65535u16);
/// assert!(0u16 * 1u16 == 0u16);
/// assert!(238u16 * 84u16 == 19992u16);
/// assert!(215u16 * 247u16 == 53105u16);
/// assert!(236u16 * 150u16 == 35400u16);
/// assert!(204u16 * 138u16 == 28152u16);
/// assert!(114u16 * 211u16 == 24054u16);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65535u16 * 65534u16));
/// assert!(panics!(65534u16 * 65534u16));
/// assert!(panics!(65534u16 * 65535u16));
/// assert!(panics!(65535u16 * 65535u16));
/// assert!(panics!(50641u16 * 1244u16));
/// assert!(panics!(17258u16 * 7281u16));
/// assert!(panics!(16444u16 * 6009u16));
/// assert!(panics!(30475u16 * 6303u16));
/// assert!(panics!(2986u16 * 50648u16));
/// assert!(panics!(7328u16 * 26272u16));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 1 == 0u16);
/// assert!(1u16 * 1 == 1u16);
/// assert!(65535u16 * 1 == 65535u16);
/// assert!(65534u16 * 1 == 65534u16);
/// assert!(60630u16 * 1 == 60630u16);
/// assert!(56323u16 * 1 == 56323u16);
/// assert!(35238u16 * 1 == 35238u16);
/// assert!(5682u16 * 1 == 5682u16);
/// assert!(39818u16 * 1 == 39818u16);
/// assert!(16490u16 * 1 == 16490u16);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 65534u16 == 65534u16);
/// assert!(1 * 0u16 == 0u16);
/// assert!(1 * 65535u16 == 65535u16);
/// assert!(1 * 1u16 == 1u16);
/// assert!(1 * 640u16 == 640u16);
/// assert!(1 * 29650u16 == 29650u16);
/// assert!(1 * 27185u16 == 27185u16);
/// assert!(1 * 27470u16 == 27470u16);
/// assert!(1 * 54854u16 == 54854u16);
/// assert!(1 * 22953u16 == 22953u16);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 0u16 == 0u16 * 0u16);
/// assert!(0u16 * 65535u16 == 65535u16 * 0u16);
/// assert!(0u16 * 1u16 == 1u16 * 0u16);
/// assert!(65534u16 * 0u16 == 0u16 * 65534u16);
/// assert!(1u16 * 0u16 == 0u16 * 1u16);
/// assert!(133u16 * 103u16 == 103u16 * 133u16);
/// assert!(169u16 * 139u16 == 139u16 * 169u16);
/// assert!(226u16 * 180u16 == 180u16 * 226u16);
/// assert!(202u16 * 25u16 == 25u16 * 202u16);
/// assert!(121u16 * 233u16 == 233u16 * 121u16);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u16::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((65534u16 * 1u16) * 1u16 == 65534u16 * (1u16 * 1u16));
/// assert!((1u16 * 1u16) * 1u16 == 1u16 * (1u16 * 1u16));
/// assert!((1u16 * 1u16) * 65535u16 == 1u16 * (1u16 * 65535u16));
/// assert!((65535u16 * 1u16) * 1u16 == 65535u16 * (1u16 * 1u16));
/// assert!((1u16 * 65535u16) * 1u16 == 1u16 * (65535u16 * 1u16));
/// assert!((1u16 * 1u16) * 65534u16 == 1u16 * (1u16 * 65534u16));
/// assert!((32u16 * 32u16) * 39u16 == 32u16 * (32u16 * 39u16));
/// assert!((36u16 * 30u16) * 25u16 == 36u16 * (30u16 * 25u16));
/// assert!((38u16 * 35u16) * 34u16 == 38u16 * (35u16 * 34u16));
/// assert!((26u16 * 31u16) * 38u16 == 26u16 * (31u16 * 38u16));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u16::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16 * (0u16 + 65535u16) == 1u16 * 0u16 + 1u16 * 65535u16);
/// assert!(65535u16 * (1u16 + 0u16) == 65535u16 * 1u16 + 65535u16 * 0u16);
/// assert!(65534u16 * (1u16 + 0u16) == 65534u16 * 1u16 + 65534u16 * 0u16);
/// assert!(1u16 * (65534u16 + 1u16) == 1u16 * 65534u16 + 1u16 * 1u16);
/// assert!(65535u16 * (0u16 + 0u16) == 65535u16 * 0u16 + 65535u16 * 0u16);
/// assert!(188u16 * (88u16 + 214u16) == 188u16 * 88u16 + 188u16 * 214u16);
/// assert!(212u16 * (63u16 + 246u16) == 212u16 * 63u16 + 212u16 * 246u16);
/// assert!(223u16 * (128u16 + 137u16) == 223u16 * 128u16 + 223u16 * 137u16);
/// assert!(137u16 * (244u16 + 179u16) == 137u16 * 244u16 + 137u16 * 179u16);
/// assert!(6u16 * (158u16 + 189u16) == 6u16 * 158u16 + 6u16 * 189u16);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16 * 0 == 0);
/// assert!(65535u16 * 0 == 0);
/// assert!(0u16 * 0 == 0);
/// assert!(65534u16 * 0 == 0);
/// assert!(180u16 * 0 == 0);
/// assert!(106u16 * 0 == 0);
/// assert!(111u16 * 0 == 0);
/// assert!(236u16 * 0 == 0);
/// assert!(81u16 * 0 == 0);
/// assert!(187u16 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u16_mul() {}
/// # Properties for [`u16::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u16::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_mul(65534u16) == Some(0u16));
/// assert!(1u16.checked_mul(0u16) == Some(0u16));
/// assert!(0u16.checked_mul(0u16) == Some(0u16));
/// assert!(65534u16.checked_mul(0u16) == Some(0u16));
/// assert!(251u16.checked_mul(138u16) == Some(34638u16));
/// assert!(214u16.checked_mul(229u16) == Some(49006u16));
/// assert!(46u16.checked_mul(238u16) == Some(10948u16));
/// assert!(94u16.checked_mul(228u16) == Some(21432u16));
/// assert!(160u16.checked_mul(215u16) == Some(34400u16));
/// assert!(240u16.checked_mul(95u16) == Some(22800u16));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.checked_mul(65535u16) == None);
/// assert!(65535u16.checked_mul(65534u16) == None);
/// assert!(65535u16.checked_mul(65535u16) == None);
/// assert!(65534u16.checked_mul(65534u16) == None);
/// assert!(44242u16.checked_mul(5212u16) == None);
/// assert!(49203u16.checked_mul(5611u16) == None);
/// assert!(25650u16.checked_mul(4304u16) == None);
/// assert!(33469u16.checked_mul(63605u16) == None);
/// assert!(46651u16.checked_mul(7795u16) == None);
/// assert!(7164u16.checked_mul(23138u16) == None);
/// # }
/// ```
pub fn core_ops_mul_u16_checked_mul() {}
/// # Properties for [`u16::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u16::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(65534u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(65535u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(1u16) == (0u16, false));
/// assert!(1u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(138u16.overflowing_mul(176u16) == (24288u16, false));
/// assert!(221u16.overflowing_mul(173u16) == (38233u16, false));
/// assert!(191u16.overflowing_mul(154u16) == (29414u16, false));
/// assert!(219u16.overflowing_mul(220u16) == (48180u16, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.overflowing_mul(65535u16) == (1u16, true));
/// assert!(65535u16.overflowing_mul(65534u16) == (2u16, true));
/// assert!(65534u16.overflowing_mul(65535u16) == (2u16, true));
/// assert!(65534u16.overflowing_mul(65534u16) == (4u16, true));
/// assert!(43630u16.overflowing_mul(44366u16) == (17284u16, true));
/// assert!(8074u16.overflowing_mul(5669u16) == (27378u16, true));
/// assert!(32191u16.overflowing_mul(35607u16) == (297u16, true));
/// assert!(39810u16.overflowing_mul(21686u16) == (13932u16, true));
/// assert!(49832u16.overflowing_mul(54232u16) == (46528u16, true));
/// assert!(29329u16.overflowing_mul(2497u16) == (30801u16, true));
/// # }
/// ```
pub fn core_ops_mul_u16_overflowing_mul() {}
/// # Properties for [`u16::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::down((x.up() * y.up()).min(u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.saturating_mul(65534u16) == 65535u16);
/// assert!(1u16.saturating_mul(1u16) == 1u16);
/// assert!(65535u16.saturating_mul(1u16) == 65535u16);
/// assert!(65535u16.saturating_mul(0u16) == 0u16);
/// assert!(65535u16.saturating_mul(65535u16) == 65535u16);
/// assert!(0u16.saturating_mul(0u16) == 0u16);
/// assert!(54243u16.saturating_mul(9204u16) == 65535u16);
/// assert!(39873u16.saturating_mul(49222u16) == 65535u16);
/// assert!(47455u16.saturating_mul(12605u16) == 65535u16);
/// assert!(33842u16.saturating_mul(56161u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_mul(0u16) == 0u16);
/// assert!(1u16.saturating_mul(65534u16) == 65534u16);
/// assert!(65535u16.saturating_mul(0u16) == 0u16);
/// assert!(1u16.saturating_mul(0u16) == 0u16);
/// assert!(65535u16.saturating_mul(1u16) == 65535u16);
/// assert!(165u16.saturating_mul(173u16) == 28545u16);
/// assert!(168u16.saturating_mul(140u16) == 23520u16);
/// assert!(205u16.saturating_mul(129u16) == 26445u16);
/// assert!(88u16.saturating_mul(221u16) == 19448u16);
/// assert!(104u16.saturating_mul(80u16) == 8320u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.saturating_mul(65535u16) == u16::MAX);
/// assert!(65534u16.saturating_mul(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_mul(65534u16) == u16::MAX);
/// assert!(65534u16.saturating_mul(65534u16) == u16::MAX);
/// assert!(63692u16.saturating_mul(35424u16) == u16::MAX);
/// assert!(7136u16.saturating_mul(10064u16) == u16::MAX);
/// assert!(52937u16.saturating_mul(57034u16) == u16::MAX);
/// assert!(33869u16.saturating_mul(23869u16) == u16::MAX);
/// assert!(4251u16.saturating_mul(37925u16) == u16::MAX);
/// assert!(58683u16.saturating_mul(3132u16) == u16::MAX);
/// # }
/// ```
pub fn core_ops_mul_u16_saturating_mul() {}
/// # Properties for [`u16::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u16::down((x.up() * y.up()) % (u16::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_mul(0u16) == 0u16);
/// assert!(1u16.wrapping_mul(65534u16) == 65534u16);
/// assert!(65535u16.wrapping_mul(65534u16) == 2u16);
/// assert!(0u16.wrapping_mul(1u16) == 0u16);
/// assert!(65535u16.wrapping_mul(1u16) == 65535u16);
/// assert!(65534u16.wrapping_mul(0u16) == 0u16);
/// assert!(26750u16.wrapping_mul(21788u16) == 17352u16);
/// assert!(54880u16.wrapping_mul(7773u16) == 8416u16);
/// assert!(63692u16.wrapping_mul(12490u16) == 37112u16);
/// assert!(16209u16.wrapping_mul(1529u16) == 10953u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.wrapping_mul(0u16) == 0u16);
/// assert!(0u16.wrapping_mul(0u16) == 0u16);
/// assert!(1u16.wrapping_mul(1u16) == 1u16);
/// assert!(0u16.wrapping_mul(1u16) == 0u16);
/// assert!(195u16.wrapping_mul(198u16) == 38610u16);
/// assert!(123u16.wrapping_mul(211u16) == 25953u16);
/// assert!(155u16.wrapping_mul(63u16) == 9765u16);
/// assert!(96u16.wrapping_mul(65u16) == 6240u16);
/// assert!(149u16.wrapping_mul(235u16) == 35015u16);
/// assert!(227u16.wrapping_mul(108u16) == 24516u16);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 1 == 0u16);
/// assert!(1u16 * 1 == 1u16);
/// assert!(65535u16 * 1 == 65535u16);
/// assert!(65534u16 * 1 == 65534u16);
/// assert!(6341u16 * 1 == 6341u16);
/// assert!(4734u16 * 1 == 4734u16);
/// assert!(24047u16 * 1 == 24047u16);
/// assert!(9266u16 * 1 == 9266u16);
/// assert!(35771u16 * 1 == 35771u16);
/// assert!(2379u16 * 1 == 2379u16);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u16 == 0u16);
/// assert!(1 * 65534u16 == 65534u16);
/// assert!(1 * 65535u16 == 65535u16);
/// assert!(1 * 1u16 == 1u16);
/// assert!(1 * 25692u16 == 25692u16);
/// assert!(1 * 10762u16 == 10762u16);
/// assert!(1 * 437u16 == 437u16);
/// assert!(1 * 20572u16 == 20572u16);
/// assert!(1 * 35718u16 == 35718u16);
/// assert!(1 * 19236u16 == 19236u16);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 1u16 == 1u16 * 0u16);
/// assert!(0u16 * 65534u16 == 65534u16 * 0u16);
/// assert!(1u16 * 0u16 == 0u16 * 1u16);
/// assert!(1u16 * 65535u16 == 65535u16 * 1u16);
/// assert!(65535u16 * 1u16 == 1u16 * 65535u16);
/// assert!(65u16 * 152u16 == 152u16 * 65u16);
/// assert!(160u16 * 228u16 == 228u16 * 160u16);
/// assert!(145u16 * 210u16 == 210u16 * 145u16);
/// assert!(250u16 * 97u16 == 97u16 * 250u16);
/// assert!(215u16 * 40u16 == 40u16 * 215u16);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u16::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u16 * 1u16) * 65535u16 == 1u16 * (1u16 * 65535u16));
/// assert!((1u16 * 1u16) * 1u16 == 1u16 * (1u16 * 1u16));
/// assert!((65534u16 * 1u16) * 1u16 == 65534u16 * (1u16 * 1u16));
/// assert!((1u16 * 65535u16) * 1u16 == 1u16 * (65535u16 * 1u16));
/// assert!((65535u16 * 1u16) * 1u16 == 65535u16 * (1u16 * 1u16));
/// assert!((1u16 * 65534u16) * 1u16 == 1u16 * (65534u16 * 1u16));
/// assert!((35u16 * 34u16) * 37u16 == 35u16 * (34u16 * 37u16));
/// assert!((38u16 * 40u16) * 38u16 == 38u16 * (40u16 * 38u16));
/// assert!((28u16 * 31u16) * 37u16 == 28u16 * (31u16 * 37u16));
/// assert!((25u16 * 36u16) * 25u16 == 25u16 * (36u16 * 25u16));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u16::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16 * (65535u16 + 0u16) == 1u16 * 65535u16 + 1u16 * 0u16);
/// assert!(1u16 * (0u16 + 65535u16) == 1u16 * 0u16 + 1u16 * 65535u16);
/// assert!(1u16 * (0u16 + 0u16) == 1u16 * 0u16 + 1u16 * 0u16);
/// assert!(1u16 * (1u16 + 1u16) == 1u16 * 1u16 + 1u16 * 1u16);
/// assert!(65534u16 * (0u16 + 0u16) == 65534u16 * 0u16 + 65534u16 * 0u16);
/// assert!(1u16 * (1u16 + 0u16) == 1u16 * 1u16 + 1u16 * 0u16);
/// assert!(103u16 * (243u16 + 158u16) == 103u16 * 243u16 + 103u16 * 158u16);
/// assert!(72u16 * (230u16 + 211u16) == 72u16 * 230u16 + 72u16 * 211u16);
/// assert!(93u16 * (168u16 + 210u16) == 93u16 * 168u16 + 93u16 * 210u16);
/// assert!(59u16 * (56u16 + 218u16) == 59u16 * 56u16 + 59u16 * 218u16);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16 * 0 == 0);
/// assert!(1u16 * 0 == 0);
/// assert!(0u16 * 0 == 0);
/// assert!(65534u16 * 0 == 0);
/// assert!(242u16 * 0 == 0);
/// assert!(217u16 * 0 == 0);
/// assert!(178u16 * 0 == 0);
/// assert!(246u16 * 0 == 0);
/// assert!(80u16 * 0 == 0);
/// assert!(245u16 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u16_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u16>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 - 0u16 == 0u16);
/// assert!(1u16 - 0u16 == 1u16);
/// assert!(65535u16 - 0u16 == 65535u16);
/// assert!(65534u16 - 0u16 == 65534u16);
/// assert!(37341u16 - 28418u16 == 8923u16);
/// assert!(30517u16 - 16824u16 == 13693u16);
/// assert!(23378u16 - 15931u16 == 7447u16);
/// assert!(62500u16 - 11028u16 == 51472u16);
/// assert!(56182u16 - 22644u16 == 33538u16);
/// assert!(43352u16 - 5751u16 == 37601u16);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u16 - 1u16));
/// assert!(panics!(0u16 - 65534u16));
/// assert!(panics!(1u16 - 65534u16));
/// assert!(panics!(0u16 - 65535u16));
/// assert!(panics!(23035u16 - 26004u16));
/// assert!(panics!(25339u16 - 29425u16));
/// assert!(panics!(9815u16 - 58105u16));
/// assert!(panics!(18924u16 - 35006u16));
/// assert!(panics!(23399u16 - 28185u16));
/// assert!(panics!(14373u16 - 61563u16));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((65534u16 - 65534u16) + 65534u16 == 65534u16);
/// assert!((1u16 - 0u16) + 0u16 == 1u16);
/// assert!((65535u16 - 0u16) + 0u16 == 65535u16);
/// assert!((0u16 - 0u16) + 0u16 == 0u16);
/// assert!((65535u16 - 65534u16) + 65534u16 == 65535u16);
/// assert!((1329u16 - 149u16) + 149u16 == 1329u16);
/// assert!((63004u16 - 19480u16) + 19480u16 == 63004u16);
/// assert!((48082u16 - 45676u16) + 45676u16 == 48082u16);
/// assert!((63837u16 - 23363u16) + 23363u16 == 63837u16);
/// assert!((20333u16 - 4481u16) + 4481u16 == 20333u16);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 - 0 == 0u16);
/// assert!(65535u16 - 0 == 65535u16);
/// assert!(1u16 - 0 == 1u16);
/// assert!(65534u16 - 0 == 65534u16);
/// assert!(7579u16 - 0 == 7579u16);
/// assert!(39658u16 - 0 == 39658u16);
/// assert!(44227u16 - 0 == 44227u16);
/// assert!(9007u16 - 0 == 9007u16);
/// assert!(39802u16 - 0 == 39802u16);
/// assert!(22412u16 - 0 == 22412u16);
/// # }
/// ```
pub fn core_ops_add_u16_sub() {}
/// # Properties for [`u16::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.wrapping_sub(0u16) == 65535u16);
/// assert!(65534u16.wrapping_sub(1u16) == 65533u16);
/// assert!(0u16.wrapping_sub(0u16) == 0u16);
/// assert!(1u16.wrapping_sub(0u16) == 1u16);
/// assert!(26294u16.wrapping_sub(1003u16) == 25291u16);
/// assert!(56418u16.wrapping_sub(1443u16) == 54975u16);
/// assert!(10538u16.wrapping_sub(9595u16) == 943u16);
/// assert!(11878u16.wrapping_sub(5716u16) == 6162u16);
/// assert!(62224u16.wrapping_sub(51209u16) == 11015u16);
/// assert!(63956u16.wrapping_sub(3822u16) == 60134u16);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down(x.up() - y.up() + u16::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.wrapping_sub(65535u16) == 65535u16);
/// assert!(0u16.wrapping_sub(1u16) == 65535u16);
/// assert!(1u16.wrapping_sub(65534u16) == 3u16);
/// assert!(0u16.wrapping_sub(65534u16) == 2u16);
/// assert!(0u16.wrapping_sub(65535u16) == 1u16);
/// assert!(29396u16.wrapping_sub(36336u16) == 58596u16);
/// assert!(7784u16.wrapping_sub(51547u16) == 21773u16);
/// assert!(27602u16.wrapping_sub(52780u16) == 40358u16);
/// assert!(9814u16.wrapping_sub(41816u16) == 33534u16);
/// assert!(511u16.wrapping_sub(11526u16) == 54521u16);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down((x.up() - y.up()).rem_euclid(&(u16::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_sub(65535u16) == 1u16);
/// assert!(65535u16.wrapping_sub(65534u16) == 1u16);
/// assert!(1u16.wrapping_sub(0u16) == 1u16);
/// assert!(0u16.wrapping_sub(0u16) == 0u16);
/// assert!(0u16.wrapping_sub(1u16) == 65535u16);
/// assert!(49500u16.wrapping_sub(50530u16) == 64506u16);
/// assert!(63465u16.wrapping_sub(13404u16) == 50061u16);
/// assert!(17938u16.wrapping_sub(47510u16) == 35964u16);
/// assert!(12181u16.wrapping_sub(35117u16) == 42600u16);
/// assert!(34620u16.wrapping_sub(19193u16) == 15427u16);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((65535u16.wrapping_sub(65534u16)).wrapping_add(65534u16) == 65535u16);
/// assert!((0u16.wrapping_sub(65535u16)).wrapping_add(65535u16) == 0u16);
/// assert!((65535u16.wrapping_sub(1u16)).wrapping_add(1u16) == 65535u16);
/// assert!((0u16.wrapping_sub(0u16)).wrapping_add(0u16) == 0u16);
/// assert!((1u16.wrapping_sub(0u16)).wrapping_add(0u16) == 1u16);
/// assert!((33383u16.wrapping_sub(32249u16)).wrapping_add(32249u16) == 33383u16);
/// assert!((25454u16.wrapping_sub(49443u16)).wrapping_add(49443u16) == 25454u16);
/// assert!((21259u16.wrapping_sub(1436u16)).wrapping_add(1436u16) == 21259u16);
/// assert!((20551u16.wrapping_sub(48315u16)).wrapping_add(48315u16) == 20551u16);
/// assert!((256u16.wrapping_sub(41509u16)).wrapping_add(41509u16) == 256u16);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.wrapping_sub(0) == 65535u16);
/// assert!(0u16.wrapping_sub(0) == 0u16);
/// assert!(1u16.wrapping_sub(0) == 1u16);
/// assert!(65534u16.wrapping_sub(0) == 65534u16);
/// assert!(5624u16.wrapping_sub(0) == 5624u16);
/// assert!(29513u16.wrapping_sub(0) == 29513u16);
/// assert!(53002u16.wrapping_sub(0) == 53002u16);
/// assert!(20725u16.wrapping_sub(0) == 20725u16);
/// assert!(21191u16.wrapping_sub(0) == 21191u16);
/// assert!(6854u16.wrapping_sub(0) == 6854u16);
/// # }
/// ```
pub fn core_ops_add_u16_wrapping_sub() {}
/// # Properties for [`u16::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u16::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.checked_sub(0u16) == Some(65534u16));
/// assert!(65535u16.checked_sub(65534u16) == Some(1u16));
/// assert!(0u16.checked_sub(0u16) == Some(0u16));
/// assert!(65534u16.checked_sub(65534u16) == Some(0u16));
/// assert!(1u16.checked_sub(0u16) == Some(1u16));
/// assert!(62002u16.checked_sub(26596u16) == Some(35406u16));
/// assert!(34833u16.checked_sub(27586u16) == Some(7247u16));
/// assert!(27039u16.checked_sub(10493u16) == Some(16546u16));
/// assert!(57326u16.checked_sub(20427u16) == Some(36899u16));
/// assert!(47340u16.checked_sub(40490u16) == Some(6850u16));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_sub(65534u16) == None);
/// assert!(0u16.checked_sub(1u16) == None);
/// assert!(0u16.checked_sub(65535u16) == None);
/// assert!(65534u16.checked_sub(65535u16) == None);
/// assert!(52407u16.checked_sub(64411u16) == None);
/// assert!(43929u16.checked_sub(62551u16) == None);
/// assert!(28414u16.checked_sub(61687u16) == None);
/// assert!(18757u16.checked_sub(32865u16) == None);
/// assert!(31715u16.checked_sub(62819u16) == None);
/// assert!(38137u16.checked_sub(55397u16) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((65535u16.checked_sub(65534u16)).and_then(|r| r.checked_add(65534u16))
///         == Some(65535u16));
/// assert!((65534u16.checked_sub(0u16)).and_then(|r| r.checked_add(0u16)) == Some(65534u16));
/// assert!((1u16.checked_sub(0u16)).and_then(|r| r.checked_add(0u16)) == Some(1u16));
/// assert!((0u16.checked_sub(0u16)).and_then(|r| r.checked_add(0u16)) == Some(0u16));
/// assert!((32173u16.checked_sub(19796u16)).and_then(|r| r.checked_add(19796u16))
///         == Some(32173u16));
/// assert!((56022u16.checked_sub(31100u16)).and_then(|r| r.checked_add(31100u16))
///         == Some(56022u16));
/// assert!((21592u16.checked_sub(4584u16)).and_then(|r| r.checked_add(4584u16))
///         == Some(21592u16));
/// assert!((64473u16.checked_sub(14654u16)).and_then(|r| r.checked_add(14654u16))
///         == Some(64473u16));
/// assert!((5652u16.checked_sub(5469u16)).and_then(|r| r.checked_add(5469u16)) == Some(5652u16));
/// assert!((53636u16.checked_sub(45156u16)).and_then(|r| r.checked_add(45156u16))
///         == Some(53636u16));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_sub(0) == Some(0u16));
/// assert!(65535u16.checked_sub(0) == Some(65535u16));
/// assert!(1u16.checked_sub(0) == Some(1u16));
/// assert!(65534u16.checked_sub(0) == Some(65534u16));
/// assert!(56706u16.checked_sub(0) == Some(56706u16));
/// assert!(48875u16.checked_sub(0) == Some(48875u16));
/// assert!(41879u16.checked_sub(0) == Some(41879u16));
/// assert!(6060u16.checked_sub(0) == Some(6060u16));
/// assert!(3186u16.checked_sub(0) == Some(3186u16));
/// assert!(60725u16.checked_sub(0) == Some(60725u16));
/// # }
/// ```
pub fn core_ops_add_u16_checked_sub() {}
/// # Properties for [`u16::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u16::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_sub(65534u16) == 0u16);
/// assert!(65535u16.saturating_sub(1u16) == 65534u16);
/// assert!(65534u16.saturating_sub(65535u16) == 0u16);
/// assert!(65534u16.saturating_sub(0u16) == 65534u16);
/// assert!(65535u16.saturating_sub(0u16) == 65535u16);
/// assert!(0u16.saturating_sub(0u16) == 0u16);
/// assert!(46861u16.saturating_sub(38322u16) == 8539u16);
/// assert!(9142u16.saturating_sub(8478u16) == 664u16);
/// assert!(26335u16.saturating_sub(40936u16) == 0u16);
/// assert!(10363u16.saturating_sub(64494u16) == 0u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.saturating_sub(0u16) == 65534u16);
/// assert!(65535u16.saturating_sub(0u16) == 65535u16);
/// assert!(0u16.saturating_sub(0u16) == 0u16);
/// assert!(65534u16.saturating_sub(65534u16) == 0u16);
/// assert!(1u16.saturating_sub(0u16) == 1u16);
/// assert!(65534u16.saturating_sub(1u16) == 65533u16);
/// assert!(18984u16.saturating_sub(17564u16) == 1420u16);
/// assert!(63390u16.saturating_sub(62743u16) == 647u16);
/// assert!(59292u16.saturating_sub(29109u16) == 30183u16);
/// assert!(30491u16.saturating_sub(18940u16) == 11551u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_sub(65535u16) == 0);
/// assert!(0u16.saturating_sub(1u16) == 0);
/// assert!(65534u16.saturating_sub(65535u16) == 0);
/// assert!(1u16.saturating_sub(65534u16) == 0);
/// assert!(1u16.saturating_sub(65535u16) == 0);
/// assert!(0u16.saturating_sub(65534u16) == 0);
/// assert!(20474u16.saturating_sub(35497u16) == 0);
/// assert!(11835u16.saturating_sub(24607u16) == 0);
/// assert!(40445u16.saturating_sub(46310u16) == 0);
/// assert!(63874u16.saturating_sub(65349u16) == 0);
/// # }
/// ```
pub fn core_ops_sub_u16_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u16>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x + y == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 + 0u16 == 0u16);
/// assert!(0u16 + 1u16 == 1u16);
/// assert!(65535u16 + 0u16 == 65535u16);
/// assert!(1u16 + 0u16 == 1u16);
/// assert!(65534u16 + 0u16 == 65534u16);
/// assert!(42568u16 + 15938u16 == 58506u16);
/// assert!(11747u16 + 31648u16 == 43395u16);
/// assert!(25495u16 + 3030u16 == 28525u16);
/// assert!(9188u16 + 37950u16 == 47138u16);
/// assert!(3300u16 + 56932u16 == 60232u16);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65535u16 + 65535u16));
/// assert!(panics!(65535u16 + 65534u16));
/// assert!(panics!(1u16 + 65535u16));
/// assert!(panics!(65534u16 + 65534u16));
/// assert!(panics!(65535u16 + 1u16));
/// assert!(panics!(65534u16 + 65535u16));
/// assert!(panics!(60660u16 + 11333u16));
/// assert!(panics!(53448u16 + 20084u16));
/// assert!(panics!(58841u16 + 64737u16));
/// assert!(panics!(43639u16 + 27277u16));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16 + 0u16 == 0u16 + 65534u16);
/// assert!(0u16 + 65534u16 == 65534u16 + 0u16);
/// assert!(0u16 + 1u16 == 1u16 + 0u16);
/// assert!(65535u16 + 0u16 == 0u16 + 65535u16);
/// assert!(6382u16 + 975u16 == 975u16 + 6382u16);
/// assert!(9586u16 + 38669u16 == 38669u16 + 9586u16);
/// assert!(18037u16 + 21257u16 == 21257u16 + 18037u16);
/// assert!(7602u16 + 53361u16 == 53361u16 + 7602u16);
/// assert!(8684u16 + 41263u16 == 41263u16 + 8684u16);
/// assert!(2971u16 + 19692u16 == 19692u16 + 2971u16);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16 + 0 == 65534u16);
/// assert!(0u16 + 0 == 0u16);
/// assert!(1u16 + 0 == 1u16);
/// assert!(65535u16 + 0 == 65535u16);
/// assert!(62973u16 + 0 == 62973u16);
/// assert!(4894u16 + 0 == 4894u16);
/// assert!(5235u16 + 0 == 5235u16);
/// assert!(58297u16 + 0 == 58297u16);
/// assert!(49903u16 + 0 == 49903u16);
/// assert!(9910u16 + 0 == 9910u16);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u16 == 0u16);
/// assert!(0 + 65535u16 == 65535u16);
/// assert!(0 + 65534u16 == 65534u16);
/// assert!(0 + 1u16 == 1u16);
/// assert!(0 + 23821u16 == 23821u16);
/// assert!(0 + 5554u16 == 5554u16);
/// assert!(0 + 22205u16 == 22205u16);
/// assert!(0 + 38464u16 == 38464u16);
/// assert!(0 + 41706u16 == 41706u16);
/// assert!(0 + 51762u16 == 51762u16);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u16::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((65534u16 + 0u16) + 0u16 == 65534u16 + (0u16 + 0u16));
/// assert!((0u16 + 0u16) + 0u16 == 0u16 + (0u16 + 0u16));
/// assert!((0u16 + 0u16) + 1u16 == 0u16 + (0u16 + 1u16));
/// assert!((0u16 + 0u16) + 65535u16 == 0u16 + (0u16 + 65535u16));
/// assert!((0u16 + 1u16) + 0u16 == 0u16 + (1u16 + 0u16));
/// assert!((6010u16 + 25382u16) + 25259u16 == 6010u16 + (25382u16 + 25259u16));
/// assert!((1435u16 + 30741u16) + 8901u16 == 1435u16 + (30741u16 + 8901u16));
/// assert!((17356u16 + 21912u16) + 16459u16 == 17356u16 + (21912u16 + 16459u16));
/// assert!((3519u16 + 15741u16) + 42873u16 == 3519u16 + (15741u16 + 42873u16));
/// assert!((10314u16 + 1184u16) + 24977u16 == 10314u16 + (1184u16 + 24977u16));
/// # }
/// ```
pub fn core_ops_add_u16_add() {}
/// # Properties for [`u16::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down((x.up() + y.up()) % (u16::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.wrapping_add(65535u16) == 65534u16);
/// assert!(0u16.wrapping_add(65534u16) == 65534u16);
/// assert!(65534u16.wrapping_add(1u16) == 65535u16);
/// assert!(1u16.wrapping_add(1u16) == 2u16);
/// assert!(65535u16.wrapping_add(1u16) == 0u16);
/// assert!(0u16.wrapping_add(0u16) == 0u16);
/// assert!(15900u16.wrapping_add(29662u16) == 45562u16);
/// assert!(1581u16.wrapping_add(57914u16) == 59495u16);
/// assert!(38679u16.wrapping_add(22430u16) == 61109u16);
/// assert!(4505u16.wrapping_add(33352u16) == 37857u16);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.wrapping_add(0u16) == 65534u16);
/// assert!(65535u16.wrapping_add(0u16) == 65535u16);
/// assert!(1u16.wrapping_add(0u16) == 1u16);
/// assert!(0u16.wrapping_add(65534u16) == 65534u16);
/// assert!(0u16.wrapping_add(0u16) == 0u16);
/// assert!(23292u16.wrapping_add(23251u16) == 46543u16);
/// assert!(41783u16.wrapping_add(6600u16) == 48383u16);
/// assert!(62220u16.wrapping_add(1298u16) == 63518u16);
/// assert!(25103u16.wrapping_add(33609u16) == 58712u16);
/// assert!(11374u16.wrapping_add(14776u16) == 26150u16);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down(x.up() + y.up() - u16::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.wrapping_add(65535u16) == 0u16);
/// assert!(65534u16.wrapping_add(65534u16) == 65532u16);
/// assert!(65535u16.wrapping_add(65535u16) == 65534u16);
/// assert!(65534u16.wrapping_add(65535u16) == 65533u16);
/// assert!(65535u16.wrapping_add(65534u16) == 65533u16);
/// assert!(65535u16.wrapping_add(1u16) == 0u16);
/// assert!(65501u16.wrapping_add(31178u16) == 31143u16);
/// assert!(46096u16.wrapping_add(57654u16) == 38214u16);
/// assert!(59627u16.wrapping_add(35398u16) == 29489u16);
/// assert!(64384u16.wrapping_add(13591u16) == 12439u16);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_add(0u16) == 0u16.wrapping_add(0u16));
/// assert!(65534u16.wrapping_add(0u16) == 0u16.wrapping_add(65534u16));
/// assert!(0u16.wrapping_add(65534u16) == 65534u16.wrapping_add(0u16));
/// assert!(65535u16.wrapping_add(0u16) == 0u16.wrapping_add(65535u16));
/// assert!(22885u16.wrapping_add(43435u16) == 43435u16.wrapping_add(22885u16));
/// assert!(5018u16.wrapping_add(36254u16) == 36254u16.wrapping_add(5018u16));
/// assert!(52644u16.wrapping_add(37678u16) == 37678u16.wrapping_add(52644u16));
/// assert!(166u16.wrapping_add(4096u16) == 4096u16.wrapping_add(166u16));
/// assert!(48902u16.wrapping_add(54965u16) == 54965u16.wrapping_add(48902u16));
/// assert!(50080u16.wrapping_add(48360u16) == 48360u16.wrapping_add(50080u16));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.wrapping_add(0) == 1u16);
/// assert!(0u16.wrapping_add(0) == 0u16);
/// assert!(65535u16.wrapping_add(0) == 65535u16);
/// assert!(65534u16.wrapping_add(0) == 65534u16);
/// assert!(11976u16.wrapping_add(0) == 11976u16);
/// assert!(34623u16.wrapping_add(0) == 34623u16);
/// assert!(12097u16.wrapping_add(0) == 12097u16);
/// assert!(20725u16.wrapping_add(0) == 20725u16);
/// assert!(721u16.wrapping_add(0) == 721u16);
/// assert!(16141u16.wrapping_add(0) == 16141u16);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: u16 = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(1u16) == 1u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(0u16) == 0u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(65534u16) == 65534u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(65535u16) == 65535u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(39407u16) == 39407u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(63091u16) == 63091u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(49857u16) == 49857u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(62417u16) == 62417u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(35706u16) == 35706u16
///     });
/// assert!({
///         let zero: u16 = 0;
///         zero.wrapping_add(22469u16) == 22469u16
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u16, y : u16, z : u16`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u16::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u16.wrapping_add(65534u16)).wrapping_add(1u16)
///         == 0u16.wrapping_add(65534u16.wrapping_add(1u16)));
/// assert!((0u16.wrapping_add(1u16)).wrapping_add(0u16)
///         == 0u16.wrapping_add(1u16.wrapping_add(0u16)));
/// assert!((65534u16.wrapping_add(0u16)).wrapping_add(0u16)
///         == 65534u16.wrapping_add(0u16.wrapping_add(0u16)));
/// assert!((0u16.wrapping_add(1u16)).wrapping_add(1u16)
///         == 0u16.wrapping_add(1u16.wrapping_add(1u16)));
/// assert!((0u16.wrapping_add(0u16)).wrapping_add(1u16)
///         == 0u16.wrapping_add(0u16.wrapping_add(1u16)));
/// assert!((931u16.wrapping_add(21483u16)).wrapping_add(30346u16)
///         == 931u16.wrapping_add(21483u16.wrapping_add(30346u16)));
/// assert!((7368u16.wrapping_add(9463u16)).wrapping_add(33778u16)
///         == 7368u16.wrapping_add(9463u16.wrapping_add(33778u16)));
/// assert!((6857u16.wrapping_add(1731u16)).wrapping_add(1378u16)
///         == 6857u16.wrapping_add(1731u16.wrapping_add(1378u16)));
/// assert!((25344u16.wrapping_add(723u16)).wrapping_add(22404u16)
///         == 25344u16.wrapping_add(723u16.wrapping_add(22404u16)));
/// assert!((1465u16.wrapping_add(5439u16)).wrapping_add(28896u16)
///         == 1465u16.wrapping_add(5439u16.wrapping_add(28896u16)));
/// # }
/// ```
pub fn core_ops_add_u16_wrapping_add() {}
/// # Properties for [`u16::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u16::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_add(65534u16) == Some(65535u16));
/// assert!(65534u16.checked_add(0u16) == Some(65534u16));
/// assert!(0u16.checked_add(65535u16) == Some(65535u16));
/// assert!(65535u16.checked_add(0u16) == Some(65535u16));
/// assert!(46663u16.checked_add(17576u16) == Some(64239u16));
/// assert!(11925u16.checked_add(15201u16) == Some(27126u16));
/// assert!(14008u16.checked_add(38921u16) == Some(52929u16));
/// assert!(18210u16.checked_add(37654u16) == Some(55864u16));
/// assert!(53678u16.checked_add(5847u16) == Some(59525u16));
/// assert!(52570u16.checked_add(854u16) == Some(53424u16));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.checked_add(1u16) == None);
/// assert!(1u16.checked_add(65535u16) == None);
/// assert!(65535u16.checked_add(65535u16) == None);
/// assert!(65535u16.checked_add(65534u16) == None);
/// assert!(65534u16.checked_add(65535u16) == None);
/// assert!(65534u16.checked_add(65534u16) == None);
/// assert!(23974u16.checked_add(58796u16) == None);
/// assert!(49857u16.checked_add(21852u16) == None);
/// assert!(39401u16.checked_add(42707u16) == None);
/// assert!(50183u16.checked_add(26806u16) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.checked_add(65535u16) == 65535u16.checked_add(65535u16));
/// assert!(65534u16.checked_add(65534u16) == 65534u16.checked_add(65534u16));
/// assert!(1u16.checked_add(65534u16) == 65534u16.checked_add(1u16));
/// assert!(0u16.checked_add(65534u16) == 65534u16.checked_add(0u16));
/// assert!(65534u16.checked_add(1u16) == 1u16.checked_add(65534u16));
/// assert!(1u16.checked_add(0u16) == 0u16.checked_add(1u16));
/// assert!(1274u16.checked_add(24256u16) == 24256u16.checked_add(1274u16));
/// assert!(9026u16.checked_add(25285u16) == 25285u16.checked_add(9026u16));
/// assert!(55793u16.checked_add(40857u16) == 40857u16.checked_add(55793u16));
/// assert!(34228u16.checked_add(33863u16) == 33863u16.checked_add(34228u16));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u16) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65535u16.checked_add(0u16) == Some(65535u16));
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(1u16.checked_add(0u16) == Some(1u16));
/// assert!(65534u16.checked_add(0u16) == Some(65534u16));
/// assert!(59893u16.checked_add(0u16) == Some(59893u16));
/// assert!(64066u16.checked_add(0u16) == Some(64066u16));
/// assert!(36118u16.checked_add(0u16) == Some(36118u16));
/// assert!(5953u16.checked_add(0u16) == Some(5953u16));
/// assert!(41975u16.checked_add(0u16) == Some(41975u16));
/// assert!(27788u16.checked_add(0u16) == Some(27788u16));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u16.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(65534u16) == Some(65534u16));
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(0u16.checked_add(1u16) == Some(1u16));
/// assert!(0u16.checked_add(65535u16) == Some(65535u16));
/// assert!(0u16.checked_add(12822u16) == Some(12822u16));
/// assert!(0u16.checked_add(25226u16) == Some(25226u16));
/// assert!(0u16.checked_add(57297u16) == Some(57297u16));
/// assert!(0u16.checked_add(47734u16) == Some(47734u16));
/// assert!(0u16.checked_add(12925u16) == Some(12925u16));
/// assert!(0u16.checked_add(42535u16) == Some(42535u16));
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(1u16).and_then(|iv| iv.checked_add(0u16))
///         == 1u16.checked_add(0u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(65534u16.checked_add(1u16).and_then(|iv| iv.checked_add(0u16))
///         == 1u16.checked_add(0u16).and_then(|iv| 65534u16.checked_add(iv)));
/// assert!(65534u16.checked_add(65535u16).and_then(|iv| iv.checked_add(0u16))
///         == 65535u16.checked_add(0u16).and_then(|iv| 65534u16.checked_add(iv)));
/// assert!(0u16.checked_add(0u16).and_then(|iv| iv.checked_add(65534u16))
///         == 0u16.checked_add(65534u16).and_then(|iv| 0u16.checked_add(iv)));
/// assert!(65535u16.checked_add(65534u16).and_then(|iv| iv.checked_add(65534u16))
///         == 65534u16.checked_add(65534u16).and_then(|iv| 65535u16.checked_add(iv)));
/// assert!(65535u16.checked_add(65535u16).and_then(|iv| iv.checked_add(1u16))
///         == 65535u16.checked_add(1u16).and_then(|iv| 65535u16.checked_add(iv)));
/// assert!(33745u16.checked_add(56583u16).and_then(|iv| iv.checked_add(44218u16))
///         == 56583u16.checked_add(44218u16).and_then(|iv| 33745u16.checked_add(iv)));
/// assert!(14612u16.checked_add(24141u16).and_then(|iv| iv.checked_add(45395u16))
///         == 24141u16.checked_add(45395u16).and_then(|iv| 14612u16.checked_add(iv)));
/// assert!(42017u16.checked_add(60260u16).and_then(|iv| iv.checked_add(55964u16))
///         == 60260u16.checked_add(55964u16).and_then(|iv| 42017u16.checked_add(iv)));
/// assert!(40217u16.checked_add(28973u16).and_then(|iv| iv.checked_add(49003u16))
///         == 28973u16.checked_add(49003u16).and_then(|iv| 40217u16.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u16_checked_add() {}
/// # Properties for [`u16::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u16::down((x.up() + y.up()).min(u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_add(65535u16) == 65535u16);
/// assert!(1u16.saturating_add(0u16) == 1u16);
/// assert!(65534u16.saturating_add(65534u16) == 65535u16);
/// assert!(0u16.saturating_add(0u16) == 0u16);
/// assert!(65534u16.saturating_add(1u16) == 65535u16);
/// assert!(65534u16.saturating_add(0u16) == 65534u16);
/// assert!(61602u16.saturating_add(1065u16) == 62667u16);
/// assert!(53573u16.saturating_add(20002u16) == 65535u16);
/// assert!(21638u16.saturating_add(54695u16) == 65535u16);
/// assert!(40072u16.saturating_add(41494u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_add(65534u16) == 65534u16);
/// assert!(65535u16.saturating_add(0u16) == 65535u16);
/// assert!(0u16.saturating_add(65535u16) == 65535u16);
/// assert!(0u16.saturating_add(0u16) == 0u16);
/// assert!(0u16.saturating_add(1u16) == 1u16);
/// assert!(337u16.saturating_add(36228u16) == 36565u16);
/// assert!(39675u16.saturating_add(8547u16) == 48222u16);
/// assert!(57886u16.saturating_add(2900u16) == 60786u16);
/// assert!(49754u16.saturating_add(8255u16) == 58009u16);
/// assert!(17822u16.saturating_add(24353u16) == 42175u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u16::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.saturating_add(65534u16) == u16::MAX);
/// assert!(1u16.saturating_add(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_add(1u16) == u16::MAX);
/// assert!(65534u16.saturating_add(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_add(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_add(65534u16) == u16::MAX);
/// assert!(41644u16.saturating_add(31909u16) == u16::MAX);
/// assert!(38538u16.saturating_add(51354u16) == u16::MAX);
/// assert!(35522u16.saturating_add(55052u16) == u16::MAX);
/// assert!(38035u16.saturating_add(33192u16) == u16::MAX);
/// # }
/// ```
pub fn core_ops_add_u16_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u32>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u32::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32 % 4294967294u32 == 1u32);
/// assert!(4294967295u32 % 4294967294u32 == 1u32);
/// assert!(1u32 % 4294967295u32 == 1u32);
/// assert!(0u32 % 4294967294u32 == 0u32);
/// assert!(1u32 % 1u32 == 0u32);
/// assert!(4294967294u32 % 4294967295u32 == 4294967294u32);
/// assert!(1665141294u32 % 3521223205u32 == 1665141294u32);
/// assert!(1594413091u32 % 1283216045u32 == 311197046u32);
/// assert!(2508215082u32 % 3020134885u32 == 2508215082u32);
/// assert!(3720625737u32 % 2619152249u32 == 1101473488u32);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u32_rem() {}
/// # Properties for [`u32::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u32::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_rem(4294967294u32) == Some(0u32));
/// assert!(1u32.checked_rem(4294967295u32) == Some(1u32));
/// assert!(1u32.checked_rem(4294967294u32) == Some(1u32));
/// assert!(1u32.checked_rem(1u32) == Some(0u32));
/// assert!(4294967294u32.checked_rem(4294967294u32) == Some(0u32));
/// assert!(4061251399u32.checked_rem(3901772549u32) == Some(159478850u32));
/// assert!(1098831361u32.checked_rem(777484684u32) == Some(321346677u32));
/// assert!(4189430057u32.checked_rem(3900480810u32) == Some(288949247u32));
/// assert!(966846631u32.checked_rem(1302671549u32) == Some(966846631u32));
/// assert!(2628495683u32.checked_rem(2880530242u32) == Some(2628495683u32));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u32.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u32_checked_rem() {}
/// # Properties for [`u32::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `x == u32::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u32::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_neg() == Some(0u32));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `x != u32::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.checked_neg() == None);
/// assert!(4294967294u32.checked_neg() == None);
/// assert!(1u32.checked_neg() == None);
/// assert!(2760500992u32.checked_neg() == None);
/// assert!(2326069345u32.checked_neg() == None);
/// assert!(1036094088u32.checked_neg() == None);
/// assert!(2040456809u32.checked_neg() == None);
/// assert!(1325139039u32.checked_neg() == None);
/// assert!(212110826u32.checked_neg() == None);
/// assert!(4183165669u32.checked_neg() == None);
/// # }
/// ```
pub fn t_u32_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u32>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x << y == u32::down((x.up() << y) & u32::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 << 0u32 == 0u32);
/// assert!(0u32 << 1u32 == 0u32);
/// assert!(4294967294u32 << 1u32 == 4294967292u32);
/// assert!(4294967294u32 << 0u32 == 4294967294u32);
/// assert!(669743634u32 << 19u32 == 3499098112u32);
/// assert!(3917168294u32 << 22u32 == 2843738112u32);
/// assert!(601415413u32 << 9u32 == 2982013440u32);
/// assert!(3586237127u32 << 30u32 == 3221225472u32);
/// assert!(4027564952u32 << 25u32 == 805306368u32);
/// assert!(683762028u32 << 1u32 == 1367524056u32);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u32 << 4294967294u32));
/// assert!(panics!(4294967295u32 << 4294967295u32));
/// assert!(panics!(0u32 << 4294967295u32));
/// assert!(panics!(4294967295u32 << 4294967294u32));
/// assert!(panics!(4294967294u32 << 4294967294u32));
/// assert!(panics!(4294967294u32 << 4294967295u32));
/// assert!(panics!(3949187279u32 << 122u32));
/// assert!(panics!(3061834496u32 << 93u32));
/// assert!(panics!(2876688484u32 << 124u32));
/// assert!(panics!(2830534233u32 << 105u32));
/// # }
/// ```
pub fn core_ops_shl_u32_shl() {}
/// # Properties for [`core::ops::Shl::<u32>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u32::down((x.up() << y) & u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_shl(1u32) == Some(4294967292u32));
/// assert!(1u32.checked_shl(0u32) == Some(1u32));
/// assert!(4294967294u32.checked_shl(0u32) == Some(4294967294u32));
/// assert!(4294967295u32.checked_shl(0u32) == Some(4294967295u32));
/// assert!(0u32.checked_shl(0u32) == Some(0u32));
/// assert!(3782832683u32.checked_shl(3u32) == Some(197890392u32));
/// assert!(816830776u32.checked_shl(11u32) == Some(2127151104u32));
/// assert!(3526340929u32.checked_shl(15u32) == Some(3634397184u32));
/// assert!(1150282997u32.checked_shl(18u32) == Some(3017015296u32));
/// assert!(239166476u32.checked_shl(30u32) == Some(0u32));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_shl(4294967295u32) == None);
/// assert!(0u32.checked_shl(4294967294u32) == None);
/// assert!(1u32.checked_shl(4294967294u32) == None);
/// assert!(4294967295u32.checked_shl(4294967295u32) == None);
/// assert!(4294967295u32.checked_shl(4294967294u32) == None);
/// assert!(0u32.checked_shl(4294967295u32) == None);
/// assert!(41196520u32.checked_shl(64u32) == None);
/// assert!(2256010501u32.checked_shl(49u32) == None);
/// assert!(2665067108u32.checked_shl(77u32) == None);
/// assert!(3677581257u32.checked_shl(92u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u32_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u32>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u32::down((x.up() << y) & u32::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shl(0u32) == (0u32, false));
/// assert!(4294967294u32.overflowing_shl(1u32) == (4294967292u32, false));
/// assert!(4294967294u32.overflowing_shl(0u32) == (4294967294u32, false));
/// assert!(1u32.overflowing_shl(1u32) == (2u32, false));
/// assert!(4294967295u32.overflowing_shl(1u32) == (4294967294u32, false));
/// assert!(4205616140u32.overflowing_shl(16u32) == (2618032128u32, false));
/// assert!(1543405933u32.overflowing_shl(20u32) == (382730240u32, false));
/// assert!(1001964960u32.overflowing_shl(12u32) == (2354708480u32, false));
/// assert!(1332612026u32.overflowing_shl(21u32) == (2000683008u32, false));
/// assert!(2829980617u32.overflowing_shl(30u32) == (1073741824u32, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u32::down((x.up() << (y & (u32::BITS - 1)) & u32::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.overflowing_shl(4294967294u32) == (1073741824u32, true));
/// assert!(4294967295u32.overflowing_shl(4294967295u32) == (2147483648u32, true));
/// assert!(4294967295u32.overflowing_shl(4294967294u32) == (3221225472u32, true));
/// assert!(0u32.overflowing_shl(4294967295u32) == (0u32, true));
/// assert!(0u32.overflowing_shl(4294967294u32) == (0u32, true));
/// assert!(3668337772u32.overflowing_shl(127u32) == (0u32, true));
/// assert!(2501221230u32.overflowing_shl(102u32) == (1164368768u32, true));
/// assert!(843523130u32.overflowing_shl(82u32) == (2431123456u32, true));
/// assert!(2076188353u32.overflowing_shl(49u32) == (1031929856u32, true));
/// assert!(1907360783u32.overflowing_shl(72u32) == (2953056000u32, true));
/// # }
/// ```
pub fn core_ops_shl_u32_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u32>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x >> y == u32::down((x.up() >> y) & u32::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32 >> 0u32 == 1u32);
/// assert!(1u32 >> 1u32 == 0u32);
/// assert!(0u32 >> 1u32 == 0u32);
/// assert!(0u32 >> 0u32 == 0u32);
/// assert!(1437922376u32 >> 12u32 == 351055u32);
/// assert!(642389033u32 >> 22u32 == 153u32);
/// assert!(4253129046u32 >> 22u32 == 1014u32);
/// assert!(2125041662u32 >> 8u32 == 8300943u32);
/// assert!(174295088u32 >> 3u32 == 21786886u32);
/// assert!(1801551150u32 >> 8u32 == 7037309u32);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u32 >> 4294967294u32));
/// assert!(panics!(0u32 >> 4294967295u32));
/// assert!(panics!(4294967294u32 >> 4294967294u32));
/// assert!(panics!(4294967294u32 >> 4294967295u32));
/// assert!(panics!(1u32 >> 4294967294u32));
/// assert!(panics!(3047151849u32 >> 45u32));
/// assert!(panics!(1870812254u32 >> 79u32));
/// assert!(panics!(1373073208u32 >> 76u32));
/// assert!(panics!(2782983875u32 >> 107u32));
/// assert!(panics!(3879018723u32 >> 103u32));
/// # }
/// ```
pub fn core_ops_shr_u32_shr() {}
/// # Properties for [`core::ops::Shr::<u32>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u32::down((x.up() >> y) & u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_shr(0u32) == Some(4294967294u32));
/// assert!(0u32.checked_shr(1u32) == Some(0u32));
/// assert!(4294967295u32.checked_shr(1u32) == Some(2147483647u32));
/// assert!(0u32.checked_shr(0u32) == Some(0u32));
/// assert!(1040506545u32.checked_shr(31u32) == Some(0u32));
/// assert!(590349653u32.checked_shr(20u32) == Some(563u32));
/// assert!(4259433230u32.checked_shr(24u32) == Some(253u32));
/// assert!(2279673133u32.checked_shr(26u32) == Some(33u32));
/// assert!(3359167607u32.checked_shr(30u32) == Some(3u32));
/// assert!(1146787573u32.checked_shr(23u32) == Some(136u32));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_shr(4294967294u32) == None);
/// assert!(0u32.checked_shr(4294967295u32) == None);
/// assert!(0u32.checked_shr(4294967294u32) == None);
/// assert!(4294967295u32.checked_shr(4294967295u32) == None);
/// assert!(3326962121u32.checked_shr(113u32) == None);
/// assert!(1561554395u32.checked_shr(87u32) == None);
/// assert!(3460741704u32.checked_shr(109u32) == None);
/// assert!(1612433502u32.checked_shr(41u32) == None);
/// assert!(3897529301u32.checked_shr(114u32) == None);
/// assert!(2084917793u32.checked_shr(134u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u32_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u32>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u32::down((x.up() >> y) & u32::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shr(0u32) == (0u32, false));
/// assert!(4294967294u32.overflowing_shr(0u32) == (4294967294u32, false));
/// assert!(0u32.overflowing_shr(1u32) == (0u32, false));
/// assert!(1u32.overflowing_shr(0u32) == (1u32, false));
/// assert!(4294967295u32.overflowing_shr(0u32) == (4294967295u32, false));
/// assert!(3329395042u32.overflowing_shr(15u32) == (101605u32, false));
/// assert!(3524196966u32.overflowing_shr(28u32) == (13u32, false));
/// assert!(1214424336u32.overflowing_shr(9u32) == (2371922u32, false));
/// assert!(129137063u32.overflowing_shr(21u32) == (61u32, false));
/// assert!(2106702816u32.overflowing_shr(28u32) == (7u32, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u32::down((x.up() >> (y & (u32::BITS - 1)) & u32::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.overflowing_shr(4294967295u32) == (1u32, true));
/// assert!(4294967294u32.overflowing_shr(4294967295u32) == (1u32, true));
/// assert!(0u32.overflowing_shr(4294967294u32) == (0u32, true));
/// assert!(1u32.overflowing_shr(4294967295u32) == (0u32, true));
/// assert!(0u32.overflowing_shr(4294967295u32) == (0u32, true));
/// assert!(1740666444u32.overflowing_shr(83u32) == (3320u32, true));
/// assert!(1070135533u32.overflowing_shr(64u32) == (1070135533u32, true));
/// assert!(1991672550u32.overflowing_shr(47u32) == (60781u32, true));
/// assert!(1061548907u32.overflowing_shr(89u32) == (31u32, true));
/// assert!(1355303559u32.overflowing_shr(57u32) == (40u32, true));
/// # }
/// ```
pub fn core_ops_shr_u32_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u32>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u32::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32 / 4294967295u32 == 1u32);
/// assert!(4294967295u32 / 1u32 == 4294967295u32);
/// assert!(0u32 / 4294967294u32 == 0u32);
/// assert!(1u32 / 4294967294u32 == 0u32);
/// assert!(4294967294u32 / 1u32 == 4294967294u32);
/// assert!(1u32 / 1u32 == 1u32);
/// assert!(33017u32 / 55210u32 == 0u32);
/// assert!(27293u32 / 38604u32 == 0u32);
/// assert!(60610u32 / 61710u32 == 0u32);
/// assert!(29692u32 / 19631u32 == 1u32);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(1u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967294u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967295u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1661446076u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1292851075u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2062014865u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(721252113u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65492122u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(3981364148u32 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u32_div() {}
/// # Properties for [`u32::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u32::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_div(4294967294u32) == 0u32);
/// assert!(4294967295u32.saturating_div(1u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_div(4294967294u32) == 1u32);
/// assert!(4294967294u32.saturating_div(4294967294u32) == 1u32);
/// assert!(4294967295u32.saturating_div(4294967295u32) == 1u32);
/// assert!(0u32.saturating_div(1u32) == 0u32);
/// assert!(19000u32.saturating_div(62303u32) == 0u32);
/// assert!(61057u32.saturating_div(65180u32) == 0u32);
/// assert!(32040u32.saturating_div(19637u32) == 1u32);
/// assert!(22727u32.saturating_div(43663u32) == 0u32);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967294u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967295u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4117435034u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2424381597u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(3952739913u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(76031108u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(3623311327u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2313520947u32.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u32_saturating_div() {}
/// # Properties for [`u32::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u32::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_div(4294967295u32) == Some(0u32));
/// assert!(4294967294u32.checked_div(4294967294u32) == Some(1u32));
/// assert!(1u32.checked_div(4294967294u32) == Some(0u32));
/// assert!(4294967295u32.checked_div(1u32) == Some(4294967295u32));
/// assert!(1u32.checked_div(1u32) == Some(1u32));
/// assert!(0u32.checked_div(4294967295u32) == Some(0u32));
/// assert!(11840u32.checked_div(51541u32) == Some(0u32));
/// assert!(37965u32.checked_div(52750u32) == Some(0u32));
/// assert!(9641u32.checked_div(43257u32) == Some(0u32));
/// assert!(49076u32.checked_div(58650u32) == Some(0u32));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_div(0) == None);
/// assert!(4294967294u32.checked_div(0) == None);
/// assert!(1u32.checked_div(0) == None);
/// assert!(4294967295u32.checked_div(0) == None);
/// assert!(1308688342u32.checked_div(0) == None);
/// assert!(3692526993u32.checked_div(0) == None);
/// assert!(186296453u32.checked_div(0) == None);
/// assert!(3837595303u32.checked_div(0) == None);
/// assert!(3274842980u32.checked_div(0) == None);
/// assert!(3145597644u32.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u32_checked_div() {}
/// # Properties for [`core::ops::Mul::<u32>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x * y == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32 * 0u32 == 0u32);
/// assert!(1u32 * 4294967294u32 == 4294967294u32);
/// assert!(0u32 * 0u32 == 0u32);
/// assert!(4294967294u32 * 0u32 == 0u32);
/// assert!(4294967295u32 * 1u32 == 4294967295u32);
/// assert!(0u32 * 4294967295u32 == 0u32);
/// assert!(28724u32 * 25333u32 == 727665092u32);
/// assert!(39244u32 * 50395u32 == 1977701380u32);
/// assert!(32934u32 * 63289u32 == 2084359926u32);
/// assert!(52423u32 * 32530u32 == 1705320190u32);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(4294967294u32 * 4294967295u32));
/// assert!(panics!(4294967294u32 * 4294967294u32));
/// assert!(panics!(4294967295u32 * 4294967295u32));
/// assert!(panics!(4294967295u32 * 4294967294u32));
/// assert!(panics!(768117562u32 * 1894435572u32));
/// assert!(panics!(434337426u32 * 818403565u32));
/// assert!(panics!(746125644u32 * 2744757811u32));
/// assert!(panics!(1495472173u32 * 1696332112u32));
/// assert!(panics!(2133725478u32 * 121929865u32));
/// assert!(panics!(464517455u32 * 1167236039u32));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 * 1 == 4294967294u32);
/// assert!(1u32 * 1 == 1u32);
/// assert!(4294967295u32 * 1 == 4294967295u32);
/// assert!(0u32 * 1 == 0u32);
/// assert!(3240419707u32 * 1 == 3240419707u32);
/// assert!(3952418409u32 * 1 == 3952418409u32);
/// assert!(2520597756u32 * 1 == 2520597756u32);
/// assert!(224749813u32 * 1 == 224749813u32);
/// assert!(1567412279u32 * 1 == 1567412279u32);
/// assert!(583069060u32 * 1 == 583069060u32);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u32 == 0u32);
/// assert!(1 * 4294967295u32 == 4294967295u32);
/// assert!(1 * 4294967294u32 == 4294967294u32);
/// assert!(1 * 1u32 == 1u32);
/// assert!(1 * 1152425395u32 == 1152425395u32);
/// assert!(1 * 3340178982u32 == 3340178982u32);
/// assert!(1 * 931281705u32 == 931281705u32);
/// assert!(1 * 2462979289u32 == 2462979289u32);
/// assert!(1 * 2832329954u32 == 2832329954u32);
/// assert!(1 * 3857791011u32 == 3857791011u32);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 * 4294967295u32 == 4294967295u32 * 0u32);
/// assert!(0u32 * 0u32 == 0u32 * 0u32);
/// assert!(1u32 * 1u32 == 1u32 * 1u32);
/// assert!(4294967294u32 * 0u32 == 0u32 * 4294967294u32);
/// assert!(1u32 * 4294967294u32 == 4294967294u32 * 1u32);
/// assert!(27006u32 * 63093u32 == 63093u32 * 27006u32);
/// assert!(17042u32 * 58982u32 == 58982u32 * 17042u32);
/// assert!(31279u32 * 55510u32 == 55510u32 * 31279u32);
/// assert!(9410u32 * 40476u32 == 40476u32 * 9410u32);
/// assert!(28800u32 * 59615u32 == 59615u32 * 28800u32);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u32::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u32 * 4294967295u32) * 1u32 == 1u32 * (4294967295u32 * 1u32));
/// assert!((4294967295u32 * 1u32) * 1u32 == 4294967295u32 * (1u32 * 1u32));
/// assert!((4294967294u32 * 1u32) * 1u32 == 4294967294u32 * (1u32 * 1u32));
/// assert!((1u32 * 1u32) * 4294967294u32 == 1u32 * (1u32 * 4294967294u32));
/// assert!((1u32 * 1u32) * 4294967295u32 == 1u32 * (1u32 * 4294967295u32));
/// assert!((1u32 * 1u32) * 1u32 == 1u32 * (1u32 * 1u32));
/// assert!((1075u32 * 1085u32) * 1328u32 == 1075u32 * (1085u32 * 1328u32));
/// assert!((764u32 * 1620u32) * 1538u32 == 764u32 * (1620u32 * 1538u32));
/// assert!((1584u32 * 1193u32) * 1295u32 == 1584u32 * (1193u32 * 1295u32));
/// assert!((1452u32 * 1516u32) * 1259u32 == 1452u32 * (1516u32 * 1259u32));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u32::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 * (1u32 + 0u32) == 4294967294u32 * 1u32 + 4294967294u32 * 0u32);
/// assert!(1u32 * (1u32 + 0u32) == 1u32 * 1u32 + 1u32 * 0u32);
/// assert!(4294967295u32 * (0u32 + 1u32) == 4294967295u32 * 0u32 + 4294967295u32 * 1u32);
/// assert!(4294967295u32 * (0u32 + 0u32) == 4294967295u32 * 0u32 + 4294967295u32 * 0u32);
/// assert!(1u32 * (4294967295u32 + 0u32) == 1u32 * 4294967295u32 + 1u32 * 0u32);
/// assert!(24892u32 * (34663u32 + 36102u32) == 24892u32 * 34663u32 + 24892u32 * 36102u32);
/// assert!(45003u32 * (44344u32 + 27514u32) == 45003u32 * 44344u32 + 45003u32 * 27514u32);
/// assert!(44621u32 * (26103u32 + 41522u32) == 44621u32 * 26103u32 + 44621u32 * 41522u32);
/// assert!(34029u32 * (13028u32 + 54008u32) == 34029u32 * 13028u32 + 34029u32 * 54008u32);
/// assert!(19040u32 * (23891u32 + 56728u32) == 19040u32 * 23891u32 + 19040u32 * 56728u32);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 * 0 == 0);
/// assert!(1u32 * 0 == 0);
/// assert!(4294967294u32 * 0 == 0);
/// assert!(4294967295u32 * 0 == 0);
/// assert!(54324u32 * 0 == 0);
/// assert!(57087u32 * 0 == 0);
/// assert!(63870u32 * 0 == 0);
/// assert!(42332u32 * 0 == 0);
/// assert!(9532u32 * 0 == 0);
/// assert!(37712u32 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u32_mul() {}
/// # Properties for [`u32::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u32::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_mul(1u32) == Some(0u32));
/// assert!(0u32.checked_mul(4294967294u32) == Some(0u32));
/// assert!(4294967295u32.checked_mul(1u32) == Some(4294967295u32));
/// assert!(1u32.checked_mul(0u32) == Some(0u32));
/// assert!(4294967294u32.checked_mul(0u32) == Some(0u32));
/// assert!(42253u32.checked_mul(31297u32) == Some(1322392141u32));
/// assert!(38683u32.checked_mul(18031u32) == Some(697493173u32));
/// assert!(50050u32.checked_mul(53662u32) == Some(2685783100u32));
/// assert!(8872u32.checked_mul(60791u32) == Some(539337752u32));
/// assert!(62044u32.checked_mul(53263u32) == Some(3304649572u32));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_mul(4294967295u32) == None);
/// assert!(4294967294u32.checked_mul(4294967294u32) == None);
/// assert!(4294967295u32.checked_mul(4294967294u32) == None);
/// assert!(4294967295u32.checked_mul(4294967295u32) == None);
/// assert!(3070751973u32.checked_mul(3621290198u32) == None);
/// assert!(109615226u32.checked_mul(3779581710u32) == None);
/// assert!(3972551129u32.checked_mul(450902805u32) == None);
/// assert!(637640393u32.checked_mul(1440428817u32) == None);
/// assert!(3252459824u32.checked_mul(1146404571u32) == None);
/// assert!(1880850571u32.checked_mul(4102814759u32) == None);
/// # }
/// ```
pub fn core_ops_mul_u32_checked_mul() {}
/// # Properties for [`u32::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u32::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.overflowing_mul(1u32) == (4294967294u32, false));
/// assert!(1u32.overflowing_mul(1u32) == (1u32, false));
/// assert!(0u32.overflowing_mul(0u32) == (0u32, false));
/// assert!(0u32.overflowing_mul(4294967295u32) == (0u32, false));
/// assert!(0u32.overflowing_mul(4294967294u32) == (0u32, false));
/// assert!(20092u32.overflowing_mul(41799u32) == (839825508u32, false));
/// assert!(41348u32.overflowing_mul(34734u32) == (1436181432u32, false));
/// assert!(49211u32.overflowing_mul(53257u32) == (2620830227u32, false));
/// assert!(20225u32.overflowing_mul(37263u32) == (753644175u32, false));
/// assert!(56402u32.overflowing_mul(62535u32) == (3527099070u32, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.overflowing_mul(4294967295u32) == (1u32, true));
/// assert!(4294967294u32.overflowing_mul(4294967294u32) == (4u32, true));
/// assert!(4294967295u32.overflowing_mul(4294967294u32) == (2u32, true));
/// assert!(4294967294u32.overflowing_mul(4294967295u32) == (2u32, true));
/// assert!(2406555849u32.overflowing_mul(585895872u32) == (554720704u32, true));
/// assert!(349256722u32.overflowing_mul(646742555u32) == (4167727590u32, true));
/// assert!(4555846u32.overflowing_mul(855812816u32) == (1558050016u32, true));
/// assert!(2454075126u32.overflowing_mul(2654650751u32) == (3355490826u32, true));
/// assert!(557226462u32.overflowing_mul(3121449898u32) == (3490953068u32, true));
/// assert!(3147968497u32.overflowing_mul(2794620889u32) == (825539145u32, true));
/// # }
/// ```
pub fn core_ops_mul_u32_overflowing_mul() {}
/// # Properties for [`u32::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::down((x.up() * y.up()).min(u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.saturating_mul(1u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_mul(4294967294u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_mul(0u32) == 0u32);
/// assert!(0u32.saturating_mul(0u32) == 0u32);
/// assert!(4294967295u32.saturating_mul(4294967294u32) == 4294967295u32);
/// assert!(0u32.saturating_mul(4294967295u32) == 0u32);
/// assert!(853487294u32.saturating_mul(82141593u32) == 4294967295u32);
/// assert!(4053380717u32.saturating_mul(365171957u32) == 4294967295u32);
/// assert!(4085412011u32.saturating_mul(1101463899u32) == 4294967295u32);
/// assert!(979534168u32.saturating_mul(2809601350u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.saturating_mul(0u32) == 0u32);
/// assert!(1u32.saturating_mul(4294967294u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_mul(1u32) == 4294967294u32);
/// assert!(0u32.saturating_mul(4294967294u32) == 0u32);
/// assert!(0u32.saturating_mul(0u32) == 0u32);
/// assert!(21499u32.saturating_mul(54823u32) == 1178639677u32);
/// assert!(54586u32.saturating_mul(42422u32) == 2315647292u32);
/// assert!(50395u32.saturating_mul(35155u32) == 1771636225u32);
/// assert!(1989u32.saturating_mul(61716u32) == 122753124u32);
/// assert!(41752u32.saturating_mul(34529u32) == 1441654808u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.saturating_mul(4294967294u32) == u32::MAX);
/// assert!(4294967295u32.saturating_mul(4294967295u32) == u32::MAX);
/// assert!(4294967294u32.saturating_mul(4294967295u32) == u32::MAX);
/// assert!(4294967294u32.saturating_mul(4294967294u32) == u32::MAX);
/// assert!(2965414999u32.saturating_mul(1748974354u32) == u32::MAX);
/// assert!(4091086397u32.saturating_mul(3091489287u32) == u32::MAX);
/// assert!(369477996u32.saturating_mul(3804305579u32) == u32::MAX);
/// assert!(4209746515u32.saturating_mul(1154375292u32) == u32::MAX);
/// assert!(1521051765u32.saturating_mul(3192343072u32) == u32::MAX);
/// assert!(2651469890u32.saturating_mul(3351461184u32) == u32::MAX);
/// # }
/// ```
pub fn core_ops_mul_u32_saturating_mul() {}
/// # Properties for [`u32::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u32::down((x.up() * y.up()) % (u32::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.wrapping_mul(4294967294u32) == 4294967294u32);
/// assert!(1u32.wrapping_mul(4294967295u32) == 4294967295u32);
/// assert!(0u32.wrapping_mul(1u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967295u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967294u32) == 0u32);
/// assert!(4294967295u32.wrapping_mul(1u32) == 4294967295u32);
/// assert!(1226591367u32.wrapping_mul(571215177u32) == 3444092287u32);
/// assert!(635186500u32.wrapping_mul(1000122049u32) == 2445817924u32);
/// assert!(3668241672u32.wrapping_mul(4035573315u32) == 238532888u32);
/// assert!(2278524157u32.wrapping_mul(177612215u32) == 3788415451u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.wrapping_mul(1u32) == 4294967295u32);
/// assert!(0u32.wrapping_mul(4294967295u32) == 0u32);
/// assert!(4294967294u32.wrapping_mul(1u32) == 4294967294u32);
/// assert!(1u32.wrapping_mul(1u32) == 1u32);
/// assert!(1u32.wrapping_mul(0u32) == 0u32);
/// assert!(62475u32.wrapping_mul(27415u32) == 1712752125u32);
/// assert!(58443u32.wrapping_mul(60471u32) == 3534106653u32);
/// assert!(27266u32.wrapping_mul(29281u32) == 798375746u32);
/// assert!(30185u32.wrapping_mul(27163u32) == 819915155u32);
/// assert!(11507u32.wrapping_mul(47990u32) == 552220930u32);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32 * 1 == 4294967295u32);
/// assert!(1u32 * 1 == 1u32);
/// assert!(4294967294u32 * 1 == 4294967294u32);
/// assert!(0u32 * 1 == 0u32);
/// assert!(3317255885u32 * 1 == 3317255885u32);
/// assert!(2845783535u32 * 1 == 2845783535u32);
/// assert!(111188253u32 * 1 == 111188253u32);
/// assert!(1011577122u32 * 1 == 1011577122u32);
/// assert!(2951685304u32 * 1 == 2951685304u32);
/// assert!(3968760111u32 * 1 == 3968760111u32);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u32 == 0u32);
/// assert!(1 * 1u32 == 1u32);
/// assert!(1 * 4294967295u32 == 4294967295u32);
/// assert!(1 * 4294967294u32 == 4294967294u32);
/// assert!(1 * 602185037u32 == 602185037u32);
/// assert!(1 * 3653640287u32 == 3653640287u32);
/// assert!(1 * 3066014464u32 == 3066014464u32);
/// assert!(1 * 2267749849u32 == 2267749849u32);
/// assert!(1 * 2561454479u32 == 2561454479u32);
/// assert!(1 * 3197302574u32 == 3197302574u32);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 * 0u32 == 0u32 * 0u32);
/// assert!(0u32 * 1u32 == 1u32 * 0u32);
/// assert!(4294967294u32 * 0u32 == 0u32 * 4294967294u32);
/// assert!(0u32 * 4294967294u32 == 4294967294u32 * 0u32);
/// assert!(0u32 * 4294967295u32 == 4294967295u32 * 0u32);
/// assert!(53737u32 * 61277u32 == 61277u32 * 53737u32);
/// assert!(37332u32 * 49891u32 == 49891u32 * 37332u32);
/// assert!(20443u32 * 52405u32 == 52405u32 * 20443u32);
/// assert!(53152u32 * 52302u32 == 52302u32 * 53152u32);
/// assert!(37919u32 * 63676u32 == 63676u32 * 37919u32);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u32::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((4294967294u32 * 1u32) * 1u32 == 4294967294u32 * (1u32 * 1u32));
/// assert!((1u32 * 1u32) * 1u32 == 1u32 * (1u32 * 1u32));
/// assert!((1u32 * 1u32) * 4294967295u32 == 1u32 * (1u32 * 4294967295u32));
/// assert!((4294967295u32 * 1u32) * 1u32 == 4294967295u32 * (1u32 * 1u32));
/// assert!((1u32 * 4294967295u32) * 1u32 == 1u32 * (4294967295u32 * 1u32));
/// assert!((1u32 * 4294967294u32) * 1u32 == 1u32 * (4294967294u32 * 1u32));
/// assert!((1589u32 * 1594u32) * 1318u32 == 1589u32 * (1594u32 * 1318u32));
/// assert!((1567u32 * 1270u32) * 453u32 == 1567u32 * (1270u32 * 453u32));
/// assert!((1424u32 * 1379u32) * 1010u32 == 1424u32 * (1379u32 * 1010u32));
/// assert!((1184u32 * 662u32) * 1587u32 == 1184u32 * (662u32 * 1587u32));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u32::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32 * (4294967294u32 + 0u32) == 1u32 * 4294967294u32 + 1u32 * 0u32);
/// assert!(4294967295u32 * (0u32 + 0u32) == 4294967295u32 * 0u32 + 4294967295u32 * 0u32);
/// assert!(1u32 * (1u32 + 0u32) == 1u32 * 1u32 + 1u32 * 0u32);
/// assert!(4294967294u32 * (1u32 + 0u32) == 4294967294u32 * 1u32 + 4294967294u32 * 0u32);
/// assert!(1u32 * (0u32 + 4294967294u32) == 1u32 * 0u32 + 1u32 * 4294967294u32);
/// assert!(1u32 * (1u32 + 4294967294u32) == 1u32 * 1u32 + 1u32 * 4294967294u32);
/// assert!(30580u32 * (42169u32 + 40935u32) == 30580u32 * 42169u32 + 30580u32 * 40935u32);
/// assert!(60706u32 * (27091u32 + 32486u32) == 60706u32 * 27091u32 + 60706u32 * 32486u32);
/// assert!(55126u32 * (52912u32 + 22453u32) == 55126u32 * 52912u32 + 55126u32 * 22453u32);
/// assert!(39566u32 * (27179u32 + 37343u32) == 39566u32 * 27179u32 + 39566u32 * 37343u32);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 * 0 == 0);
/// assert!(0u32 * 0 == 0);
/// assert!(4294967295u32 * 0 == 0);
/// assert!(1u32 * 0 == 0);
/// assert!(30873u32 * 0 == 0);
/// assert!(42025u32 * 0 == 0);
/// assert!(18839u32 * 0 == 0);
/// assert!(64326u32 * 0 == 0);
/// assert!(64427u32 * 0 == 0);
/// assert!(47024u32 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u32_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u32>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 - 0u32 == 0u32);
/// assert!(4294967294u32 - 4294967294u32 == 0u32);
/// assert!(4294967294u32 - 0u32 == 4294967294u32);
/// assert!(4294967295u32 - 4294967295u32 == 0u32);
/// assert!(486498204u32 - 471145156u32 == 15353048u32);
/// assert!(2069356306u32 - 1045582120u32 == 1023774186u32);
/// assert!(4173983357u32 - 4117931255u32 == 56052102u32);
/// assert!(2926510434u32 - 1820533347u32 == 1105977087u32);
/// assert!(4284767434u32 - 2312368263u32 == 1972399171u32);
/// assert!(3948821746u32 - 1802915888u32 == 2145905858u32);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(4294967294u32 - 4294967295u32));
/// assert!(panics!(1u32 - 4294967294u32));
/// assert!(panics!(0u32 - 4294967294u32));
/// assert!(panics!(0u32 - 1u32));
/// assert!(panics!(1u32 - 4294967295u32));
/// assert!(panics!(692831869u32 - 1001716772u32));
/// assert!(panics!(3224835718u32 - 3475345696u32));
/// assert!(panics!(372367092u32 - 3072983951u32));
/// assert!(panics!(2232322308u32 - 4272730747u32));
/// assert!(panics!(1709891287u32 - 1775353057u32));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((4294967294u32 - 0u32) + 0u32 == 4294967294u32);
/// assert!((1u32 - 1u32) + 1u32 == 1u32);
/// assert!((4294967294u32 - 1u32) + 1u32 == 4294967294u32);
/// assert!((0u32 - 0u32) + 0u32 == 0u32);
/// assert!((4294967295u32 - 4294967294u32) + 4294967294u32 == 4294967295u32);
/// assert!((3503504232u32 - 2207408220u32) + 2207408220u32 == 3503504232u32);
/// assert!((4074165778u32 - 2872566228u32) + 2872566228u32 == 4074165778u32);
/// assert!((3002324950u32 - 736977113u32) + 736977113u32 == 3002324950u32);
/// assert!((4057212946u32 - 23196142u32) + 23196142u32 == 4057212946u32);
/// assert!((3755717078u32 - 1067965460u32) + 1067965460u32 == 3755717078u32);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 - 0 == 4294967294u32);
/// assert!(0u32 - 0 == 0u32);
/// assert!(4294967295u32 - 0 == 4294967295u32);
/// assert!(1u32 - 0 == 1u32);
/// assert!(561840035u32 - 0 == 561840035u32);
/// assert!(1378744294u32 - 0 == 1378744294u32);
/// assert!(2423851624u32 - 0 == 2423851624u32);
/// assert!(2972672985u32 - 0 == 2972672985u32);
/// assert!(307820938u32 - 0 == 307820938u32);
/// assert!(1801042867u32 - 0 == 1801042867u32);
/// # }
/// ```
pub fn core_ops_add_u32_sub() {}
/// # Properties for [`u32::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_sub(0u32) == 0u32);
/// assert!(1u32.wrapping_sub(1u32) == 0u32);
/// assert!(4294967295u32.wrapping_sub(0u32) == 4294967295u32);
/// assert!(4294967294u32.wrapping_sub(0u32) == 4294967294u32);
/// assert!(3679527968u32.wrapping_sub(2746379614u32) == 933148354u32);
/// assert!(3644553313u32.wrapping_sub(1881986643u32) == 1762566670u32);
/// assert!(1018447169u32.wrapping_sub(891574822u32) == 126872347u32);
/// assert!(3272597130u32.wrapping_sub(1476364557u32) == 1796232573u32);
/// assert!(3296135198u32.wrapping_sub(2947709710u32) == 348425488u32);
/// assert!(1367674310u32.wrapping_sub(881177497u32) == 486496813u32);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down(x.up() - y.up() + u32::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.wrapping_sub(4294967295u32) == 4294967295u32);
/// assert!(0u32.wrapping_sub(1u32) == 4294967295u32);
/// assert!(1u32.wrapping_sub(4294967295u32) == 2u32);
/// assert!(1u32.wrapping_sub(4294967294u32) == 3u32);
/// assert!(0u32.wrapping_sub(4294967294u32) == 2u32);
/// assert!(1843648987u32.wrapping_sub(2352633637u32) == 3785982646u32);
/// assert!(216873297u32.wrapping_sub(3432805729u32) == 1079034864u32);
/// assert!(1721787815u32.wrapping_sub(2737439611u32) == 3279315500u32);
/// assert!(3428183843u32.wrapping_sub(3661356118u32) == 4061795021u32);
/// assert!(1477959897u32.wrapping_sub(2156008187u32) == 3616919006u32);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down((x.up() - y.up()).rem_euclid(&(u32::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_sub(4294967294u32) == 2u32);
/// assert!(1u32.wrapping_sub(0u32) == 1u32);
/// assert!(4294967294u32.wrapping_sub(4294967294u32) == 0u32);
/// assert!(4294967294u32.wrapping_sub(4294967295u32) == 4294967295u32);
/// assert!(1u32.wrapping_sub(4294967294u32) == 3u32);
/// assert!(4083006305u32.wrapping_sub(2560888122u32) == 1522118183u32);
/// assert!(1273145691u32.wrapping_sub(492035073u32) == 781110618u32);
/// assert!(2043808745u32.wrapping_sub(1311716982u32) == 732091763u32);
/// assert!(3257880794u32.wrapping_sub(676952463u32) == 2580928331u32);
/// assert!(356866752u32.wrapping_sub(167402291u32) == 189464461u32);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u32.wrapping_sub(0u32)).wrapping_add(0u32) == 1u32);
/// assert!((0u32.wrapping_sub(0u32)).wrapping_add(0u32) == 0u32);
/// assert!((1u32.wrapping_sub(4294967294u32)).wrapping_add(4294967294u32) == 1u32);
/// assert!((0u32.wrapping_sub(4294967295u32)).wrapping_add(4294967295u32) == 0u32);
/// assert!((4294967295u32.wrapping_sub(0u32)).wrapping_add(0u32) == 4294967295u32);
/// assert!((3076129507u32.wrapping_sub(2809785564u32)).wrapping_add(2809785564u32)
///         == 3076129507u32);
/// assert!((2062194708u32.wrapping_sub(2953437153u32)).wrapping_add(2953437153u32)
///         == 2062194708u32);
/// assert!((653721344u32.wrapping_sub(1839067770u32)).wrapping_add(1839067770u32)
///         == 653721344u32);
/// assert!((3530026658u32.wrapping_sub(1773066552u32)).wrapping_add(1773066552u32)
///         == 3530026658u32);
/// assert!((255804295u32.wrapping_sub(3218584940u32)).wrapping_add(3218584940u32)
///         == 255804295u32);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.wrapping_sub(0) == 1u32);
/// assert!(4294967295u32.wrapping_sub(0) == 4294967295u32);
/// assert!(0u32.wrapping_sub(0) == 0u32);
/// assert!(4294967294u32.wrapping_sub(0) == 4294967294u32);
/// assert!(701366904u32.wrapping_sub(0) == 701366904u32);
/// assert!(2044346625u32.wrapping_sub(0) == 2044346625u32);
/// assert!(3209614181u32.wrapping_sub(0) == 3209614181u32);
/// assert!(2176341157u32.wrapping_sub(0) == 2176341157u32);
/// assert!(2296555359u32.wrapping_sub(0) == 2296555359u32);
/// assert!(3438605976u32.wrapping_sub(0) == 3438605976u32);
/// # }
/// ```
pub fn core_ops_add_u32_wrapping_sub() {}
/// # Properties for [`u32::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u32::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.checked_sub(0u32) == Some(1u32));
/// assert!(4294967295u32.checked_sub(1u32) == Some(4294967294u32));
/// assert!(0u32.checked_sub(0u32) == Some(0u32));
/// assert!(4294967294u32.checked_sub(0u32) == Some(4294967294u32));
/// assert!(2260100400u32.checked_sub(420951308u32) == Some(1839149092u32));
/// assert!(2772559873u32.checked_sub(1618367586u32) == Some(1154192287u32));
/// assert!(2354042238u32.checked_sub(1356623786u32) == Some(997418452u32));
/// assert!(2349847705u32.checked_sub(1866134542u32) == Some(483713163u32));
/// assert!(1497236157u32.checked_sub(80633444u32) == Some(1416602713u32));
/// assert!(3750951798u32.checked_sub(1997055613u32) == Some(1753896185u32));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_sub(4294967294u32) == None);
/// assert!(0u32.checked_sub(1u32) == None);
/// assert!(1u32.checked_sub(4294967295u32) == None);
/// assert!(1u32.checked_sub(4294967294u32) == None);
/// assert!(4294967294u32.checked_sub(4294967295u32) == None);
/// assert!(2795059956u32.checked_sub(3751907827u32) == None);
/// assert!(950700434u32.checked_sub(2070431374u32) == None);
/// assert!(325922811u32.checked_sub(2866148702u32) == None);
/// assert!(3419206513u32.checked_sub(3735490579u32) == None);
/// assert!(594423471u32.checked_sub(1933210256u32) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((4294967294u32.checked_sub(0u32)).and_then(|r| r.checked_add(0u32))
///         == Some(4294967294u32));
/// assert!((4294967295u32.checked_sub(1u32)).and_then(|r| r.checked_add(1u32))
///         == Some(4294967295u32));
/// assert!((0u32.checked_sub(0u32)).and_then(|r| r.checked_add(0u32)) == Some(0u32));
/// assert!((4294967295u32.checked_sub(0u32)).and_then(|r| r.checked_add(0u32))
///         == Some(4294967295u32));
/// assert!((3321772758u32.checked_sub(2666334970u32)).and_then(|r| r.checked_add(2666334970u32))
///         == Some(3321772758u32));
/// assert!((4289707070u32.checked_sub(2035481719u32)).and_then(|r| r.checked_add(2035481719u32))
///         == Some(4289707070u32));
/// assert!((3954304948u32.checked_sub(1135233182u32)).and_then(|r| r.checked_add(1135233182u32))
///         == Some(3954304948u32));
/// assert!((3502068592u32.checked_sub(2911981990u32)).and_then(|r| r.checked_add(2911981990u32))
///         == Some(3502068592u32));
/// assert!((2041834116u32.checked_sub(1343177476u32)).and_then(|r| r.checked_add(1343177476u32))
///         == Some(2041834116u32));
/// assert!((834027195u32.checked_sub(496540351u32)).and_then(|r| r.checked_add(496540351u32))
///         == Some(834027195u32));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.checked_sub(0) == Some(1u32));
/// assert!(4294967294u32.checked_sub(0) == Some(4294967294u32));
/// assert!(0u32.checked_sub(0) == Some(0u32));
/// assert!(4294967295u32.checked_sub(0) == Some(4294967295u32));
/// assert!(1291869373u32.checked_sub(0) == Some(1291869373u32));
/// assert!(2382725693u32.checked_sub(0) == Some(2382725693u32));
/// assert!(2477921226u32.checked_sub(0) == Some(2477921226u32));
/// assert!(322146782u32.checked_sub(0) == Some(322146782u32));
/// assert!(1718039221u32.checked_sub(0) == Some(1718039221u32));
/// assert!(3693108226u32.checked_sub(0) == Some(3693108226u32));
/// # }
/// ```
pub fn core_ops_add_u32_checked_sub() {}
/// # Properties for [`u32::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u32::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.saturating_sub(4294967294u32) == 0u32);
/// assert!(4294967294u32.saturating_sub(0u32) == 4294967294u32);
/// assert!(4294967295u32.saturating_sub(0u32) == 4294967295u32);
/// assert!(1u32.saturating_sub(1u32) == 0u32);
/// assert!(0u32.saturating_sub(0u32) == 0u32);
/// assert!(0u32.saturating_sub(4294967294u32) == 0u32);
/// assert!(1084986489u32.saturating_sub(2898881554u32) == 0u32);
/// assert!(3953064583u32.saturating_sub(2411030344u32) == 1542034239u32);
/// assert!(2949316430u32.saturating_sub(676097397u32) == 2273219033u32);
/// assert!(4177412709u32.saturating_sub(1852285511u32) == 2325127198u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.saturating_sub(4294967294u32) == 1u32);
/// assert!(4294967295u32.saturating_sub(0u32) == 4294967295u32);
/// assert!(0u32.saturating_sub(0u32) == 0u32);
/// assert!(4294967295u32.saturating_sub(1u32) == 4294967294u32);
/// assert!(3010202241u32.saturating_sub(1297492073u32) == 1712710168u32);
/// assert!(1319959239u32.saturating_sub(42330453u32) == 1277628786u32);
/// assert!(2057892114u32.saturating_sub(2009634851u32) == 48257263u32);
/// assert!(4250215487u32.saturating_sub(3069273189u32) == 1180942298u32);
/// assert!(3881029914u32.saturating_sub(1417515641u32) == 2463514273u32);
/// assert!(688475509u32.saturating_sub(117499645u32) == 570975864u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_sub(4294967294u32) == 0);
/// assert!(1u32.saturating_sub(4294967294u32) == 0);
/// assert!(4294967294u32.saturating_sub(4294967295u32) == 0);
/// assert!(0u32.saturating_sub(4294967295u32) == 0);
/// assert!(0u32.saturating_sub(1u32) == 0);
/// assert!(1u32.saturating_sub(4294967295u32) == 0);
/// assert!(2691282591u32.saturating_sub(3463991976u32) == 0);
/// assert!(112659324u32.saturating_sub(376414842u32) == 0);
/// assert!(1667086028u32.saturating_sub(2587129391u32) == 0);
/// assert!(328326150u32.saturating_sub(1996068899u32) == 0);
/// # }
/// ```
pub fn core_ops_sub_u32_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u32>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x + y == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 + 0u32 == 4294967294u32);
/// assert!(0u32 + 0u32 == 0u32);
/// assert!(0u32 + 4294967295u32 == 4294967295u32);
/// assert!(1u32 + 0u32 == 1u32);
/// assert!(1u32 + 1u32 == 2u32);
/// assert!(4294967295u32 + 0u32 == 4294967295u32);
/// assert!(1783579018u32 + 930655341u32 == 2714234359u32);
/// assert!(1154908562u32 + 1140082041u32 == 2294990603u32);
/// assert!(1053206625u32 + 789112552u32 == 1842319177u32);
/// assert!(1987338097u32 + 1509385325u32 == 3496723422u32);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(4294967294u32 + 4294967294u32));
/// assert!(panics!(1u32 + 4294967295u32));
/// assert!(panics!(4294967294u32 + 4294967295u32));
/// assert!(panics!(4294967295u32 + 4294967295u32));
/// assert!(panics!(4294967295u32 + 4294967294u32));
/// assert!(panics!(4294967295u32 + 1u32));
/// assert!(panics!(3658607095u32 + 3593881462u32));
/// assert!(panics!(4078325526u32 + 3596863869u32));
/// assert!(panics!(3093352554u32 + 2234703603u32));
/// assert!(panics!(3847422279u32 + 2296308796u32));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 + 1u32 == 1u32 + 0u32);
/// assert!(1u32 + 0u32 == 0u32 + 1u32);
/// assert!(0u32 + 4294967294u32 == 4294967294u32 + 0u32);
/// assert!(1u32 + 4294967294u32 == 4294967294u32 + 1u32);
/// assert!(4294967294u32 + 0u32 == 0u32 + 4294967294u32);
/// assert!(861643195u32 + 947703315u32 == 947703315u32 + 861643195u32);
/// assert!(421641907u32 + 1960168096u32 == 1960168096u32 + 421641907u32);
/// assert!(673471463u32 + 1558117419u32 == 1558117419u32 + 673471463u32);
/// assert!(1442389728u32 + 2585643127u32 == 2585643127u32 + 1442389728u32);
/// assert!(902758234u32 + 1072452011u32 == 1072452011u32 + 902758234u32);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32 + 0 == 4294967295u32);
/// assert!(0u32 + 0 == 0u32);
/// assert!(1u32 + 0 == 1u32);
/// assert!(4294967294u32 + 0 == 4294967294u32);
/// assert!(135061664u32 + 0 == 135061664u32);
/// assert!(2074750162u32 + 0 == 2074750162u32);
/// assert!(4259971343u32 + 0 == 4259971343u32);
/// assert!(2765680535u32 + 0 == 2765680535u32);
/// assert!(3658735360u32 + 0 == 3658735360u32);
/// assert!(1578562965u32 + 0 == 1578562965u32);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 4294967295u32 == 4294967295u32);
/// assert!(0 + 1u32 == 1u32);
/// assert!(0 + 0u32 == 0u32);
/// assert!(0 + 4294967294u32 == 4294967294u32);
/// assert!(0 + 902230423u32 == 902230423u32);
/// assert!(0 + 364649242u32 == 364649242u32);
/// assert!(0 + 3761329765u32 == 3761329765u32);
/// assert!(0 + 4202304802u32 == 4202304802u32);
/// assert!(0 + 3396273517u32 == 3396273517u32);
/// assert!(0 + 4177882604u32 == 4177882604u32);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u32::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((4294967294u32 + 0u32) + 1u32 == 4294967294u32 + (0u32 + 1u32));
/// assert!((1u32 + 0u32) + 4294967294u32 == 1u32 + (0u32 + 4294967294u32));
/// assert!((1u32 + 0u32) + 0u32 == 1u32 + (0u32 + 0u32));
/// assert!((0u32 + 0u32) + 4294967295u32 == 0u32 + (0u32 + 4294967295u32));
/// assert!((0u32 + 1u32) + 0u32 == 0u32 + (1u32 + 0u32));
/// assert!((0u32 + 0u32) + 0u32 == 0u32 + (0u32 + 0u32));
/// assert!((500862081u32 + 1803599966u32) + 1953219780u32
///         == 500862081u32 + (1803599966u32 + 1953219780u32));
/// assert!((1367687511u32 + 2547152385u32) + 134579657u32
///         == 1367687511u32 + (2547152385u32 + 134579657u32));
/// assert!((210804204u32 + 502366967u32) + 91835742u32
///         == 210804204u32 + (502366967u32 + 91835742u32));
/// assert!((1218861788u32 + 77200669u32) + 1205971927u32
///         == 1218861788u32 + (77200669u32 + 1205971927u32));
/// # }
/// ```
pub fn core_ops_add_u32_add() {}
/// # Properties for [`u32::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down((x.up() + y.up()) % (u32::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_add(0u32) == 0u32);
/// assert!(1u32.wrapping_add(4294967294u32) == 4294967295u32);
/// assert!(1u32.wrapping_add(0u32) == 1u32);
/// assert!(4294967294u32.wrapping_add(4294967294u32) == 4294967292u32);
/// assert!(4294967294u32.wrapping_add(0u32) == 4294967294u32);
/// assert!(1832894647u32.wrapping_add(1901530449u32) == 3734425096u32);
/// assert!(764771108u32.wrapping_add(1316154218u32) == 2080925326u32);
/// assert!(3680634080u32.wrapping_add(3008191516u32) == 2393858300u32);
/// assert!(2097643535u32.wrapping_add(3627242236u32) == 1429918475u32);
/// assert!(1001152390u32.wrapping_add(3109549883u32) == 4110702273u32);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_add(0u32) == 0u32);
/// assert!(1u32.wrapping_add(0u32) == 1u32);
/// assert!(4294967294u32.wrapping_add(1u32) == 4294967295u32);
/// assert!(0u32.wrapping_add(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.wrapping_add(0u32) == 4294967294u32);
/// assert!(476071040u32.wrapping_add(1364123194u32) == 1840194234u32);
/// assert!(1032670939u32.wrapping_add(1237320039u32) == 2269990978u32);
/// assert!(708371212u32.wrapping_add(3506211784u32) == 4214582996u32);
/// assert!(3135725165u32.wrapping_add(933800361u32) == 4069525526u32);
/// assert!(2473876952u32.wrapping_add(1361011339u32) == 3834888291u32);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down(x.up() + y.up() - u32::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.wrapping_add(4294967295u32) == 4294967293u32);
/// assert!(4294967295u32.wrapping_add(4294967294u32) == 4294967293u32);
/// assert!(1u32.wrapping_add(4294967295u32) == 0u32);
/// assert!(4294967294u32.wrapping_add(4294967294u32) == 4294967292u32);
/// assert!(4294967295u32.wrapping_add(1u32) == 0u32);
/// assert!(4294967295u32.wrapping_add(4294967295u32) == 4294967294u32);
/// assert!(2957379562u32.wrapping_add(1741052521u32) == 403464787u32);
/// assert!(707521867u32.wrapping_add(4141069919u32) == 553624490u32);
/// assert!(3008030473u32.wrapping_add(1956491792u32) == 669554969u32);
/// assert!(1579351767u32.wrapping_add(3674145609u32) == 958530080u32);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.wrapping_add(1u32) == 1u32.wrapping_add(4294967294u32));
/// assert!(4294967295u32.wrapping_add(0u32) == 0u32.wrapping_add(4294967295u32));
/// assert!(0u32.wrapping_add(4294967295u32) == 4294967295u32.wrapping_add(0u32));
/// assert!(4294967294u32.wrapping_add(0u32) == 0u32.wrapping_add(4294967294u32));
/// assert!(1u32.wrapping_add(4294967295u32) == 4294967295u32.wrapping_add(1u32));
/// assert!(3230403307u32.wrapping_add(3513354023u32)
///         == 3513354023u32.wrapping_add(3230403307u32));
/// assert!(374844381u32.wrapping_add(2820812497u32) == 2820812497u32.wrapping_add(374844381u32));
/// assert!(4149312623u32.wrapping_add(94561244u32) == 94561244u32.wrapping_add(4149312623u32));
/// assert!(365125764u32.wrapping_add(2007144080u32) == 2007144080u32.wrapping_add(365125764u32));
/// assert!(1693978471u32.wrapping_add(432925522u32) == 432925522u32.wrapping_add(1693978471u32));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.wrapping_add(0) == 4294967295u32);
/// assert!(1u32.wrapping_add(0) == 1u32);
/// assert!(0u32.wrapping_add(0) == 0u32);
/// assert!(4294967294u32.wrapping_add(0) == 4294967294u32);
/// assert!(3266540472u32.wrapping_add(0) == 3266540472u32);
/// assert!(3160388417u32.wrapping_add(0) == 3160388417u32);
/// assert!(3988257572u32.wrapping_add(0) == 3988257572u32);
/// assert!(3053871370u32.wrapping_add(0) == 3053871370u32);
/// assert!(2989108163u32.wrapping_add(0) == 2989108163u32);
/// assert!(2878522292u32.wrapping_add(0) == 2878522292u32);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: u32 = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(4294967295u32) == 4294967295u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(4294967294u32) == 4294967294u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(0u32) == 0u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(1u32) == 1u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(3849186647u32) == 3849186647u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(3396501398u32) == 3396501398u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(3817751201u32) == 3817751201u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(4094546292u32) == 4094546292u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(3430586882u32) == 3430586882u32
///     });
/// assert!({
///         let zero: u32 = 0;
///         zero.wrapping_add(748822340u32) == 748822340u32
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u32, y : u32, z : u32`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u32::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u32.wrapping_add(4294967294u32)).wrapping_add(1u32)
///         == 0u32.wrapping_add(4294967294u32.wrapping_add(1u32)));
/// assert!((4294967295u32.wrapping_add(0u32)).wrapping_add(0u32)
///         == 4294967295u32.wrapping_add(0u32.wrapping_add(0u32)));
/// assert!((0u32.wrapping_add(0u32)).wrapping_add(0u32)
///         == 0u32.wrapping_add(0u32.wrapping_add(0u32)));
/// assert!((0u32.wrapping_add(1u32)).wrapping_add(0u32)
///         == 0u32.wrapping_add(1u32.wrapping_add(0u32)));
/// assert!((1u32.wrapping_add(4294967294u32)).wrapping_add(0u32)
///         == 1u32.wrapping_add(4294967294u32.wrapping_add(0u32)));
/// assert!((1u32.wrapping_add(0u32)).wrapping_add(1u32)
///         == 1u32.wrapping_add(0u32.wrapping_add(1u32)));
/// assert!((688689882u32.wrapping_add(1707652041u32)).wrapping_add(1236479323u32)
///         == 688689882u32.wrapping_add(1707652041u32.wrapping_add(1236479323u32)));
/// assert!((1316850802u32.wrapping_add(490826500u32)).wrapping_add(1619863470u32)
///         == 1316850802u32.wrapping_add(490826500u32.wrapping_add(1619863470u32)));
/// assert!((1473888480u32.wrapping_add(51943013u32)).wrapping_add(1924892433u32)
///         == 1473888480u32.wrapping_add(51943013u32.wrapping_add(1924892433u32)));
/// assert!((307894063u32.wrapping_add(1351584304u32)).wrapping_add(1293721713u32)
///         == 307894063u32.wrapping_add(1351584304u32.wrapping_add(1293721713u32)));
/// # }
/// ```
pub fn core_ops_add_u32_wrapping_add() {}
/// # Properties for [`u32::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u32::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(4294967294u32) == Some(4294967294u32));
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(1u32.checked_add(0u32) == Some(1u32));
/// assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
/// assert!(257375952u32.checked_add(3997879390u32) == Some(4255255342u32));
/// assert!(1398664160u32.checked_add(2690990501u32) == Some(4089654661u32));
/// assert!(2344022801u32.checked_add(1888591548u32) == Some(4232614349u32));
/// assert!(1164397896u32.checked_add(2892969928u32) == Some(4057367824u32));
/// assert!(1063132814u32.checked_add(2910795984u32) == Some(3973928798u32));
/// assert!(2628993967u32.checked_add(516258013u32) == Some(3145251980u32));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32.checked_add(4294967294u32) == None);
/// assert!(4294967294u32.checked_add(4294967295u32) == None);
/// assert!(1u32.checked_add(4294967295u32) == None);
/// assert!(4294967295u32.checked_add(4294967295u32) == None);
/// assert!(4294967295u32.checked_add(1u32) == None);
/// assert!(4294967294u32.checked_add(4294967294u32) == None);
/// assert!(3666517968u32.checked_add(1979692562u32) == None);
/// assert!(3084715550u32.checked_add(2506991265u32) == None);
/// assert!(919309114u32.checked_add(3492628288u32) == None);
/// assert!(2844322279u32.checked_add(2137008858u32) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_add(4294967294u32) == 4294967294u32.checked_add(4294967294u32));
/// assert!(1u32.checked_add(0u32) == 0u32.checked_add(1u32));
/// assert!(0u32.checked_add(0u32) == 0u32.checked_add(0u32));
/// assert!(4294967295u32.checked_add(0u32) == 0u32.checked_add(4294967295u32));
/// assert!(4294967294u32.checked_add(0u32) == 0u32.checked_add(4294967294u32));
/// assert!(4294967294u32.checked_add(1u32) == 1u32.checked_add(4294967294u32));
/// assert!(137050237u32.checked_add(149487836u32) == 149487836u32.checked_add(137050237u32));
/// assert!(3411158154u32.checked_add(4009517711u32) == 4009517711u32.checked_add(3411158154u32));
/// assert!(1569064231u32.checked_add(924925773u32) == 924925773u32.checked_add(1569064231u32));
/// assert!(3301161257u32.checked_add(1973255712u32) == 1973255712u32.checked_add(3301161257u32));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u32) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_add(0u32) == Some(4294967294u32));
/// assert!(1u32.checked_add(0u32) == Some(1u32));
/// assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(2603264096u32.checked_add(0u32) == Some(2603264096u32));
/// assert!(2579472490u32.checked_add(0u32) == Some(2579472490u32));
/// assert!(775776935u32.checked_add(0u32) == Some(775776935u32));
/// assert!(2838619938u32.checked_add(0u32) == Some(2838619938u32));
/// assert!(904637167u32.checked_add(0u32) == Some(904637167u32));
/// assert!(1426968805u32.checked_add(0u32) == Some(1426968805u32));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u32.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(1u32) == Some(1u32));
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(0u32.checked_add(4294967294u32) == Some(4294967294u32));
/// assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
/// assert!(0u32.checked_add(262879798u32) == Some(262879798u32));
/// assert!(0u32.checked_add(344899395u32) == Some(344899395u32));
/// assert!(0u32.checked_add(353001501u32) == Some(353001501u32));
/// assert!(0u32.checked_add(107692207u32) == Some(107692207u32));
/// assert!(0u32.checked_add(165319706u32) == Some(165319706u32));
/// assert!(0u32.checked_add(1898263596u32) == Some(1898263596u32));
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(4294967295u32).and_then(|iv| iv.checked_add(4294967295u32))
///         == 4294967295u32.checked_add(4294967295u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(0u32.checked_add(4294967294u32).and_then(|iv| iv.checked_add(4294967294u32))
///         == 4294967294u32.checked_add(4294967294u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(1u32.checked_add(4294967295u32).and_then(|iv| iv.checked_add(0u32))
///         == 4294967295u32.checked_add(0u32).and_then(|iv| 1u32.checked_add(iv)));
/// assert!(4294967294u32.checked_add(0u32).and_then(|iv| iv.checked_add(0u32))
///         == 0u32.checked_add(0u32).and_then(|iv| 4294967294u32.checked_add(iv)));
/// assert!(0u32.checked_add(0u32).and_then(|iv| iv.checked_add(4294967295u32))
///         == 0u32.checked_add(4294967295u32).and_then(|iv| 0u32.checked_add(iv)));
/// assert!(4294967295u32.checked_add(1u32).and_then(|iv| iv.checked_add(4294967294u32))
///         == 1u32.checked_add(4294967294u32).and_then(|iv| 4294967295u32.checked_add(iv)));
/// assert!(3732905487u32.checked_add(2755779625u32).and_then(|iv| iv.checked_add(3358094072u32))
///         == 2755779625u32
///             .checked_add(3358094072u32)
///             .and_then(|iv| 3732905487u32.checked_add(iv)));
/// assert!(3391689185u32.checked_add(2556167309u32).and_then(|iv| iv.checked_add(141157205u32))
///         == 2556167309u32
///             .checked_add(141157205u32)
///             .and_then(|iv| 3391689185u32.checked_add(iv)));
/// assert!(694700122u32.checked_add(2916154247u32).and_then(|iv| iv.checked_add(1439383947u32))
///         == 2916154247u32
///             .checked_add(1439383947u32)
///             .and_then(|iv| 694700122u32.checked_add(iv)));
/// assert!(1366529711u32.checked_add(639090145u32).and_then(|iv| iv.checked_add(3201752294u32))
///         == 639090145u32
///             .checked_add(3201752294u32)
///             .and_then(|iv| 1366529711u32.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u32_checked_add() {}
/// # Properties for [`u32::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u32::down((x.up() + y.up()).min(u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_add(0u32) == 0u32);
/// assert!(0u32.saturating_add(4294967294u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_add(4294967294u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_add(1u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_add(0u32) == 4294967295u32);
/// assert!(2500021599u32.saturating_add(154901749u32) == 2654923348u32);
/// assert!(2033953139u32.saturating_add(146324939u32) == 2180278078u32);
/// assert!(884328852u32.saturating_add(3860779253u32) == 4294967295u32);
/// assert!(240792719u32.saturating_add(1079807927u32) == 1320600646u32);
/// assert!(3477086111u32.saturating_add(2864816605u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_add(0u32) == 0u32);
/// assert!(4294967295u32.saturating_add(0u32) == 4294967295u32);
/// assert!(0u32.saturating_add(1u32) == 1u32);
/// assert!(0u32.saturating_add(4294967295u32) == 4294967295u32);
/// assert!(539986449u32.saturating_add(425199150u32) == 965185599u32);
/// assert!(774168557u32.saturating_add(1975089100u32) == 2749257657u32);
/// assert!(3619607385u32.saturating_add(57107631u32) == 3676715016u32);
/// assert!(472323026u32.saturating_add(1247166866u32) == 1719489892u32);
/// assert!(487768845u32.saturating_add(636884899u32) == 1124653744u32);
/// assert!(201679835u32.saturating_add(2576799215u32) == 2778479050u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u32::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(4294967294u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(4294967294u32.saturating_add(4294967294u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(1u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(4294967294u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(573177135u32.saturating_add(4224186891u32) == u32::MAX);
/// assert!(4039982496u32.saturating_add(1161182320u32) == u32::MAX);
/// assert!(1232440438u32.saturating_add(3753168164u32) == u32::MAX);
/// assert!(646401892u32.saturating_add(4099875587u32) == u32::MAX);
/// # }
/// ```
pub fn core_ops_add_u32_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u64>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u64::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 % 1u64 == 0u64);
/// assert!(1u64 % 18446744073709551615u64 == 1u64);
/// assert!(1u64 % 18446744073709551614u64 == 1u64);
/// assert!(18446744073709551615u64 % 18446744073709551615u64 == 0u64);
/// assert!(0u64 % 1u64 == 0u64);
/// assert!(0u64 % 18446744073709551615u64 == 0u64);
/// assert!(6605062259421180841u64 % 11668984884631342599u64 == 6605062259421180841u64);
/// assert!(9637159464222857519u64 % 8446916546920658919u64 == 1190242917302198600u64);
/// assert!(7862704447262183899u64 % 9405124160754743063u64 == 7862704447262183899u64);
/// assert!(13610817638816406432u64 % 14286680845057393865u64 == 13610817638816406432u64);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u64_rem() {}
/// # Properties for [`u64::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u64::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_rem(18446744073709551615u64) == Some(0u64));
/// assert!(0u64.checked_rem(1u64) == Some(0u64));
/// assert!(0u64.checked_rem(18446744073709551615u64) == Some(0u64));
/// assert!(18446744073709551615u64.checked_rem(1u64) == Some(0u64));
/// assert!(0u64.checked_rem(18446744073709551614u64) == Some(0u64));
/// assert!(1u64.checked_rem(18446744073709551615u64) == Some(1u64));
/// assert!(15190343420006766453u64.checked_rem(3463313369277229636u64)
///         == Some(1337089942897847909u64));
/// assert!(1951620070264938349u64.checked_rem(15077326295837186647u64)
///         == Some(1951620070264938349u64));
/// assert!(36461994823924471u64.checked_rem(11870157710381094389u64)
///         == Some(36461994823924471u64));
/// assert!(12505486218545677295u64.checked_rem(16769084475771579103u64)
///         == Some(12505486218545677295u64));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u64.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u64_checked_rem() {}
/// # Properties for [`u64::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `x == u64::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u64::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_neg() == Some(0u64));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `x != u64::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.checked_neg() == None);
/// assert!(1u64.checked_neg() == None);
/// assert!(18446744073709551615u64.checked_neg() == None);
/// assert!(16683082411338753952u64.checked_neg() == None);
/// assert!(6122189179001505382u64.checked_neg() == None);
/// assert!(12360905567502355554u64.checked_neg() == None);
/// assert!(6261856138010133106u64.checked_neg() == None);
/// assert!(5156234697279058432u64.checked_neg() == None);
/// assert!(9689009548797482521u64.checked_neg() == None);
/// assert!(6785577388020846196u64.checked_neg() == None);
/// # }
/// ```
pub fn t_u64_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u64>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x << y == u64::down((x.up() << y) & u64::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 << 0u32 == 18446744073709551614u64);
/// assert!(18446744073709551615u64 << 1u32 == 18446744073709551614u64);
/// assert!(0u64 << 0u32 == 0u64);
/// assert!(18446744073709551614u64 << 1u32 == 18446744073709551612u64);
/// assert!(0u64 << 1u32 == 0u64);
/// assert!(8738353799319697514u64 << 45u32 == 6272740224741146624u64);
/// assert!(632959753653784717u64 << 19u32 == 14724181674357686272u64);
/// assert!(14258402711588898105u64 << 44u32 == 10399814294203006976u64);
/// assert!(12595626253484096362u64 << 2u32 == 13489016866517282216u64);
/// assert!(2566288453914814737u64 << 41u32 == 10291325481890349056u64);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614u64 << 4294967295u32));
/// assert!(panics!(18446744073709551615u64 << 4294967294u32));
/// assert!(panics!(18446744073709551614u64 << 4294967294u32));
/// assert!(panics!(18446744073709551615u64 << 4294967295u32));
/// assert!(panics!(0u64 << 4294967295u32));
/// assert!(panics!(1u64 << 4294967294u32));
/// assert!(panics!(17354080255059594001u64 << 104u32));
/// assert!(panics!(6053363572481102430u64 << 124u32));
/// assert!(panics!(2491480549319532214u64 << 84u32));
/// assert!(panics!(13197848864517817284u64 << 115u32));
/// # }
/// ```
pub fn core_ops_shl_u64_shl() {}
/// # Properties for [`core::ops::Shl::<u64>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u64::down((x.up() << y) & u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_shl(0u32) == Some(18446744073709551615u64));
/// assert!(0u64.checked_shl(0u32) == Some(0u64));
/// assert!(1u64.checked_shl(1u32) == Some(2u64));
/// assert!(9172004766024843657u64.checked_shl(52u32) == Some(1769914653556604928u64));
/// assert!(8784818660255101579u64.checked_shl(10u32) == Some(12089944204672379904u64));
/// assert!(16193156072920674129u64.checked_shl(56u32) == Some(5836665117072162816u64));
/// assert!(11179006074202491295u64.checked_shl(49u32) == Some(1386545735276691456u64));
/// assert!(7507153642203917906u64.checked_shl(15u32) == Some(7078324821111144448u64));
/// assert!(6067521332611600048u64.checked_shl(29u32) == Some(2895086503313014784u64));
/// assert!(6999533969929822147u64.checked_shl(46u32) == Some(10948461800370208768u64));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shl(4294967295u32) == None);
/// assert!(0u64.checked_shl(4294967294u32) == None);
/// assert!(1u64.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551614u64.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551614u64.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551615u64.checked_shl(4294967294u32) == None);
/// assert!(2134922103306596607u64.checked_shl(126u32) == None);
/// assert!(1018690163849765347u64.checked_shl(82u32) == None);
/// assert!(12541306122617361455u64.checked_shl(83u32) == None);
/// assert!(16580926637602726494u64.checked_shl(106u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u64_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u64>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u64::down((x.up() << y) & u64::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.overflowing_shl(0u32) == (1u64, false));
/// assert!(0u64.overflowing_shl(0u32) == (0u64, false));
/// assert!(18446744073709551614u64.overflowing_shl(0u32) == (18446744073709551614u64, false));
/// assert!(18446744073709551615u64.overflowing_shl(0u32) == (18446744073709551615u64, false));
/// assert!(0u64.overflowing_shl(1u32) == (0u64, false));
/// assert!(3098733454864372414u64.overflowing_shl(9u32) == (131538551537236992u64, false));
/// assert!(14460678480712260888u64.overflowing_shl(28u32) == (7304792216390598656u64, false));
/// assert!(10558894745132867133u64.overflowing_shl(42u32) == (9833878067200655360u64, false));
/// assert!(7833964441077951494u64.overflowing_shl(45u32) == (14231585928723300352u64, false));
/// assert!(3473313271279598036u64.overflowing_shl(36u32) == (311765697422163968u64, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u64::down((x.up() << (y & (u64::BITS - 1)) & u64::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.overflowing_shl(4294967294u32) == (4611686018427387904u64, true));
/// assert!(18446744073709551614u64.overflowing_shl(4294967294u32)
///         == (9223372036854775808u64, true));
/// assert!(0u64.overflowing_shl(4294967294u32) == (0u64, true));
/// assert!(18446744073709551615u64.overflowing_shl(4294967295u32)
///         == (9223372036854775808u64, true));
/// assert!(18446744073709551615u64.overflowing_shl(4294967294u32)
///         == (13835058055282163712u64, true));
/// assert!(838298046987069293u64.overflowing_shl(138u32) == (9866972724119581696u64, true));
/// assert!(1355999517917055243u64.overflowing_shl(112u32) == (651614571085168640u64, true));
/// assert!(10262026758862594365u64.overflowing_shl(70u32) == (11133669987371732800u64, true));
/// assert!(15179776357827497459u64.overflowing_shl(86u32) == (10014470186981130240u64, true));
/// assert!(11328085336078882015u64.overflowing_shl(82u32) == (14294611225118244864u64, true));
/// # }
/// ```
pub fn core_ops_shl_u64_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u64>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x >> y == u64::down((x.up() >> y) & u64::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 >> 0u32 == 0u64);
/// assert!(18446744073709551614u64 >> 1u32 == 9223372036854775807u64);
/// assert!(1u64 >> 1u32 == 0u64);
/// assert!(18446744073709551615u64 >> 0u32 == 18446744073709551615u64);
/// assert!(0u64 >> 1u32 == 0u64);
/// assert!(7105741433455766064u64 >> 60u32 == 6u64);
/// assert!(13319142969868155357u64 >> 46u32 == 189276u64);
/// assert!(9917080515611409484u64 >> 45u32 == 281860u64);
/// assert!(18326611224992910540u64 >> 32u32 == 4266996687u64);
/// assert!(11309045097783706702u64 >> 8u32 == 44175957413217604u64);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551615u64 >> 4294967294u32));
/// assert!(panics!(0u64 >> 4294967295u32));
/// assert!(panics!(1u64 >> 4294967295u32));
/// assert!(panics!(0u64 >> 4294967294u32));
/// assert!(panics!(18446744073709551614u64 >> 4294967294u32));
/// assert!(panics!(15826692330144860098u64 >> 111u32));
/// assert!(panics!(5544544584979774279u64 >> 67u32));
/// assert!(panics!(7602871773818363156u64 >> 116u32));
/// assert!(panics!(7052053841008407904u64 >> 70u32));
/// assert!(panics!(3135062994967194975u64 >> 130u32));
/// # }
/// ```
pub fn core_ops_shr_u64_shr() {}
/// # Properties for [`core::ops::Shr::<u64>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u64::down((x.up() >> y) & u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shr(0u32) == Some(0u64));
/// assert!(18446744073709551614u64.checked_shr(1u32) == Some(9223372036854775807u64));
/// assert!(18446744073709551615u64.checked_shr(0u32) == Some(18446744073709551615u64));
/// assert!(18446744073709551614u64.checked_shr(0u32) == Some(18446744073709551614u64));
/// assert!(1u64.checked_shr(1u32) == Some(0u64));
/// assert!(38061546562383833u64.checked_shr(12u32) == Some(9292369766206u64));
/// assert!(18371228666753401850u64.checked_shr(33u32) == Some(2138692497u64));
/// assert!(1813594846032231646u64.checked_shr(19u32) == Some(3459157650055u64));
/// assert!(16952779331293104957u64.checked_shr(4u32) == Some(1059548708205819059u64));
/// assert!(2529010410859437904u64.checked_shr(46u32) == Some(35939u64));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.checked_shr(4294967294u32) == None);
/// assert!(1u64.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551615u64.checked_shr(4294967295u32) == None);
/// assert!(1u64.checked_shr(4294967294u32) == None);
/// assert!(0u64.checked_shr(4294967295u32) == None);
/// assert!(0u64.checked_shr(4294967294u32) == None);
/// assert!(9992873185973646651u64.checked_shr(99u32) == None);
/// assert!(4870982161663361654u64.checked_shr(86u32) == None);
/// assert!(3563915707750590226u64.checked_shr(76u32) == None);
/// assert!(15024007330211978869u64.checked_shr(72u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u64_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u64>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u64::down((x.up() >> y) & u64::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shr(0u32) == (0u64, false));
/// assert!(18446744073709551615u64.overflowing_shr(0u32) == (18446744073709551615u64, false));
/// assert!(0u64.overflowing_shr(1u32) == (0u64, false));
/// assert!(1u64.overflowing_shr(0u32) == (1u64, false));
/// assert!(18446744073709551614u64.overflowing_shr(0u32) == (18446744073709551614u64, false));
/// assert!(16907730779960917899u64.overflowing_shr(17u32) == (128995748748481u64, false));
/// assert!(7623892664389710042u64.overflowing_shr(60u32) == (6u64, false));
/// assert!(16308862480872621132u64.overflowing_shr(24u32) == (972083954863u64, false));
/// assert!(12073889344239489827u64.overflowing_shr(16u32) == (184232930667716u64, false));
/// assert!(13426415685757775489u64.overflowing_shr(17u32) == (102435422407209u64, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u64::down((x.up() >> (y & (u64::BITS - 1)) & u64::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shr(4294967295u32) == (0u64, true));
/// assert!(18446744073709551615u64.overflowing_shr(4294967295u32) == (1u64, true));
/// assert!(0u64.overflowing_shr(4294967294u32) == (0u64, true));
/// assert!(18446744073709551614u64.overflowing_shr(4294967294u32) == (3u64, true));
/// assert!(18446744073709551614u64.overflowing_shr(4294967295u32) == (1u64, true));
/// assert!(1u64.overflowing_shr(4294967294u32) == (0u64, true));
/// assert!(4838071146703175084u64.overflowing_shr(130u32) == (1209517786675793771u64, true));
/// assert!(16303349605268409051u64.overflowing_shr(112u32) == (57921u64, true));
/// assert!(2404042137564696797u64.overflowing_shr(110u32) == (34163u64, true));
/// assert!(3301833564936129079u64.overflowing_shr(73u32) == (6448893681515877u64, true));
/// # }
/// ```
pub fn core_ops_shr_u64_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u64>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u64::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64 / 1u64 == 1u64);
/// assert!(0u64 / 18446744073709551615u64 == 0u64);
/// assert!(18446744073709551614u64 / 18446744073709551615u64 == 0u64);
/// assert!(18446744073709551615u64 / 18446744073709551615u64 == 1u64);
/// assert!(0u64 / 18446744073709551614u64 == 0u64);
/// assert!(1u64 / 18446744073709551615u64 == 0u64);
/// assert!(2482754376u64 / 1176688392u64 == 2u64);
/// assert!(4091341645u64 / 2622385256u64 == 1u64);
/// assert!(3396584924u64 / 4055095382u64 == 0u64);
/// assert!(1972751752u64 / 3117761488u64 == 0u64);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(1u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551614u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551615u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4134595096586999757u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(11601631929676441577u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18292638891768135550u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(11224464402441731037u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(9318127153531994564u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1293189655105820760u64 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u64_div() {}
/// # Properties for [`u64::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u64::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.saturating_div(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_div(18446744073709551615u64) == 1u64);
/// assert!(1u64.saturating_div(18446744073709551614u64) == 0u64);
/// assert!(0u64.saturating_div(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_div(18446744073709551615u64) == 0u64);
/// assert!(1u64.saturating_div(1u64) == 1u64);
/// assert!(2465494577u64.saturating_div(1924874541u64) == 1u64);
/// assert!(3802467945u64.saturating_div(3752055909u64) == 1u64);
/// assert!(2877279402u64.saturating_div(2435490582u64) == 1u64);
/// assert!(2983893324u64.saturating_div(3728223223u64) == 0u64);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551615u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551614u64.saturating_div(0)) }
///     });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u64.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(13644659173782331282u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(5862169616604515465u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(10239065070017805580u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(3130757434371134590u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(11819998928905745613u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(9627869943006624476u64.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_u64_saturating_div() {}
/// # Properties for [`u64::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u64::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.checked_div(1u64) == Some(1u64));
/// assert!(18446744073709551614u64.checked_div(1u64) == Some(18446744073709551614u64));
/// assert!(1u64.checked_div(18446744073709551614u64) == Some(0u64));
/// assert!(0u64.checked_div(18446744073709551615u64) == Some(0u64));
/// assert!(18446744073709551615u64.checked_div(18446744073709551614u64) == Some(1u64));
/// assert!(0u64.checked_div(18446744073709551614u64) == Some(0u64));
/// assert!(4042743886u64.checked_div(3653652508u64) == Some(1u64));
/// assert!(3126412288u64.checked_div(2602011104u64) == Some(1u64));
/// assert!(2509481911u64.checked_div(2633435363u64) == Some(0u64));
/// assert!(1180847429u64.checked_div(2793705119u64) == Some(0u64));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_div(0) == None);
/// assert!(1u64.checked_div(0) == None);
/// assert!(18446744073709551615u64.checked_div(0) == None);
/// assert!(18446744073709551614u64.checked_div(0) == None);
/// assert!(17274604204095468026u64.checked_div(0) == None);
/// assert!(9039211185023761308u64.checked_div(0) == None);
/// assert!(7626720782654101530u64.checked_div(0) == None);
/// assert!(11932387654212031665u64.checked_div(0) == None);
/// assert!(8649932920905241101u64.checked_div(0) == None);
/// assert!(2163071089442588900u64.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u64_checked_div() {}
/// # Properties for [`core::ops::Mul::<u64>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x * y == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 * 0u64 == 0u64);
/// assert!(18446744073709551615u64 * 1u64 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 * 0u64 == 0u64);
/// assert!(18446744073709551614u64 * 0u64 == 0u64);
/// assert!(1u64 * 0u64 == 0u64);
/// assert!(0u64 * 1u64 == 0u64);
/// assert!(663890992u64 * 4039637301u64 == 2681878815081092592u64);
/// assert!(490858245u64 * 4171771162u64 == 2047748271120930690u64);
/// assert!(3560110470u64 * 3491567587u64 == 12430366323191335890u64);
/// assert!(3899307381u64 * 2380378440u64 == 9281827220665265640u64);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551615u64 * 18446744073709551615u64));
/// assert!(panics!(18446744073709551614u64 * 18446744073709551615u64));
/// assert!(panics!(18446744073709551615u64 * 18446744073709551614u64));
/// assert!(panics!(18446744073709551614u64 * 18446744073709551614u64));
/// assert!(panics!(13999986523808689660u64 * 12731695380579282655u64));
/// assert!(panics!(17583469659304009273u64 * 16138584994487611912u64));
/// assert!(panics!(5763444787381973989u64 * 11704151251681043821u64));
/// assert!(panics!(17173984286543770190u64 * 12543396615237846531u64));
/// assert!(panics!(4313334468710694041u64 * 18338432681460591127u64));
/// assert!(panics!(14555841582932360690u64 * 13366298731843405121u64));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 * 1 == 18446744073709551614u64);
/// assert!(0u64 * 1 == 0u64);
/// assert!(18446744073709551615u64 * 1 == 18446744073709551615u64);
/// assert!(1u64 * 1 == 1u64);
/// assert!(3229875330827508693u64 * 1 == 3229875330827508693u64);
/// assert!(8102764324903568547u64 * 1 == 8102764324903568547u64);
/// assert!(9020718907688007314u64 * 1 == 9020718907688007314u64);
/// assert!(1909002084953141876u64 * 1 == 1909002084953141876u64);
/// assert!(1589856624102238787u64 * 1 == 1589856624102238787u64);
/// assert!(12535209937413623360u64 * 1 == 12535209937413623360u64);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u64 == 0u64);
/// assert!(1 * 1u64 == 1u64);
/// assert!(1 * 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(1 * 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1 * 18254891255173967758u64 == 18254891255173967758u64);
/// assert!(1 * 8110400845224346977u64 == 8110400845224346977u64);
/// assert!(1 * 6437624299466491987u64 == 6437624299466491987u64);
/// assert!(1 * 2748032206488775868u64 == 2748032206488775868u64);
/// assert!(1 * 9821025558858467443u64 == 9821025558858467443u64);
/// assert!(1 * 1645986951029471059u64 == 1645986951029471059u64);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 * 1u64 == 1u64 * 18446744073709551614u64);
/// assert!(0u64 * 0u64 == 0u64 * 0u64);
/// assert!(1u64 * 0u64 == 0u64 * 1u64);
/// assert!(18446744073709551614u64 * 0u64 == 0u64 * 18446744073709551614u64);
/// assert!(1563821371u64 * 541442647u64 == 541442647u64 * 1563821371u64);
/// assert!(2956454493u64 * 3263019485u64 == 3263019485u64 * 2956454493u64);
/// assert!(3410952020u64 * 2955488029u64 == 2955488029u64 * 3410952020u64);
/// assert!(3106501035u64 * 3984016318u64 == 3984016318u64 * 3106501035u64);
/// assert!(2504487468u64 * 4080487662u64 == 4080487662u64 * 2504487468u64);
/// assert!(2406746432u64 * 3892449129u64 == 3892449129u64 * 2406746432u64);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u64::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u64 * 1u64) * 1u64 == 1u64 * (1u64 * 1u64));
/// assert!((1u64 * 1u64) * 18446744073709551615u64 == 1u64 * (1u64 * 18446744073709551615u64));
/// assert!((18446744073709551614u64 * 1u64) * 1u64 == 18446744073709551614u64 * (1u64 * 1u64));
/// assert!((1u64 * 1u64) * 18446744073709551614u64 == 1u64 * (1u64 * 18446744073709551614u64));
/// assert!((18446744073709551615u64 * 1u64) * 1u64 == 18446744073709551615u64 * (1u64 * 1u64));
/// assert!((1u64 * 18446744073709551614u64) * 1u64 == 1u64 * (18446744073709551614u64 * 1u64));
/// assert!((2434643u64 * 1812721u64) * 2070235u64 == 2434643u64 * (1812721u64 * 2070235u64));
/// assert!((1149261u64 * 2376011u64) * 2250082u64 == 1149261u64 * (2376011u64 * 2250082u64));
/// assert!((1304571u64 * 2461545u64) * 1968080u64 == 1304571u64 * (2461545u64 * 1968080u64));
/// assert!((1334173u64 * 2299175u64) * 2515824u64 == 1334173u64 * (2299175u64 * 2515824u64));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u64::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 * (0u64 + 1u64)
///         == 18446744073709551614u64 * 0u64 + 18446744073709551614u64 * 1u64);
/// assert!(1u64 * (18446744073709551614u64 + 0u64)
///         == 1u64 * 18446744073709551614u64 + 1u64 * 0u64);
/// assert!(1u64 * (0u64 + 1u64) == 1u64 * 0u64 + 1u64 * 1u64);
/// assert!(18446744073709551615u64 * (0u64 + 0u64)
///         == 18446744073709551615u64 * 0u64 + 18446744073709551615u64 * 0u64);
/// assert!(18446744073709551615u64 * (1u64 + 0u64)
///         == 18446744073709551615u64 * 1u64 + 18446744073709551615u64 * 0u64);
/// assert!(18446744073709551614u64 * (1u64 + 0u64)
///         == 18446744073709551614u64 * 1u64 + 18446744073709551614u64 * 0u64);
/// assert!(2706834421u64 * (965209301u64 + 3196886143u64)
///         == 2706834421u64 * 965209301u64 + 2706834421u64 * 3196886143u64);
/// assert!(2053918105u64 * (4084731389u64 + 2145833086u64)
///         == 2053918105u64 * 4084731389u64 + 2053918105u64 * 2145833086u64);
/// assert!(3488446670u64 * (388514434u64 + 3557184670u64)
///         == 3488446670u64 * 388514434u64 + 3488446670u64 * 3557184670u64);
/// assert!(1187689377u64 * (4079007318u64 + 2994852508u64)
///         == 1187689377u64 * 4079007318u64 + 1187689377u64 * 2994852508u64);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64 * 0 == 0);
/// assert!(18446744073709551614u64 * 0 == 0);
/// assert!(0u64 * 0 == 0);
/// assert!(18446744073709551615u64 * 0 == 0);
/// assert!(4197531702u64 * 0 == 0);
/// assert!(2197655261u64 * 0 == 0);
/// assert!(1715585220u64 * 0 == 0);
/// assert!(3130765367u64 * 0 == 0);
/// assert!(3948718699u64 * 0 == 0);
/// assert!(2290974021u64 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u64_mul() {}
/// # Properties for [`u64::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u64::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.checked_mul(0u64) == Some(0u64));
/// assert!(0u64.checked_mul(0u64) == Some(0u64));
/// assert!(1u64.checked_mul(0u64) == Some(0u64));
/// assert!(0u64.checked_mul(1u64) == Some(0u64));
/// assert!(2772667960u64.checked_mul(3038831766u64) == Some(8425671473418417360u64));
/// assert!(2922799805u64.checked_mul(2661362099u64) == Some(7778628623991590695u64));
/// assert!(3252092285u64.checked_mul(2915431416u64) == Some(9481252015420225560u64));
/// assert!(4263028521u64.checked_mul(3088366369u64) == Some(13165793914344210249u64));
/// assert!(3397139553u64.checked_mul(3159180565u64) == Some(10732177252430387445u64));
/// assert!(4226813207u64.checked_mul(1268731711u64) == Some(5362691952194507177u64));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_mul(18446744073709551615u64) == None);
/// assert!(18446744073709551614u64.checked_mul(18446744073709551614u64) == None);
/// assert!(18446744073709551614u64.checked_mul(18446744073709551615u64) == None);
/// assert!(18446744073709551615u64.checked_mul(18446744073709551614u64) == None);
/// assert!(16843568249989904632u64.checked_mul(17053849066341864768u64) == None);
/// assert!(4123785003990819510u64.checked_mul(16545998502818670735u64) == None);
/// assert!(2957910271988761164u64.checked_mul(2891439095517091191u64) == None);
/// assert!(4848872187131095004u64.checked_mul(3880494857293296573u64) == None);
/// assert!(8633899776830629661u64.checked_mul(4385068179316592298u64) == None);
/// assert!(13920242624989258039u64.checked_mul(12829211060795776962u64) == None);
/// # }
/// ```
pub fn core_ops_mul_u64_checked_mul() {}
/// # Properties for [`u64::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u64::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(0u64.overflowing_mul(1u64) == (0u64, false));
/// assert!(18446744073709551614u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(1u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(2538077348u64.overflowing_mul(4014410130u64) == (10188883416534735240u64, false));
/// assert!(2240905198u64.overflowing_mul(3641599270u64) == (8160478733176005460u64, false));
/// assert!(2303529384u64.overflowing_mul(3872446305u64) == (8920293851529726120u64, false));
/// assert!(1379413580u64.overflowing_mul(2135950012u64) == (2946358452753962960u64, false));
/// assert!(1914629968u64.overflowing_mul(3401828273u64) == (6513242357475485264u64, false));
/// assert!(3892712532u64.overflowing_mul(2946454076u64) == (11469698706607680432u64, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.overflowing_mul(18446744073709551614u64) == (4u64, true));
/// assert!(18446744073709551614u64.overflowing_mul(18446744073709551615u64) == (2u64, true));
/// assert!(18446744073709551615u64.overflowing_mul(18446744073709551615u64) == (1u64, true));
/// assert!(18446744073709551615u64.overflowing_mul(18446744073709551614u64) == (2u64, true));
/// assert!(1303655627196217444u64.overflowing_mul(13549311689848883103u64)
///         == (17609718009402475036u64, true));
/// assert!(6725832618175000799u64.overflowing_mul(16741300694647574752u64)
///         == (10950610652606542624u64, true));
/// assert!(1395182454678035481u64.overflowing_mul(12203968943236976897u64)
///         == (1694468870133826841u64, true));
/// assert!(7697399895972380707u64.overflowing_mul(8919293145244213227u64)
///         == (506198501910568225u64, true));
/// assert!(15323309810660617105u64.overflowing_mul(14323623050897051334u64)
///         == (1422420803753500710u64, true));
/// assert!(2744253587011671760u64.overflowing_mul(6626066546375654547u64)
///         == (6808874453705283952u64, true));
/// # }
/// ```
pub fn core_ops_mul_u64_overflowing_mul() {}
/// # Properties for [`u64::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::down((x.up() * y.up()).min(u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.saturating_mul(0u64) == 0u64);
/// assert!(1u64.saturating_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(18446744073709551615u64.saturating_mul(18446744073709551615u64)
///         == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_mul(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_mul(0u64) == 0u64);
/// assert!(9881313458577801838u64.saturating_mul(14838910037206159185u64)
///         == 18446744073709551615u64);
/// assert!(10059282949459022608u64.saturating_mul(8891251196185004628u64)
///         == 18446744073709551615u64);
/// assert!(4702321361377098870u64.saturating_mul(14985769649829701040u64)
///         == 18446744073709551615u64);
/// assert!(14747864928272885034u64.saturating_mul(6328669897182828278u64)
///         == 18446744073709551615u64);
/// assert!(16818969505459717763u64.saturating_mul(9980144830089487543u64)
///         == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.saturating_mul(1u64) == 18446744073709551614u64);
/// assert!(1u64.saturating_mul(0u64) == 0u64);
/// assert!(18446744073709551615u64.saturating_mul(1u64) == 18446744073709551615u64);
/// assert!(0u64.saturating_mul(0u64) == 0u64);
/// assert!(0u64.saturating_mul(1u64) == 0u64);
/// assert!(3778189412u64.saturating_mul(4257257646u64) == 16084725762273244152u64);
/// assert!(3952100105u64.saturating_mul(4108430589u64) == 16236928962172111845u64);
/// assert!(2988732223u64.saturating_mul(2658356471u64) == 7945115645098265033u64);
/// assert!(3672704724u64.saturating_mul(3850037726u64) == 14140051743858417624u64);
/// assert!(1267346771u64.saturating_mul(3919766199u64) == 4967703035377593429u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.saturating_mul(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_mul(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_mul(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_mul(18446744073709551614u64) == u64::MAX);
/// assert!(2582987572662752444u64.saturating_mul(16073616028051469841u64) == u64::MAX);
/// assert!(10740000476184664149u64.saturating_mul(16361814829057973189u64) == u64::MAX);
/// assert!(2912313630089168565u64.saturating_mul(13255317576431107019u64) == u64::MAX);
/// assert!(16670610473140884679u64.saturating_mul(13404804965088425717u64) == u64::MAX);
/// assert!(2092626611375284586u64.saturating_mul(9027744048079795410u64) == u64::MAX);
/// assert!(1350360039411521560u64.saturating_mul(8992753560671608265u64) == u64::MAX);
/// # }
/// ```
pub fn core_ops_mul_u64_saturating_mul() {}
/// # Properties for [`u64::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u64::down((x.up() * y.up()) % (u64::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.wrapping_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.wrapping_mul(1u64) == 18446744073709551614u64);
/// assert!(0u64.wrapping_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_mul(0u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551615u64) == 0u64);
/// assert!(5432415670996629373u64.wrapping_mul(16592920368332096913u64)
///         == 6225195289057156813u64);
/// assert!(2518857302628044116u64.wrapping_mul(10025297938972593643u64)
///         == 17338698809363496988u64);
/// assert!(4280720747492907308u64.wrapping_mul(4224406619351706851u64)
///         == 14332598317284155908u64);
/// assert!(11721522810526901407u64.wrapping_mul(16358703112234022177u64)
///         == 18338709598632452991u64);
/// assert!(2875970237586455994u64.wrapping_mul(7979239098626718637u64)
///         == 17984300689677620402u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_mul(0u64) == 0u64);
/// assert!(1u64.wrapping_mul(0u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_mul(0u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551615u64) == 0u64);
/// assert!(3008798543u64.wrapping_mul(1280385807u64) == 3852422950579479201u64);
/// assert!(3906564458u64.wrapping_mul(2870777389u64) == 11214876914697440162u64);
/// assert!(3376965693u64.wrapping_mul(3795835397u64) == 12818405911944035121u64);
/// assert!(4171309579u64.wrapping_mul(3468892337u64) == 14469823833847796123u64);
/// assert!(4238075397u64.wrapping_mul(2403107952u64) == 10184552687706256944u64);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 * 1 == 18446744073709551614u64);
/// assert!(1u64 * 1 == 1u64);
/// assert!(0u64 * 1 == 0u64);
/// assert!(18446744073709551615u64 * 1 == 18446744073709551615u64);
/// assert!(9871709417107520792u64 * 1 == 9871709417107520792u64);
/// assert!(16900067095863550908u64 * 1 == 16900067095863550908u64);
/// assert!(8708711698678139603u64 * 1 == 8708711698678139603u64);
/// assert!(1993149073709005083u64 * 1 == 1993149073709005083u64);
/// assert!(16337387895370689926u64 * 1 == 16337387895370689926u64);
/// assert!(1028729621659703716u64 * 1 == 1028729621659703716u64);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u64 == 0u64);
/// assert!(1 * 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(1 * 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1 * 1u64 == 1u64);
/// assert!(1 * 18125173672522540640u64 == 18125173672522540640u64);
/// assert!(1 * 8540098176560981029u64 == 8540098176560981029u64);
/// assert!(1 * 16829445294061754482u64 == 16829445294061754482u64);
/// assert!(1 * 10387740702649265310u64 == 10387740702649265310u64);
/// assert!(1 * 9548490876106208945u64 == 9548490876106208945u64);
/// assert!(1 * 12970950056956674108u64 == 12970950056956674108u64);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 * 0u64 == 0u64 * 0u64);
/// assert!(1u64 * 0u64 == 0u64 * 1u64);
/// assert!(1u64 * 1u64 == 1u64 * 1u64);
/// assert!(18446744073709551614u64 * 1u64 == 1u64 * 18446744073709551614u64);
/// assert!(0u64 * 18446744073709551614u64 == 18446744073709551614u64 * 0u64);
/// assert!(1815631535u64 * 1375053367u64 == 1375053367u64 * 1815631535u64);
/// assert!(3108607828u64 * 3785922603u64 == 3785922603u64 * 3108607828u64);
/// assert!(1750722096u64 * 2050976068u64 == 2050976068u64 * 1750722096u64);
/// assert!(1942965036u64 * 3271277844u64 == 3271277844u64 * 1942965036u64);
/// assert!(4110886489u64 * 4024359082u64 == 4024359082u64 * 4110886489u64);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u64::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u64 * 1u64) * 18446744073709551615u64 == 1u64 * (1u64 * 18446744073709551615u64));
/// assert!((1u64 * 1u64) * 1u64 == 1u64 * (1u64 * 1u64));
/// assert!((1u64 * 1u64) * 18446744073709551614u64 == 1u64 * (1u64 * 18446744073709551614u64));
/// assert!((1u64 * 18446744073709551615u64) * 1u64 == 1u64 * (18446744073709551615u64 * 1u64));
/// assert!((18446744073709551614u64 * 1u64) * 1u64 == 18446744073709551614u64 * (1u64 * 1u64));
/// assert!((1u64 * 18446744073709551614u64) * 1u64 == 1u64 * (18446744073709551614u64 * 1u64));
/// assert!((2621685u64 * 1708912u64) * 2559944u64 == 2621685u64 * (1708912u64 * 2559944u64));
/// assert!((2394746u64 * 2356200u64) * 2248346u64 == 2394746u64 * (2356200u64 * 2248346u64));
/// assert!((1158736u64 * 2299032u64) * 2510409u64 == 1158736u64 * (2299032u64 * 2510409u64));
/// assert!((2505001u64 * 1186775u64) * 1700351u64 == 2505001u64 * (1186775u64 * 1700351u64));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u64::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64 * (0u64 + 18446744073709551614u64)
///         == 1u64 * 0u64 + 1u64 * 18446744073709551614u64);
/// assert!(1u64 * (0u64 + 0u64) == 1u64 * 0u64 + 1u64 * 0u64);
/// assert!(18446744073709551614u64 * (0u64 + 1u64)
///         == 18446744073709551614u64 * 0u64 + 18446744073709551614u64 * 1u64);
/// assert!(1u64 * (18446744073709551615u64 + 0u64)
///         == 1u64 * 18446744073709551615u64 + 1u64 * 0u64);
/// assert!(1u64 * (0u64 + 18446744073709551615u64)
///         == 1u64 * 0u64 + 1u64 * 18446744073709551615u64);
/// assert!(18446744073709551615u64 * (0u64 + 0u64)
///         == 18446744073709551615u64 * 0u64 + 18446744073709551615u64 * 0u64);
/// assert!(2372248043u64 * (3907817072u64 + 2471283988u64)
///         == 2372248043u64 * 3907817072u64 + 2372248043u64 * 2471283988u64);
/// assert!(702656507u64 * (2430633808u64 + 2341885034u64)
///         == 702656507u64 * 2430633808u64 + 702656507u64 * 2341885034u64);
/// assert!(3216368632u64 * (1701147040u64 + 4014550335u64)
///         == 3216368632u64 * 1701147040u64 + 3216368632u64 * 4014550335u64);
/// assert!(4176171699u64 * (408265064u64 + 3114388602u64)
///         == 4176171699u64 * 408265064u64 + 4176171699u64 * 3114388602u64);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64 * 0 == 0);
/// assert!(18446744073709551614u64 * 0 == 0);
/// assert!(1u64 * 0 == 0);
/// assert!(0u64 * 0 == 0);
/// assert!(3214567487u64 * 0 == 0);
/// assert!(3402899611u64 * 0 == 0);
/// assert!(671866966u64 * 0 == 0);
/// assert!(2855920396u64 * 0 == 0);
/// assert!(3238753003u64 * 0 == 0);
/// assert!(1686647959u64 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u64_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u64>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64 - 18446744073709551614u64 == 1u64);
/// assert!(1u64 - 0u64 == 1u64);
/// assert!(0u64 - 0u64 == 0u64);
/// assert!(16131849513219342516u64 - 12484706957574755797u64 == 3647142555644586719u64);
/// assert!(14204333789351579268u64 - 7299821454030283338u64 == 6904512335321295930u64);
/// assert!(13980504168778513267u64 - 13203086805098414649u64 == 777417363680098618u64);
/// assert!(18403733418012459036u64 - 5184095253279955583u64 == 13219638164732503453u64);
/// assert!(9774935175403742736u64 - 2207500854963775592u64 == 7567434320439967144u64);
/// assert!(11485911850768564546u64 - 62909503900050706u64 == 11423002346868513840u64);
/// assert!(7272511738653404307u64 - 3321013058179241314u64 == 3951498680474162993u64);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614u64 - 18446744073709551615u64));
/// assert!(panics!(0u64 - 18446744073709551615u64));
/// assert!(panics!(0u64 - 18446744073709551614u64));
/// assert!(panics!(0u64 - 1u64));
/// assert!(panics!(11034056263428277270u64 - 13719439366910924178u64));
/// assert!(panics!(13164225337128732393u64 - 14891414436632503512u64));
/// assert!(panics!(9911323403828291055u64 - 16909156765010387066u64));
/// assert!(panics!(9063066053007096588u64 - 12290964067630482475u64));
/// assert!(panics!(363357225188219135u64 - 11086775242280045550u64));
/// assert!(panics!(8477566117531021662u64 - 15618931209094801081u64));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((18446744073709551615u64 - 0u64) + 0u64 == 18446744073709551615u64);
/// assert!((0u64 - 0u64) + 0u64 == 0u64);
/// assert!((1u64 - 0u64) + 0u64 == 1u64);
/// assert!((18446744073709551614u64 - 1u64) + 1u64 == 18446744073709551614u64);
/// assert!((1982518616540436121u64 - 105772265504642273u64) + 105772265504642273u64
///         == 1982518616540436121u64);
/// assert!((9402821684265331485u64 - 8186099806271199278u64) + 8186099806271199278u64
///         == 9402821684265331485u64);
/// assert!((17863537011890604140u64 - 1742308977411726754u64) + 1742308977411726754u64
///         == 17863537011890604140u64);
/// assert!((17623714523438472493u64 - 4061459252554808638u64) + 4061459252554808638u64
///         == 17623714523438472493u64);
/// assert!((5537948368172586122u64 - 1349377895950166936u64) + 1349377895950166936u64
///         == 5537948368172586122u64);
/// assert!((11885240300150246849u64 - 8847082319360437869u64) + 8847082319360437869u64
///         == 11885240300150246849u64);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 - 0 == 0u64);
/// assert!(18446744073709551614u64 - 0 == 18446744073709551614u64);
/// assert!(18446744073709551615u64 - 0 == 18446744073709551615u64);
/// assert!(1u64 - 0 == 1u64);
/// assert!(9431380586121609587u64 - 0 == 9431380586121609587u64);
/// assert!(6819883416746695756u64 - 0 == 6819883416746695756u64);
/// assert!(15595493945962022175u64 - 0 == 15595493945962022175u64);
/// assert!(5610310133161694372u64 - 0 == 5610310133161694372u64);
/// assert!(17783206152058994329u64 - 0 == 17783206152058994329u64);
/// assert!(14095325718314043330u64 - 0 == 14095325718314043330u64);
/// # }
/// ```
pub fn core_ops_add_u64_sub() {}
/// # Properties for [`u64::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.wrapping_sub(1u64) == 18446744073709551614u64);
/// assert!(18446744073709551615u64.wrapping_sub(0u64) == 18446744073709551615u64);
/// assert!(0u64.wrapping_sub(0u64) == 0u64);
/// assert!(1u64.wrapping_sub(0u64) == 1u64);
/// assert!(4295550399245113325u64.wrapping_sub(3147990774445253290u64) == 1147559624799860035u64);
/// assert!(18265225007942576899u64.wrapping_sub(4793280566019501597u64)
///         == 13471944441923075302u64);
/// assert!(5193605276663690913u64.wrapping_sub(5073508740881909483u64) == 120096535781781430u64);
/// assert!(14894656982776678141u64.wrapping_sub(8781773428527472416u64)
///         == 6112883554249205725u64);
/// assert!(17274840230231201528u64.wrapping_sub(6487398083804802028u64)
///         == 10787442146426399500u64);
/// assert!(15144640083175735678u64.wrapping_sub(1995539115987487138u64)
///         == 13149100967188248540u64);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down(x.up() - y.up() + u64::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_sub(18446744073709551615u64) == 1u64);
/// assert!(0u64.wrapping_sub(1u64) == 18446744073709551615u64);
/// assert!(1u64.wrapping_sub(18446744073709551615u64) == 2u64);
/// assert!(1u64.wrapping_sub(18446744073709551614u64) == 3u64);
/// assert!(0u64.wrapping_sub(18446744073709551614u64) == 2u64);
/// assert!(7659176418402310679u64.wrapping_sub(11197173811801213850u64)
///         == 14908746680310648445u64);
/// assert!(5828290851460186584u64.wrapping_sub(8567057937518386626u64)
///         == 15707976987651351574u64);
/// assert!(13903035624928381945u64.wrapping_sub(14322821320173388365u64)
///         == 18026958378464545196u64);
/// assert!(12596077344238981323u64.wrapping_sub(16589997695581851160u64)
///         == 14452823722366681779u64);
/// assert!(13785663631092317216u64.wrapping_sub(17778433844810101484u64)
///         == 14453973859991767348u64);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down((x.up() - y.up()).rem_euclid(&(u64::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.wrapping_sub(0u64) == 18446744073709551614u64);
/// assert!(0u64.wrapping_sub(18446744073709551615u64) == 1u64);
/// assert!(18446744073709551614u64.wrapping_sub(18446744073709551615u64)
///         == 18446744073709551615u64);
/// assert!(0u64.wrapping_sub(18446744073709551614u64) == 2u64);
/// assert!(1u64.wrapping_sub(1u64) == 0u64);
/// assert!(18053880868088225997u64.wrapping_sub(7947735979830574634u64)
///         == 10106144888257651363u64);
/// assert!(8904942849206518349u64.wrapping_sub(8640816945855508782u64) == 264125903351009567u64);
/// assert!(7007524204023294449u64.wrapping_sub(6968744497169713346u64) == 38779706853581103u64);
/// assert!(3391067057377774432u64.wrapping_sub(14002964176719122596u64)
///         == 7834846954368203452u64);
/// assert!(13037521112532359648u64.wrapping_sub(12686920894512140250u64)
///         == 350600218020219398u64);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u64.wrapping_sub(0u64)).wrapping_add(0u64) == 0u64);
/// assert!((1u64.wrapping_sub(0u64)).wrapping_add(0u64) == 1u64);
/// assert!((18446744073709551615u64.wrapping_sub(18446744073709551615u64))
///         .wrapping_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!((18446744073709551615u64.wrapping_sub(1u64)).wrapping_add(1u64)
///         == 18446744073709551615u64);
/// assert!((0u64.wrapping_sub(18446744073709551614u64)).wrapping_add(18446744073709551614u64)
///         == 0u64);
/// assert!((9586776892199351783u64.wrapping_sub(14804343499298723784u64))
///         .wrapping_add(14804343499298723784u64) == 9586776892199351783u64);
/// assert!((5461347418438641227u64.wrapping_sub(7244397506374584039u64))
///         .wrapping_add(7244397506374584039u64) == 5461347418438641227u64);
/// assert!((15233655541217232357u64.wrapping_sub(14283253323550432907u64))
///         .wrapping_add(14283253323550432907u64) == 15233655541217232357u64);
/// assert!((7138101779155896809u64.wrapping_sub(12244600732773038107u64))
///         .wrapping_add(12244600732773038107u64) == 7138101779155896809u64);
/// assert!((12239864750045454708u64.wrapping_sub(15482655176856963924u64))
///         .wrapping_add(15482655176856963924u64) == 12239864750045454708u64);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_sub(0) == 0u64);
/// assert!(18446744073709551615u64.wrapping_sub(0) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_sub(0) == 18446744073709551614u64);
/// assert!(1u64.wrapping_sub(0) == 1u64);
/// assert!(22927912405536232u64.wrapping_sub(0) == 22927912405536232u64);
/// assert!(7096189434884450654u64.wrapping_sub(0) == 7096189434884450654u64);
/// assert!(12464580925333022146u64.wrapping_sub(0) == 12464580925333022146u64);
/// assert!(9727854507170441433u64.wrapping_sub(0) == 9727854507170441433u64);
/// assert!(13618982065693263776u64.wrapping_sub(0) == 13618982065693263776u64);
/// assert!(1151178214184889326u64.wrapping_sub(0) == 1151178214184889326u64);
/// # }
/// ```
pub fn core_ops_add_u64_wrapping_sub() {}
/// # Properties for [`u64::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u64::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_sub(0u64) == Some(0u64));
/// assert!(1u64.checked_sub(0u64) == Some(1u64));
/// assert!(18446744073709551614u64.checked_sub(18446744073709551614u64) == Some(0u64));
/// assert!(2886275021644884670u64.checked_sub(1581189677190709656u64)
///         == Some(1305085344454175014u64));
/// assert!(12271864926647756730u64.checked_sub(10388910563731901193u64)
///         == Some(1882954362915855537u64));
/// assert!(5432036582764001613u64.checked_sub(3229984707734507197u64)
///         == Some(2202051875029494416u64));
/// assert!(4268359746359127824u64.checked_sub(545900495087335535u64)
///         == Some(3722459251271792289u64));
/// assert!(15979193445098486764u64.checked_sub(7524437615397424091u64)
///         == Some(8454755829701062673u64));
/// assert!(12346791969837155010u64.checked_sub(7570312225275793264u64)
///         == Some(4776479744561361746u64));
/// assert!(5905321547289298441u64.checked_sub(1804922977740839927u64)
///         == Some(4100398569548458514u64));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_sub(1u64) == None);
/// assert!(0u64.checked_sub(18446744073709551614u64) == None);
/// assert!(1u64.checked_sub(18446744073709551614u64) == None);
/// assert!(0u64.checked_sub(18446744073709551615u64) == None);
/// assert!(18446744073709551614u64.checked_sub(18446744073709551615u64) == None);
/// assert!(6991458578804593913u64.checked_sub(7201675119529162532u64) == None);
/// assert!(10918914596263596521u64.checked_sub(12879975240046933097u64) == None);
/// assert!(6436052559085231210u64.checked_sub(13990269288958188338u64) == None);
/// assert!(12774864241622201987u64.checked_sub(15498701072776248339u64) == None);
/// assert!(6473421666723681823u64.checked_sub(7553330010330030574u64) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u64.checked_sub(1u64)).and_then(|r| r.checked_add(1u64)) == Some(1u64));
/// assert!((18446744073709551615u64.checked_sub(0u64)).and_then(|r| r.checked_add(0u64))
///         == Some(18446744073709551615u64));
/// assert!((0u64.checked_sub(0u64)).and_then(|r| r.checked_add(0u64)) == Some(0u64));
/// assert!((18446744073709551614u64.checked_sub(0u64)).and_then(|r| r.checked_add(0u64))
///         == Some(18446744073709551614u64));
/// assert!((14238827884214723001u64.checked_sub(5985162945507309663u64))
///         .and_then(|r| r.checked_add(5985162945507309663u64))
///         == Some(14238827884214723001u64));
/// assert!((9359873196497029141u64.checked_sub(4745478315627871410u64))
///         .and_then(|r| r.checked_add(4745478315627871410u64))
///         == Some(9359873196497029141u64));
/// assert!((10568277332962212470u64.checked_sub(3919942074448354962u64))
///         .and_then(|r| r.checked_add(3919942074448354962u64))
///         == Some(10568277332962212470u64));
/// assert!((10005536208508713755u64.checked_sub(4098263209856545351u64))
///         .and_then(|r| r.checked_add(4098263209856545351u64))
///         == Some(10005536208508713755u64));
/// assert!((14571165561067018276u64.checked_sub(929896035577545843u64))
///         .and_then(|r| r.checked_add(929896035577545843u64))
///         == Some(14571165561067018276u64));
/// assert!((13812250452306220802u64.checked_sub(6065329553096652017u64))
///         .and_then(|r| r.checked_add(6065329553096652017u64))
///         == Some(13812250452306220802u64));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_sub(0) == Some(0u64));
/// assert!(18446744073709551615u64.checked_sub(0) == Some(18446744073709551615u64));
/// assert!(18446744073709551614u64.checked_sub(0) == Some(18446744073709551614u64));
/// assert!(1u64.checked_sub(0) == Some(1u64));
/// assert!(9536498815589735313u64.checked_sub(0) == Some(9536498815589735313u64));
/// assert!(18351328503248671312u64.checked_sub(0) == Some(18351328503248671312u64));
/// assert!(13084580782911222975u64.checked_sub(0) == Some(13084580782911222975u64));
/// assert!(13586191306568519095u64.checked_sub(0) == Some(13586191306568519095u64));
/// assert!(9455254375528292444u64.checked_sub(0) == Some(9455254375528292444u64));
/// assert!(17942079496412852231u64.checked_sub(0) == Some(17942079496412852231u64));
/// # }
/// ```
pub fn core_ops_add_u64_checked_sub() {}
/// # Properties for [`u64::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u64::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_sub(0u64) == 0u64);
/// assert!(0u64.saturating_sub(1u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_sub(0u64) == 18446744073709551614u64);
/// assert!(1u64.saturating_sub(0u64) == 1u64);
/// assert!(0u64.saturating_sub(18446744073709551614u64) == 0u64);
/// assert!(1u64.saturating_sub(18446744073709551615u64) == 0u64);
/// assert!(8358365239597853095u64.saturating_sub(15080855100580174968u64) == 0u64);
/// assert!(17811903049872051074u64.saturating_sub(5248577990810077986u64)
///         == 12563325059061973088u64);
/// assert!(16090473597048357249u64.saturating_sub(15634927699458788116u64)
///         == 455545897589569133u64);
/// assert!(15714514304358922607u64.saturating_sub(132002802910082977u64)
///         == 15582511501448839630u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.saturating_sub(1u64) == 0u64);
/// assert!(0u64.saturating_sub(0u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_sub(18446744073709551614u64) == 0u64);
/// assert!(18446744073709551615u64.saturating_sub(1u64) == 18446744073709551614u64);
/// assert!(18446744073709551615u64.saturating_sub(0u64) == 18446744073709551615u64);
/// assert!(6970554964460043766u64.saturating_sub(649278899521555957u64)
///         == 6321276064938487809u64);
/// assert!(16708850109370470318u64.saturating_sub(5581528993991986871u64)
///         == 11127321115378483447u64);
/// assert!(17256974872256090380u64.saturating_sub(2098396596098643408u64)
///         == 15158578276157446972u64);
/// assert!(16465277308234684917u64.saturating_sub(8724892251868553553u64)
///         == 7740385056366131364u64);
/// assert!(14182904310960017581u64.saturating_sub(13745592713603204808u64)
///         == 437311597356812773u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_sub(18446744073709551614u64) == 0);
/// assert!(0u64.saturating_sub(18446744073709551615u64) == 0);
/// assert!(0u64.saturating_sub(1u64) == 0);
/// assert!(1u64.saturating_sub(18446744073709551615u64) == 0);
/// assert!(1u64.saturating_sub(18446744073709551614u64) == 0);
/// assert!(1734279126257848389u64.saturating_sub(15975665495950548962u64) == 0);
/// assert!(8977599426825364030u64.saturating_sub(11143478943684998172u64) == 0);
/// assert!(3925617668309751388u64.saturating_sub(7786315328817659701u64) == 0);
/// assert!(2655903105448625359u64.saturating_sub(12033094168161062012u64) == 0);
/// assert!(4386192396822847281u64.saturating_sub(11857447349225462235u64) == 0);
/// # }
/// ```
pub fn core_ops_sub_u64_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u64>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x + y == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 + 0u64 == 18446744073709551614u64);
/// assert!(0u64 + 1u64 == 1u64);
/// assert!(0u64 + 0u64 == 0u64);
/// assert!(1u64 + 1u64 == 2u64);
/// assert!(0u64 + 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(6873112793834508117u64 + 4705906887961707975u64 == 11579019681796216092u64);
/// assert!(16490075889274401239u64 + 1577868017487921950u64 == 18067943906762323189u64);
/// assert!(11985208548019170625u64 + 6224806249965871195u64 == 18210014797985041820u64);
/// assert!(400935623095651049u64 + 10544684427873020473u64 == 10945620050968671522u64);
/// assert!(4078048769596619446u64 + 1479019509620531813u64 == 5557068279217151259u64);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551615u64 + 1u64));
/// assert!(panics!(18446744073709551615u64 + 18446744073709551614u64));
/// assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
/// assert!(panics!(18446744073709551614u64 + 18446744073709551615u64));
/// assert!(panics!(18446744073709551614u64 + 18446744073709551614u64));
/// assert!(panics!(1u64 + 18446744073709551615u64));
/// assert!(panics!(14444681164332930187u64 + 14119864446268830473u64));
/// assert!(panics!(6537267689692392579u64 + 17008311551053505697u64));
/// assert!(panics!(846328542866194168u64 + 18124651156741785163u64));
/// assert!(panics!(14308236378458981402u64 + 5925310829185753173u64));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 + 0u64 == 0u64 + 18446744073709551614u64);
/// assert!(18446744073709551615u64 + 0u64 == 0u64 + 18446744073709551615u64);
/// assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64 + 0u64);
/// assert!(0u64 + 18446744073709551614u64 == 18446744073709551614u64 + 0u64);
/// assert!(1440915559954330757u64 + 9779481805413251048u64
///         == 9779481805413251048u64 + 1440915559954330757u64);
/// assert!(8623412268454678230u64 + 7953187344132322397u64
///         == 7953187344132322397u64 + 8623412268454678230u64);
/// assert!(16893466379395401071u64 + 1119554388119524567u64
///         == 1119554388119524567u64 + 16893466379395401071u64);
/// assert!(13064162570492401382u64 + 4559277057822536287u64
///         == 4559277057822536287u64 + 13064162570492401382u64);
/// assert!(2420322760408236841u64 + 3134979596377233738u64
///         == 3134979596377233738u64 + 2420322760408236841u64);
/// assert!(12142414795174052203u64 + 510087825373511993u64
///         == 510087825373511993u64 + 12142414795174052203u64);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 + 0 == 0u64);
/// assert!(18446744073709551615u64 + 0 == 18446744073709551615u64);
/// assert!(1u64 + 0 == 1u64);
/// assert!(18446744073709551614u64 + 0 == 18446744073709551614u64);
/// assert!(18077636891963261775u64 + 0 == 18077636891963261775u64);
/// assert!(16405448248678216698u64 + 0 == 16405448248678216698u64);
/// assert!(285388733063728427u64 + 0 == 285388733063728427u64);
/// assert!(14953525728737854786u64 + 0 == 14953525728737854786u64);
/// assert!(13335983163845854369u64 + 0 == 13335983163845854369u64);
/// assert!(10670143945233247107u64 + 0 == 10670143945233247107u64);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(0 + 1u64 == 1u64);
/// assert!(0 + 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(0 + 0u64 == 0u64);
/// assert!(0 + 2281966169165579560u64 == 2281966169165579560u64);
/// assert!(0 + 17479505390687062467u64 == 17479505390687062467u64);
/// assert!(0 + 9140406268186511217u64 == 9140406268186511217u64);
/// assert!(0 + 7140341220052750329u64 == 7140341220052750329u64);
/// assert!(0 + 9001777193593862867u64 == 9001777193593862867u64);
/// assert!(0 + 2887047014197860014u64 == 2887047014197860014u64);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u64::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((18446744073709551614u64 + 0u64) + 1u64 == 18446744073709551614u64 + (0u64 + 1u64));
/// assert!((0u64 + 0u64) + 0u64 == 0u64 + (0u64 + 0u64));
/// assert!((0u64 + 0u64) + 18446744073709551615u64 == 0u64 + (0u64 + 18446744073709551615u64));
/// assert!((0u64 + 0u64) + 1u64 == 0u64 + (0u64 + 1u64));
/// assert!((6772924411745524338u64 + 3991143820784504265u64) + 7677454072088173114u64
///         == 6772924411745524338u64 + (3991143820784504265u64 + 7677454072088173114u64));
/// assert!((7975504051753882484u64 + 1117604192633163277u64) + 1157806766306542515u64
///         == 7975504051753882484u64 + (1117604192633163277u64 + 1157806766306542515u64));
/// assert!((1236175131835354794u64 + 1911499553850055381u64) + 12532304426770503373u64
///         == 1236175131835354794u64 + (1911499553850055381u64 + 12532304426770503373u64));
/// assert!((2878522988990916737u64 + 961086353646417547u64) + 8139890420100179774u64
///         == 2878522988990916737u64 + (961086353646417547u64 + 8139890420100179774u64));
/// assert!((9111351404294541087u64 + 1292300137441700257u64) + 2370423992765119540u64
///         == 9111351404294541087u64 + (1292300137441700257u64 + 2370423992765119540u64));
/// assert!((7238949254334058444u64 + 3259783680902300061u64) + 5407889371983978426u64
///         == 7238949254334058444u64 + (3259783680902300061u64 + 5407889371983978426u64));
/// # }
/// ```
pub fn core_ops_add_u64_add() {}
/// # Properties for [`u64::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down((x.up() + y.up()) % (u64::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.wrapping_add(0u64) == 18446744073709551615u64);
/// assert!(0u64.wrapping_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_add(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551613u64);
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551613u64);
/// assert!(11497329531226800046u64.wrapping_add(17645961346674826711u64)
///         == 10696546804192075141u64);
/// assert!(2523504136897224141u64.wrapping_add(9228819589347923377u64)
///         == 11752323726245147518u64);
/// assert!(5752567711560929977u64.wrapping_add(10165562949348682446u64)
///         == 15918130660909612423u64);
/// assert!(14074322408488401055u64.wrapping_add(11571250499169383567u64)
///         == 7198828833948233006u64);
/// assert!(9517898234019986828u64.wrapping_add(15099555483445413901u64)
///         == 6170709643755849113u64);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_add(0u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_add(0u64) == 18446744073709551615u64);
/// assert!(1u64.wrapping_add(1u64) == 2u64);
/// assert!(18446744073709551614u64.wrapping_add(1u64) == 18446744073709551615u64);
/// assert!(11282832288790894640u64.wrapping_add(3810074944710141816u64)
///         == 15092907233501036456u64);
/// assert!(1021382167855581400u64.wrapping_add(6225662797027278438u64) == 7247044964882859838u64);
/// assert!(323516598024964659u64.wrapping_add(13035419856752040698u64)
///         == 13358936454777005357u64);
/// assert!(12289997749622152191u64.wrapping_add(3491378156440941595u64)
///         == 15781375906063093786u64);
/// assert!(8668033732465009278u64.wrapping_add(5962115806564720216u64)
///         == 14630149539029729494u64);
/// assert!(4772276148552105127u64.wrapping_add(12882300185907866151u64)
///         == 17654576334459971278u64);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down(x.up() + y.up() - u64::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551612u64);
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551613u64);
/// assert!(18446744073709551615u64.wrapping_add(1u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551614u64);
/// assert!(1u64.wrapping_add(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551613u64);
/// assert!(8395306402826299605u64.wrapping_add(18268856737362686463u64)
///         == 8217419066479434452u64);
/// assert!(10855466145140215156u64.wrapping_add(11174015099546791945u64)
///         == 3582737170977455485u64);
/// assert!(5261055245441060330u64.wrapping_add(16946809371679253683u64)
///         == 3761120543410762397u64);
/// assert!(11784821798150687613u64.wrapping_add(13658261105965021065u64)
///         == 6996338830406157062u64);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.wrapping_add(0u64) == 0u64.wrapping_add(1u64));
/// assert!(0u64.wrapping_add(1u64) == 1u64.wrapping_add(0u64));
/// assert!(18446744073709551614u64.wrapping_add(0u64)
///         == 0u64.wrapping_add(18446744073709551614u64));
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551614u64.wrapping_add(18446744073709551615u64));
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551615u64.wrapping_add(18446744073709551614u64));
/// assert!(17629253343937043598u64.wrapping_add(4760858582179607883u64)
///         == 4760858582179607883u64.wrapping_add(17629253343937043598u64));
/// assert!(9381576514058642618u64.wrapping_add(10827321634667638631u64)
///         == 10827321634667638631u64.wrapping_add(9381576514058642618u64));
/// assert!(16190602992964236701u64.wrapping_add(2142813998948933493u64)
///         == 2142813998948933493u64.wrapping_add(16190602992964236701u64));
/// assert!(9760671215100479997u64.wrapping_add(13115199433494600937u64)
///         == 13115199433494600937u64.wrapping_add(9760671215100479997u64));
/// assert!(10642062179775747426u64.wrapping_add(16078920244474320563u64)
///         == 16078920244474320563u64.wrapping_add(10642062179775747426u64));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.wrapping_add(0) == 1u64);
/// assert!(18446744073709551614u64.wrapping_add(0) == 18446744073709551614u64);
/// assert!(0u64.wrapping_add(0) == 0u64);
/// assert!(18446744073709551615u64.wrapping_add(0) == 18446744073709551615u64);
/// assert!(13802959724544829180u64.wrapping_add(0) == 13802959724544829180u64);
/// assert!(1683173445396880880u64.wrapping_add(0) == 1683173445396880880u64);
/// assert!(17038121217206612764u64.wrapping_add(0) == 17038121217206612764u64);
/// assert!(16747438262720273582u64.wrapping_add(0) == 16747438262720273582u64);
/// assert!(1010039358995111349u64.wrapping_add(0) == 1010039358995111349u64);
/// assert!(11918231430825303135u64.wrapping_add(0) == 11918231430825303135u64);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: u64 = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(0u64) == 0u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(1u64) == 1u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(18446744073709551614u64) == 18446744073709551614u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(18446744073709551615u64) == 18446744073709551615u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(15827214620578983104u64) == 15827214620578983104u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(8849339316339505384u64) == 8849339316339505384u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(12502807881322385773u64) == 12502807881322385773u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(16800170780523519921u64) == 16800170780523519921u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(1611195586856796780u64) == 1611195586856796780u64
///     });
/// assert!({
///         let zero: u64 = 0;
///         zero.wrapping_add(18370829050716078361u64) == 18370829050716078361u64
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u64, y : u64, z : u64`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u64::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u64.wrapping_add(0u64)).wrapping_add(18446744073709551615u64)
///         == 0u64.wrapping_add(0u64.wrapping_add(18446744073709551615u64)));
/// assert!((18446744073709551614u64.wrapping_add(0u64)).wrapping_add(0u64)
///         == 18446744073709551614u64.wrapping_add(0u64.wrapping_add(0u64)));
/// assert!((0u64.wrapping_add(18446744073709551614u64)).wrapping_add(1u64)
///         == 0u64.wrapping_add(18446744073709551614u64.wrapping_add(1u64)));
/// assert!((0u64.wrapping_add(18446744073709551614u64)).wrapping_add(0u64)
///         == 0u64.wrapping_add(18446744073709551614u64.wrapping_add(0u64)));
/// assert!((18446744073709551614u64.wrapping_add(0u64)).wrapping_add(1u64)
///         == 18446744073709551614u64.wrapping_add(0u64.wrapping_add(1u64)));
/// assert!((0u64.wrapping_add(1u64)).wrapping_add(0u64)
///         == 0u64.wrapping_add(1u64.wrapping_add(0u64)));
/// assert!((7992379714619961784u64.wrapping_add(5395939545173631554u64))
///         .wrapping_add(4938777108214060357u64)
///         == 7992379714619961784u64
///             .wrapping_add(5395939545173631554u64.wrapping_add(4938777108214060357u64)));
/// assert!((2839265273189607235u64.wrapping_add(4064859409454933605u64))
///         .wrapping_add(1634630257615510750u64)
///         == 2839265273189607235u64
///             .wrapping_add(4064859409454933605u64.wrapping_add(1634630257615510750u64)));
/// assert!((9318122716365391050u64.wrapping_add(361746444172479746u64))
///         .wrapping_add(6088884781733803660u64)
///         == 9318122716365391050u64
///             .wrapping_add(361746444172479746u64.wrapping_add(6088884781733803660u64)));
/// assert!((303657065953992066u64.wrapping_add(2954525626561174298u64))
///         .wrapping_add(12223088994190475670u64)
///         == 303657065953992066u64
///             .wrapping_add(2954525626561174298u64.wrapping_add(12223088994190475670u64)));
/// # }
/// ```
pub fn core_ops_add_u64_wrapping_add() {}
/// # Properties for [`u64::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u64::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.checked_add(0u64) == Some(1u64));
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(0u64.checked_add(18446744073709551614u64) == Some(18446744073709551614u64));
/// assert!(1u64.checked_add(1u64) == Some(2u64));
/// assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(4067254209563317489u64.checked_add(7306047880005491721u64)
///         == Some(11373302089568809210u64));
/// assert!(3476509367821100462u64.checked_add(10261534538333706118u64)
///         == Some(13738043906154806580u64));
/// assert!(9516601890714965454u64.checked_add(389146804558578938u64)
///         == Some(9905748695273544392u64));
/// assert!(5569709721449974863u64.checked_add(5404991929729897711u64)
///         == Some(10974701651179872574u64));
/// assert!(4327954126007052286u64.checked_add(5035172111931420468u64)
///         == Some(9363126237938472754u64));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_add(1u64) == None);
/// assert!(18446744073709551614u64.checked_add(18446744073709551615u64) == None);
/// assert!(18446744073709551614u64.checked_add(18446744073709551614u64) == None);
/// assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
/// assert!(18446744073709551615u64.checked_add(18446744073709551614u64) == None);
/// assert!(1u64.checked_add(18446744073709551615u64) == None);
/// assert!(12250979992971223094u64.checked_add(12350196568842173433u64) == None);
/// assert!(14683466782065559749u64.checked_add(5959532274398196115u64) == None);
/// assert!(10537258269582633235u64.checked_add(15005794710375072140u64) == None);
/// assert!(9965315163287415550u64.checked_add(15974399092845821926u64) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.checked_add(0u64) == 0u64.checked_add(1u64));
/// assert!(0u64.checked_add(0u64) == 0u64.checked_add(0u64));
/// assert!(18446744073709551615u64.checked_add(1u64)
///         == 1u64.checked_add(18446744073709551615u64));
/// assert!(0u64.checked_add(18446744073709551615u64)
///         == 18446744073709551615u64.checked_add(0u64));
/// assert!(18446744073709551614u64.checked_add(18446744073709551615u64)
///         == 18446744073709551615u64.checked_add(18446744073709551614u64));
/// assert!(1072994177747969064u64.checked_add(6945007417353748628u64)
///         == 6945007417353748628u64.checked_add(1072994177747969064u64));
/// assert!(8525389765938437305u64.checked_add(4777747194165159641u64)
///         == 4777747194165159641u64.checked_add(8525389765938437305u64));
/// assert!(3307483026636985139u64.checked_add(9362931579535312760u64)
///         == 9362931579535312760u64.checked_add(3307483026636985139u64));
/// assert!(9162319209807707428u64.checked_add(13086863034179529216u64)
///         == 13086863034179529216u64.checked_add(9162319209807707428u64));
/// assert!(5260534946306240568u64.checked_add(14622473656912468386u64)
///         == 14622473656912468386u64.checked_add(5260534946306240568u64));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u64) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
/// assert!(1u64.checked_add(0u64) == Some(1u64));
/// assert!(18446744073709551614u64.checked_add(0u64) == Some(18446744073709551614u64));
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(12298513002030925230u64.checked_add(0u64) == Some(12298513002030925230u64));
/// assert!(226102688386191601u64.checked_add(0u64) == Some(226102688386191601u64));
/// assert!(4253529209422362195u64.checked_add(0u64) == Some(4253529209422362195u64));
/// assert!(17492172460224443003u64.checked_add(0u64) == Some(17492172460224443003u64));
/// assert!(55936551739335770u64.checked_add(0u64) == Some(55936551739335770u64));
/// assert!(14498750234820050267u64.checked_add(0u64) == Some(14498750234820050267u64));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u64.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(0u64.checked_add(18446744073709551614u64) == Some(18446744073709551614u64));
/// assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(0u64.checked_add(1u64) == Some(1u64));
/// assert!(0u64.checked_add(3042698875917523617u64) == Some(3042698875917523617u64));
/// assert!(0u64.checked_add(720844263123838290u64) == Some(720844263123838290u64));
/// assert!(0u64.checked_add(12128759048026225208u64) == Some(12128759048026225208u64));
/// assert!(0u64.checked_add(10131181112486102638u64) == Some(10131181112486102638u64));
/// assert!(0u64.checked_add(12500966797585984083u64) == Some(12500966797585984083u64));
/// assert!(0u64.checked_add(3778082269997760375u64) == Some(3778082269997760375u64));
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(18446744073709551615u64).and_then(|iv| iv.checked_add(1u64))
///         == 18446744073709551615u64.checked_add(1u64).and_then(|iv| 0u64.checked_add(iv)));
/// assert!(18446744073709551614u64.checked_add(1u64).and_then(|iv| iv.checked_add(0u64))
///         == 1u64.checked_add(0u64).and_then(|iv| 18446744073709551614u64.checked_add(iv)));
/// assert!(18446744073709551615u64
///         .checked_add(1u64)
///         .and_then(|iv| iv.checked_add(18446744073709551614u64))
///         == 1u64
///             .checked_add(18446744073709551614u64)
///             .and_then(|iv| 18446744073709551615u64.checked_add(iv)));
/// assert!(0u64.checked_add(0u64).and_then(|iv| iv.checked_add(18446744073709551615u64))
///         == 0u64.checked_add(18446744073709551615u64).and_then(|iv| 0u64.checked_add(iv)));
/// assert!(18446744073709551615u64
///         .checked_add(18446744073709551615u64)
///         .and_then(|iv| iv.checked_add(1u64))
///         == 18446744073709551615u64
///             .checked_add(1u64)
///             .and_then(|iv| 18446744073709551615u64.checked_add(iv)));
/// assert!(5725126155127346011u64
///         .checked_add(14392022331477153677u64)
///         .and_then(|iv| iv.checked_add(12391541669760136159u64))
///         == 14392022331477153677u64
///             .checked_add(12391541669760136159u64)
///             .and_then(|iv| 5725126155127346011u64.checked_add(iv)));
/// assert!(5592437869356100533u64
///         .checked_add(11184408706276311160u64)
///         .and_then(|iv| iv.checked_add(13754005039685437843u64))
///         == 11184408706276311160u64
///             .checked_add(13754005039685437843u64)
///             .and_then(|iv| 5592437869356100533u64.checked_add(iv)));
/// assert!(18227719229281161737u64
///         .checked_add(5728495174683669304u64)
///         .and_then(|iv| iv.checked_add(16302867871413741907u64))
///         == 5728495174683669304u64
///             .checked_add(16302867871413741907u64)
///             .and_then(|iv| 18227719229281161737u64.checked_add(iv)));
/// assert!(14937588429320229848u64
///         .checked_add(6090701150718775707u64)
///         .and_then(|iv| iv.checked_add(17713077468231606043u64))
///         == 6090701150718775707u64
///             .checked_add(17713077468231606043u64)
///             .and_then(|iv| 14937588429320229848u64.checked_add(iv)));
/// assert!(11866679109962455629u64
///         .checked_add(6958480622894740185u64)
///         .and_then(|iv| iv.checked_add(15573252205226869417u64))
///         == 6958480622894740185u64
///             .checked_add(15573252205226869417u64)
///             .and_then(|iv| 11866679109962455629u64.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u64_checked_add() {}
/// # Properties for [`u64::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u64::down((x.up() + y.up()).min(u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_add(0u64) == 0u64);
/// assert!(1u64.saturating_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_add(18446744073709551614u64)
///         == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_add(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_add(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_add(18446744073709551615u64)
///         == 18446744073709551615u64);
/// assert!(541387966228545948u64.saturating_add(63915623614872842u64) == 605303589843418790u64);
/// assert!(15239498299998639604u64.saturating_add(16105248499582893708u64)
///         == 18446744073709551615u64);
/// assert!(2291202630201986527u64.saturating_add(16950175483481505316u64)
///         == 18446744073709551615u64);
/// assert!(13339534060194688678u64.saturating_add(16744578176459419040u64)
///         == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_add(0u64) == 0u64);
/// assert!(18446744073709551615u64.saturating_add(0u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_add(1u64) == 18446744073709551615u64);
/// assert!(0u64.saturating_add(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(1u64.saturating_add(0u64) == 1u64);
/// assert!(702133583256102253u64.saturating_add(8002151453192877111u64)
///         == 8704285036448979364u64);
/// assert!(422236904932363843u64.saturating_add(17016393797660234318u64)
///         == 17438630702592598161u64);
/// assert!(2225441287772132075u64.saturating_add(517188340883590026u64)
///         == 2742629628655722101u64);
/// assert!(9662402388100658631u64.saturating_add(872315084759208151u64)
///         == 10534717472859866782u64);
/// assert!(5162698224122104094u64.saturating_add(11408621137122469995u64)
///         == 16571319361244574089u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u64::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_add(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(1u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(18446744073709551614u64) == u64::MAX);
/// assert!(7939920722882736418u64.saturating_add(16569439831176199779u64) == u64::MAX);
/// assert!(14416864976143788388u64.saturating_add(16093611464358467441u64) == u64::MAX);
/// assert!(12506343134089974554u64.saturating_add(11999605770822851986u64) == u64::MAX);
/// assert!(14429720263009563451u64.saturating_add(9454724718220209159u64) == u64::MAX);
/// # }
/// ```
pub fn core_ops_add_u64_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u128>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u128::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         % 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(0u128 % 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(0u128 % 1u128 == 0u128);
/// assert!(1u128 % 340282366920938463463374607431768211455u128 == 1u128);
/// assert!(165279567009091147950063193432337719155u128
///         % 155749280621017155545022556905659026770u128
///         == 9530286388073992405040636526678692385u128);
/// assert!(118592575597622308504946721463665697917u128
///         % 106897882138788686043765743338769687940u128
///         == 11694693458833622461180978124896009977u128);
/// assert!(52305314987529907065768160104289217579u128
///         % 241553628623671790686835999193538214453u128
///         == 52305314987529907065768160104289217579u128);
/// assert!(329139448216202008444380637023426590410u128
///         % 13926876413228973622391292715593217868u128
///         == 8821290711935615129380904564782579446u128);
/// assert!(239014478764977911136584437733855150507u128
///         % 207427859183084608140333812995769014719u128
///         == 31586619581893302996250624738086135788u128);
/// assert!(148944798795309530305483726261865358940u128
///         % 185608793377222334346054498380434558236u128
///         == 148944798795309530305483726261865358940u128);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u128_rem() {}
/// # Properties for [`u128::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u128::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.checked_rem(1u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_rem(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(0u128.checked_rem(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_rem(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(0u128.checked_rem(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_rem(1u128) == Some(0u128));
/// assert!(197904710257740101543340148781458343076u128
///         .checked_rem(115503796870237375513125712704717685596u128)
///         == Some(82400913387502726030214436076740657480u128));
/// assert!(97121252504131859423921592623360977243u128
///         .checked_rem(101151238775934452958559572713164867221u128)
///         == Some(97121252504131859423921592623360977243u128));
/// assert!(226116338896600061115620443403251572982u128
///         .checked_rem(188485115233952484376969270584297994054u128)
///         == Some(37631223662647576738651172818953578928u128));
/// assert!(279672753523447787978569448995181667332u128
///         .checked_rem(90537114182342278778857143503214086490u128)
///         == Some(8061410976420951641998018485539407862u128));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u128.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u128_checked_rem() {}
/// # Properties for [`u128::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `x == u128::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u128::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_neg() == Some(0u128));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `x != u128::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.checked_neg() == None);
/// assert!(1u128.checked_neg() == None);
/// assert!(340282366920938463463374607431768211455u128.checked_neg() == None);
/// assert!(339530611474069295205038294600828375454u128.checked_neg() == None);
/// assert!(269878819611935561407320199580557796311u128.checked_neg() == None);
/// assert!(142323578124129226970613185704305106651u128.checked_neg() == None);
/// assert!(19296530820116901095112675171532364630u128.checked_neg() == None);
/// assert!(17403234409956085069582701893392399476u128.checked_neg() == None);
/// assert!(102932055168727243570965303189836142142u128.checked_neg() == None);
/// assert!(103389873277986063460015231355786615498u128.checked_neg() == None);
/// # }
/// ```
pub fn t_u128_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u128>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x << y == u128::down((x.up() << y) & u128::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128 << 0u32
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128 << 1u32 == 2u128);
/// assert!(0u128 << 1u32 == 0u128);
/// assert!(0u128 << 0u32 == 0u128);
/// assert!(1u128 << 0u32 == 1u128);
/// assert!(113699248844852967389607659790730454124u128 << 40u32
///         == 799980777798157323152109115403141120u128);
/// assert!(90613784064571567748980140286851503002u128 << 16u32
///         == 197367318465138097812199547313042620416u128);
/// assert!(157815862717777113875853390533849819491u128 << 44u32
///         == 72223912803883472540372308726386786304u128);
/// assert!(213072217016016640595196483961503348645u128 << 119u32
///         == 279802493112724791246251386189012533248u128);
/// assert!(327476622907742198811587862309285562577u128 << 115u32
///         == 93752112077704847660746641720398053376u128);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(340282366920938463463374607431768211454u128 << 4294967295u32));
/// assert!(panics!(0u128 << 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 << 4294967294u32));
/// assert!(panics!(1u128 << 4294967295u32));
/// assert!(panics!(0u128 << 4294967294u32));
/// assert!(panics!(1u128 << 4294967294u32));
/// assert!(panics!(298345759450429921398906082834388091650u128 << 133u32));
/// assert!(panics!(218370933246023631170111892818660806576u128 << 135u32));
/// assert!(panics!(163203166328356871898192568439542855733u128 << 130u32));
/// assert!(panics!(248200426393977888511028413289271196069u128 << 134u32));
/// # }
/// ```
pub fn core_ops_shl_u128_shl() {}
/// # Properties for [`core::ops::Shl::<u128>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u128::down((x.up() << y) & u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.checked_shl(1u32)
///         == Some(340282366920938463463374607431768211452u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shl(0u32)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(1u128.checked_shl(0u32) == Some(1u128));
/// assert!(0u128.checked_shl(0u32) == Some(0u128));
/// assert!(290378029557449922155347396468709090772u128.checked_shl(35u32)
///         == Some(129957054640666072357467003393098645504u128));
/// assert!(160915236427804063474725500722692375970u128.checked_shl(93u32)
///         == Some(151681446594316408214864499455731695616u128));
/// assert!(330830281648505852581083907648998636656u128.checked_shl(85u32)
///         == Some(167124429794415400510601513089808465920u128));
/// assert!(240262733941476922113253463484465635792u128.checked_shl(2u32)
///         == Some(280486201924030761526264639074326120256u128));
/// assert!(146734302021290512383664444249087110056u128.checked_shl(3u32)
///         == Some(153027315407508708679191731697392246080u128));
/// assert!(300910112010925888762557769234171023349u128.checked_shl(29u32)
///         == Some(238942911719646214728913202375569702912u128));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.checked_shl(4294967295u32) == None);
/// assert!(0u128.checked_shl(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shl(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shl(4294967294u32) == None);
/// assert!(1u128.checked_shl(4294967294u32) == None);
/// assert!(0u128.checked_shl(4294967294u32) == None);
/// assert!(228081454727526150680928661228989955970u128.checked_shl(138u32) == None);
/// assert!(262512550781997195308786273298620155469u128.checked_shl(135u32) == None);
/// assert!(164157117123825529974338843589094611014u128.checked_shl(137u32) == None);
/// assert!(338985561121407914362737876894957236568u128.checked_shl(135u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u128_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u128>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u128::down((x.up() << y) & u128::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(0u32)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(1u128.overflowing_shl(1u32) == (2u128, false));
/// assert!(0u128.overflowing_shl(0u32) == (0u128, false));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(0u32)
///         == (340282366920938463463374607431768211455u128, false));
/// assert!(273062793211525393081434930449763094349u128.overflowing_shl(42u32)
///         == (203646907561105097508361117802061365248u128, false));
/// assert!(222687750525370170390805015705304183567u128.overflowing_shl(67u32)
///         == (127072533518383489178496267763515916288u128, false));
/// assert!(23262435629076802430100176875137600124u128.overflowing_shl(117u32)
///         == (105673625664900811895852661292287393792u128, false));
/// assert!(215721255784434349554365950908791109239u128.overflowing_shl(43u32)
///         == (305933467357588934554990651985602019328u128, false));
/// assert!(256613329837541252657572489782251104195u128.overflowing_shl(92u32)
///         == (47749061352160231268963283348291059712u128, false));
/// assert!(190753225078727606021418083476136186571u128.overflowing_shl(67u32)
///         == (234884971023335225237363460808942551040u128, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u128::down((x.up() << (y & (u128::BITS - 1)) & u128::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(4294967294u32)
///         == (170141183460469231731687303715884105728u128, true));
/// assert!(0u128.overflowing_shl(4294967294u32) == (0u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(4294967294u32)
///         == (255211775190703847597530955573826158592u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(4294967295u32)
///         == (0u128, true));
/// assert!(0u128.overflowing_shl(4294967295u32) == (0u128, true));
/// assert!(1u128.overflowing_shl(4294967295u32)
///         == (170141183460469231731687303715884105728u128, true));
/// assert!(4117347195829555112140947184055320116u128.overflowing_shl(132u32)
///         == (65877555133272881794255154944885121856u128, true));
/// assert!(150132961496137677501697823274894503378u128.overflowing_shl(131u32)
///         == (180216591206286029623458763903851392656u128, true));
/// assert!(127753090255186701627295598903286917366u128.overflowing_shl(128u32)
///         == (127753090255186701627295598903286917366u128, true));
/// assert!(205846817790232398794448435929604951720u128.overflowing_shl(135u32)
///         == (146650424237485359009555026743281538048u128, true));
/// # }
/// ```
pub fn core_ops_shl_u128_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u128>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x >> y == u128::down((x.up() >> y) & u128::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 >> 1u32 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 >> 0u32
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128 >> 0u32 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 >> 1u32
///         == 170141183460469231731687303715884105727u128);
/// assert!(1u128 >> 1u32 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 >> 1u32
///         == 170141183460469231731687303715884105727u128);
/// assert!(227089335066104853425697743040453938492u128 >> 46u32 == 3227133547996243882859314u128);
/// assert!(153896055151542895544406335027172480795u128 >> 80u32 == 127299833169747u128);
/// assert!(120571554284612410647106899814344738847u128 >> 73u32 == 12766009872590889u128);
/// assert!(38193876580722059971635640592871971759u128 >> 76u32 == 505491747613062u128);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u128 >> 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 >> 4294967294u32));
/// assert!(panics!(0u128 >> 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 >> 4294967295u32));
/// assert!(panics!(1u128 >> 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 >> 4294967295u32));
/// assert!(panics!(25897848721839680065148633815835921403u128 >> 135u32));
/// assert!(panics!(286374049010926090547132480538131864086u128 >> 130u32));
/// assert!(panics!(304252035078002645993141153496263197842u128 >> 137u32));
/// assert!(panics!(260508253520686154068986738155264691776u128 >> 135u32));
/// # }
/// ```
pub fn core_ops_shr_u128_shr() {}
/// # Properties for [`core::ops::Shr::<u128>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u128::down((x.up() >> y) & u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_shr(0u32) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_shr(1u32)
///         == Some(170141183460469231731687303715884105727u128));
/// assert!(0u128.checked_shr(1u32) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shr(0u32)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(252316218034588459971116333202084596065u128.checked_shr(66u32)
///         == Some(3419522396830337775u128));
/// assert!(334190594110102711535156507242418690490u128.checked_shr(83u32)
///         == Some(34554497543181u128));
/// assert!(66145232996116336173724471662930760468u128.checked_shr(28u32)
///         == Some(246410194769935072115527360375u128));
/// assert!(169198543961681512031294027829486227440u128.checked_shr(120u32) == Some(127u128));
/// assert!(76905542726712586382966936419089068481u128.checked_shr(98u32) == Some(242671104u128));
/// assert!(278076771542737860680363066365121079009u128.checked_shr(27u32)
///         == Some(2071833398511542831960045295693u128));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.checked_shr(4294967295u32) == None);
/// assert!(0u128.checked_shr(4294967295u32) == None);
/// assert!(1u128.checked_shr(4294967294u32) == None);
/// assert!(0u128.checked_shr(4294967294u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shr(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shr(4294967294u32) == None);
/// assert!(252393010883719508952987541764220805046u128.checked_shr(128u32) == None);
/// assert!(137759473692456006391822059612556642048u128.checked_shr(137u32) == None);
/// assert!(211694376451414918974489954232653878969u128.checked_shr(134u32) == None);
/// assert!(189351340235140125576720953758836008517u128.checked_shr(129u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u128_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u128>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u128::down((x.up() >> y) & u128::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shr(0u32) == (0u128, false));
/// assert!(0u128.overflowing_shr(1u32) == (0u128, false));
/// assert!(1u128.overflowing_shr(0u32) == (1u128, false));
/// assert!(1u128.overflowing_shr(1u32) == (0u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(0u32)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(145147086045401703383115577531452243504u128.overflowing_shr(2u32)
///         == (36286771511350425845778894382863060876u128, false));
/// assert!(7380747509478876327918292409468917876u128.overflowing_shr(64u32)
///         == (400111124217198697u128, false));
/// assert!(986798216271145287180268589045388798u128.overflowing_shr(21u32)
///         == (470542057166645663824209494135u128, false));
/// assert!(117546134513307262468459419464613233595u128.overflowing_shr(57u32)
///         == (815640156202246824821u128, false));
/// assert!(91801043877071481930841570278220029760u128.overflowing_shr(52u32)
///         == (20383926519389801586177u128, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u128::down((x.up() >> (y & (u128::BITS - 1)) & u128::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shr(4294967294u32) == (0u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(4294967294u32)
///         == (3u128, true));
/// assert!(0u128.overflowing_shr(4294967295u32) == (0u128, true));
/// assert!(1u128.overflowing_shr(4294967294u32) == (0u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shr(4294967294u32)
///         == (3u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(4294967295u32)
///         == (1u128, true));
/// assert!(205157128168528260251075194667511472862u128.overflowing_shr(132u32)
///         == (12822320510533016265692199666719467053u128, true));
/// assert!(330820758495165359424854114445849632634u128.overflowing_shr(132u32)
///         == (20676297405947834964053382152865602039u128, true));
/// assert!(332873604912822982328031719059567428156u128.overflowing_shr(132u32)
///         == (20804600307051436395501982441222964259u128, true));
/// assert!(311981185646969099307405436089504526249u128.overflowing_shr(133u32)
///         == (9749412051467784353356419877797016445u128, true));
/// # }
/// ```
pub fn core_ops_shr_u128_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u128>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u128::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 / 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128
///         / 340282366920938463463374607431768211455u128 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 / 1u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128
///         / 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 / 1u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128 / 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(13808680693247794100u128 / 2526761807744671121u128 == 5u128);
/// assert!(12644260822957630356u128 / 16726678196147543378u128 == 0u128);
/// assert!(3548684484801008816u128 / 14992055290432514636u128 == 0u128);
/// assert!(7165918506156176373u128 / 9391540838924134476u128 == 0u128);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211455u128 / 0) }
///     });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u128 / 0) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211454u128 / 0) }
///     });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128 / 0) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(225131655944674591278497499339539469205u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(222179127607791981911361601808483216032u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(321417287055251149223702520388281932982u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(315055085413821099768960658470641601648u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(289234439264713091505889648752547939525u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(210855449565762498842121252714946051399u128 / 0) }
///     });
/// # }
/// ```
pub fn core_ops_div_u128_div() {}
/// # Properties for [`u128::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u128::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_div(1u128) == 0u128);
/// assert!(1u128.saturating_div(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(1u128.saturating_div(1u128) == 1u128);
/// assert!(0u128.saturating_div(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(0u128.saturating_div(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_div(340282366920938463463374607431768211455u128) == 1u128);
/// assert!(12339861408428590369u128.saturating_div(3177173441718590382u128) == 3u128);
/// assert!(11520006454850179762u128.saturating_div(2535884170206035972u128) == 4u128);
/// assert!(9228644481763328757u128.saturating_div(14303107639225285917u128) == 0u128);
/// assert!(10309170576446809193u128.saturating_div(17021042222807194787u128) == 0u128);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211454u128.saturating_div(0)) }
///     });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u128.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211455u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(122085590719108873440959121395426160042u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(214959012288811394683508257838025342361u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(83063533035402852270398974896405331679u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(94577046907241836256838743669170202880u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(331976788507232655650799767806004703085u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(94300693928066838452584567040672958778u128.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_u128_saturating_div() {}
/// # Properties for [`u128::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u128::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.checked_div(1u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(1u128.checked_div(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(1u128.checked_div(1u128) == Some(1u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(0u128.checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(1u128.checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(6287645985062559137u128.checked_div(10233330436682720202u128) == Some(0u128));
/// assert!(4557345732333069866u128.checked_div(15124360416655914256u128) == Some(0u128));
/// assert!(8886360492881572522u128.checked_div(17114709241249023537u128) == Some(0u128));
/// assert!(11864534233453572578u128.checked_div(17958438132477114402u128) == Some(0u128));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.checked_div(0) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_div(0) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_div(0) == None);
/// assert!(0u128.checked_div(0) == None);
/// assert!(35095900735441115712733513677132751685u128.checked_div(0) == None);
/// assert!(225094784386771068938448132430195973855u128.checked_div(0) == None);
/// assert!(70660198104789789594012410685419918133u128.checked_div(0) == None);
/// assert!(249304095751593295709556694899084863661u128.checked_div(0) == None);
/// assert!(322848801470344930098065584989401446128u128.checked_div(0) == None);
/// assert!(127550666266170902895855004456249022551u128.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u128_checked_div() {}
/// # Properties for [`core::ops::Mul::<u128>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x * y == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 1u128 == 0u128);
/// assert!(0u128 * 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 * 0u128 == 0u128);
/// assert!(0u128 * 0u128 == 0u128);
/// assert!(1u128 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(7639995825759736334u128 * 17860086828532616057u128
///         == 136450988817695634443839018347574715038u128);
/// assert!(16281002121915483873u128 * 14148649613035636975u128
///         == 230354194372071895384022840199895004175u128);
/// assert!(11845496803890765291u128 * 17181543683154905681u128
///         == 203523920784721002956806729200013518171u128);
/// assert!(14384219805716337283u128 * 12666709440447847079u128
///         == 182200732806544026248484060674770346357u128);
/// assert!(13152133472268433799u128 * 6558971202327073955u128
///         == 86264464693770643212747175862294605045u128);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 *
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 *
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 *
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 *
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         92215479319180434583445357332287493602u128 *
///         94588336230098751161255056992706707629u128
///     ));
/// assert!(panics!(
///         269903697712837404445239807603999833638u128 *
///         147059596920777773184174165839429329833u128
///     ));
/// assert!(panics!(
///         2143163717716865484452562791705878744u128 *
///         95899835590197703861268552289701909710u128
///     ));
/// assert!(panics!(
///         109915570040052715445139256783338342646u128 *
///         34510106315589169885546618121939058661u128
///     ));
/// assert!(panics!(
///         329623161812438487872281163915726780779u128 *
///         132200644406331157118291200355931451901u128
///     ));
/// assert!(panics!(
///         224048592893230593122708917372229001423u128 *
///         110210059487275370044606646937503712724u128
///     ));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 1 == 0u128);
/// assert!(1u128 * 1 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 * 1
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128 * 1
///         == 340282366920938463463374607431768211455u128);
/// assert!(323398883111353050535216765365422167135u128 * 1
///         == 323398883111353050535216765365422167135u128);
/// assert!(27084669087016087507040569092277562128u128 * 1
///         == 27084669087016087507040569092277562128u128);
/// assert!(301728536402245270044733024919450119512u128 * 1
///         == 301728536402245270044733024919450119512u128);
/// assert!(43693108635911089097878490538692307499u128 * 1
///         == 43693108635911089097878490538692307499u128);
/// assert!(139345012202111874863187473531434707926u128 * 1
///         == 139345012202111874863187473531434707926u128);
/// assert!(192561938780087606125017869802848647664u128 * 1
///         == 192561938780087606125017869802848647664u128);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u128 == 0u128);
/// assert!(1 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(1 * 1u128 == 1u128);
/// assert!(1 * 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1 * 83773179705874749868750107536879646905u128
///         == 83773179705874749868750107536879646905u128);
/// assert!(1 * 149234296699894407131051174300472455942u128
///         == 149234296699894407131051174300472455942u128);
/// assert!(1 * 289075967976799078005570046526840138916u128
///         == 289075967976799078005570046526840138916u128);
/// assert!(1 * 284262508681214563494059192615803605835u128
///         == 284262508681214563494059192615803605835u128);
/// assert!(1 * 124319690496438992226313912496406185831u128
///         == 124319690496438992226313912496406185831u128);
/// assert!(1 * 300904288351288325486295304777504366228u128
///         == 300904288351288325486295304777504366228u128);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 1u128 == 1u128 * 0u128);
/// assert!(0u128 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128 * 0u128);
/// assert!(1u128 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128 * 1u128);
/// assert!(1u128 * 0u128 == 0u128 * 1u128);
/// assert!(340282366920938463463374607431768211455u128 * 0u128
///         == 0u128 * 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 * 1u128
///         == 1u128 * 340282366920938463463374607431768211455u128);
/// assert!(15016942196816340231u128 * 12345843113174583806u128
///         == 12345843113174583806u128 * 15016942196816340231u128);
/// assert!(15806478996168778861u128 * 2740748031492392192u128
///         == 2740748031492392192u128 * 15806478996168778861u128);
/// assert!(11679538403219276618u128 * 14230299515892364820u128
///         == 14230299515892364820u128 * 11679538403219276618u128);
/// assert!(16531867865092608363u128 * 9033901760114313633u128
///         == 9033901760114313633u128 * 16531867865092608363u128);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u128::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u128 * 1u128) * 340282366920938463463374607431768211455u128
///         == 1u128 * (1u128 * 340282366920938463463374607431768211455u128));
/// assert!((1u128 * 1u128) * 340282366920938463463374607431768211454u128
///         == 1u128 * (1u128 * 340282366920938463463374607431768211454u128));
/// assert!((340282366920938463463374607431768211454u128 * 1u128) * 1u128
///         == 340282366920938463463374607431768211454u128 * (1u128 * 1u128));
/// assert!((1u128 * 340282366920938463463374607431768211455u128) * 1u128
///         == 1u128 * (340282366920938463463374607431768211455u128 * 1u128));
/// assert!((1u128 * 340282366920938463463374607431768211454u128) * 1u128
///         == 1u128 * (340282366920938463463374607431768211454u128 * 1u128));
/// assert!((340282366920938463463374607431768211455u128 * 1u128) * 1u128
///         == 340282366920938463463374607431768211455u128 * (1u128 * 1u128));
/// assert!((5866595874555u128 * 4666966822854u128) * 5125724479421u128
///         == 5866595874555u128 * (4666966822854u128 * 5125724479421u128));
/// assert!((5872858091085u128 * 2943043492412u128) * 5473055270618u128
///         == 5872858091085u128 * (2943043492412u128 * 5473055270618u128));
/// assert!((4744615218362u128 * 5288166931940u128) * 6765316926853u128
///         == 4744615218362u128 * (5288166931940u128 * 6765316926853u128));
/// assert!((2060683028327u128 * 6224734491284u128) * 6430996901166u128
///         == 2060683028327u128 * (6224734491284u128 * 6430996901166u128));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u128::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128 * (0u128 + 1u128)
///         == 340282366920938463463374607431768211455u128 * 0u128
///             + 340282366920938463463374607431768211455u128 * 1u128);
/// assert!(1u128 * (340282366920938463463374607431768211455u128 + 0u128)
///         == 1u128 * 340282366920938463463374607431768211455u128 + 1u128 * 0u128);
/// assert!(1u128 * (340282366920938463463374607431768211454u128 + 0u128)
///         == 1u128 * 340282366920938463463374607431768211454u128 + 1u128 * 0u128);
/// assert!(1u128 * (340282366920938463463374607431768211454u128 + 1u128)
///         == 1u128 * 340282366920938463463374607431768211454u128 + 1u128 * 1u128);
/// assert!(1u128 * (0u128 + 1u128) == 1u128 * 0u128 + 1u128 * 1u128);
/// assert!(340282366920938463463374607431768211455u128 * (0u128 + 0u128)
///         == 340282366920938463463374607431768211455u128 * 0u128
///             + 340282366920938463463374607431768211455u128 * 0u128);
/// assert!(5898985982122711071u128 * (5957257819438293248u128 + 8652392982135836625u128)
///         == 5898985982122711071u128 * 5957257819438293248u128
///             + 5898985982122711071u128 * 8652392982135836625u128);
/// assert!(9340906336555069035u128 * (16556051974108873240u128 + 13947051649383322990u128)
///         == 9340906336555069035u128 * 16556051974108873240u128
///             + 9340906336555069035u128 * 13947051649383322990u128);
/// assert!(11427994037182123460u128 * (7786704897076756510u128 + 13858717242893798616u128)
///         == 11427994037182123460u128 * 7786704897076756510u128
///             + 11427994037182123460u128 * 13858717242893798616u128);
/// assert!(6964794453614375117u128 * (17600176743911101129u128 + 16202619245868132820u128)
///         == 6964794453614375117u128 * 17600176743911101129u128
///             + 6964794453614375117u128 * 16202619245868132820u128);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128 * 0 == 0);
/// assert!(0u128 * 0 == 0);
/// assert!(1u128 * 0 == 0);
/// assert!(340282366920938463463374607431768211454u128 * 0 == 0);
/// assert!(16891333348841096977u128 * 0 == 0);
/// assert!(8652969449211194708u128 * 0 == 0);
/// assert!(6637754669693205148u128 * 0 == 0);
/// assert!(12343864007091949823u128 * 0 == 0);
/// assert!(16930269188653362184u128 * 0 == 0);
/// assert!(12725795890232059910u128 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u128_mul() {}
/// # Properties for [`u128::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u128::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_mul(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(0u128.checked_mul(1u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_mul(1u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128.checked_mul(1u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(1u128.checked_mul(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(0u128.checked_mul(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(6136768359858718133u128.checked_mul(15347231840232234668u128)
///         == Some(94182406768553467195007648652522834844u128));
/// assert!(10908014639762018711u128.checked_mul(12898676648544309170u128)
///         == Some(140698953715877815419831474130108879870u128));
/// assert!(12898939477827596505u128.checked_mul(17492212708577413464u128)
///         == Some(225630993061226789143676169232746343320u128));
/// assert!(9169616702823755461u128.checked_mul(5205773636491371835u128)
///         == Some(47734948888290844318897984598462840935u128));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .checked_mul(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_mul(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_mul(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_mul(340282366920938463463374607431768211454u128) == None);
/// assert!(221748335776609949256544284742171585881u128
///         .checked_mul(190362380640128762393949254536487829340u128) == None);
/// assert!(235301169679648115261743680948244280774u128
///         .checked_mul(306380302565617946266920760430976921209u128) == None);
/// assert!(204224071637174755534954224039238200006u128
///         .checked_mul(18936636328705431901592599959960136588u128) == None);
/// assert!(331896815991296150407279098636294031100u128
///         .checked_mul(88301723616399097115663870940643555475u128) == None);
/// assert!(205732473912876210947930349937433490002u128
///         .checked_mul(217134854608511427202510890834433496580u128) == None);
/// assert!(186993511356311878625085581084737497995u128
///         .checked_mul(186988225838583575157246587679914581726u128) == None);
/// # }
/// ```
pub fn core_ops_mul_u128_checked_mul() {}
/// # Properties for [`u128::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u128::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.overflowing_mul(0u128) == (0u128, false));
/// assert!(0u128.overflowing_mul(1u128) == (0u128, false));
/// assert!(1u128.overflowing_mul(0u128) == (0u128, false));
/// assert!(1u128.overflowing_mul(340282366920938463463374607431768211454u128)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(0u128.overflowing_mul(340282366920938463463374607431768211455u128) == (0u128, false));
/// assert!(10232124421129252380u128.overflowing_mul(3543015323713380429u128)
///         == (36252573618202843455000781467293671020u128, false));
/// assert!(17476534979218915543u128.overflowing_mul(16057178037469094367u128)
///         == (280623833639374366185447584432070046281u128, false));
/// assert!(14114500363151530245u128.overflowing_mul(7163060420961511969u128)
///         == (101103018912937613796017445791233002405u128, false));
/// assert!(12624718350544214166u128.overflowing_mul(14674724001239809405u128)
///         == (185264257387623837229420987591341031230u128, false));
/// assert!(15358422219417496785u128.overflowing_mul(16052642578017461333u128)
///         == (246543262450590745773796959707389314405u128, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .overflowing_mul(340282366920938463463374607431768211455u128) == (1u128, true));
/// assert!(340282366920938463463374607431768211455u128
///         .overflowing_mul(340282366920938463463374607431768211454u128) == (2u128, true));
/// assert!(340282366920938463463374607431768211454u128
///         .overflowing_mul(340282366920938463463374607431768211454u128) == (4u128, true));
/// assert!(340282366920938463463374607431768211454u128
///         .overflowing_mul(340282366920938463463374607431768211455u128) == (2u128, true));
/// assert!(310954860234253790375557447810410877799u128
///         .overflowing_mul(226927644053716082321860327931908780573u128)
///         == (244777656477996203249715596690694181035u128, true));
/// assert!(167920220952563351143601447276961004428u128
///         .overflowing_mul(299536295762764318898323921910384642397u128)
///         == (214923837979726671280863916090548833756u128, true));
/// assert!(138543574234681334672951903533922172796u128
///         .overflowing_mul(258888029181179654030472657407166703418u128)
///         == (332555086589794282446058693665100940824u128, true));
/// assert!(335615163000578191229458544133477527531u128
///         .overflowing_mul(336628999946098032995343665408288354377u128)
///         == (151527855186622985339477183373267909123u128, true));
/// assert!(284713051555731914345164621201332882259u128
///         .overflowing_mul(217588777521048705942790393029520501903u128)
///         == (27483448718519506409865641814242449245u128, true));
/// assert!(67658192404142144548736579809034720123u128
///         .overflowing_mul(210940770502952007147326084965114393285u128)
///         == (68115051404768832067596068627719275431u128, true));
/// # }
/// ```
pub fn core_ops_mul_u128_overflowing_mul() {}
/// # Properties for [`u128::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::down((x.up() * y.up()).min(u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_mul(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.saturating_mul(0u128) == 0u128);
/// assert!(0u128.saturating_mul(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_mul(0u128) == 0u128);
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_mul(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(258691584727659225991160373492104849172u128
///         .saturating_mul(333778984771286831165040166391037869452u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(156840435759198852685381607872266659220u128
///         .saturating_mul(220563139729595266865516483127411526394u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(234690958390017073310395520422756067460u128
///         .saturating_mul(145497210326973168460841626882359209422u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(287884363366809805761964455233485125326u128
///         .saturating_mul(5322770066737616280328414963755361223u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_mul(1u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_mul(0u128) == 0u128);
/// assert!(18168050045269303466u128.saturating_mul(8261079935280064872u128)
///         == 150087713692138317144816496390734446352u128);
/// assert!(7274003378862387191u128.saturating_mul(13753017313961715456u128)
///         == 100039494411310430758780619241641124096u128);
/// assert!(15710251315056545666u128.saturating_mul(13024228629657693130u128)
///         == 204613904956676885177449227560059474580u128);
/// assert!(16016299688410010759u128.saturating_mul(14394901454365648286u128)
///         == 230553055678249343352014198085269909074u128);
/// assert!(1047053115119855844u128.saturating_mul(2874681260604707387u128)
///         == 3009943968892833001942621659241919628u128);
/// assert!(6888895277811543253u128.saturating_mul(13114439470974434990u128)
///         == 90344000142741098659900734281621622470u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_mul(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_mul(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_mul(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_mul(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(152655857682783739587724770132257899302u128
///         .saturating_mul(871986886696027129086090492793655398u128) == u128::MAX);
/// assert!(200067121025386001049477070757751746616u128
///         .saturating_mul(205278458886601658283074201746925767725u128) == u128::MAX);
/// assert!(262293487562446711520297159954927324048u128
///         .saturating_mul(125767360997567494472567630003926370127u128) == u128::MAX);
/// assert!(61157613018998877327259364970692897676u128
///         .saturating_mul(219182754227738990891335401600982023503u128) == u128::MAX);
/// assert!(340221532097369931781236547007425212446u128
///         .saturating_mul(21344994603407580659752222081881278726u128) == u128::MAX);
/// assert!(138294415375063936906823893086764848731u128
///         .saturating_mul(264313627132524095141221102690857120451u128) == u128::MAX);
/// # }
/// ```
pub fn core_ops_mul_u128_saturating_mul() {}
/// # Properties for [`u128::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u128::down((x.up() * y.up()) % (u128::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_mul(0u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.wrapping_mul(1u128) == 1u128);
/// assert!(0u128.wrapping_mul(1u128) == 0u128);
/// assert!(263616032452553687270295487716874772498u128
///         .wrapping_mul(4508290671758962265415366721844865142u128)
///         == 227718586600316307431114386415288424524u128);
/// assert!(188723645813966286531179686878228457311u128
///         .wrapping_mul(252925568161844301176686774724042229795u128)
///         == 308123296179079730816395373477427960317u128);
/// assert!(48699971980791010247762668180654188920u128
///         .wrapping_mul(128456602697808343957457783445642563854u128)
///         == 295099549498001540026031803650131606672u128);
/// assert!(277192764552050625733830776750595075742u128
///         .wrapping_mul(251049334956711480609755339263496624964u128)
///         == 41282205591513492879706950989199051768u128);
/// assert!(226186798002402420484922554122071609742u128
///         .wrapping_mul(314207760782124192382126882836431059146u128)
///         == 241727327662032401110584110116921141772u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(1u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(0u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(6706775253665056244u128.wrapping_mul(10322403795401668106u128)
///         == 69230042333338161945461358679910953864u128);
/// assert!(17248674751419608654u128.wrapping_mul(12446606697342508389u128)
///         == 214687470681301927131489261461091998406u128);
/// assert!(4160913603170403992u128.wrapping_mul(7756968626799266473u128)
///         == 32276076278615116637752885408670960216u128);
/// assert!(17973624695585511250u128.wrapping_mul(15508857771688031017u128)
///         == 278750389045535276823050438465802441250u128);
/// assert!(11337436215167763360u128.wrapping_mul(11498826207341162568u128)
///         == 130367208675029877079245210218713908480u128);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128 * 1
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128 * 1 == 1u128);
/// assert!(0u128 * 1 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 * 1
///         == 340282366920938463463374607431768211455u128);
/// assert!(92863575057343933443545159766182736853u128 * 1
///         == 92863575057343933443545159766182736853u128);
/// assert!(198752970825428019385407160876006302092u128 * 1
///         == 198752970825428019385407160876006302092u128);
/// assert!(287668735832803286290181217460546414343u128 * 1
///         == 287668735832803286290181217460546414343u128);
/// assert!(84047572791268238329799534663139296567u128 * 1
///         == 84047572791268238329799534663139296567u128);
/// assert!(59901799106292277935294546515479776520u128 * 1
///         == 59901799106292277935294546515479776520u128);
/// assert!(302008068677625209419888759010573610748u128 * 1
///         == 302008068677625209419888759010573610748u128);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0u128 == 0u128);
/// assert!(1 * 1u128 == 1u128);
/// assert!(1 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(1 * 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1 * 124938293316028451678258228193558418771u128
///         == 124938293316028451678258228193558418771u128);
/// assert!(1 * 97832907324038184495489921042580196548u128
///         == 97832907324038184495489921042580196548u128);
/// assert!(1 * 96704978626616652805801710737304681515u128
///         == 96704978626616652805801710737304681515u128);
/// assert!(1 * 67483562640087098402294644169981875089u128
///         == 67483562640087098402294644169981875089u128);
/// assert!(1 * 15652904113442507816759326997384029866u128
///         == 15652904113442507816759326997384029866u128);
/// assert!(1 * 164011194762317928508866230883293220507u128
///         == 164011194762317928508866230883293220507u128);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128 * 0u128);
/// assert!(340282366920938463463374607431768211454u128 * 1u128
///         == 1u128 * 340282366920938463463374607431768211454u128);
/// assert!(0u128 * 0u128 == 0u128 * 0u128);
/// assert!(1u128 * 0u128 == 0u128 * 1u128);
/// assert!(0u128 * 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128 * 0u128);
/// assert!(6218433177738553010u128 * 12934635555881116211u128
///         == 12934635555881116211u128 * 6218433177738553010u128);
/// assert!(17163154773858441826u128 * 12544206052579421996u128
///         == 12544206052579421996u128 * 17163154773858441826u128);
/// assert!(15935218679389863603u128 * 10897245658253586361u128
///         == 10897245658253586361u128 * 15935218679389863603u128);
/// assert!(17043376793527952834u128 * 17448616984493853765u128
///         == 17448616984493853765u128 * 17043376793527952834u128);
/// assert!(5388883090749838654u128 * 17654972217719228667u128
///         == 17654972217719228667u128 * 5388883090749838654u128);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= u128::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1u128 * 340282366920938463463374607431768211455u128) * 1u128
///         == 1u128 * (340282366920938463463374607431768211455u128 * 1u128));
/// assert!((1u128 * 1u128) * 340282366920938463463374607431768211455u128
///         == 1u128 * (1u128 * 340282366920938463463374607431768211455u128));
/// assert!((340282366920938463463374607431768211454u128 * 1u128) * 1u128
///         == 340282366920938463463374607431768211454u128 * (1u128 * 1u128));
/// assert!((1u128 * 340282366920938463463374607431768211454u128) * 1u128
///         == 1u128 * (340282366920938463463374607431768211454u128 * 1u128));
/// assert!((1u128 * 1u128) * 1u128 == 1u128 * (1u128 * 1u128));
/// assert!((340282366920938463463374607431768211455u128 * 1u128) * 1u128
///         == 340282366920938463463374607431768211455u128 * (1u128 * 1u128));
/// assert!((5120867387914u128 * 5870353122219u128) * 4818145389142u128
///         == 5120867387914u128 * (5870353122219u128 * 4818145389142u128));
/// assert!((5947303823336u128 * 5614300123937u128) * 6733279833105u128
///         == 5947303823336u128 * (5614300123937u128 * 6733279833105u128));
/// assert!((6402245170027u128 * 6044716485995u128) * 3479939530010u128
///         == 6402245170027u128 * (6044716485995u128 * 3479939530010u128));
/// assert!((6684206578512u128 * 6515075090789u128) * 4868562722436u128
///         == 6684206578512u128 * (6515075090789u128 * 4868562722436u128));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= u128::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128 * (340282366920938463463374607431768211455u128 + 0u128)
///         == 1u128 * 340282366920938463463374607431768211455u128 + 1u128 * 0u128);
/// assert!(1u128 * (1u128 + 340282366920938463463374607431768211454u128)
///         == 1u128 * 1u128 + 1u128 * 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128 * (1u128 + 0u128)
///         == 340282366920938463463374607431768211455u128 * 1u128
///             + 340282366920938463463374607431768211455u128 * 0u128);
/// assert!(340282366920938463463374607431768211455u128 * (0u128 + 0u128)
///         == 340282366920938463463374607431768211455u128 * 0u128
///             + 340282366920938463463374607431768211455u128 * 0u128);
/// assert!(1u128 * (0u128 + 0u128) == 1u128 * 0u128 + 1u128 * 0u128);
/// assert!(340282366920938463463374607431768211454u128 * (0u128 + 1u128)
///         == 340282366920938463463374607431768211454u128 * 0u128
///             + 340282366920938463463374607431768211454u128 * 1u128);
/// assert!(4411281305389873256u128 * (18333013477917433226u128 + 12758968112387997964u128)
///         == 4411281305389873256u128 * 18333013477917433226u128
///             + 4411281305389873256u128 * 12758968112387997964u128);
/// assert!(18292798235004740926u128 * (4184227115489703159u128 + 5135888625247464584u128)
///         == 18292798235004740926u128 * 4184227115489703159u128
///             + 18292798235004740926u128 * 5135888625247464584u128);
/// assert!(12843481744384164754u128 * (5847697820990077348u128 + 12415087972943749689u128)
///         == 12843481744384164754u128 * 5847697820990077348u128
///             + 12843481744384164754u128 * 12415087972943749689u128);
/// assert!(3115621852453512841u128 * (9034150424233014682u128 + 13065566186211447271u128)
///         == 3115621852453512841u128 * 9034150424233014682u128
///             + 3115621852453512841u128 * 13065566186211447271u128);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 0 == 0);
/// assert!(1u128 * 0 == 0);
/// assert!(340282366920938463463374607431768211455u128 * 0 == 0);
/// assert!(340282366920938463463374607431768211454u128 * 0 == 0);
/// assert!(9074530936302340601u128 * 0 == 0);
/// assert!(10789341955272310077u128 * 0 == 0);
/// assert!(8059837386490433772u128 * 0 == 0);
/// assert!(3150631987291184869u128 * 0 == 0);
/// assert!(14832101392909100683u128 * 0 == 0);
/// assert!(17465440595389125054u128 * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_u128_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u128>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128 - 0u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128 - 0u128 == 0u128);
/// assert!(1u128 - 1u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211454u128 == 1u128);
/// assert!(340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 - 1u128
///         == 340282366920938463463374607431768211453u128);
/// assert!(273057541233507799794724847181187607670u128
///         - 46460466433426485803696050538714220470u128
///         == 226597074800081313991028796642473387200u128);
/// assert!(135108172782770294634266409448827808070u128
///         - 134688091027611036962376553338379371769u128
///         == 420081755159257671889856110448436301u128);
/// assert!(188775543919531974697980069312625423650u128
///         - 79144671940445688615713701601441565259u128
///         == 109630871979086286082266367711183858391u128);
/// assert!(74788819890941689912622997863910095562u128
///         - 72399110646721061240961443131344047413u128
///         == 2389709244220628671661554732566048149u128);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u128 - 1u128));
/// assert!(panics!(0u128 - 340282366920938463463374607431768211454u128));
/// assert!(panics!(1u128 - 340282366920938463463374607431768211455u128));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 -
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(1u128 - 340282366920938463463374607431768211454u128));
/// assert!(panics!(
///         113183370613588937027964116555833817313u128 -
///         236892324769422959653336210774436472791u128
///     ));
/// assert!(panics!(
///         82297879538332801547842780306335781347u128 -
///         128487334998991795939743270788309761317u128
///     ));
/// assert!(panics!(
///         34438888893699000798726751069633871025u128 -
///         169950208739500761650913002864052217541u128
///     ));
/// assert!(panics!(
///         65895352350454334812816763733322286454u128 -
///         102268116924293504586949185009420282797u128
///     ));
/// assert!(panics!(
///         235710397349482520894433702390545308305u128 -
///         310559502455964713142062720417867025681u128
///     ));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211454u128)
///         + 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211455u128);
/// assert!((1u128 - 1u128) + 1u128 == 1u128);
/// assert!((0u128 - 0u128) + 0u128 == 0u128);
/// assert!((340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211455u128)
///         + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!((1u128 - 0u128) + 0u128 == 1u128);
/// assert!((340282366920938463463374607431768211454u128 - 0u128) + 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!((110225965516026792674634314036343353319u128
///         - 26998285815748975702920094747705727499u128)
///         + 26998285815748975702920094747705727499u128
///         == 110225965516026792674634314036343353319u128);
/// assert!((239026410528557617391727640757449649807u128
///         - 124086240998091363361341490799676326177u128)
///         + 124086240998091363361341490799676326177u128
///         == 239026410528557617391727640757449649807u128);
/// assert!((270624730875081724668429352008998919288u128
///         - 73706082992761777665024954620093180918u128)
///         + 73706082992761777665024954620093180918u128
///         == 270624730875081724668429352008998919288u128);
/// assert!((232546856115377425073132012333025109983u128
///         - 84019177834514515382631798605380645345u128)
///         + 84019177834514515382631798605380645345u128
///         == 232546856115377425073132012333025109983u128);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 - 0 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 - 0
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 - 0 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 - 0
///         == 340282366920938463463374607431768211454u128);
/// assert!(102113077015681685439940293961327685316u128 - 0
///         == 102113077015681685439940293961327685316u128);
/// assert!(232143579340495455340564653734530585074u128 - 0
///         == 232143579340495455340564653734530585074u128);
/// assert!(230578159457531543485743650564392778084u128 - 0
///         == 230578159457531543485743650564392778084u128);
/// assert!(51964755331951094876823476764326328018u128 - 0
///         == 51964755331951094876823476764326328018u128);
/// assert!(190704915079162475221843123736397469462u128 - 0
///         == 190704915079162475221843123736397469462u128);
/// assert!(252374353068261827444968925591396553691u128 - 0
///         == 252374353068261827444968925591396553691u128);
/// # }
/// ```
pub fn core_ops_add_u128_sub() {}
/// # Properties for [`u128::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_sub(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(0u128.wrapping_sub(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(1u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_sub(1u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(331212102269665705423070394652588756454u128
///         .wrapping_sub(265983025411672115516787985297771616866u128)
///         == 65229076857993589906282409354817139588u128);
/// assert!(310787618222722702417388155513825930283u128
///         .wrapping_sub(54078497135665599492774104073159880701u128)
///         == 256709121087057102924614051440666049582u128);
/// assert!(219457690023692169263346807525226151943u128
///         .wrapping_sub(92776153890829518341998243460376486936u128)
///         == 126681536132862650921348564064849665007u128);
/// assert!(305957126811589579516926325646657526884u128
///         .wrapping_sub(92837606085896084196276515964515836843u128)
///         == 213119520725693495320649809682141690041u128);
/// assert!(148821587805439394773725988333198410208u128
///         .wrapping_sub(104685778720858036726188147932771991029u128)
///         == 44135809084581358047537840400426419179u128);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down(x.up() - y.up() + u128::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211455u128) == 1u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211454u128) == 2u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211454u128) == 3u128);
/// assert!(0u128.wrapping_sub(1u128) == 340282366920938463463374607431768211455u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211455u128) == 2u128);
/// assert!(177044139791112945759573212307361148384u128
///         .wrapping_sub(241469999143486472821378652436560949594u128)
///         == 275856507568564936401569167302568410246u128);
/// assert!(137478682839344930936042246487581872462u128
///         .wrapping_sub(261127493742794488304209686888910111305u128)
///         == 216633556017488906095207167030439972613u128);
/// assert!(2464010611453571330923405758802004797u128
///         .wrapping_sub(59812727716714663980238504706783260681u128)
///         == 282933649815677370814059508483786955572u128);
/// assert!(259357198523430506819069054191482485258u128
///         .wrapping_sub(334281119188594597773000225246573360107u128)
///         == 265358446255774372509443436376677336607u128);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down((x.up() - y.up()).rem_euclid(&(u128::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.wrapping_sub(0u128) == 1u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211454u128) == 2u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.wrapping_sub(1u128) == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(207703375200517748833624692579381908713u128
///         .wrapping_sub(53529135102909735398431280815349830944u128)
///         == 154174240097608013435193411764032077769u128);
/// assert!(307331477918717793987835947365105215944u128
///         .wrapping_sub(172268591273852345608335912481116139157u128)
///         == 135062886644865448379500034883989076787u128);
/// assert!(183244842805569463389496178447702282867u128
///         .wrapping_sub(58380316007043025663539560304075713301u128)
///         == 124864526798526437725956618143626569566u128);
/// assert!(3162389274128888343294188633849927789u128
///         .wrapping_sub(261808337988844113755620099845714789531u128)
///         == 81636418206223238051048696219903349714u128);
/// assert!(41725933513615714439113931458847330389u128
///         .wrapping_sub(3903902621858827184033760846282829249u128)
///         == 37822030891756887255080170612564501140u128);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211455u128))
///         .wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!((1u128.wrapping_sub(0u128)).wrapping_add(0u128) == 1u128);
/// assert!((0u128.wrapping_sub(1u128)).wrapping_add(1u128) == 0u128);
/// assert!((0u128.wrapping_sub(340282366920938463463374607431768211455u128))
///         .wrapping_add(340282366920938463463374607431768211455u128) == 0u128);
/// assert!((0u128.wrapping_sub(0u128)).wrapping_add(0u128) == 0u128);
/// assert!((259734992844393051246380482411162665237u128
///         .wrapping_sub(188837477896519390799433729256259552589u128))
///         .wrapping_add(188837477896519390799433729256259552589u128)
///         == 259734992844393051246380482411162665237u128);
/// assert!((29709845853722577012128373805678015662u128
///         .wrapping_sub(118866466645827754439086724730695440496u128))
///         .wrapping_add(118866466645827754439086724730695440496u128)
///         == 29709845853722577012128373805678015662u128);
/// assert!((96604707123824198698594102162956550349u128
///         .wrapping_sub(310540327424685392997130612861691291670u128))
///         .wrapping_add(310540327424685392997130612861691291670u128)
///         == 96604707123824198698594102162956550349u128);
/// assert!((17594362172487504807612616649709452636u128
///         .wrapping_sub(191308190853991744546611311918556684196u128))
///         .wrapping_add(191308190853991744546611311918556684196u128)
///         == 17594362172487504807612616649709452636u128);
/// assert!((158319271609598130604213159460960930220u128
///         .wrapping_sub(117028825160792731473913600148386753744u128))
///         .wrapping_add(117028825160792731473913600148386753744u128)
///         == 158319271609598130604213159460960930220u128);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.wrapping_sub(0) == 1u128);
/// assert!(0u128.wrapping_sub(0) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_sub(0)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(0)
///         == 340282366920938463463374607431768211454u128);
/// assert!(296842306751223418847129440026079402460u128.wrapping_sub(0)
///         == 296842306751223418847129440026079402460u128);
/// assert!(125763885868815211103411711606252770435u128.wrapping_sub(0)
///         == 125763885868815211103411711606252770435u128);
/// assert!(208830102679249089206257293293923307653u128.wrapping_sub(0)
///         == 208830102679249089206257293293923307653u128);
/// assert!(60440491316098117593351879995532430705u128.wrapping_sub(0)
///         == 60440491316098117593351879995532430705u128);
/// assert!(8207322446052094959614503653303215546u128.wrapping_sub(0)
///         == 8207322446052094959614503653303215546u128);
/// assert!(84928516961523580438020624670252398493u128.wrapping_sub(0)
///         == 84928516961523580438020624670252398493u128);
/// # }
/// ```
pub fn core_ops_add_u128_wrapping_sub() {}
/// # Properties for [`u128::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u128::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.checked_sub(1u128)
///         == Some(340282366920938463463374607431768211453u128));
/// assert!(340282366920938463463374607431768211455u128.checked_sub(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_sub(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(0u128.checked_sub(0u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_sub(1u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(1u128.checked_sub(1u128) == Some(0u128));
/// assert!(238805007175376601584641411845903771770u128
///         .checked_sub(146741827841805168343665352220083218949u128)
///         == Some(92063179333571433240976059625820552821u128));
/// assert!(78508116052516106470384164246921613084u128
///         .checked_sub(27982112545810563998520362793202562090u128)
///         == Some(50526003506705542471863801453719050994u128));
/// assert!(88149572985622071122553638621200991013u128
///         .checked_sub(58321643839337179385313664344181587028u128)
///         == Some(29827929146284891737239974277019403985u128));
/// assert!(100987158430301709211921816499785890651u128
///         .checked_sub(63030254878935990468013065593438584406u128)
///         == Some(37956903551365718743908750906347306245u128));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_sub(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_sub(340282366920938463463374607431768211455u128) == None);
/// assert!(0u128.checked_sub(1u128) == None);
/// assert!(0u128.checked_sub(340282366920938463463374607431768211455u128) == None);
/// assert!(1u128.checked_sub(340282366920938463463374607431768211454u128) == None);
/// assert!(3573155333155607439973779129569079563u128
///         .checked_sub(295099082308046163133549843217490607341u128) == None);
/// assert!(90393277041472185105602856517476929333u128
///         .checked_sub(123486629688607242816543740178982449194u128) == None);
/// assert!(120826033967346355946613454509464297492u128
///         .checked_sub(309736537325948896290458410665516657999u128) == None);
/// assert!(168945371201032833571013500763894358262u128
///         .checked_sub(222854800006775922049790743498845978652u128) == None);
/// assert!(128912419501300525373252954299520902321u128
///         .checked_sub(251615150623898974087978002169231852453u128) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u128.checked_sub(0u128)).and_then(|r| r.checked_add(0u128)) == Some(0u128));
/// assert!((340282366920938463463374607431768211455u128
///         .checked_sub(340282366920938463463374607431768211455u128))
///         .and_then(|r| r.checked_add(340282366920938463463374607431768211455u128))
///         == Some(340282366920938463463374607431768211455u128));
/// assert!((340282366920938463463374607431768211455u128.checked_sub(0u128))
///         .and_then(|r| r.checked_add(0u128))
///         == Some(340282366920938463463374607431768211455u128));
/// assert!((1u128.checked_sub(1u128)).and_then(|r| r.checked_add(1u128)) == Some(1u128));
/// assert!((307738931443466688447833788417334376736u128
///         .checked_sub(123404696879381453154818650016462169207u128))
///         .and_then(|r| r.checked_add(123404696879381453154818650016462169207u128))
///         == Some(307738931443466688447833788417334376736u128));
/// assert!((296820961309910155576870179870545267253u128
///         .checked_sub(155148029234714390675344023716945520990u128))
///         .and_then(|r| r.checked_add(155148029234714390675344023716945520990u128))
///         == Some(296820961309910155576870179870545267253u128));
/// assert!((119791999257336495746335682330627997367u128
///         .checked_sub(58755941792674403763966194639549883373u128))
///         .and_then(|r| r.checked_add(58755941792674403763966194639549883373u128))
///         == Some(119791999257336495746335682330627997367u128));
/// assert!((77674113626624670245487546541793060744u128
///         .checked_sub(49006980797447701125864043755349655531u128))
///         .and_then(|r| r.checked_add(49006980797447701125864043755349655531u128))
///         == Some(77674113626624670245487546541793060744u128));
/// assert!((257493873211306374617510930773382620962u128
///         .checked_sub(42587963522908801517781600405461077346u128))
///         .and_then(|r| r.checked_add(42587963522908801517781600405461077346u128))
///         == Some(257493873211306374617510930773382620962u128));
/// assert!((291504101662103912241597043271972172251u128
///         .checked_sub(73767101877308461738834277073002758735u128))
///         .and_then(|r| r.checked_add(73767101877308461738834277073002758735u128))
///         == Some(291504101662103912241597043271972172251u128));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.checked_sub(0) == Some(1u128));
/// assert!(340282366920938463463374607431768211454u128.checked_sub(0)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128.checked_sub(0)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(0u128.checked_sub(0) == Some(0u128));
/// assert!(58248140096010563214276624709667003393u128.checked_sub(0)
///         == Some(58248140096010563214276624709667003393u128));
/// assert!(233033253878850905527726372579491873861u128.checked_sub(0)
///         == Some(233033253878850905527726372579491873861u128));
/// assert!(321644061561264169155837343187837950989u128.checked_sub(0)
///         == Some(321644061561264169155837343187837950989u128));
/// assert!(124922121132025500670959031561818016353u128.checked_sub(0)
///         == Some(124922121132025500670959031561818016353u128));
/// assert!(130229793495196763542320204450724117273u128.checked_sub(0)
///         == Some(130229793495196763542320204450724117273u128));
/// assert!(286147228978110681187709145664224590161u128.checked_sub(0)
///         == Some(286147228978110681187709145664224590161u128));
/// # }
/// ```
pub fn core_ops_add_u128_checked_sub() {}
/// # Properties for [`u128::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u128::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.saturating_sub(1u128) == 0u128);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_sub(0u128) == 0u128);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(212771345942265575709097622646549462788u128
///         .saturating_sub(212940492699729623059057010602793944939u128) == 0u128);
/// assert!(88651915468057252608315744109464897238u128
///         .saturating_sub(37212010419471683334601529186001298444u128)
///         == 51439905048585569273714214923463598794u128);
/// assert!(294028574184221987844192650193766999457u128
///         .saturating_sub(190818468687017625853429609951681843199u128)
///         == 103210105497204361990763040242085156258u128);
/// assert!(10151632302752216511708251664948717061u128
///         .saturating_sub(226389655873033814261106634099658298070u128) == 0u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_sub(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_sub(340282366920938463463374607431768211454u128) == 1u128);
/// assert!(1u128.saturating_sub(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_sub(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(283417187596155849625269411893302294151u128
///         .saturating_sub(168484208688893961706920038907756986704u128)
///         == 114932978907261887918349372985545307447u128);
/// assert!(303499037418939014609107585039341885913u128
///         .saturating_sub(167762839240960406187307124179759697664u128)
///         == 135736198177978608421800460859582188249u128);
/// assert!(233093824466364747930847119814261143339u128
///         .saturating_sub(17713284334274336407230004715362474918u128)
///         == 215380540132090411523617115098898668421u128);
/// assert!(228703798211963700770339586287806312881u128
///         .saturating_sub(202688591610183142496593365221095757245u128)
///         == 26015206601780558273746221066710555636u128);
/// assert!(279310912651501949904231362542449972223u128
///         .saturating_sub(193053628781932019000526354421061950916u128)
///         == 86257283869569930903705008121388021307u128);
/// assert!(129166705038325305016847615830857668492u128
///         .saturating_sub(63074862231054991631867323905581991427u128)
///         == 66091842807270313384980291925275677065u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211454u128) == 0);
/// assert!(0u128.saturating_sub(1u128) == 0);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211455u128) == 0);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211454u128) == 0);
/// assert!(29108012290890390052269254938395357452u128
///         .saturating_sub(86927524167343890209446733157318089075u128) == 0);
/// assert!(149782262324170800553744029757852099634u128
///         .saturating_sub(249940680675008946507357696349261692435u128) == 0);
/// assert!(41408892082187155529703700007354167343u128
///         .saturating_sub(151013982893428427069922389849045696998u128) == 0);
/// assert!(197828722165431089600002215202170885693u128
///         .saturating_sub(267108352084172017528154524854891298521u128) == 0);
/// assert!(65271919398206813884420043203621990066u128
///         .saturating_sub(303521788259598006211644396972251908090u128) == 0);
/// assert!(181925156618926734138796026523559226915u128
///         .saturating_sub(221117934993052571917490212672703848215u128) == 0);
/// # }
/// ```
pub fn core_ops_sub_u128_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u128>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x + y == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 + 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128 + 0u128 == 1u128);
/// assert!(0u128 + 1u128 == 1u128);
/// assert!(0u128 + 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(31260428026565787699205391969663969716u128
///         + 8813049861258178571983467463330261825u128
///         == 40073477887823966271188859432994231541u128);
/// assert!(26579532723013893507528579586811191051u128
///         + 176615356947651334767459367617919468626u128
///         == 203194889670665228274987947204730659677u128);
/// assert!(23203015145443803039863053781828558910u128
///         + 203971162721054368374408170395367296643u128
///         == 227174177866498171414271224177195855553u128);
/// assert!(8490538342001926739051480324506348141u128
///         + 146139455483337632769053827213470893033u128
///         == 154629993825339559508105307537977241174u128);
/// assert!(54920313343234889718480393564112832945u128
///         + 74401548510693126373451374013171631979u128
///         == 129321861853928016091931767577284464924u128);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u128 + 340282366920938463463374607431768211455u128));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 +
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 +
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 +
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 +
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(340282366920938463463374607431768211455u128 + 1u128));
/// assert!(panics!(
///         182813364775676563809991783484926046025u128 +
///         270620275997962997364150824793351585417u128
///     ));
/// assert!(panics!(
///         220435349177773028030515436355598333179u128 +
///         135516717906010365521352370962619703062u128
///     ));
/// assert!(panics!(
///         116215947613701520125274909778878696858u128 +
///         300143555299757002364276855524623450771u128
///     ));
/// assert!(panics!(
///         204329396716370137292733000541494412344u128 +
///         174724141291565067838311617102512293772u128
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128 + 0u128
///         == 0u128 + 340282366920938463463374607431768211455u128);
/// assert!(0u128 + 0u128 == 0u128 + 0u128);
/// assert!(0u128 + 1u128 == 1u128 + 0u128);
/// assert!(1u128 + 0u128 == 0u128 + 1u128);
/// assert!(340282366920938463463374607431768211454u128 + 0u128
///         == 0u128 + 340282366920938463463374607431768211454u128);
/// assert!(132534807192017487123877976253456672529u128
///         + 86374685773110776279825295033832528845u128
///         == 86374685773110776279825295033832528845u128
///             + 132534807192017487123877976253456672529u128);
/// assert!(193838758556683180612400828235032747865u128
///         + 110564974964708816997038317174737096619u128
///         == 110564974964708816997038317174737096619u128
///             + 193838758556683180612400828235032747865u128);
/// assert!(23838838131199547712973586907957920591u128
///         + 182072397139060281078413519110069659085u128
///         == 182072397139060281078413519110069659085u128
///             + 23838838131199547712973586907957920591u128);
/// assert!(281417698294249655417739856526533250397u128
///         + 55288720168893826058721509085078642261u128
///         == 55288720168893826058721509085078642261u128
///             + 281417698294249655417739856526533250397u128);
/// assert!(81437501676382383541757664652001679235u128
///         + 66860497665477457406970272320051459246u128
///         == 66860497665477457406970272320051459246u128
///             + 81437501676382383541757664652001679235u128);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 + 0
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 + 0 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 + 0
///         == 340282366920938463463374607431768211454u128);
/// assert!(212370292985986765105212048148260531385u128 + 0
///         == 212370292985986765105212048148260531385u128);
/// assert!(67771181602578205616749005235766596184u128 + 0
///         == 67771181602578205616749005235766596184u128);
/// assert!(339445703344308354334043753011425119791u128 + 0
///         == 339445703344308354334043753011425119791u128);
/// assert!(45248522375968810379034228292674061463u128 + 0
///         == 45248522375968810379034228292674061463u128);
/// assert!(112840324329566792291820000193893145995u128 + 0
///         == 112840324329566792291820000193893145995u128);
/// assert!(50526360278671693303338594509923181584u128 + 0
///         == 50526360278671693303338594509923181584u128);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u128 == 0u128);
/// assert!(0 + 1u128 == 1u128);
/// assert!(0 + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(0 + 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(0 + 120870557475871112294413400453042418199u128
///         == 120870557475871112294413400453042418199u128);
/// assert!(0 + 201287459350042088859177076472916879293u128
///         == 201287459350042088859177076472916879293u128);
/// assert!(0 + 14629941313787401990914527735304414553u128
///         == 14629941313787401990914527735304414553u128);
/// assert!(0 + 133004994005792120092963278370266013415u128
///         == 133004994005792120092963278370266013415u128);
/// assert!(0 + 276370362254611122677357305962264476403u128
///         == 276370362254611122677357305962264476403u128);
/// assert!(0 + 246686880293652174700357539518937784865u128
///         == 246686880293652174700357539518937784865u128);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u128::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u128 + 1u128) + 1u128 == 0u128 + (1u128 + 1u128));
/// assert!((0u128 + 0u128) + 0u128 == 0u128 + (0u128 + 0u128));
/// assert!((0u128 + 1u128) + 340282366920938463463374607431768211454u128
///         == 0u128 + (1u128 + 340282366920938463463374607431768211454u128));
/// assert!((340282366920938463463374607431768211455u128 + 0u128) + 0u128
///         == 340282366920938463463374607431768211455u128 + (0u128 + 0u128));
/// assert!((0u128 + 340282366920938463463374607431768211454u128) + 0u128
///         == 0u128 + (340282366920938463463374607431768211454u128 + 0u128));
/// assert!((13850944368636022689886989930357446312u128
///         + 41308621811060969903087803260932799296u128)
///         + 13633231115404982301636023669268764262u128
///         == 13850944368636022689886989930357446312u128
///             + (41308621811060969903087803260932799296u128
///                 + 13633231115404982301636023669268764262u128));
/// assert!((32473215462105490102978288058764778696u128
///         + 172990129179665102563111140055585644904u128)
///         + 36987517190095272507047495476649167180u128
///         == 32473215462105490102978288058764778696u128
///             + (172990129179665102563111140055585644904u128
///                 + 36987517190095272507047495476649167180u128));
/// assert!((99830676592182027448964708855620733970u128
///         + 93643604002335929658571950043813001139u128)
///         + 55981340601821579126183600916641486842u128
///         == 99830676592182027448964708855620733970u128
///             + (93643604002335929658571950043813001139u128
///                 + 55981340601821579126183600916641486842u128));
/// assert!((169660475862904220856841838160641905523u128
///         + 89265690721450668338110978179135362184u128)
///         + 40003477983245713550545853265347387954u128
///         == 169660475862904220856841838160641905523u128
///             + (89265690721450668338110978179135362184u128
///                 + 40003477983245713550545853265347387954u128));
/// assert!((71799649227770285008050941400495975296u128
///         + 61607187008428344840134677187574222185u128)
///         + 170582227119066979786688466224991643350u128
///         == 71799649227770285008050941400495975296u128
///             + (61607187008428344840134677187574222185u128
///                 + 170582227119066979786688466224991643350u128));
/// # }
/// ```
pub fn core_ops_add_u128_add() {}
/// # Properties for [`u128::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down((x.up() + y.up()) % (u128::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.wrapping_add(0u128) == 0u128);
/// assert!(1u128.wrapping_add(1u128) == 2u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(305674280039254554516286772643961034497u128
///         .wrapping_add(53716616980298852424431398390420090066u128)
///         == 19108530098614943477343563602612913107u128);
/// assert!(208670115694446712201571079250061793117u128
///         .wrapping_add(172036069796874673263709971028256409547u128)
///         == 40423818570382922001906442846549991208u128);
/// assert!(155323720221460856783618009450971859591u128
///         .wrapping_add(44798499831292699032161359742237549136u128)
///         == 200122220052753555815779369193209408727u128);
/// assert!(170723693204022218187123532409121246419u128
///         .wrapping_add(141911901042273061975570870139696653516u128)
///         == 312635594246295280162694402548817899935u128);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.wrapping_add(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(13945721021670868598142203535716809186u128
///         .wrapping_add(131975101497011796041434791910752901488u128)
///         == 145920822518682664639576995446469710674u128);
/// assert!(32455107227483290787749419046876839787u128
///         .wrapping_add(88102531093079050559310993308540312884u128)
///         == 120557638320562341347060412355417152671u128);
/// assert!(181882469537861027945546788127507435108u128
///         .wrapping_add(40538215751460503975936135364036581308u128)
///         == 222420685289321531921482923491544016416u128);
/// assert!(183757819900560746604234388533702342426u128
///         .wrapping_add(96328991876432629548114815846209154909u128)
///         == 280086811776993376152349204379911497335u128);
/// assert!(77598097441976899571109907778763138180u128
///         .wrapping_add(67036102569100059324265607424853839263u128)
///         == 144634200011076958895375515203616977443u128);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down(x.up() + y.up() - u128::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(1u128) == 0u128);
/// assert!(1u128.wrapping_add(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211452u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(194003321927906320815372635188988715802u128
///         .wrapping_add(208823174820516834501289169634068300828u128)
///         == 62544129827484691853287197391288805174u128);
/// assert!(292029267685982102181708926581110546094u128
///         .wrapping_add(224750556637312336885653158952463119303u128)
///         == 176497457402355975603987478101805453941u128);
/// assert!(177127271006931873353893728115580462493u128
///         .wrapping_add(227241393602290531715360683060989400159u128)
///         == 64086297688283941605879803744801651196u128);
/// assert!(154076160959539635342025400456475063440u128
///         .wrapping_add(277810905731723909001685919935357560399u128)
///         == 91604699770325080880336712960064412383u128);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(0u128)
///         == 0u128.wrapping_add(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(1u128)
///         == 1u128.wrapping_add(340282366920938463463374607431768211454u128));
/// assert!(0u128.wrapping_add(0u128) == 0u128.wrapping_add(0u128));
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128
///             .wrapping_add(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(1u128)
///         == 1u128.wrapping_add(340282366920938463463374607431768211455u128));
/// assert!(44519606772400966076677875555967533312u128
///         .wrapping_add(149831387153514043729337106446511587297u128)
///         == 149831387153514043729337106446511587297u128
///             .wrapping_add(44519606772400966076677875555967533312u128));
/// assert!(132564707117559214188740531150018812439u128
///         .wrapping_add(65986664498791089376797380903191780773u128)
///         == 65986664498791089376797380903191780773u128
///             .wrapping_add(132564707117559214188740531150018812439u128));
/// assert!(124420467991881265241698223998321691943u128
///         .wrapping_add(243433605850307954264465036042909631956u128)
///         == 243433605850307954264465036042909631956u128
///             .wrapping_add(124420467991881265241698223998321691943u128));
/// assert!(247277000117004692738531503260757946475u128
///         .wrapping_add(295347211449493077912760871637732104962u128)
///         == 295347211449493077912760871637732104962u128
///             .wrapping_add(247277000117004692738531503260757946475u128));
/// assert!(180956882296782419368705635627271990813u128
///         .wrapping_add(51046521336685100335579808167789893636u128)
///         == 51046521336685100335579808167789893636u128
///             .wrapping_add(180956882296782419368705635627271990813u128));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_add(0) == 0u128);
/// assert!(1u128.wrapping_add(0) == 1u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(0)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(0)
///         == 340282366920938463463374607431768211455u128);
/// assert!(54133492327068534212689444164682200115u128.wrapping_add(0)
///         == 54133492327068534212689444164682200115u128);
/// assert!(117340417229796565695111050913478401557u128.wrapping_add(0)
///         == 117340417229796565695111050913478401557u128);
/// assert!(210847686842078248027124137107895408718u128.wrapping_add(0)
///         == 210847686842078248027124137107895408718u128);
/// assert!(97950583331651851568600387576231061122u128.wrapping_add(0)
///         == 97950583331651851568600387576231061122u128);
/// assert!(138823441328994184522831901166579679151u128.wrapping_add(0)
///         == 138823441328994184522831901166579679151u128);
/// assert!(199498747531517303902630071690238983832u128.wrapping_add(0)
///         == 199498747531517303902630071690238983832u128);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: u128 = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(340282366920938463463374607431768211455u128)
///             == 340282366920938463463374607431768211455u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(0u128) == 0u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(340282366920938463463374607431768211454u128)
///             == 340282366920938463463374607431768211454u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(1u128) == 1u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(268431079428811482461266076196561136236u128)
///             == 268431079428811482461266076196561136236u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(291025913271297372938301709911494923979u128)
///             == 291025913271297372938301709911494923979u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(173136822019023649960636041634046073324u128)
///             == 173136822019023649960636041634046073324u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(120374479850552809846439927914663124276u128)
///             == 120374479850552809846439927914663124276u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(152413701335038888239338181686678927488u128)
///             == 152413701335038888239338181686678927488u128
///     });
/// assert!({
///         let zero: u128 = 0;
///         zero.wrapping_add(194690606265189174104992635656545603131u128)
///             == 194690606265189174104992635656545603131u128
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u128, y : u128, z : u128`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= u128::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u128.wrapping_add(0u128)).wrapping_add(0u128)
///         == 0u128.wrapping_add(0u128.wrapping_add(0u128)));
/// assert!((0u128.wrapping_add(1u128)).wrapping_add(1u128)
///         == 0u128.wrapping_add(1u128.wrapping_add(1u128)));
/// assert!((0u128.wrapping_add(340282366920938463463374607431768211454u128)).wrapping_add(1u128)
///         == 0u128
///             .wrapping_add(
///                 340282366920938463463374607431768211454u128.wrapping_add(1u128),
///             ));
/// assert!((340282366920938463463374607431768211455u128.wrapping_add(0u128)).wrapping_add(0u128)
///         == 340282366920938463463374607431768211455u128
///             .wrapping_add(0u128.wrapping_add(0u128)));
/// assert!((0u128.wrapping_add(1u128)).wrapping_add(0u128)
///         == 0u128.wrapping_add(1u128.wrapping_add(0u128)));
/// assert!((0u128.wrapping_add(340282366920938463463374607431768211455u128)).wrapping_add(0u128)
///         == 0u128
///             .wrapping_add(
///                 340282366920938463463374607431768211455u128.wrapping_add(0u128),
///             ));
/// assert!((15504112552480245600253346032458649055u128
///         .wrapping_add(65562724468852487930788877145275726450u128))
///         .wrapping_add(597291081928901870437529320540504337u128)
///         == 15504112552480245600253346032458649055u128
///             .wrapping_add(
///                 65562724468852487930788877145275726450u128
///                     .wrapping_add(597291081928901870437529320540504337u128),
///             ));
/// assert!((11777338193848155079324584227671016340u128
///         .wrapping_add(43437745188559532856816356551943269362u128))
///         .wrapping_add(73606642365885032650253200504946917857u128)
///         == 11777338193848155079324584227671016340u128
///             .wrapping_add(
///                 43437745188559532856816356551943269362u128
///                     .wrapping_add(73606642365885032650253200504946917857u128),
///             ));
/// assert!((82571736886964476570984074426108394822u128
///         .wrapping_add(168718211781629955536926391812431874949u128))
///         .wrapping_add(11485477793605046245358031705730920761u128)
///         == 82571736886964476570984074426108394822u128
///             .wrapping_add(
///                 168718211781629955536926391812431874949u128
///                     .wrapping_add(11485477793605046245358031705730920761u128),
///             ));
/// assert!((21441519883273966463641844823564417866u128
///         .wrapping_add(102127254616773812043719882983259264250u128))
///         .wrapping_add(516467323985977652731342686499674497u128)
///         == 21441519883273966463641844823564417866u128
///             .wrapping_add(
///                 102127254616773812043719882983259264250u128
///                     .wrapping_add(516467323985977652731342686499674497u128),
///             ));
/// # }
/// ```
pub fn core_ops_add_u128_wrapping_add() {}
/// # Properties for [`u128::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u128::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_add(1u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(65954108048096303524253580763842493837u128
///         .checked_add(88085049930442823042121241641353376563u128)
///         == Some(154039157978539126566374822405195870400u128));
/// assert!(14727208866888862704592030955694999162u128
///         .checked_add(185396572441074612179017810442165915348u128)
///         == Some(200123781307963474883609841397860914510u128));
/// assert!(24613189143567993947160154875456991078u128
///         .checked_add(311534391345948082462225011194567666173u128)
///         == Some(336147580489516076409385166070024657251u128));
/// assert!(134824875089133711376550130278204937440u128
///         .checked_add(22172021395193790896548834056056292086u128)
///         == Some(156996896484327502273098964334261229526u128));
/// assert!(21407464511793611625344611928171036522u128
///         .checked_add(16535514371309833374705768981551342280u128)
///         == Some(37942978883103445000050380909722378802u128));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211454u128) == None);
/// assert!(1u128.checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_add(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_add(1u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(309501773591659534595607403613475089698u128
///         .checked_add(214732482049775202665533815351942767021u128) == None);
/// assert!(107003499999086825983349737574701749983u128
///         .checked_add(340240172877488693838031160349436983343u128) == None);
/// assert!(114158794601093977887317899219464045784u128
///         .checked_add(313722503116604457665135290279052844854u128) == None);
/// assert!(276278614716976805018922235338577627871u128
///         .checked_add(67083826260676700400815006103980446645u128) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128.checked_add(0u128));
/// assert!(0u128.checked_add(0u128) == 0u128.checked_add(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_add(0u128)
///         == 0u128.checked_add(340282366920938463463374607431768211454u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128.checked_add(0u128));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128
///             .checked_add(340282366920938463463374607431768211455u128));
/// assert!(237595766136034124012057740023886154347u128
///         .checked_add(218665674346776656782146277839476653617u128)
///         == 218665674346776656782146277839476653617u128
///             .checked_add(237595766136034124012057740023886154347u128));
/// assert!(205547517706496394916232382448597904334u128
///         .checked_add(74812021717409180811061136161676167272u128)
///         == 74812021717409180811061136161676167272u128
///             .checked_add(205547517706496394916232382448597904334u128));
/// assert!(27552976708036324590422648232809826676u128
///         .checked_add(38778504966474099133445335398146650347u128)
///         == 38778504966474099133445335398146650347u128
///             .checked_add(27552976708036324590422648232809826676u128));
/// assert!(207060228453483245581539405907254676567u128
///         .checked_add(40329448776749280123603188965882772562u128)
///         == 40329448776749280123603188965882772562u128
///             .checked_add(207060228453483245581539405907254676567u128));
/// assert!(188476846338279241483649507225877327068u128
///         .checked_add(10556192487358570215282359210359871857u128)
///         == 10556192487358570215282359210359871857u128
///             .checked_add(188476846338279241483649507225877327068u128));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0u128) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(1u128.checked_add(0u128) == Some(1u128));
/// assert!(340282366920938463463374607431768211454u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(75030266591803476592615014930662884824u128.checked_add(0u128)
///         == Some(75030266591803476592615014930662884824u128));
/// assert!(11457686889215267385215361145129116901u128.checked_add(0u128)
///         == Some(11457686889215267385215361145129116901u128));
/// assert!(20933923478159774185515442296622368660u128.checked_add(0u128)
///         == Some(20933923478159774185515442296622368660u128));
/// assert!(157276967044919865897295139411036839548u128.checked_add(0u128)
///         == Some(157276967044919865897295139411036839548u128));
/// assert!(138773838477336850951583348765392894511u128.checked_add(0u128)
///         == Some(138773838477336850951583348765392894511u128));
/// assert!(313903236904223345789895378726860058598u128.checked_add(0u128)
///         == Some(313903236904223345789895378726860058598u128));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0u128.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(0u128.checked_add(1u128) == Some(1u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(0u128.checked_add(306134191640695087524567760617816791075u128)
///         == Some(306134191640695087524567760617816791075u128));
/// assert!(0u128.checked_add(318578428616860112091537844954110596225u128)
///         == Some(318578428616860112091537844954110596225u128));
/// assert!(0u128.checked_add(313630214728920392790928180968968333817u128)
///         == Some(313630214728920392790928180968968333817u128));
/// assert!(0u128.checked_add(257845377313547057086836958872196726885u128)
///         == Some(257845377313547057086836958872196726885u128));
/// assert!(0u128.checked_add(181537328384071247898613333248664596275u128)
///         == Some(181537328384071247898613333248664596275u128));
/// assert!(0u128.checked_add(134111443424895414636184145021550671836u128)
///         == Some(134111443424895414636184145021550671836u128));
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
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128).and_then(|iv| iv.checked_add(0u128))
///         == 0u128.checked_add(0u128).and_then(|iv| 0u128.checked_add(iv)));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_add(0u128)
///         .and_then(|iv| iv.checked_add(340282366920938463463374607431768211455u128))
///         == 0u128
///             .checked_add(340282366920938463463374607431768211455u128)
///             .and_then(|iv| 340282366920938463463374607431768211454u128.checked_add(iv)));
/// assert!(0u128.checked_add(0u128).and_then(|iv| iv.checked_add(1u128))
///         == 0u128.checked_add(1u128).and_then(|iv| 0u128.checked_add(iv)));
/// assert!(1u128.checked_add(0u128).and_then(|iv| iv.checked_add(1u128))
///         == 0u128.checked_add(1u128).and_then(|iv| 1u128.checked_add(iv)));
/// assert!(1u128
///         .checked_add(340282366920938463463374607431768211454u128)
///         .and_then(|iv| iv.checked_add(1u128))
///         == 340282366920938463463374607431768211454u128
///             .checked_add(1u128)
///             .and_then(|iv| 1u128.checked_add(iv)));
/// assert!(318774967170143800612177093420144578982u128
///         .checked_add(216926181098484863153187319554969667095u128)
///         .and_then(|iv| iv.checked_add(140237414128086576209045152163977978412u128))
///         == 216926181098484863153187319554969667095u128
///             .checked_add(140237414128086576209045152163977978412u128)
///             .and_then(|iv| 318774967170143800612177093420144578982u128.checked_add(iv)));
/// assert!(147857797279422181678708978852108577760u128
///         .checked_add(271726809260671263256883051849609444714u128)
///         .and_then(|iv| iv.checked_add(48388963660492445562236598165105133048u128))
///         == 271726809260671263256883051849609444714u128
///             .checked_add(48388963660492445562236598165105133048u128)
///             .and_then(|iv| 147857797279422181678708978852108577760u128.checked_add(iv)));
/// assert!(64934913851359240392675547582045641013u128
///         .checked_add(278765889274296026181245030050275839057u128)
///         .and_then(|iv| iv.checked_add(263543196684813434250806948798027334614u128))
///         == 278765889274296026181245030050275839057u128
///             .checked_add(263543196684813434250806948798027334614u128)
///             .and_then(|iv| 64934913851359240392675547582045641013u128.checked_add(iv)));
/// assert!(98243382631985090572423823991257364883u128
///         .checked_add(170514289843016664345179435825046092630u128)
///         .and_then(|iv| iv.checked_add(196861467441351732706200940441802120633u128))
///         == 170514289843016664345179435825046092630u128
///             .checked_add(196861467441351732706200940441802120633u128)
///             .and_then(|iv| 98243382631985090572423823991257364883u128.checked_add(iv)));
/// assert!(181043248718462050000872445698872853315u128
///         .checked_add(243382938521142325094127867321500107626u128)
///         .and_then(|iv| iv.checked_add(253397143366151527294692659311233368276u128))
///         == 243382938521142325094127867321500107626u128
///             .checked_add(253397143366151527294692659311233368276u128)
///             .and_then(|iv| 181043248718462050000872445698872853315u128.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_u128_checked_add() {}
/// # Properties for [`u128::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u128::down((x.up() + y.up()).min(u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128.saturating_add(0u128) == 0u128);
/// assert!(92669588435223271797820683629100255889u128
///         .saturating_add(35188430413782497835211017496731661778u128)
///         == 127858018849005769633031701125831917667u128);
/// assert!(12851986703492133645779148401803332374u128
///         .saturating_add(297283224180340217907643207756762759354u128)
///         == 310135210883832351553422356158566091728u128);
/// assert!(51208355609012531467032276439297163143u128
///         .saturating_add(238719576443059765680066410070713589061u128)
///         == 289927932052072297147098686510010752204u128);
/// assert!(5587636009801594879225645192417455588u128
///         .saturating_add(72848161357440326237134322951571401490u128)
///         == 78435797367241921116359968143988857078u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_add(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.saturating_add(0u128) == 1u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(118853962799628089893455923014421044575u128
///         .saturating_add(32882302706573412649932243779549316680u128)
///         == 151736265506201502543388166793970361255u128);
/// assert!(46368828265702414382993478489068624639u128
///         .saturating_add(95682842596711064846879088846090137013u128)
///         == 142051670862413479229872567335158761652u128);
/// assert!(92399801473825898900130687683463106159u128
///         .saturating_add(72858134790688348596095977822794813607u128)
///         == 165257936264514247496226665506257919766u128);
/// assert!(27403215890628499931644683226853788053u128
///         .saturating_add(215138666727269597715951677558813310697u128)
///         == 242541882617898097647596360785667098750u128);
/// assert!(2486787191040784827717220538013759257u128
///         .saturating_add(30738120156043293617035096505868707255u128)
///         == 33224907347084078444752317043882466512u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u128::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_add(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128.saturating_add(1u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_add(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(1u128.saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(87275455623633662107696689340241681012u128
///         .saturating_add(336710644903255472216025176276934773221u128) == u128::MAX);
/// assert!(167006100359884391157121180512385012483u128
///         .saturating_add(252520611075255214790444828703024809713u128) == u128::MAX);
/// assert!(97898449377870206414082796747437005652u128
///         .saturating_add(325384812433553582171141758309393673324u128) == u128::MAX);
/// assert!(337426167745264976070126812258795640446u128
///         .saturating_add(296121561831788197273545289949290902573u128) == u128::MAX);
/// # }
/// ```
pub fn core_ops_add_u128_saturating_add() {}
/// # Properties for [`core::ops::Rem::<usize>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == usize::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize % 1usize == 0usize);
/// assert!(18446744073709551614usize % 18446744073709551615usize == 18446744073709551614usize);
/// assert!(18446744073709551615usize % 18446744073709551614usize == 1usize);
/// assert!(1usize % 18446744073709551615usize == 1usize);
/// assert!(0usize % 18446744073709551614usize == 0usize);
/// assert!(18446744073709551614usize % 18446744073709551614usize == 0usize);
/// assert!(10039919564426926303usize % 6727073869520790223usize == 3312845694906136080usize);
/// assert!(17855406404854798704usize % 9486306676393916126usize == 8369099728460882578usize);
/// assert!(17826708505423048425usize % 9817244674986441147usize == 8009463830436607278usize);
/// assert!(8652537000068506543usize % 16092767333857970059usize == 8652537000068506543usize);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize % 0) } });
/// # }
/// ```
pub fn core_ops_rem_usize_rem() {}
/// # Properties for [`usize::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(usize::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.checked_rem(1usize) == Some(0usize));
/// assert!(0usize.checked_rem(18446744073709551615usize) == Some(0usize));
/// assert!(18446744073709551615usize.checked_rem(18446744073709551614usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_rem(18446744073709551615usize)
///         == Some(18446744073709551614usize));
/// assert!(18446744073709551615usize.checked_rem(1usize) == Some(0usize));
/// assert!(1usize.checked_rem(18446744073709551615usize) == Some(1usize));
/// assert!(14119586892509753237usize.checked_rem(2132848150489044270usize)
///         == Some(1322497989575487617usize));
/// assert!(14269138334217754229usize.checked_rem(2354551194310871546usize)
///         == Some(141831168352524953usize));
/// assert!(10805739091458340663usize.checked_rem(14941705138694464462usize)
///         == Some(10805739091458340663usize));
/// assert!(286728536247391742usize.checked_rem(7121376054063824048usize)
///         == Some(286728536247391742usize));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0usize.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_usize_checked_rem() {}
/// # Properties for [`usize::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `x == usize::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(usize::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_neg() == Some(0usize));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `x != usize::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.checked_neg() == None);
/// assert!(18446744073709551615usize.checked_neg() == None);
/// assert!(1usize.checked_neg() == None);
/// assert!(2434296333903474331usize.checked_neg() == None);
/// assert!(8284930021548022127usize.checked_neg() == None);
/// assert!(2483614293009126027usize.checked_neg() == None);
/// assert!(17920900290821242547usize.checked_neg() == None);
/// assert!(7756163885041330593usize.checked_neg() == None);
/// assert!(1113228863980140032usize.checked_neg() == None);
/// assert!(18189853469084708290usize.checked_neg() == None);
/// # }
/// ```
pub fn t_usize_checked_neg() {}
/// # Properties for [`core::ops::Shl::<usize>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x << y == usize::down((x.up() << y) & usize::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize << 1u32 == 0usize);
/// assert!(0usize << 0u32 == 0usize);
/// assert!(18446744073709551614usize << 0u32 == 18446744073709551614usize);
/// assert!(18446744073709551615usize << 0u32 == 18446744073709551615usize);
/// assert!(18446744073709551615usize << 1u32 == 18446744073709551614usize);
/// assert!(9242252836687068612usize << 47u32 == 2225341165874446336usize);
/// assert!(12385097977419832619usize << 31u32 == 16591910381190905856usize);
/// assert!(12385831681657606550usize << 13u32 == 7640730736578969600usize);
/// assert!(5313186843970633021usize << 39u32 == 1368705609360211968usize);
/// assert!(14556685562786720158usize << 37u32 == 11436103728503980032usize);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551615usize << 4294967295u32));
/// assert!(panics!(0usize << 4294967294u32));
/// assert!(panics!(0usize << 4294967295u32));
/// assert!(panics!(18446744073709551614usize << 4294967294u32));
/// assert!(panics!(1usize << 4294967295u32));
/// assert!(panics!(1usize << 4294967294u32));
/// assert!(panics!(8170320710312818228usize << 92u32));
/// assert!(panics!(10266705221142617469usize << 108u32));
/// assert!(panics!(14951763948986386078usize << 109u32));
/// assert!(panics!(11749507282761954882usize << 134u32));
/// # }
/// ```
pub fn core_ops_shl_usize_shl() {}
/// # Properties for [`core::ops::Shl::<usize>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(usize::down((x.up() << y) & usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.checked_shl(0u32) == Some(18446744073709551615usize));
/// assert!(18446744073709551614usize.checked_shl(0u32) == Some(18446744073709551614usize));
/// assert!(0usize.checked_shl(0u32) == Some(0usize));
/// assert!(1usize.checked_shl(1u32) == Some(2usize));
/// assert!(1usize.checked_shl(0u32) == Some(1usize));
/// assert!(18446744073709551614usize.checked_shl(1u32) == Some(18446744073709551612usize));
/// assert!(7607180562986770655usize.checked_shl(23u32) == Some(3850448559375646720usize));
/// assert!(7865278857090306079usize.checked_shl(1u32) == Some(15730557714180612158usize));
/// assert!(15348940884375867766usize.checked_shl(18u32) == Some(10495092226371616768usize));
/// assert!(7152879227356860117usize.checked_shl(23u32) == Some(16230392684194299904usize));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_shl(4294967294u32) == None);
/// assert!(0usize.checked_shl(4294967295u32) == None);
/// assert!(1usize.checked_shl(4294967295u32) == None);
/// assert!(0usize.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551614usize.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551614usize.checked_shl(4294967294u32) == None);
/// assert!(13204572281557530919usize.checked_shl(73u32) == None);
/// assert!(349155475182426164usize.checked_shl(90u32) == None);
/// assert!(5102336933749614163usize.checked_shl(118u32) == None);
/// assert!(13544218909158620124usize.checked_shl(102u32) == None);
/// # }
/// ```
pub fn core_ops_shl_usize_checked_shl() {}
/// # Properties for [`core::ops::Shl::<usize>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (usize::down((x.up() << y) & usize::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shl(1u32) == (0usize, false));
/// assert!(18446744073709551614usize.overflowing_shl(0u32) == (18446744073709551614usize, false));
/// assert!(0usize.overflowing_shl(0u32) == (0usize, false));
/// assert!(15867813355090826313usize.overflowing_shl(19u32) == (7018511588462690304usize, false));
/// assert!(6892421183486171610usize.overflowing_shl(60u32) == (11529215046068469760usize, false));
/// assert!(13769335653585112222usize.overflowing_shl(62u32) == (9223372036854775808usize, false));
/// assert!(6473683240919787057usize.overflowing_shl(38u32) == (15787804175229779968usize, false));
/// assert!(7826618729451075893usize.overflowing_shl(47u32) == (15896158572245942272usize, false));
/// assert!(203399862213772129usize.overflowing_shl(21u32) == (16364624950685859840usize, false));
/// assert!(11046333707493418511usize.overflowing_shl(46u32) == (7603131702164062208usize, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (usize::down((x.up() << (y & (usize::BITS - 1)) & usize::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.overflowing_shl(4294967295u32) == (9223372036854775808usize, true));
/// assert!(18446744073709551615usize.overflowing_shl(4294967294u32)
///         == (13835058055282163712usize, true));
/// assert!(0usize.overflowing_shl(4294967295u32) == (0usize, true));
/// assert!(1usize.overflowing_shl(4294967294u32) == (4611686018427387904usize, true));
/// assert!(0usize.overflowing_shl(4294967294u32) == (0usize, true));
/// assert!(18446744073709551614usize.overflowing_shl(4294967295u32) == (0usize, true));
/// assert!(435892116789176629usize.overflowing_shl(78u32) == (2766484948273414144usize, true));
/// assert!(8419705401190674703usize.overflowing_shl(126u32) == (13835058055282163712usize, true));
/// assert!(15389357193705231286usize.overflowing_shl(78u32) == (9130262204357902336usize, true));
/// assert!(9846453716013877465usize.overflowing_shl(131u32) == (4984653433272813256usize, true));
/// # }
/// ```
pub fn core_ops_shl_usize_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<usize>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x >> y == usize::down((x.up() >> y) & usize::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize >> 0u32 == 18446744073709551614usize);
/// assert!(0usize >> 0u32 == 0usize);
/// assert!(18446744073709551614usize >> 1u32 == 9223372036854775807usize);
/// assert!(18446744073709551615usize >> 0u32 == 18446744073709551615usize);
/// assert!(2065480707875916884usize >> 23u32 == 246224487766usize);
/// assert!(7306440460462004298usize >> 43u32 == 830646usize);
/// assert!(14606240306934862455usize >> 28u32 == 54412485312usize);
/// assert!(4041404703280422854usize >> 28u32 == 15055405733usize);
/// assert!(17602379834195092427usize >> 57u32 == 122usize);
/// assert!(1260984050574950035usize >> 14u32 == 76964358555599usize);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614usize >> 4294967295u32));
/// assert!(panics!(18446744073709551614usize >> 4294967294u32));
/// assert!(panics!(18446744073709551615usize >> 4294967294u32));
/// assert!(panics!(1usize >> 4294967294u32));
/// assert!(panics!(0usize >> 4294967295u32));
/// assert!(panics!(0usize >> 4294967294u32));
/// assert!(panics!(3730771125332880699usize >> 72u32));
/// assert!(panics!(9398392274802254507usize >> 123u32));
/// assert!(panics!(9056265901768152369usize >> 138u32));
/// assert!(panics!(2592562385785239685usize >> 67u32));
/// # }
/// ```
pub fn core_ops_shr_usize_shr() {}
/// # Properties for [`core::ops::Shr::<usize>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(usize::down((x.up() >> y) & usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_shr(0u32) == Some(1usize));
/// assert!(0usize.checked_shr(1u32) == Some(0usize));
/// assert!(18446744073709551614usize.checked_shr(1u32) == Some(9223372036854775807usize));
/// assert!(18446744073709551615usize.checked_shr(1u32) == Some(9223372036854775807usize));
/// assert!(0usize.checked_shr(0u32) == Some(0usize));
/// assert!(2682293852073932845usize.checked_shr(62u32) == Some(0usize));
/// assert!(8112912848689057720usize.checked_shr(55u32) == Some(225usize));
/// assert!(2024618054929865507usize.checked_shr(43u32) == Some(230172usize));
/// assert!(3623699482023448049usize.checked_shr(59u32) == Some(6usize));
/// assert!(848498489154870239usize.checked_shr(3u32) == Some(106062311144358779usize));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551614usize.checked_shr(4294967295u32) == None);
/// assert!(0usize.checked_shr(4294967294u32) == None);
/// assert!(1usize.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551614usize.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551615usize.checked_shr(4294967294u32) == None);
/// assert!(6383111924168896410usize.checked_shr(119u32) == None);
/// assert!(15032255357460907750usize.checked_shr(65u32) == None);
/// assert!(3752494922676589757usize.checked_shr(77u32) == None);
/// assert!(16539380261612030834usize.checked_shr(75u32) == None);
/// # }
/// ```
pub fn core_ops_shr_usize_checked_shr() {}
/// # Properties for [`core::ops::Shr::<usize>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (usize::down((x.up() >> y) & usize::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shr(0u32) == (0usize, false));
/// assert!(0usize.overflowing_shr(1u32) == (0usize, false));
/// assert!(18446744073709551615usize.overflowing_shr(0u32) == (18446744073709551615usize, false));
/// assert!(18446744073709551614usize.overflowing_shr(0u32) == (18446744073709551614usize, false));
/// assert!(1440259909964936213usize.overflowing_shr(19u32) == (2747077770166usize, false));
/// assert!(7612208023255893450usize.overflowing_shr(47u32) == (54087usize, false));
/// assert!(16962337677224651864usize.overflowing_shr(24u32) == (1011034111811usize, false));
/// assert!(18430169972938491724usize.overflowing_shr(9u32) == (35996425728395491usize, false));
/// assert!(7281739908800461262usize.overflowing_shr(31u32) == (3390824379usize, false));
/// assert!(12375224463572919200usize.overflowing_shr(9u32) == (24170360280415857usize, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (usize::down((x.up() >> (y & (usize::BITS - 1)) & usize::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.overflowing_shr(4294967294u32) == (0usize, true));
/// assert!(18446744073709551615usize.overflowing_shr(4294967295u32) == (1usize, true));
/// assert!(18446744073709551615usize.overflowing_shr(4294967294u32) == (3usize, true));
/// assert!(0usize.overflowing_shr(4294967295u32) == (0usize, true));
/// assert!(18446744073709551614usize.overflowing_shr(4294967294u32) == (3usize, true));
/// assert!(1usize.overflowing_shr(4294967295u32) == (0usize, true));
/// assert!(15137676901753836671usize.overflowing_shr(89u32) == (451137927226usize, true));
/// assert!(166807694845896801usize.overflowing_shr(125u32) == (0usize, true));
/// assert!(3420357223361652958usize.overflowing_shr(64u32) == (3420357223361652958usize, true));
/// assert!(16098067900464932022usize.overflowing_shr(70u32) == (251532310944764562usize, true));
/// # }
/// ```
pub fn core_ops_shr_usize_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<usize>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == usize::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize / 18446744073709551615usize == 0usize);
/// assert!(18446744073709551614usize / 18446744073709551615usize == 0usize);
/// assert!(1usize / 18446744073709551614usize == 0usize);
/// assert!(18446744073709551615usize / 18446744073709551614usize == 1usize);
/// assert!(0usize / 1usize == 0usize);
/// assert!(4075618853usize / 1029665204usize == 3usize);
/// assert!(4265581174usize / 3972698948usize == 1usize);
/// assert!(4149834425usize / 3201469631usize == 1usize);
/// assert!(3382522007usize / 2824639974usize == 1usize);
/// assert!(4143727843usize / 3178536667usize == 1usize);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551614usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551615usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(16680128449987326697usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(9605006638616355691usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2715739621160822106usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(7341635105764792779usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2153266043876155996usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4164769598989491820usize / 0) } });
/// # }
/// ```
pub fn core_ops_div_usize_div() {}
/// # Properties for [`usize::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == usize::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_div(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_div(18446744073709551614usize) == 1usize);
/// assert!(18446744073709551615usize.saturating_div(1usize) == 18446744073709551615usize);
/// assert!(1usize.saturating_div(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_div(18446744073709551615usize) == 1usize);
/// assert!(0usize.saturating_div(18446744073709551614usize) == 0usize);
/// assert!(3322345582usize.saturating_div(2478525216usize) == 1usize);
/// assert!(3033078158usize.saturating_div(1234375609usize) == 2usize);
/// assert!(1233800423usize.saturating_div(3189707054usize) == 0usize);
/// assert!(2198958861usize.saturating_div(2600248647usize) == 0usize);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551614usize.saturating_div(0)) }
///     });
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1usize.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551615usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(13553736196198781686usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(15964677000939679992usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(2635959633986074301usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(5176466206478316732usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(17890675263628869526usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(3947475203366397834usize.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_usize_saturating_div() {}
/// # Properties for [`usize::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(usize::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_div(18446744073709551614usize) == Some(0usize));
/// assert!(1usize.checked_div(1usize) == Some(1usize));
/// assert!(0usize.checked_div(1usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_div(18446744073709551614usize) == Some(1usize));
/// assert!(18446744073709551615usize.checked_div(18446744073709551614usize) == Some(1usize));
/// assert!(1478303609usize.checked_div(3109022783usize) == Some(0usize));
/// assert!(3177975438usize.checked_div(3126867414usize) == Some(1usize));
/// assert!(2908504443usize.checked_div(4193636188usize) == Some(0usize));
/// assert!(3828339167usize.checked_div(2661912987usize) == Some(1usize));
/// assert!(2764381419usize.checked_div(4201520053usize) == Some(0usize));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_div(0) == None);
/// assert!(0usize.checked_div(0) == None);
/// assert!(18446744073709551615usize.checked_div(0) == None);
/// assert!(18446744073709551614usize.checked_div(0) == None);
/// assert!(9448127766904078235usize.checked_div(0) == None);
/// assert!(12414008549316772096usize.checked_div(0) == None);
/// assert!(4846218654084671748usize.checked_div(0) == None);
/// assert!(10101914998681519750usize.checked_div(0) == None);
/// assert!(9804252050009120424usize.checked_div(0) == None);
/// assert!(16405632801263527423usize.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_usize_checked_div() {}
/// # Properties for [`core::ops::Mul::<usize>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x * y == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize * 0usize == 0usize);
/// assert!(0usize * 1usize == 0usize);
/// assert!(1usize * 0usize == 0usize);
/// assert!(18446744073709551615usize * 0usize == 0usize);
/// assert!(2809068762usize * 2913759410usize == 8184950538614550420usize);
/// assert!(3651378554usize * 1772946212usize == 6473697775892337448usize);
/// assert!(950088307usize * 2734231262usize == 2597761150660053434usize);
/// assert!(1726273468usize * 3925270039usize == 6776089523061025252usize);
/// assert!(3189597528usize * 2813370423usize == 8973519346549114344usize);
/// assert!(2419253429usize * 3788060595usize == 9164278583713530255usize);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614usize * 18446744073709551614usize));
/// assert!(panics!(18446744073709551615usize * 18446744073709551615usize));
/// assert!(panics!(18446744073709551614usize * 18446744073709551615usize));
/// assert!(panics!(18446744073709551615usize * 18446744073709551614usize));
/// assert!(panics!(4246902401213591213usize * 5920912703605098720usize));
/// assert!(panics!(18020892827008775563usize * 17521116796223147057usize));
/// assert!(panics!(14572316386170312207usize * 6423616553461701347usize));
/// assert!(panics!(12870164199461361001usize * 9280220150025981702usize));
/// assert!(panics!(3808755501510454794usize * 6216435481024830875usize));
/// assert!(panics!(1879316987029203219usize * 7432159227745790432usize));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize * 1 == 18446744073709551614usize);
/// assert!(1usize * 1 == 1usize);
/// assert!(0usize * 1 == 0usize);
/// assert!(18446744073709551615usize * 1 == 18446744073709551615usize);
/// assert!(2045606789884735090usize * 1 == 2045606789884735090usize);
/// assert!(6183254732266399916usize * 1 == 6183254732266399916usize);
/// assert!(8279134460858575499usize * 1 == 8279134460858575499usize);
/// assert!(4633787623238785820usize * 1 == 4633787623238785820usize);
/// assert!(13471843046663004812usize * 1 == 13471843046663004812usize);
/// assert!(16169422093624655505usize * 1 == 16169422093624655505usize);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0usize == 0usize);
/// assert!(1 * 18446744073709551615usize == 18446744073709551615usize);
/// assert!(1 * 18446744073709551614usize == 18446744073709551614usize);
/// assert!(1 * 1usize == 1usize);
/// assert!(1 * 8798811039054656058usize == 8798811039054656058usize);
/// assert!(1 * 9927087386772519066usize == 9927087386772519066usize);
/// assert!(1 * 16864841959181022913usize == 16864841959181022913usize);
/// assert!(1 * 14111361600449246129usize == 14111361600449246129usize);
/// assert!(1 * 9137540142461290914usize == 9137540142461290914usize);
/// assert!(1 * 133323745646214102usize == 133323745646214102usize);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize * 1usize == 1usize * 1usize);
/// assert!(0usize * 1usize == 1usize * 0usize);
/// assert!(18446744073709551614usize * 0usize == 0usize * 18446744073709551614usize);
/// assert!(18446744073709551614usize * 1usize == 1usize * 18446744073709551614usize);
/// assert!(1usize * 0usize == 0usize * 1usize);
/// assert!(0usize * 0usize == 0usize * 0usize);
/// assert!(4063191055usize * 4122759171usize == 4122759171usize * 4063191055usize);
/// assert!(1303896009usize * 3274620629usize == 3274620629usize * 1303896009usize);
/// assert!(3878916311usize * 3353442027usize == 3353442027usize * 3878916311usize);
/// assert!(3524176421usize * 2774929261usize == 2774929261usize * 3524176421usize);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= usize::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1usize * 1usize) * 18446744073709551615usize
///         == 1usize * (1usize * 18446744073709551615usize));
/// assert!((1usize * 18446744073709551614usize) * 1usize
///         == 1usize * (18446744073709551614usize * 1usize));
/// assert!((1usize * 18446744073709551615usize) * 1usize
///         == 1usize * (18446744073709551615usize * 1usize));
/// assert!((1usize * 1usize) * 1usize == 1usize * (1usize * 1usize));
/// assert!((1usize * 1usize) * 18446744073709551614usize
///         == 1usize * (1usize * 18446744073709551614usize));
/// assert!((18446744073709551614usize * 1usize) * 1usize
///         == 18446744073709551614usize * (1usize * 1usize));
/// assert!((2599736usize * 1961788usize) * 1985230usize
///         == 2599736usize * (1961788usize * 1985230usize));
/// assert!((1523943usize * 2124559usize) * 1533724usize
///         == 1523943usize * (2124559usize * 1533724usize));
/// assert!((2335334usize * 1712049usize) * 2184381usize
///         == 2335334usize * (1712049usize * 2184381usize));
/// assert!((1166100usize * 2002187usize) * 2355660usize
///         == 1166100usize * (2002187usize * 2355660usize));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= usize::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize * (0usize + 1usize)
///         == 18446744073709551615usize * 0usize + 18446744073709551615usize * 1usize);
/// assert!(1usize * (1usize + 0usize) == 1usize * 1usize + 1usize * 0usize);
/// assert!(18446744073709551614usize * (0usize + 1usize)
///         == 18446744073709551614usize * 0usize + 18446744073709551614usize * 1usize);
/// assert!(1usize * (0usize + 18446744073709551615usize)
///         == 1usize * 0usize + 1usize * 18446744073709551615usize);
/// assert!(1usize * (1usize + 1usize) == 1usize * 1usize + 1usize * 1usize);
/// assert!(917075625usize * (1601518823usize + 3989573790usize)
///         == 917075625usize * 1601518823usize + 917075625usize * 3989573790usize);
/// assert!(2280014584usize * (3859680048usize + 3249940688usize)
///         == 2280014584usize * 3859680048usize + 2280014584usize * 3249940688usize);
/// assert!(1336343554usize * (2348831342usize + 3442891645usize)
///         == 1336343554usize * 2348831342usize + 1336343554usize * 3442891645usize);
/// assert!(2655124811usize * (1639527351usize + 4258436361usize)
///         == 2655124811usize * 1639527351usize + 2655124811usize * 4258436361usize);
/// assert!(1676505401usize * (4122020061usize + 2526811083usize)
///         == 1676505401usize * 4122020061usize + 1676505401usize * 2526811083usize);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize * 0 == 0);
/// assert!(1usize * 0 == 0);
/// assert!(18446744073709551614usize * 0 == 0);
/// assert!(18446744073709551615usize * 0 == 0);
/// assert!(3630820329usize * 0 == 0);
/// assert!(3465777278usize * 0 == 0);
/// assert!(3966534226usize * 0 == 0);
/// assert!(3557990619usize * 0 == 0);
/// assert!(2500373090usize * 0 == 0);
/// assert!(4043924456usize * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_usize_mul() {}
/// # Properties for [`usize::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(usize::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_mul(0usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_mul(1usize) == Some(18446744073709551614usize));
/// assert!(1usize.checked_mul(18446744073709551615usize) == Some(18446744073709551615usize));
/// assert!(1usize.checked_mul(0usize) == Some(0usize));
/// assert!(1usize.checked_mul(18446744073709551614usize) == Some(18446744073709551614usize));
/// assert!(1usize.checked_mul(1usize) == Some(1usize));
/// assert!(4293916342usize.checked_mul(3507628008usize) == Some(15061461225208106736usize));
/// assert!(1693257494usize.checked_mul(1915699635usize) == Some(3243772763216814690usize));
/// assert!(4158235127usize.checked_mul(2626393565usize) == Some(10921161979309757755usize));
/// assert!(2160787504usize.checked_mul(4072919735usize) == Some(8800714068182991440usize));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.checked_mul(18446744073709551615usize) == None);
/// assert!(18446744073709551614usize.checked_mul(18446744073709551615usize) == None);
/// assert!(18446744073709551614usize.checked_mul(18446744073709551614usize) == None);
/// assert!(18446744073709551615usize.checked_mul(18446744073709551614usize) == None);
/// assert!(14053245225314725003usize.checked_mul(13039063036028863642usize) == None);
/// assert!(7243173582121593123usize.checked_mul(17928641617000513719usize) == None);
/// assert!(6633636878034785014usize.checked_mul(7866281558454390332usize) == None);
/// assert!(12814779346491591251usize.checked_mul(11116620298178280217usize) == None);
/// assert!(4083168635789556647usize.checked_mul(15838728016233804208usize) == None);
/// assert!(2729957256467381546usize.checked_mul(12130442277734813753usize) == None);
/// # }
/// ```
pub fn core_ops_mul_usize_checked_mul() {}
/// # Properties for [`usize::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (usize::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.overflowing_mul(0usize) == (0usize, false));
/// assert!(0usize.overflowing_mul(1usize) == (0usize, false));
/// assert!(0usize.overflowing_mul(18446744073709551615usize) == (0usize, false));
/// assert!(18446744073709551614usize.overflowing_mul(0usize) == (0usize, false));
/// assert!(1213709560usize.overflowing_mul(3117306887usize) == (3783505170205739720usize, false));
/// assert!(1521575579usize.overflowing_mul(2642904319usize) == (4021378669424025701usize, false));
/// assert!(3559221032usize.overflowing_mul(2189456071usize) == (7792758096543285272usize, false));
/// assert!(2786117801usize.overflowing_mul(859978183usize) == (2396000524127935583usize, false));
/// assert!(3220377871usize.overflowing_mul(3297437700usize)
///         == (10618995400081136700usize, false));
/// assert!(2806538027usize.overflowing_mul(4253929964usize)
///         == (11938816208160741028usize, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.overflowing_mul(18446744073709551614usize)
///         == (2usize, true));
/// assert!(18446744073709551615usize.overflowing_mul(18446744073709551615usize)
///         == (1usize, true));
/// assert!(18446744073709551614usize.overflowing_mul(18446744073709551614usize)
///         == (4usize, true));
/// assert!(18446744073709551614usize.overflowing_mul(18446744073709551615usize)
///         == (2usize, true));
/// assert!(2984813091513258770usize.overflowing_mul(3253999293998930784usize)
///         == (5175961572693314752usize, true));
/// assert!(4944671647444128688usize.overflowing_mul(17867882411095943610usize)
///         == (6141691223538925024usize, true));
/// assert!(170672305380167992usize.overflowing_mul(5182641788585481758usize)
///         == (14800755590253761680usize, true));
/// assert!(13371320694887560382usize.overflowing_mul(10837579643558261415usize)
///         == (14913850775347069938usize, true));
/// assert!(15838653568912680813usize.overflowing_mul(5433950436544696559usize)
///         == (7074624757288050371usize, true));
/// assert!(3553343937050982791usize.overflowing_mul(15387743040150678481usize)
///         == (9466852333240394807usize, true));
/// # }
/// ```
pub fn core_ops_mul_usize_overflowing_mul() {}
/// # Properties for [`usize::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::down((x.up() * y.up()).min(usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_mul(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_mul(1usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551615usize)
///         == 18446744073709551615usize);
/// assert!(1usize.saturating_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551614usize)
///         == 18446744073709551615usize);
/// assert!(8066106433330115032usize.saturating_mul(11684952187620457164usize)
///         == 18446744073709551615usize);
/// assert!(1764736033412924511usize.saturating_mul(3796132533754322783usize)
///         == 18446744073709551615usize);
/// assert!(8346019181201749863usize.saturating_mul(3257811310763673317usize)
///         == 18446744073709551615usize);
/// assert!(9136966644361857397usize.saturating_mul(1602752880872706098usize)
///         == 18446744073709551615usize);
/// assert!(6891633035226532748usize.saturating_mul(4177969595199353031usize)
///         == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.saturating_mul(0usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_mul(1usize) == 18446744073709551615usize);
/// assert!(1usize.saturating_mul(1usize) == 1usize);
/// assert!(0usize.saturating_mul(18446744073709551615usize) == 0usize);
/// assert!(0usize.saturating_mul(1usize) == 0usize);
/// assert!(0usize.saturating_mul(18446744073709551614usize) == 0usize);
/// assert!(2107439070usize.saturating_mul(1496426679usize) == 3153628048714948530usize);
/// assert!(1795981269usize.saturating_mul(2556563059usize) == 4591539366981341871usize);
/// assert!(485106206usize.saturating_mul(508261349usize) == 246560734669831894usize);
/// assert!(1460886310usize.saturating_mul(3548569945usize) == 5184057252727952950usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_mul(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_mul(18446744073709551615usize) == usize::MAX);
/// assert!(802749651704122297usize.saturating_mul(2729671462744061599usize) == usize::MAX);
/// assert!(6414047890185348238usize.saturating_mul(14915648033446487948usize) == usize::MAX);
/// assert!(8878975554846746492usize.saturating_mul(13555215893687216198usize) == usize::MAX);
/// assert!(4346507644506623746usize.saturating_mul(4964600585480515559usize) == usize::MAX);
/// assert!(3240443976628673593usize.saturating_mul(14023158699182619969usize) == usize::MAX);
/// assert!(14450109800432337485usize.saturating_mul(11718035359804571110usize) == usize::MAX);
/// # }
/// ```
pub fn core_ops_mul_usize_saturating_mul() {}
/// # Properties for [`usize::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == usize::down((x.up() * y.up()) % (usize::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_mul(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_mul(18446744073709551614usize) == 2usize);
/// assert!(0usize.wrapping_mul(18446744073709551615usize) == 0usize);
/// assert!(0usize.wrapping_mul(0usize) == 0usize);
/// assert!(1usize.wrapping_mul(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_mul(1usize) == 18446744073709551614usize);
/// assert!(18199225839339507784usize.wrapping_mul(2464485787756397267usize)
///         == 5242558069192325976usize);
/// assert!(4976255843189435361usize.wrapping_mul(14096030979498471850usize)
///         == 2035911812219696234usize);
/// assert!(6968615943836015702usize.wrapping_mul(15996251836502209607usize)
///         == 7211782125178115034usize);
/// assert!(18441590983222119058usize.wrapping_mul(2445256036724001291usize)
///         == 6736689097692244038usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.wrapping_mul(0usize) == 0usize);
/// assert!(1usize.wrapping_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(0usize.wrapping_mul(0usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_mul(0usize) == 0usize);
/// assert!(0usize.wrapping_mul(18446744073709551615usize) == 0usize);
/// assert!(3199486883usize.wrapping_mul(2264205109usize) == 7244294546667085247usize);
/// assert!(3408959716usize.wrapping_mul(3660412121usize) == 12478197464447117636usize);
/// assert!(1117805465usize.wrapping_mul(3316802753usize) == 3707540243630445145usize);
/// assert!(1773654689usize.wrapping_mul(2188603156usize) == 3881826249999598484usize);
/// assert!(1286288300usize.wrapping_mul(4240408127usize) == 5454387360985014100usize);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 1 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize * 1 == 18446744073709551614usize);
/// assert!(0usize * 1 == 0usize);
/// assert!(18446744073709551615usize * 1 == 18446744073709551615usize);
/// assert!(1usize * 1 == 1usize);
/// assert!(9367720297903247303usize * 1 == 9367720297903247303usize);
/// assert!(4110007786482093976usize * 1 == 4110007786482093976usize);
/// assert!(13780025793918239530usize * 1 == 13780025793918239530usize);
/// assert!(13170455747112060188usize * 1 == 13170455747112060188usize);
/// assert!(4010071654968326793usize * 1 == 4010071654968326793usize);
/// assert!(11843801827281541065usize * 1 == 11843801827281541065usize);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `1 * x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1 * 0usize == 0usize);
/// assert!(1 * 1usize == 1usize);
/// assert!(1 * 18446744073709551614usize == 18446744073709551614usize);
/// assert!(1 * 18446744073709551615usize == 18446744073709551615usize);
/// assert!(1 * 12303770602317992580usize == 12303770602317992580usize);
/// assert!(1 * 2901101551071813055usize == 2901101551071813055usize);
/// assert!(1 * 17486481694122149899usize == 17486481694122149899usize);
/// assert!(1 * 9188722661764115380usize == 9188722661764115380usize);
/// assert!(1 * 5436296369427640250usize == 5436296369427640250usize);
/// assert!(1 * 12760874558445818011usize == 12760874558445818011usize);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x * y == y * x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize * 18446744073709551615usize == 18446744073709551615usize * 0usize);
/// assert!(18446744073709551615usize * 0usize == 0usize * 18446744073709551615usize);
/// assert!(0usize * 0usize == 0usize * 0usize);
/// assert!(18446744073709551615usize * 1usize == 1usize * 18446744073709551615usize);
/// assert!(3854220730usize * 3791519318usize == 3791519318usize * 3854220730usize);
/// assert!(4141223610usize * 3247293320usize == 3247293320usize * 4141223610usize);
/// assert!(3954433803usize * 3967175220usize == 3967175220usize * 3954433803usize);
/// assert!(3008056924usize * 1826869572usize == 1826869572usize * 3008056924usize);
/// assert!(3464438925usize * 690442839usize == 690442839usize * 3464438925usize);
/// assert!(3058192344usize * 3432764409usize == 3432764409usize * 3058192344usize);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `(x.up() * y.up() * z.up() <= usize::MAX.up() && x > 0 && y > 0 && z > 0)`  
/// __Postcondition:__ `(x * y) * z == x * (y * z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1usize * 1usize) * 1usize == 1usize * (1usize * 1usize));
/// assert!((18446744073709551614usize * 1usize) * 1usize
///         == 18446744073709551614usize * (1usize * 1usize));
/// assert!((1usize * 1usize) * 18446744073709551614usize
///         == 1usize * (1usize * 18446744073709551614usize));
/// assert!((1usize * 18446744073709551615usize) * 1usize
///         == 1usize * (18446744073709551615usize * 1usize));
/// assert!((1usize * 1usize) * 18446744073709551615usize
///         == 1usize * (1usize * 18446744073709551615usize));
/// assert!((18446744073709551615usize * 1usize) * 1usize
///         == 18446744073709551615usize * (1usize * 1usize));
/// assert!((1649168usize * 2540638usize) * 1654079usize
///         == 1649168usize * (2540638usize * 1654079usize));
/// assert!((1716693usize * 823774usize) * 915754usize
///         == 1716693usize * (823774usize * 915754usize));
/// assert!((2089325usize * 1385703usize) * 2613348usize
///         == 2089325usize * (1385703usize * 2613348usize));
/// assert!((2258928usize * 2213350usize) * 2390373usize
///         == 2258928usize * (2213350usize * 2390373usize));
/// # }
/// ```
/// ## Distributivity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `(x.up() * (y.up() + z.up()) <= usize::MAX.up() && x > 0)`  
/// __Postcondition:__ `x * (y + z) == x * y + x * z`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize * (0usize + 0usize) == 1usize * 0usize + 1usize * 0usize);
/// assert!(18446744073709551614usize * (0usize + 0usize)
///         == 18446744073709551614usize * 0usize + 18446744073709551614usize * 0usize);
/// assert!(18446744073709551615usize * (0usize + 1usize)
///         == 18446744073709551615usize * 0usize + 18446744073709551615usize * 1usize);
/// assert!(18446744073709551615usize * (1usize + 0usize)
///         == 18446744073709551615usize * 1usize + 18446744073709551615usize * 0usize);
/// assert!(1usize * (18446744073709551615usize + 0usize)
///         == 1usize * 18446744073709551615usize + 1usize * 0usize);
/// assert!(2806489230usize * (3567053591usize + 2248038156usize)
///         == 2806489230usize * 3567053591usize + 2806489230usize * 2248038156usize);
/// assert!(1164863905usize * (2200974783usize + 1609504432usize)
///         == 1164863905usize * 2200974783usize + 1164863905usize * 1609504432usize);
/// assert!(1169138212usize * (1932611482usize + 3853568385usize)
///         == 1169138212usize * 1932611482usize + 1169138212usize * 3853568385usize);
/// assert!(2986660045usize * (2908521377usize + 3141853577usize)
///         == 2986660045usize * 2908521377usize + 2986660045usize * 3141853577usize);
/// assert!(2409174571usize * (1345671093usize + 2783408230usize)
///         == 2409174571usize * 1345671093usize + 2409174571usize * 2783408230usize);
/// # }
/// ```
/// ## Zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x * 0 == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize * 0 == 0);
/// assert!(18446744073709551615usize * 0 == 0);
/// assert!(0usize * 0 == 0);
/// assert!(1usize * 0 == 0);
/// assert!(3451252396usize * 0 == 0);
/// assert!(655298939usize * 0 == 0);
/// assert!(4069990457usize * 0 == 0);
/// assert!(3206132173usize * 0 == 0);
/// assert!(826943105usize * 0 == 0);
/// assert!(3193533895usize * 0 == 0);
/// # }
/// ```
pub fn core_ops_mul_usize_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<usize>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize - 0usize == 1usize);
/// assert!(18446744073709551614usize - 0usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize - 18446744073709551614usize == 0usize);
/// assert!(18446744073709551615usize - 0usize == 18446744073709551615usize);
/// assert!(18446744073709551615usize - 18446744073709551615usize == 0usize);
/// assert!(0usize - 0usize == 0usize);
/// assert!(8142853404161125090usize - 5368018394586965824usize == 2774835009574159266usize);
/// assert!(8663917095874502518usize - 7645184149234187244usize == 1018732946640315274usize);
/// assert!(9887223399469622147usize - 9578407462305988569usize == 308815937163633578usize);
/// assert!(3491273977149240610usize - 53970920125254425usize == 3437303057023986185usize);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1usize - 18446744073709551614usize));
/// assert!(panics!(0usize - 18446744073709551614usize));
/// assert!(panics!(18446744073709551614usize - 18446744073709551615usize));
/// assert!(panics!(0usize - 18446744073709551615usize));
/// assert!(panics!(1usize - 18446744073709551615usize));
/// assert!(panics!(10008001737330562144usize - 10794788476270211805usize));
/// assert!(panics!(1713898964998039585usize - 4689212225853307237usize));
/// assert!(panics!(1006199177340080048usize - 14406582031551508367usize));
/// assert!(panics!(5331667419182117725usize - 16353101313237034631usize));
/// assert!(panics!(1469668556580630529usize - 16755197154863959040usize));
/// # }
/// ```
/// ## Subtraction is the reverse of addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x - y) + y == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((1usize - 1usize) + 1usize == 1usize);
/// assert!((18446744073709551614usize - 0usize) + 0usize == 18446744073709551614usize);
/// assert!((18446744073709551615usize - 18446744073709551614usize) + 18446744073709551614usize
///         == 18446744073709551615usize);
/// assert!((18446744073709551615usize - 18446744073709551615usize) + 18446744073709551615usize
///         == 18446744073709551615usize);
/// assert!((1usize - 0usize) + 0usize == 1usize);
/// assert!((18446744073709551615usize - 0usize) + 0usize == 18446744073709551615usize);
/// assert!((15648276777229511765usize - 4569254080613650786usize) + 4569254080613650786usize
///         == 15648276777229511765usize);
/// assert!((5986174390327718178usize - 5770749895494975749usize) + 5770749895494975749usize
///         == 5986174390327718178usize);
/// assert!((17261072257105501671usize - 11285076950106828105usize) + 11285076950106828105usize
///         == 17261072257105501671usize);
/// assert!((13540026440620963897usize - 3786550932260136959usize) + 3786550932260136959usize
///         == 13540026440620963897usize);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x - 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize - 0 == 0usize);
/// assert!(1usize - 0 == 1usize);
/// assert!(18446744073709551614usize - 0 == 18446744073709551614usize);
/// assert!(18446744073709551615usize - 0 == 18446744073709551615usize);
/// assert!(13488685390438589869usize - 0 == 13488685390438589869usize);
/// assert!(11443658673687419030usize - 0 == 11443658673687419030usize);
/// assert!(4601291067988090191usize - 0 == 4601291067988090191usize);
/// assert!(3263758057316415644usize - 0 == 3263758057316415644usize);
/// assert!(2618509300315392853usize - 0 == 2618509300315392853usize);
/// assert!(11147174984025895373usize - 0 == 11147174984025895373usize);
/// # }
/// ```
pub fn core_ops_add_usize_sub() {}
/// # Properties for [`usize::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.wrapping_sub(1usize) == 0usize);
/// assert!(1usize.wrapping_sub(0usize) == 1usize);
/// assert!(18446744073709551615usize.wrapping_sub(0usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_sub(18446744073709551614usize) == 0usize);
/// assert!(0usize.wrapping_sub(0usize) == 0usize);
/// assert!(9618329076396404692usize.wrapping_sub(2524219794991593736usize)
///         == 7094109281404810956usize);
/// assert!(8032143646255138464usize.wrapping_sub(4833867149834665782usize)
///         == 3198276496420472682usize);
/// assert!(18023290624867980503usize.wrapping_sub(16227474899174132712usize)
///         == 1795815725693847791usize);
/// assert!(12834869958359206327usize.wrapping_sub(7140782562572098726usize)
///         == 5694087395787107601usize);
/// assert!(5161985927819903861usize.wrapping_sub(2844024543866770046usize)
///         == 2317961383953133815usize);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == usize::down(x.up() - y.up() + usize::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.wrapping_sub(18446744073709551614usize) == 3usize);
/// assert!(1usize.wrapping_sub(18446744073709551615usize) == 2usize);
/// assert!(18446744073709551614usize.wrapping_sub(18446744073709551615usize)
///         == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(1usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(18446744073709551614usize) == 2usize);
/// assert!(8528798374265485529usize.wrapping_sub(16960996106428388294usize)
///         == 10014546341546648851usize);
/// assert!(12499931743021894938usize.wrapping_sub(12720968333190574286usize)
///         == 18225707483540872268usize);
/// assert!(1871788318447475180usize.wrapping_sub(3032588117673362142usize)
///         == 17285944274483664654usize);
/// assert!(12683891304210764318usize.wrapping_sub(14239759657187578678usize)
///         == 16890875720732737256usize);
/// assert!(13592660959437109449usize.wrapping_sub(16559068177565733340usize)
///         == 15480336855580927725usize);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == usize::down((x.up() - y.up()).rem_euclid(&(usize::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_sub(1usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(0usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_sub(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_sub(1usize) == 18446744073709551614usize);
/// assert!(0usize.wrapping_sub(18446744073709551615usize) == 1usize);
/// assert!(18446744073709551614usize.wrapping_sub(0usize) == 18446744073709551614usize);
/// assert!(10310899596092219019usize.wrapping_sub(5745234032727480330usize)
///         == 4565665563364738689usize);
/// assert!(11656330812887640927usize.wrapping_sub(3801354160368896324usize)
///         == 7854976652518744603usize);
/// assert!(6312216297519966773usize.wrapping_sub(11651405377768632641usize)
///         == 13107554993460885748usize);
/// assert!(5516559987185446446usize.wrapping_sub(18155916541093719130usize)
///         == 5807387519801278932usize);
/// # }
/// ```
/// ## Wrapping subtraction is the reverse of wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `(x.wrapping_sub(y)).wrapping_add(y) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((18446744073709551615usize.wrapping_sub(1usize)).wrapping_add(1usize)
///         == 18446744073709551615usize);
/// assert!((0usize.wrapping_sub(0usize)).wrapping_add(0usize) == 0usize);
/// assert!((18446744073709551615usize.wrapping_sub(18446744073709551614usize))
///         .wrapping_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!((1usize.wrapping_sub(0usize)).wrapping_add(0usize) == 1usize);
/// assert!((18446744073709551614usize.wrapping_sub(0usize)).wrapping_add(0usize)
///         == 18446744073709551614usize);
/// assert!((3777659140235671407usize.wrapping_sub(17184108981392586987usize))
///         .wrapping_add(17184108981392586987usize) == 3777659140235671407usize);
/// assert!((2462705600177277105usize.wrapping_sub(16172497647415432226usize))
///         .wrapping_add(16172497647415432226usize) == 2462705600177277105usize);
/// assert!((15011242359811499821usize.wrapping_sub(2071096935029956885usize))
///         .wrapping_add(2071096935029956885usize) == 15011242359811499821usize);
/// assert!((12112415515546201325usize.wrapping_sub(6132671271335831002usize))
///         .wrapping_add(6132671271335831002usize) == 12112415515546201325usize);
/// assert!((9614483651844652471usize.wrapping_sub(6909438265035934954usize))
///         .wrapping_add(6909438265035934954usize) == 9614483651844652471usize);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.wrapping_sub(0) == 18446744073709551614usize);
/// assert!(18446744073709551615usize.wrapping_sub(0) == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(0) == 0usize);
/// assert!(1usize.wrapping_sub(0) == 1usize);
/// assert!(10546707344850483817usize.wrapping_sub(0) == 10546707344850483817usize);
/// assert!(17723741501407545115usize.wrapping_sub(0) == 17723741501407545115usize);
/// assert!(15373647396213578758usize.wrapping_sub(0) == 15373647396213578758usize);
/// assert!(6214836112951378206usize.wrapping_sub(0) == 6214836112951378206usize);
/// assert!(8847731337484830873usize.wrapping_sub(0) == 8847731337484830873usize);
/// assert!(7806973941879689663usize.wrapping_sub(0) == 7806973941879689663usize);
/// # }
/// ```
pub fn core_ops_add_usize_wrapping_sub() {}
/// # Properties for [`usize::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(usize::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_sub(0usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_sub(1usize) == Some(18446744073709551613usize));
/// assert!(1usize.checked_sub(1usize) == Some(0usize));
/// assert!(1usize.checked_sub(0usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_sub(0usize) == Some(18446744073709551614usize));
/// assert!(5865508096022319613usize.checked_sub(2009356697088308638usize)
///         == Some(3856151398934010975usize));
/// assert!(14145182192843625720usize.checked_sub(11054776169478634192usize)
///         == Some(3090406023364991528usize));
/// assert!(16949318387516325180usize.checked_sub(7519447801116950488usize)
///         == Some(9429870586399374692usize));
/// assert!(9099261935371435610usize.checked_sub(5527093864065406773usize)
///         == Some(3572168071306028837usize));
/// assert!(14919261016548175147usize.checked_sub(7394339025379615873usize)
///         == Some(7524921991168559274usize));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_sub(1usize) == None);
/// assert!(0usize.checked_sub(18446744073709551615usize) == None);
/// assert!(1usize.checked_sub(18446744073709551614usize) == None);
/// assert!(0usize.checked_sub(18446744073709551614usize) == None);
/// assert!(18446744073709551614usize.checked_sub(18446744073709551615usize) == None);
/// assert!(5912890625649266418usize.checked_sub(13806895533892505015usize) == None);
/// assert!(6389479433703895989usize.checked_sub(8358266209936582465usize) == None);
/// assert!(4193056963031790679usize.checked_sub(13843356409983269087usize) == None);
/// assert!(4190254659584607254usize.checked_sub(6525621109255697346usize) == None);
/// assert!(7873490844734927805usize.checked_sub(14005348640098484878usize) == None);
/// # }
/// ```
/// ## Checked subtraction is the reverse of checked addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `(x.checked_sub(y)).and_then(|r| r.checked_add(y)) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((18446744073709551615usize.checked_sub(0usize)).and_then(|r| r.checked_add(0usize))
///         == Some(18446744073709551615usize));
/// assert!((18446744073709551614usize.checked_sub(0usize)).and_then(|r| r.checked_add(0usize))
///         == Some(18446744073709551614usize));
/// assert!((18446744073709551614usize.checked_sub(1usize)).and_then(|r| r.checked_add(1usize))
///         == Some(18446744073709551614usize));
/// assert!((0usize.checked_sub(0usize)).and_then(|r| r.checked_add(0usize)) == Some(0usize));
/// assert!((8009728807838115217usize.checked_sub(67256516030928668usize))
///         .and_then(|r| r.checked_add(67256516030928668usize))
///         == Some(8009728807838115217usize));
/// assert!((10561639564784232478usize.checked_sub(8785087237299076002usize))
///         .and_then(|r| r.checked_add(8785087237299076002usize))
///         == Some(10561639564784232478usize));
/// assert!((12705662100520079542usize.checked_sub(6141599397915733761usize))
///         .and_then(|r| r.checked_add(6141599397915733761usize))
///         == Some(12705662100520079542usize));
/// assert!((16777284374870699054usize.checked_sub(6149564933671681785usize))
///         .and_then(|r| r.checked_add(6149564933671681785usize))
///         == Some(16777284374870699054usize));
/// assert!((12249011539979776523usize.checked_sub(10884483193190351510usize))
///         .and_then(|r| r.checked_add(10884483193190351510usize))
///         == Some(12249011539979776523usize));
/// assert!((11471000977050504534usize.checked_sub(1093058913877942105usize))
///         .and_then(|r| r.checked_add(1093058913877942105usize))
///         == Some(11471000977050504534usize));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_sub(0) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_sub(0) == Some(0usize));
/// assert!(18446744073709551614usize.checked_sub(0) == Some(18446744073709551614usize));
/// assert!(1usize.checked_sub(0) == Some(1usize));
/// assert!(18446744073709551615usize.checked_sub(0) == Some(18446744073709551615usize));
/// assert!(7024864795974598746usize.checked_sub(0) == Some(7024864795974598746usize));
/// assert!(8762437719200723568usize.checked_sub(0) == Some(8762437719200723568usize));
/// assert!(17825707667076486860usize.checked_sub(0) == Some(17825707667076486860usize));
/// assert!(3927486542641403320usize.checked_sub(0) == Some(3927486542641403320usize));
/// assert!(10810969114723342009usize.checked_sub(0) == Some(10810969114723342009usize));
/// assert!(9650247448560023518usize.checked_sub(0) == Some(9650247448560023518usize));
/// # }
/// ```
pub fn core_ops_add_usize_checked_sub() {}
/// # Properties for [`usize::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == usize::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_sub(0usize) == 18446744073709551614usize);
/// assert!(1usize.saturating_sub(0usize) == 1usize);
/// assert!(0usize.saturating_sub(0usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_sub(0usize) == 18446744073709551615usize);
/// assert!(7945556330300154326usize.saturating_sub(12629741080008289498usize) == 0usize);
/// assert!(14556166693713666600usize.saturating_sub(4974257548872593863usize)
///         == 9581909144841072737usize);
/// assert!(11254992869105289703usize.saturating_sub(17862103815773109130usize) == 0usize);
/// assert!(3464203996450489678usize.saturating_sub(8467972161000130327usize) == 0usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_sub(0usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_sub(18446744073709551615usize) == 0usize);
/// assert!(1usize.saturating_sub(0usize) == 1usize);
/// assert!(18446744073709551615usize.saturating_sub(18446744073709551614usize) == 1usize);
/// assert!(18446744073709551614usize.saturating_sub(0usize) == 18446744073709551614usize);
/// assert!(17785595566364221454usize.saturating_sub(17443230930833566353usize)
///         == 342364635530655101usize);
/// assert!(12155225723683947891usize.saturating_sub(7171606477013558856usize)
///         == 4983619246670389035usize);
/// assert!(6687858734855333682usize.saturating_sub(4955158948495689148usize)
///         == 1732699786359644534usize);
/// assert!(17746810948255230576usize.saturating_sub(10769109137096537950usize)
///         == 6977701811158692626usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_sub(1usize) == 0);
/// assert!(0usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(0usize.saturating_sub(18446744073709551614usize) == 0);
/// assert!(1usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(2618598081709634864usize.saturating_sub(11321646609197154772usize) == 0);
/// assert!(8883175605574204466usize.saturating_sub(13067135946992854034usize) == 0);
/// assert!(6743507871290220430usize.saturating_sub(15323563991748863683usize) == 0);
/// assert!(653217795807315169usize.saturating_sub(7419146659656573392usize) == 0);
/// assert!(4180140545197405131usize.saturating_sub(15944698905248126195usize) == 0);
/// # }
/// ```
pub fn core_ops_sub_usize_saturating_sub() {}
/// # Properties for [`core::ops::Add::<usize>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x + y == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize + 0usize == 0usize);
/// assert!(0usize + 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0usize + 18446744073709551615usize == 18446744073709551615usize);
/// assert!(18446744073709551614usize + 0usize == 18446744073709551614usize);
/// assert!(0usize + 1usize == 1usize);
/// assert!(7806825945228772866usize + 3533907004873029644usize == 11340732950101802510usize);
/// assert!(428014477794293072usize + 15219596241361542723usize == 15647610719155835795usize);
/// assert!(7689798402087650596usize + 4471721656331181860usize == 12161520058418832456usize);
/// assert!(1412496363564613007usize + 6009658429150952002usize == 7422154792715565009usize);
/// assert!(3862558146380288102usize + 14184624009280236755usize == 18047182155660524857usize);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614usize + 18446744073709551614usize));
/// assert!(panics!(18446744073709551615usize + 18446744073709551615usize));
/// assert!(panics!(1usize + 18446744073709551615usize));
/// assert!(panics!(18446744073709551615usize + 1usize));
/// assert!(panics!(18446744073709551615usize + 18446744073709551614usize));
/// assert!(panics!(18446744073709551614usize + 18446744073709551615usize));
/// assert!(panics!(10368113762359142120usize + 9963635987456779731usize));
/// assert!(panics!(16702656358892486099usize + 12451637191121733539usize));
/// assert!(panics!(9178086077756098535usize + 11171267287681670382usize));
/// assert!(panics!(15757977677384182039usize + 13605745617189664170usize));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize + 18446744073709551614usize == 18446744073709551614usize + 0usize);
/// assert!(18446744073709551614usize + 0usize == 0usize + 18446744073709551614usize);
/// assert!(0usize + 1usize == 1usize + 0usize);
/// assert!(1usize + 1usize == 1usize + 1usize);
/// assert!(0usize + 18446744073709551615usize == 18446744073709551615usize + 0usize);
/// assert!(1usize + 18446744073709551614usize == 18446744073709551614usize + 1usize);
/// assert!(5876305528482333115usize + 11908109937929691380usize
///         == 11908109937929691380usize + 5876305528482333115usize);
/// assert!(5230015868462562393usize + 8943556176772194075usize
///         == 8943556176772194075usize + 5230015868462562393usize);
/// assert!(5152756109861782526usize + 10231222079206075967usize
///         == 10231222079206075967usize + 5152756109861782526usize);
/// assert!(6209498648946889356usize + 11037203673558055150usize
///         == 11037203673558055150usize + 6209498648946889356usize);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize + 0 == 0usize);
/// assert!(18446744073709551615usize + 0 == 18446744073709551615usize);
/// assert!(18446744073709551614usize + 0 == 18446744073709551614usize);
/// assert!(1usize + 0 == 1usize);
/// assert!(13293970807673266626usize + 0 == 13293970807673266626usize);
/// assert!(3801635669912117020usize + 0 == 3801635669912117020usize);
/// assert!(8198317570902590899usize + 0 == 8198317570902590899usize);
/// assert!(7583805285162463377usize + 0 == 7583805285162463377usize);
/// assert!(11343628218470631244usize + 0 == 11343628218470631244usize);
/// assert!(1810114264916066134usize + 0 == 1810114264916066134usize);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 1usize == 1usize);
/// assert!(0 + 0usize == 0usize);
/// assert!(0 + 18446744073709551615usize == 18446744073709551615usize);
/// assert!(0 + 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0 + 3809328618584704530usize == 3809328618584704530usize);
/// assert!(0 + 15953924215763056831usize == 15953924215763056831usize);
/// assert!(0 + 12599324739762226002usize == 12599324739762226002usize);
/// assert!(0 + 9553230110641873922usize == 9553230110641873922usize);
/// assert!(0 + 1562774704114127116usize == 1562774704114127116usize);
/// assert!(0 + 16401396077750659956usize == 16401396077750659956usize);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= usize::MAX.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0usize + 0usize) + 18446744073709551615usize
///         == 0usize + (0usize + 18446744073709551615usize));
/// assert!((1usize + 18446744073709551614usize) + 0usize
///         == 1usize + (18446744073709551614usize + 0usize));
/// assert!((18446744073709551614usize + 0usize) + 0usize
///         == 18446744073709551614usize + (0usize + 0usize));
/// assert!((1usize + 0usize) + 1usize == 1usize + (0usize + 1usize));
/// assert!((246396164729737932usize + 209359616097460718usize) + 16588232868187314087usize
///         == 246396164729737932usize
///             + (209359616097460718usize + 16588232868187314087usize));
/// assert!((8316783747032347160usize + 1172275417719601977usize) + 5360233930547054744usize
///         == 8316783747032347160usize
///             + (1172275417719601977usize + 5360233930547054744usize));
/// assert!((9519153513488759590usize + 2517331321474669830usize) + 5346181409655037304usize
///         == 9519153513488759590usize
///             + (2517331321474669830usize + 5346181409655037304usize));
/// assert!((90748060878042810usize + 4339070313265244573usize) + 11277636240973771139usize
///         == 90748060878042810usize
///             + (4339070313265244573usize + 11277636240973771139usize));
/// assert!((1701848048845129566usize + 6545293069332945680usize) + 1610212533647520436usize
///         == 1701848048845129566usize
///             + (6545293069332945680usize + 1610212533647520436usize));
/// assert!((3910330855743005224usize + 1020447958184037288usize) + 13395077936000434289usize
///         == 3910330855743005224usize
///             + (1020447958184037288usize + 13395077936000434289usize));
/// # }
/// ```
pub fn core_ops_add_usize_add() {}
/// # Properties for [`usize::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down((x.up() + y.up()) % (usize::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.wrapping_add(0usize) == 18446744073709551615usize);
/// assert!(1usize.wrapping_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(1usize.wrapping_add(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551612usize);
/// assert!(0usize.wrapping_add(1usize) == 1usize);
/// assert!(15235124485451831874usize.wrapping_add(6747328178760566213usize)
///         == 3535708590502846471usize);
/// assert!(1802855230305248732usize.wrapping_add(13129040570932964408usize)
///         == 14931895801238213140usize);
/// assert!(3671613585811602930usize.wrapping_add(16388886141393815456usize)
///         == 1613755653495866770usize);
/// assert!(6703559748608055576usize.wrapping_add(12926559008937643417usize)
///         == 1183374683836147377usize);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.wrapping_add(0usize) == 18446744073709551614usize);
/// assert!(1usize.wrapping_add(0usize) == 1usize);
/// assert!(0usize.wrapping_add(0usize) == 0usize);
/// assert!(1usize.wrapping_add(1usize) == 2usize);
/// assert!(18446744073709551615usize.wrapping_add(0usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(3648819298171859381usize.wrapping_add(5315207303443876597usize)
///         == 8964026601615735978usize);
/// assert!(8643717940146586930usize.wrapping_add(1043042728609468183usize)
///         == 9686760668756055113usize);
/// assert!(4351032966922141358usize.wrapping_add(10070919490876838042usize)
///         == 14421952457798979400usize);
/// assert!(1442824337817309790usize.wrapping_add(10633846441590723547usize)
///         == 12076670779408033337usize);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down(x.up() + y.up() - usize::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551612usize);
/// assert!(18446744073709551615usize.wrapping_add(1usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551613usize);
/// assert!(1usize.wrapping_add(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551615usize)
///         == 18446744073709551613usize);
/// assert!(18446744073709551615usize.wrapping_add(18446744073709551615usize)
///         == 18446744073709551614usize);
/// assert!(16784322590542482783usize.wrapping_add(2049831613461701502usize)
///         == 387410130294632669usize);
/// assert!(17725414183693633968usize.wrapping_add(16313791575174388742usize)
///         == 15592461685158471094usize);
/// assert!(16097914311492656061usize.wrapping_add(11883608587744380932usize)
///         == 9534778825527485377usize);
/// assert!(3221882536032842762usize.wrapping_add(15661504760417905068usize)
///         == 436643222741196214usize);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == y.wrapping_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.wrapping_add(18446744073709551615usize)
///         == 18446744073709551615usize.wrapping_add(1usize));
/// assert!(1usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551614usize.wrapping_add(1usize));
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551614usize.wrapping_add(18446744073709551614usize));
/// assert!(18446744073709551614usize.wrapping_add(0usize)
///         == 0usize.wrapping_add(18446744073709551614usize));
/// assert!(18446744073709551615usize.wrapping_add(0usize)
///         == 0usize.wrapping_add(18446744073709551615usize));
/// assert!(5749525150258181653usize.wrapping_add(17042020308703899379usize)
///         == 17042020308703899379usize.wrapping_add(5749525150258181653usize));
/// assert!(9686283840241351837usize.wrapping_add(15169358232551433622usize)
///         == 15169358232551433622usize.wrapping_add(9686283840241351837usize));
/// assert!(507497753849002659usize.wrapping_add(17399171964854022573usize)
///         == 17399171964854022573usize.wrapping_add(507497753849002659usize));
/// assert!(6165197429896190800usize.wrapping_add(11385056947126196306usize)
///         == 11385056947126196306usize.wrapping_add(6165197429896190800usize));
/// assert!(11550396722627664018usize.wrapping_add(11351437391092708074usize)
///         == 11351437391092708074usize.wrapping_add(11550396722627664018usize));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(0) == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_add(0) == 0usize);
/// assert!(1usize.wrapping_add(0) == 1usize);
/// assert!(18446744073709551615usize.wrapping_add(0) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_add(0) == 18446744073709551614usize);
/// assert!(17975327953889452157usize.wrapping_add(0) == 17975327953889452157usize);
/// assert!(5217278035389553391usize.wrapping_add(0) == 5217278035389553391usize);
/// assert!(5665900091834289549usize.wrapping_add(0) == 5665900091834289549usize);
/// assert!(14591279827280609726usize.wrapping_add(0) == 14591279827280609726usize);
/// assert!(7320064059349324317usize.wrapping_add(0) == 7320064059349324317usize);
/// assert!(4658390972231756612usize.wrapping_add(0) == 4658390972231756612usize);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let zero: usize = 0;
///         zero.wrapping_add(x) == x
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(0usize) == 0usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(1usize) == 1usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(18446744073709551615usize) == 18446744073709551615usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(18446744073709551614usize) == 18446744073709551614usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(6218984787490378507usize) == 6218984787490378507usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(13858952946617097135usize) == 13858952946617097135usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(1680679577280727267usize) == 1680679577280727267usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(424543345432605235usize) == 424543345432605235usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(4113057646558255362usize) == 4113057646558255362usize
///     });
/// assert!({
///         let zero: usize = 0;
///         zero.wrapping_add(15777099638567874463usize) == 15777099638567874463usize
///     });
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `x.up() + y.up() + z.up() <= usize::MAX.up()`  
/// __Postcondition:__ `(x.wrapping_add(y)).wrapping_add(z) == x.wrapping_add(y.wrapping_add(z))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0usize.wrapping_add(0usize)).wrapping_add(18446744073709551614usize)
///         == 0usize.wrapping_add(0usize.wrapping_add(18446744073709551614usize)));
/// assert!((1usize.wrapping_add(0usize)).wrapping_add(0usize)
///         == 1usize.wrapping_add(0usize.wrapping_add(0usize)));
/// assert!((0usize.wrapping_add(0usize)).wrapping_add(0usize)
///         == 0usize.wrapping_add(0usize.wrapping_add(0usize)));
/// assert!((18446744073709551615usize.wrapping_add(0usize)).wrapping_add(0usize)
///         == 18446744073709551615usize.wrapping_add(0usize.wrapping_add(0usize)));
/// assert!((0usize.wrapping_add(18446744073709551615usize)).wrapping_add(0usize)
///         == 0usize.wrapping_add(18446744073709551615usize.wrapping_add(0usize)));
/// assert!((3648157078776609435usize.wrapping_add(3148346129954011099usize))
///         .wrapping_add(1649643555090817231usize)
///         == 3648157078776609435usize
///             .wrapping_add(
///                 3148346129954011099usize.wrapping_add(1649643555090817231usize),
///             ));
/// assert!((2718308695701804333usize.wrapping_add(10086994836505627977usize))
///         .wrapping_add(4891952302632464397usize)
///         == 2718308695701804333usize
///             .wrapping_add(
///                 10086994836505627977usize.wrapping_add(4891952302632464397usize),
///             ));
/// assert!((2816163504889826859usize.wrapping_add(2290296657238555457usize))
///         .wrapping_add(4604218036408271915usize)
///         == 2816163504889826859usize
///             .wrapping_add(
///                 2290296657238555457usize.wrapping_add(4604218036408271915usize),
///             ));
/// assert!((6819670602500472543usize.wrapping_add(7404044978454508990usize))
///         .wrapping_add(336195141141251327usize)
///         == 6819670602500472543usize
///             .wrapping_add(7404044978454508990usize.wrapping_add(336195141141251327usize)));
/// assert!((411283087666211931usize.wrapping_add(13519459569222751417usize))
///         .wrapping_add(682478499063716272usize)
///         == 411283087666211931usize
///             .wrapping_add(
///                 13519459569222751417usize.wrapping_add(682478499063716272usize),
///             ));
/// # }
/// ```
pub fn core_ops_add_usize_wrapping_add() {}
/// # Properties for [`usize::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(usize::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_add(18446744073709551615usize) == Some(18446744073709551615usize));
/// assert!(18446744073709551615usize.checked_add(0usize) == Some(18446744073709551615usize));
/// assert!(1usize.checked_add(0usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_add(1usize) == Some(18446744073709551615usize));
/// assert!(0usize.checked_add(0usize) == Some(0usize));
/// assert!(5795064554200689617usize.checked_add(7089888634905344749usize)
///         == Some(12884953189106034366usize));
/// assert!(3558026152131867016usize.checked_add(7129031300367899975usize)
///         == Some(10687057452499766991usize));
/// assert!(5458309914845725198usize.checked_add(9388159314455024305usize)
///         == Some(14846469229300749503usize));
/// assert!(650579054521983677usize.checked_add(4499550872796186994usize)
///         == Some(5150129927318170671usize));
/// assert!(8960009763094956822usize.checked_add(4661163856034892818usize)
///         == Some(13621173619129849640usize));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551615usize.checked_add(18446744073709551614usize) == None);
/// assert!(18446744073709551614usize.checked_add(18446744073709551614usize) == None);
/// assert!(18446744073709551614usize.checked_add(18446744073709551615usize) == None);
/// assert!(18446744073709551615usize.checked_add(1usize) == None);
/// assert!(18446744073709551615usize.checked_add(18446744073709551615usize) == None);
/// assert!(1usize.checked_add(18446744073709551615usize) == None);
/// assert!(6435683034382238904usize.checked_add(12702887011103373593usize) == None);
/// assert!(16151641731328205592usize.checked_add(14588585917432734874usize) == None);
/// assert!(8710356648194576439usize.checked_add(12407641689993887067usize) == None);
/// assert!(9944486796035871730usize.checked_add(17866173571127626965usize) == None);
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y) == y.checked_add(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.checked_add(1usize)
///         == 1usize.checked_add(18446744073709551614usize));
/// assert!(1usize.checked_add(18446744073709551615usize)
///         == 18446744073709551615usize.checked_add(1usize));
/// assert!(1usize.checked_add(0usize) == 0usize.checked_add(1usize));
/// assert!(1usize.checked_add(18446744073709551614usize)
///         == 18446744073709551614usize.checked_add(1usize));
/// assert!(0usize.checked_add(0usize) == 0usize.checked_add(0usize));
/// assert!(18446744073709551615usize.checked_add(1usize)
///         == 1usize.checked_add(18446744073709551615usize));
/// assert!(13177314144719991371usize.checked_add(5010917382322908355usize)
///         == 5010917382322908355usize.checked_add(13177314144719991371usize));
/// assert!(16647434911058556091usize.checked_add(1462777363391334641usize)
///         == 1462777363391334641usize.checked_add(16647434911058556091usize));
/// assert!(16316707662556971123usize.checked_add(8948323433076700077usize)
///         == 8948323433076700077usize.checked_add(16316707662556971123usize));
/// assert!(18247883173692627419usize.checked_add(6678583807915110081usize)
///         == 6678583807915110081usize.checked_add(18247883173692627419usize));
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(0usize) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_add(0usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_add(0usize) == Some(18446744073709551614usize));
/// assert!(0usize.checked_add(0usize) == Some(0usize));
/// assert!(18446744073709551615usize.checked_add(0usize) == Some(18446744073709551615usize));
/// assert!(5031901551855407090usize.checked_add(0usize) == Some(5031901551855407090usize));
/// assert!(7029012132155784068usize.checked_add(0usize) == Some(7029012132155784068usize));
/// assert!(16664333446637531133usize.checked_add(0usize) == Some(16664333446637531133usize));
/// assert!(7080642437826201678usize.checked_add(0usize) == Some(7080642437826201678usize));
/// assert!(10977903287121524444usize.checked_add(0usize) == Some(10977903287121524444usize));
/// assert!(10293186126536842916usize.checked_add(0usize) == Some(10293186126536842916usize));
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0usize.checked_add(x) == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_add(0usize) == Some(0usize));
/// assert!(0usize.checked_add(18446744073709551615usize) == Some(18446744073709551615usize));
/// assert!(0usize.checked_add(18446744073709551614usize) == Some(18446744073709551614usize));
/// assert!(0usize.checked_add(1usize) == Some(1usize));
/// assert!(0usize.checked_add(494241482872672950usize) == Some(494241482872672950usize));
/// assert!(0usize.checked_add(12658139017451472020usize) == Some(12658139017451472020usize));
/// assert!(0usize.checked_add(7574206001503588353usize) == Some(7574206001503588353usize));
/// assert!(0usize.checked_add(1158432357443489479usize) == Some(1158432357443489479usize));
/// assert!(0usize.checked_add(3354718962000177349usize) == Some(3354718962000177349usize));
/// assert!(0usize.checked_add(787979845692288965usize) == Some(787979845692288965usize));
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : usize, y : usize, z : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_add(y).and_then(|iv| iv.checked_add(z))
///         == y.checked_add(z).and_then(|iv| x.checked_add(iv))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_add(1usize).and_then(|iv| iv.checked_add(0usize))
///         == 1usize.checked_add(0usize).and_then(|iv| 0usize.checked_add(iv)));
/// assert!(0usize.checked_add(18446744073709551614usize).and_then(|iv| iv.checked_add(0usize))
///         == 18446744073709551614usize
///             .checked_add(0usize)
///             .and_then(|iv| 0usize.checked_add(iv)));
/// assert!(1usize.checked_add(0usize).and_then(|iv| iv.checked_add(1usize))
///         == 0usize.checked_add(1usize).and_then(|iv| 1usize.checked_add(iv)));
/// assert!(1usize
///         .checked_add(18446744073709551615usize)
///         .and_then(|iv| iv.checked_add(18446744073709551614usize))
///         == 18446744073709551615usize
///             .checked_add(18446744073709551614usize)
///             .and_then(|iv| 1usize.checked_add(iv)));
/// assert!(0usize.checked_add(1usize).and_then(|iv| iv.checked_add(18446744073709551614usize))
///         == 1usize
///             .checked_add(18446744073709551614usize)
///             .and_then(|iv| 0usize.checked_add(iv)));
/// assert!(1usize.checked_add(0usize).and_then(|iv| iv.checked_add(18446744073709551614usize))
///         == 0usize
///             .checked_add(18446744073709551614usize)
///             .and_then(|iv| 1usize.checked_add(iv)));
/// assert!(6550029170896886389usize
///         .checked_add(13900474709262405044usize)
///         .and_then(|iv| iv.checked_add(3997776199111108020usize))
///         == 13900474709262405044usize
///             .checked_add(3997776199111108020usize)
///             .and_then(|iv| 6550029170896886389usize.checked_add(iv)));
/// assert!(10323083454707841451usize
///         .checked_add(13950433294759401352usize)
///         .and_then(|iv| iv.checked_add(4535391916275528968usize))
///         == 13950433294759401352usize
///             .checked_add(4535391916275528968usize)
///             .and_then(|iv| 10323083454707841451usize.checked_add(iv)));
/// assert!(11395181734896756948usize
///         .checked_add(11111635814381340472usize)
///         .and_then(|iv| iv.checked_add(732135743654903880usize))
///         == 11111635814381340472usize
///             .checked_add(732135743654903880usize)
///             .and_then(|iv| 11395181734896756948usize.checked_add(iv)));
/// assert!(12955968649426807003usize
///         .checked_add(7483139163074833141usize)
///         .and_then(|iv| iv.checked_add(5637306834654658286usize))
///         == 7483139163074833141usize
///             .checked_add(5637306834654658286usize)
///             .and_then(|iv| 12955968649426807003usize.checked_add(iv)));
/// # }
/// ```
pub fn core_ops_add_usize_checked_add() {}
/// # Properties for [`usize::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == usize::down((x.up() + y.up()).min(usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_add(0usize) == 0usize);
/// assert!(0usize.saturating_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551615usize.saturating_add(1usize) == 18446744073709551615usize);
/// assert!(0usize.saturating_add(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(1usize.saturating_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(16281818915477680200usize.saturating_add(15920020320886183702usize)
///         == 18446744073709551615usize);
/// assert!(16138103697042413005usize.saturating_add(9356069411504569164usize)
///         == 18446744073709551615usize);
/// assert!(6707462531948697530usize.saturating_add(18093601426847002073usize)
///         == 18446744073709551615usize);
/// assert!(14938868503558851222usize.saturating_add(5395529792953060883usize)
///         == 18446744073709551615usize);
/// assert!(4821371318193376756usize.saturating_add(1673283564027971069usize)
///         == 6494654882221347825usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.saturating_add(0usize) == 1usize);
/// assert!(0usize.saturating_add(1usize) == 1usize);
/// assert!(1usize.saturating_add(1usize) == 2usize);
/// assert!(0usize.saturating_add(0usize) == 0usize);
/// assert!(8098007502630414795usize.saturating_add(5366467384943610202usize)
///         == 13464474887574024997usize);
/// assert!(13551396658887834946usize.saturating_add(964271122464769230usize)
///         == 14515667781352604176usize);
/// assert!(8771660348056251754usize.saturating_add(9380314853244878430usize)
///         == 18151975201301130184usize);
/// assert!(4277429013620711612usize.saturating_add(881573156142959911usize)
///         == 5159002169763671523usize);
/// assert!(2296352774178396621usize.saturating_add(12303135946414054671usize)
///         == 14599488720592451292usize);
/// assert!(11335173689290817817usize.saturating_add(3035755102787251309usize)
///         == 14370928792078069126usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == usize::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_add(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(1usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(16596871428322368808usize.saturating_add(8454341641206281822usize) == usize::MAX);
/// assert!(13565132262577654874usize.saturating_add(14735423762625033129usize) == usize::MAX);
/// assert!(10696762852669022620usize.saturating_add(13235767023908233617usize) == usize::MAX);
/// assert!(17441168467740946626usize.saturating_add(1936529413840522277usize) == usize::MAX);
/// # }
/// ```
pub fn core_ops_add_usize_saturating_add() {}
/// # Properties for [`i8::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i8::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-127i8).checked_neg() == Some(127i8));
/// assert!(1i8.checked_neg() == Some((-1i8)));
/// assert!(0i8.checked_neg() == Some(0i8));
/// assert!(126i8.checked_neg() == Some((-126i8)));
/// assert!((-1i8).checked_neg() == Some(1i8));
/// assert!(127i8.checked_neg() == Some((-127i8)));
/// assert!(110i8.checked_neg() == Some((-110i8)));
/// assert!(88i8.checked_neg() == Some((-88i8)));
/// assert!(101i8.checked_neg() == Some((-101i8)));
/// assert!(55i8.checked_neg() == Some((-55i8)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).checked_neg() == None);
/// # }
/// ```
pub fn t_i8_checked_neg() {}
/// # Properties for [`i8::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i8::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         0i8.neg() == 0i8
///     });
/// assert!({
///         use std::ops::Neg;
///         1i8.neg() == (-1i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         127i8.neg() == (-127i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-127i8).neg() == 127i8
///     });
/// assert!({
///         use std::ops::Neg;
///         126i8.neg() == (-126i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i8).neg() == 1i8
///     });
/// assert!({
///         use std::ops::Neg;
///         (-58i8).neg() == 58i8
///     });
/// assert!({
///         use std::ops::Neg;
///         (-39i8).neg() == 39i8
///     });
/// assert!({
///         use std::ops::Neg;
///         (-79i8).neg() == 79i8
///     });
/// assert!({
///         use std::ops::Neg;
///         83i8.neg() == (-83i8)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 128i8).neg())
///     });
/// # }
/// ```
pub fn t_i8_neg() {}
/// # Properties for [`i8::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i8::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-127i8).overflowing_neg() == (127i8, false));
/// assert!(127i8.overflowing_neg() == ((-127i8), false));
/// assert!(126i8.overflowing_neg() == ((-126i8), false));
/// assert!(1i8.overflowing_neg() == ((-1i8), false));
/// assert!((-1i8).overflowing_neg() == (1i8, false));
/// assert!(0i8.overflowing_neg() == (0i8, false));
/// assert!(23i8.overflowing_neg() == ((-23i8), false));
/// assert!((-52i8).overflowing_neg() == (52i8, false));
/// assert!(116i8.overflowing_neg() == ((-116i8), false));
/// assert!((-29i8).overflowing_neg() == (29i8, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i8::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).overflowing_neg() == ((-128i8), true));
/// # }
/// ```
pub fn t_i8_overflowing_neg() {}
/// # Properties for [`i8::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x - y == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i8 - 127i8 == (-126i8));
/// assert!(0i8 - 1i8 == (-1i8));
/// assert!(1i8 - (-1i8) == 2i8);
/// assert!((-128i8) - 0i8 == (-128i8));
/// assert!((-127i8) - (-128i8) == 1i8);
/// assert!((-127i8) - 1i8 == (-128i8));
/// assert!(56i8 - (-1i8) == 57i8);
/// assert!(69i8 - 65i8 == 4i8);
/// assert!((-83i8) - 18i8 == (-101i8));
/// assert!(40i8 - 83i8 == (-43i8));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 128i8) - 126i8));
/// assert!(panics!((- 127i8) - 126i8));
/// assert!(panics!(0i8 - (- 128i8)));
/// assert!(panics!(127i8 - (- 1i8)));
/// assert!(panics!(126i8 - (- 127i8)));
/// assert!(panics!(127i8 - (- 127i8)));
/// assert!(panics!(17i8 - (- 127i8)));
/// assert!(panics!(104i8 - (- 90i8)));
/// assert!(panics!((- 111i8) - 105i8));
/// assert!(panics!(71i8 - (- 127i8)));
/// # }
/// ```
pub fn core_ops_sub_i8_sub() {}
/// # Properties for [`i8::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i8::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(127i8.checked_sub(1i8) == Some(126i8));
/// assert!(126i8.checked_sub(126i8) == Some(0i8));
/// assert!((-127i8).checked_sub((-128i8)) == Some(1i8));
/// assert!(126i8.checked_sub(0i8) == Some(126i8));
/// assert!(0i8.checked_sub(127i8) == Some((-127i8)));
/// assert!(1i8.checked_sub((-1i8)) == Some(2i8));
/// assert!(94i8.checked_sub((-29i8)) == Some(123i8));
/// assert!(82i8.checked_sub((-18i8)) == Some(100i8));
/// assert!(59i8.checked_sub(41i8) == Some(18i8));
/// assert!(68i8.checked_sub(74i8) == Some((-6i8)));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(127i8.checked_sub((-128i8)) == None);
/// assert!((-128i8).checked_sub(126i8) == None);
/// assert!(127i8.checked_sub((-127i8)) == None);
/// assert!(127i8.checked_sub((-1i8)) == None);
/// assert!((-128i8).checked_sub(127i8) == None);
/// assert!(126i8.checked_sub((-127i8)) == None);
/// assert!((-57i8).checked_sub(82i8) == None);
/// assert!(26i8.checked_sub((-120i8)) == None);
/// assert!(72i8.checked_sub((-75i8)) == None);
/// assert!((-122i8).checked_sub(47i8) == None);
/// # }
/// ```
pub fn core_ops_sub_i8_checked_sub() {}
/// # Properties for [`i8::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-127i8).wrapping_sub((-127i8)) == 0i8);
/// assert!((-127i8).wrapping_sub((-128i8)) == 1i8);
/// assert!(0i8.wrapping_sub((-127i8)) == 127i8);
/// assert!((-1i8).wrapping_sub(1i8) == (-2i8));
/// assert!(127i8.wrapping_sub(1i8) == 126i8);
/// assert!(1i8.wrapping_sub(127i8) == (-126i8));
/// assert!(72i8.wrapping_sub(38i8) == 34i8);
/// assert!(29i8.wrapping_sub(4i8) == 25i8);
/// assert!(98i8.wrapping_sub(123i8) == (-25i8));
/// assert!((-38i8).wrapping_sub((-69i8)) == 31i8);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i8::down(
///             (-i8::MIN.up() + x.up() - y.up()).rem_euclid(&(-i8::MIN.up() * 2))
///                 + i8::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).wrapping_sub(127i8) == 1i8);
/// assert!(126i8.wrapping_sub((-127i8)) == (-3i8));
/// assert!(126i8.wrapping_sub((-128i8)) == (-2i8));
/// assert!((-127i8).wrapping_sub(127i8) == 2i8);
/// assert!((-127i8).wrapping_sub(126i8) == 3i8);
/// assert!((-128i8).wrapping_sub(1i8) == 127i8);
/// assert!(114i8.wrapping_sub((-82i8)) == (-60i8));
/// assert!((-96i8).wrapping_sub(93i8) == 67i8);
/// assert!((-23i8).wrapping_sub(121i8) == 112i8);
/// assert!((-112i8).wrapping_sub(100i8) == 44i8);
/// # }
/// ```
pub fn core_ops_sub_i8_wrapping_sub() {}
/// # Properties for [`i8::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i8::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(126i8.overflowing_sub(127i8) == ((-1i8), false));
/// assert!(127i8.overflowing_sub(127i8) == (0i8, false));
/// assert!(0i8.overflowing_sub((-1i8)) == (1i8, false));
/// assert!((-127i8).overflowing_sub((-1i8)) == ((-126i8), false));
/// assert!((-1i8).overflowing_sub(126i8) == ((-127i8), false));
/// assert!((-127i8).overflowing_sub((-127i8)) == (0i8, false));
/// assert!((-9i8).overflowing_sub(90i8) == ((-99i8), false));
/// assert!((-96i8).overflowing_sub(21i8) == ((-117i8), false));
/// assert!((-18i8).overflowing_sub((-88i8)) == (70i8, false));
/// assert!(40i8.overflowing_sub((-57i8)) == (97i8, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i8::down(
///                 (-i8::MIN.up() + x.up() - y.up()).rem_euclid(&(-i8::MIN.up() * 2))
///                     + i8::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).overflowing_sub(1i8) == (127i8, true));
/// assert!(0i8.overflowing_sub((-128i8)) == ((-128i8), true));
/// assert!(127i8.overflowing_sub((-128i8)) == ((-1i8), true));
/// assert!(127i8.overflowing_sub((-127i8)) == ((-2i8), true));
/// assert!((-128i8).overflowing_sub(127i8) == (1i8, true));
/// assert!((-128i8).overflowing_sub(126i8) == (2i8, true));
/// assert!(44i8.overflowing_sub((-118i8)) == ((-94i8), true));
/// assert!((-78i8).overflowing_sub(121i8) == (57i8, true));
/// assert!((-125i8).overflowing_sub(97i8) == (34i8, true));
/// assert!((-88i8).overflowing_sub(45i8) == (123i8, true));
/// # }
/// ```
pub fn core_ops_sub_i8_overflowing_sub() {}
/// # Properties for [`i8::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() <= i8::MAX.up() && x.up() + y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x + y == i8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i8) + 0i8 == (-1i8));
/// assert!(126i8 + 0i8 == 126i8);
/// assert!((-1i8) + (-127i8) == (-128i8));
/// assert!(127i8 + 0i8 == 127i8);
/// assert!((-127i8) + 1i8 == (-126i8));
/// assert!(1i8 + (-127i8) == (-126i8));
/// assert!((-97i8) + 24i8 == (-73i8));
/// assert!((-99i8) + 50i8 == (-49i8));
/// assert!(45i8 + (-87i8) == (-42i8));
/// assert!((-36i8) + 43i8 == 7i8);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() > i8::MAX.up() || x.up() + y.up() < i8::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 128i8) + (- 128i8)));
/// assert!(panics!((- 128i8) + (- 127i8)));
/// assert!(panics!(127i8 + 1i8));
/// assert!(panics!(127i8 + 126i8));
/// assert!(panics!(126i8 + 127i8));
/// assert!(panics!((- 127i8) + (- 128i8)));
/// assert!(panics!(106i8 + 37i8));
/// assert!(panics!(60i8 + 125i8));
/// assert!(panics!((- 124i8) + (- 89i8)));
/// assert!(panics!((- 66i8) + (- 112i8)));
/// # }
/// ```
pub fn core_ops_add_i8_add() {}
/// # Properties for [`i16::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i16::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i16).checked_neg() == Some(1i16));
/// assert!(0i16.checked_neg() == Some(0i16));
/// assert!(1i16.checked_neg() == Some((-1i16)));
/// assert!((-32767i16).checked_neg() == Some(32767i16));
/// assert!(32767i16.checked_neg() == Some((-32767i16)));
/// assert!(32766i16.checked_neg() == Some((-32766i16)));
/// assert!((-32508i16).checked_neg() == Some(32508i16));
/// assert!((-19941i16).checked_neg() == Some(19941i16));
/// assert!((-30536i16).checked_neg() == Some(30536i16));
/// assert!((-31510i16).checked_neg() == Some(31510i16));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).checked_neg() == None);
/// # }
/// ```
pub fn t_i16_checked_neg() {}
/// # Properties for [`i16::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i16::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         32766i16.neg() == (-32766i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         1i16.neg() == (-1i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         0i16.neg() == 0i16
///     });
/// assert!({
///         use std::ops::Neg;
///         32767i16.neg() == (-32767i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-32767i16).neg() == 32767i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i16).neg() == 1i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-14715i16).neg() == 14715i16
///     });
/// assert!({
///         use std::ops::Neg;
///         29394i16.neg() == (-29394i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-12066i16).neg() == 12066i16
///     });
/// assert!({
///         use std::ops::Neg;
///         8136i16.neg() == (-8136i16)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 32768i16).neg())
///     });
/// # }
/// ```
pub fn t_i16_neg() {}
/// # Properties for [`i16::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i16::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i16.overflowing_neg() == ((-1i16), false));
/// assert!(0i16.overflowing_neg() == (0i16, false));
/// assert!(32766i16.overflowing_neg() == ((-32766i16), false));
/// assert!(32767i16.overflowing_neg() == ((-32767i16), false));
/// assert!((-1i16).overflowing_neg() == (1i16, false));
/// assert!((-32767i16).overflowing_neg() == (32767i16, false));
/// assert!((-6080i16).overflowing_neg() == (6080i16, false));
/// assert!(24341i16.overflowing_neg() == ((-24341i16), false));
/// assert!(28529i16.overflowing_neg() == ((-28529i16), false));
/// assert!((-20100i16).overflowing_neg() == (20100i16, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i16::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).overflowing_neg() == ((-32768i16), true));
/// # }
/// ```
pub fn t_i16_overflowing_neg() {}
/// # Properties for [`i16::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x - y == i16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32766i16 - 32767i16 == (-1i16));
/// assert!((-32768i16) - (-32768i16) == 0i16);
/// assert!(1i16 - 32767i16 == (-32766i16));
/// assert!(32766i16 - (-1i16) == 32767i16);
/// assert!((-32767i16) - (-32767i16) == 0i16);
/// assert!((-32768i16) - 0i16 == (-32768i16));
/// assert!(6353i16 - (-25544i16) == 31897i16);
/// assert!(14151i16 - 21980i16 == (-7829i16));
/// assert!(21736i16 - 22294i16 == (-558i16));
/// assert!((-22943i16) - (-1578i16) == (-21365i16));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 32768i16) - 1i16));
/// assert!(panics!(1i16 - (- 32768i16)));
/// assert!(panics!(32767i16 - (- 32767i16)));
/// assert!(panics!(1i16 - (- 32767i16)));
/// assert!(panics!(0i16 - (- 32768i16)));
/// assert!(panics!((- 32768i16) - 32767i16));
/// assert!(panics!((- 27751i16) - 6044i16));
/// assert!(panics!((- 17812i16) - 21679i16));
/// assert!(panics!(28893i16 - (- 7038i16)));
/// assert!(panics!(30499i16 - (- 6068i16)));
/// # }
/// ```
pub fn core_ops_sub_i16_sub() {}
/// # Properties for [`i16::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i16::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i16).checked_sub(0i16) == Some((-1i16)));
/// assert!(1i16.checked_sub((-1i16)) == Some(2i16));
/// assert!(0i16.checked_sub(32767i16) == Some((-32767i16)));
/// assert!((-32767i16).checked_sub(1i16) == Some((-32768i16)));
/// assert!(0i16.checked_sub(0i16) == Some(0i16));
/// assert!(32766i16.checked_sub(1i16) == Some(32765i16));
/// assert!(7020i16.checked_sub(26111i16) == Some((-19091i16)));
/// assert!(542i16.checked_sub(2471i16) == Some((-1929i16)));
/// assert!(13445i16.checked_sub(6802i16) == Some(6643i16));
/// assert!((-11815i16).checked_sub((-17370i16)) == Some(5555i16));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).checked_sub(32767i16) == None);
/// assert!(32766i16.checked_sub((-32767i16)) == None);
/// assert!((-32768i16).checked_sub(32766i16) == None);
/// assert!(32767i16.checked_sub((-32767i16)) == None);
/// assert!(32767i16.checked_sub((-32768i16)) == None);
/// assert!((-32767i16).checked_sub(32766i16) == None);
/// assert!((-13383i16).checked_sub(24557i16) == None);
/// assert!((-863i16).checked_sub(32396i16) == None);
/// assert!(11969i16.checked_sub((-28183i16)) == None);
/// assert!(15294i16.checked_sub((-31826i16)) == None);
/// # }
/// ```
pub fn core_ops_sub_i16_checked_sub() {}
/// # Properties for [`i16::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16.wrapping_sub(1i16) == 32766i16);
/// assert!(32766i16.wrapping_sub(1i16) == 32765i16);
/// assert!(0i16.wrapping_sub((-32767i16)) == 32767i16);
/// assert!(1i16.wrapping_sub((-1i16)) == 2i16);
/// assert!(1i16.wrapping_sub(32766i16) == (-32765i16));
/// assert!(0i16.wrapping_sub(32767i16) == (-32767i16));
/// assert!(31533i16.wrapping_sub(12445i16) == 19088i16);
/// assert!(6498i16.wrapping_sub((-26205i16)) == 32703i16);
/// assert!(12274i16.wrapping_sub(15846i16) == (-3572i16));
/// assert!(21037i16.wrapping_sub(6129i16) == 14908i16);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i16::down(
///             (-i16::MIN.up() + x.up() - y.up()).rem_euclid(&(-i16::MIN.up() * 2))
///                 + i16::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32766i16.wrapping_sub((-32768i16)) == (-2i16));
/// assert!((-32768i16).wrapping_sub(32767i16) == 1i16);
/// assert!(0i16.wrapping_sub((-32768i16)) == (-32768i16));
/// assert!((-32768i16).wrapping_sub(32766i16) == 2i16);
/// assert!(1i16.wrapping_sub((-32767i16)) == (-32768i16));
/// assert!(32767i16.wrapping_sub((-1i16)) == (-32768i16));
/// assert!(23081i16.wrapping_sub((-9972i16)) == (-32483i16));
/// assert!(17872i16.wrapping_sub((-23410i16)) == (-24254i16));
/// assert!(7830i16.wrapping_sub((-25561i16)) == (-32145i16));
/// assert!(31494i16.wrapping_sub((-15544i16)) == (-18498i16));
/// # }
/// ```
pub fn core_ops_sub_i16_wrapping_sub() {}
/// # Properties for [`i16::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i16::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16.overflowing_sub(32766i16) == (1i16, false));
/// assert!((-1i16).overflowing_sub(32766i16) == ((-32767i16), false));
/// assert!(0i16.overflowing_sub(32766i16) == ((-32766i16), false));
/// assert!(0i16.overflowing_sub((-1i16)) == (1i16, false));
/// assert!((-1i16).overflowing_sub(0i16) == ((-1i16), false));
/// assert!(1i16.overflowing_sub(1i16) == (0i16, false));
/// assert!((-10750i16).overflowing_sub((-24077i16)) == (13327i16, false));
/// assert!((-5388i16).overflowing_sub((-26255i16)) == (20867i16, false));
/// assert!((-10751i16).overflowing_sub(20074i16) == ((-30825i16), false));
/// assert!(26288i16.overflowing_sub(7434i16) == (18854i16, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i16::down(
///                 (-i16::MIN.up() + x.up() - y.up()).rem_euclid(&(-i16::MIN.up() * 2))
///                     + i16::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16.overflowing_sub((-1i16)) == ((-32768i16), true));
/// assert!((-32768i16).overflowing_sub(1i16) == (32767i16, true));
/// assert!(32767i16.overflowing_sub((-32767i16)) == ((-2i16), true));
/// assert!(32766i16.overflowing_sub((-32767i16)) == ((-3i16), true));
/// assert!((-32768i16).overflowing_sub(32767i16) == (1i16, true));
/// assert!(0i16.overflowing_sub((-32768i16)) == ((-32768i16), true));
/// assert!(29807i16.overflowing_sub((-29164i16)) == ((-6565i16), true));
/// assert!((-27284i16).overflowing_sub(23045i16) == (15207i16, true));
/// assert!(18473i16.overflowing_sub((-26460i16)) == ((-20603i16), true));
/// assert!(22930i16.overflowing_sub((-30759i16)) == ((-11847i16), true));
/// # }
/// ```
pub fn core_ops_sub_i16_overflowing_sub() {}
/// # Properties for [`i16::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() + y.up() <= i16::MAX.up() && x.up() + y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x + y == i16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16 + 0i16 == 32767i16);
/// assert!((-1i16) + 32766i16 == 32765i16);
/// assert!(32766i16 + 0i16 == 32766i16);
/// assert!(1i16 + (-1i16) == 0i16);
/// assert!((-1i16) + (-1i16) == (-2i16));
/// assert!((-32768i16) + 1i16 == (-32767i16));
/// assert!((-12390i16) + 30545i16 == 18155i16);
/// assert!(7335i16 + (-28850i16) == (-21515i16));
/// assert!(20766i16 + (-20068i16) == 698i16);
/// assert!(6687i16 + 8286i16 == 14973i16);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() + y.up() > i16::MAX.up() || x.up() + y.up() < i16::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(32766i16 + 32767i16));
/// assert!(panics!((- 32767i16) + (- 32768i16)));
/// assert!(panics!((- 32768i16) + (- 32767i16)));
/// assert!(panics!(32767i16 + 32766i16));
/// assert!(panics!((- 1i16) + (- 32768i16)));
/// assert!(panics!(1i16 + 32767i16));
/// assert!(panics!((- 21253i16) + (- 17012i16)));
/// assert!(panics!((- 24477i16) + (- 10458i16)));
/// assert!(panics!((- 24096i16) + (- 16416i16)));
/// assert!(panics!(5566i16 + 32759i16));
/// # }
/// ```
pub fn core_ops_add_i16_add() {}
/// # Properties for [`i32::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i32::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483647i32).checked_neg() == Some(2147483647i32));
/// assert!((-1i32).checked_neg() == Some(1i32));
/// assert!(0i32.checked_neg() == Some(0i32));
/// assert!(2147483647i32.checked_neg() == Some((-2147483647i32)));
/// assert!(1i32.checked_neg() == Some((-1i32)));
/// assert!(2147483646i32.checked_neg() == Some((-2147483646i32)));
/// assert!((-1795463626i32).checked_neg() == Some(1795463626i32));
/// assert!((-1185355013i32).checked_neg() == Some(1185355013i32));
/// assert!((-1484924621i32).checked_neg() == Some(1484924621i32));
/// assert!(1720282585i32.checked_neg() == Some((-1720282585i32)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).checked_neg() == None);
/// # }
/// ```
pub fn t_i32_checked_neg() {}
/// # Properties for [`i32::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i32::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         1i32.neg() == (-1i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         0i32.neg() == 0i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-2147483647i32).neg() == 2147483647i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i32).neg() == 1i32
///     });
/// assert!({
///         use std::ops::Neg;
///         2147483647i32.neg() == (-2147483647i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         2147483646i32.neg() == (-2147483646i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1440204712i32).neg() == 1440204712i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-698913977i32).neg() == 698913977i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1902990954i32).neg() == 1902990954i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1056435958i32).neg() == 1056435958i32
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 2147483648i32).neg())
///     });
/// # }
/// ```
pub fn t_i32_neg() {}
/// # Properties for [`i32::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i32::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i32.overflowing_neg() == ((-1i32), false));
/// assert!((-2147483647i32).overflowing_neg() == (2147483647i32, false));
/// assert!(2147483646i32.overflowing_neg() == ((-2147483646i32), false));
/// assert!((-1i32).overflowing_neg() == (1i32, false));
/// assert!(0i32.overflowing_neg() == (0i32, false));
/// assert!(2147483647i32.overflowing_neg() == ((-2147483647i32), false));
/// assert!((-1266129005i32).overflowing_neg() == (1266129005i32, false));
/// assert!(1517870243i32.overflowing_neg() == ((-1517870243i32), false));
/// assert!(2080091638i32.overflowing_neg() == ((-2080091638i32), false));
/// assert!(134309401i32.overflowing_neg() == ((-134309401i32), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i32::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).overflowing_neg() == ((-2147483648i32), true));
/// # }
/// ```
pub fn t_i32_overflowing_neg() {}
/// # Properties for [`i32::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x - y == i32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i32 - (-2147483647i32) == 2147483647i32);
/// assert!((-1i32) - (-1i32) == 0i32);
/// assert!(1i32 - 2147483646i32 == (-2147483645i32));
/// assert!(0i32 - 0i32 == 0i32);
/// assert!(2147483646i32 - 0i32 == 2147483646i32);
/// assert!(2147483647i32 - 1i32 == 2147483646i32);
/// assert!(1177221903i32 - (-903080173i32) == 2080302076i32);
/// assert!((-774722573i32) - (-2076101893i32) == 1301379320i32);
/// assert!(1269423680i32 - (-148640962i32) == 1418064642i32);
/// assert!((-504492576i32) - (-1759325033i32) == 1254832457i32);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 2147483648i32) - 2147483646i32));
/// assert!(panics!((- 2147483648i32) - 1i32));
/// assert!(panics!((- 2147483648i32) - 2147483647i32));
/// assert!(panics!(2147483647i32 - (- 1i32)));
/// assert!(panics!((- 2147483647i32) - 2147483647i32));
/// assert!(panics!((- 2147483647i32) - 2147483646i32));
/// assert!(panics!((- 2124664099i32) - 1187951848i32));
/// assert!(panics!((- 1873879137i32) - 439530743i32));
/// assert!(panics!((- 617840750i32) - 1872895150i32));
/// assert!(panics!(1627064307i32 - (- 700871574i32)));
/// # }
/// ```
pub fn core_ops_sub_i32_sub() {}
/// # Properties for [`i32::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i32::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i32.checked_sub(0i32) == Some(0i32));
/// assert!(1i32.checked_sub((-1i32)) == Some(2i32));
/// assert!(2147483647i32.checked_sub(2147483646i32) == Some(1i32));
/// assert!(0i32.checked_sub(2147483647i32) == Some((-2147483647i32)));
/// assert!((-2147483647i32).checked_sub((-2147483648i32)) == Some(1i32));
/// assert!(2147483646i32.checked_sub((-1i32)) == Some(2147483647i32));
/// assert!((-166734175i32).checked_sub((-1227117812i32)) == Some(1060383637i32));
/// assert!((-485809832i32).checked_sub(1538233321i32) == Some((-2024043153i32)));
/// assert!((-1661523238i32).checked_sub((-968115363i32)) == Some((-693407875i32)));
/// assert!((-556698679i32).checked_sub((-2127887998i32)) == Some(1571189319i32));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(2147483647i32.checked_sub((-2147483647i32)) == None);
/// assert!(1i32.checked_sub((-2147483647i32)) == None);
/// assert!(2147483647i32.checked_sub((-1i32)) == None);
/// assert!(2147483647i32.checked_sub((-2147483648i32)) == None);
/// assert!(1i32.checked_sub((-2147483648i32)) == None);
/// assert!(2147483646i32.checked_sub((-2147483648i32)) == None);
/// assert!(1272769267i32.checked_sub((-1149150474i32)) == None);
/// assert!(804748131i32.checked_sub((-1678089651i32)) == None);
/// assert!(2096512515i32.checked_sub((-1576885655i32)) == None);
/// assert!(1934730067i32.checked_sub((-1298709613i32)) == None);
/// # }
/// ```
pub fn core_ops_sub_i32_checked_sub() {}
/// # Properties for [`i32::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i32.wrapping_sub(1i32) == 0i32);
/// assert!((-2147483647i32).wrapping_sub((-1i32)) == (-2147483646i32));
/// assert!((-2147483648i32).wrapping_sub((-2147483647i32)) == (-1i32));
/// assert!((-2147483648i32).wrapping_sub((-2147483648i32)) == 0i32);
/// assert!(2147483646i32.wrapping_sub(1i32) == 2147483645i32);
/// assert!((-1i32).wrapping_sub(0i32) == (-1i32));
/// assert!((-1024993163i32).wrapping_sub(921853184i32) == (-1946846347i32));
/// assert!(1113004945i32.wrapping_sub(1512942403i32) == (-399937458i32));
/// assert!(640330504i32.wrapping_sub((-199949423i32)) == 840279927i32);
/// assert!(1600389184i32.wrapping_sub(2138280919i32) == (-537891735i32));
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i32::down(
///             (-i32::MIN.up() + x.up() - y.up()).rem_euclid(&(-i32::MIN.up() * 2))
///                 + i32::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483647i32).wrapping_sub(2147483647i32) == 2i32);
/// assert!((-2147483648i32).wrapping_sub(2147483646i32) == 2i32);
/// assert!(2147483647i32.wrapping_sub((-1i32)) == (-2147483648i32));
/// assert!(2147483646i32.wrapping_sub((-2147483647i32)) == (-3i32));
/// assert!(1i32.wrapping_sub((-2147483647i32)) == (-2147483648i32));
/// assert!(1i32.wrapping_sub((-2147483648i32)) == (-2147483647i32));
/// assert!(1986463188i32.wrapping_sub((-1387908332i32)) == (-920595776i32));
/// assert!(1193911810i32.wrapping_sub((-1319984485i32)) == (-1781071001i32));
/// assert!(1761612426i32.wrapping_sub((-1645403140i32)) == (-887951730i32));
/// assert!((-1962108127i32).wrapping_sub(1385316497i32) == 947542672i32);
/// # }
/// ```
pub fn core_ops_sub_i32_wrapping_sub() {}
/// # Properties for [`i32::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i32::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i32.overflowing_sub(1i32) == (0i32, false));
/// assert!(2147483646i32.overflowing_sub(2147483647i32) == ((-1i32), false));
/// assert!(0i32.overflowing_sub(0i32) == (0i32, false));
/// assert!(0i32.overflowing_sub((-1i32)) == (1i32, false));
/// assert!((-2147483647i32).overflowing_sub((-2147483648i32)) == (1i32, false));
/// assert!(2147483647i32.overflowing_sub(1i32) == (2147483646i32, false));
/// assert!((-1627204470i32).overflowing_sub((-798162442i32)) == ((-829042028i32), false));
/// assert!((-449205863i32).overflowing_sub((-1547377240i32)) == (1098171377i32, false));
/// assert!(1333903928i32.overflowing_sub((-101649803i32)) == (1435553731i32, false));
/// assert!((-2007208927i32).overflowing_sub((-897137646i32)) == ((-1110071281i32), false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i32::down(
///                 (-i32::MIN.up() + x.up() - y.up()).rem_euclid(&(-i32::MIN.up() * 2))
///                     + i32::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).overflowing_sub(1i32) == (2147483647i32, true));
/// assert!(2147483647i32.overflowing_sub((-2147483648i32)) == ((-1i32), true));
/// assert!(2147483647i32.overflowing_sub((-1i32)) == ((-2147483648i32), true));
/// assert!(2147483646i32.overflowing_sub((-2147483648i32)) == ((-2i32), true));
/// assert!(1i32.overflowing_sub((-2147483647i32)) == ((-2147483648i32), true));
/// assert!((-2147483648i32).overflowing_sub(2147483647i32) == (1i32, true));
/// assert!((-1488669758i32).overflowing_sub(1920196632i32) == (886100906i32, true));
/// assert!(430462280i32.overflowing_sub((-2107972526i32)) == ((-1756532490i32), true));
/// assert!((-895303597i32).overflowing_sub(1298624106i32) == (2101039593i32, true));
/// assert!(1480529531i32.overflowing_sub((-1615198795i32)) == ((-1199238970i32), true));
/// # }
/// ```
pub fn core_ops_sub_i32_overflowing_sub() {}
/// # Properties for [`i32::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() + y.up() <= i32::MAX.up() && x.up() + y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x + y == i32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i32 + 1i32 == 2i32);
/// assert!(2147483647i32 + 0i32 == 2147483647i32);
/// assert!(0i32 + (-2147483648i32) == (-2147483648i32));
/// assert!((-2147483648i32) + 1i32 == (-2147483647i32));
/// assert!(2147483646i32 + (-1i32) == 2147483645i32);
/// assert!(1i32 + (-2147483648i32) == (-2147483647i32));
/// assert!(2114188615i32 + (-1888174258i32) == 226014357i32);
/// assert!((-29069752i32) + 121449318i32 == 92379566i32);
/// assert!((-123734447i32) + 394638995i32 == 270904548i32);
/// assert!(1174393701i32 + (-1836378133i32) == (-661984432i32));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() + y.up() > i32::MAX.up() || x.up() + y.up() < i32::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(2147483646i32 + 2147483646i32));
/// assert!(panics!((- 2147483648i32) + (- 1i32)));
/// assert!(panics!((- 2147483648i32) + (- 2147483648i32)));
/// assert!(panics!((- 2147483647i32) + (- 2147483647i32)));
/// assert!(panics!(2147483646i32 + 2147483647i32));
/// assert!(panics!(2147483647i32 + 2147483647i32));
/// assert!(panics!((- 2116685535i32) + (- 1347538543i32)));
/// assert!(panics!(501433528i32 + 1952816231i32));
/// assert!(panics!(601380605i32 + 1587904720i32));
/// assert!(panics!(1123586561i32 + 2144250837i32));
/// # }
/// ```
pub fn core_ops_add_i32_add() {}
/// # Properties for [`i64::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i64::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i64).checked_neg() == Some(1i64));
/// assert!(1i64.checked_neg() == Some((-1i64)));
/// assert!(0i64.checked_neg() == Some(0i64));
/// assert!((-9223372036854775807i64).checked_neg() == Some(9223372036854775807i64));
/// assert!(9223372036854775806i64.checked_neg() == Some((-9223372036854775806i64)));
/// assert!(9223372036854775807i64.checked_neg() == Some((-9223372036854775807i64)));
/// assert!((-7405427920738378430i64).checked_neg() == Some(7405427920738378430i64));
/// assert!(7129021150738091990i64.checked_neg() == Some((-7129021150738091990i64)));
/// assert!((-7238252538344930188i64).checked_neg() == Some(7238252538344930188i64));
/// assert!(3736318079526403587i64.checked_neg() == Some((-3736318079526403587i64)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).checked_neg() == None);
/// # }
/// ```
pub fn t_i64_checked_neg() {}
/// # Properties for [`i64::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i64::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         9223372036854775807i64.neg() == (-9223372036854775807i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         1i64.neg() == (-1i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         0i64.neg() == 0i64
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i64).neg() == 1i64
///     });
/// assert!({
///         use std::ops::Neg;
///         (-9223372036854775807i64).neg() == 9223372036854775807i64
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775806i64.neg() == (-9223372036854775806i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         3765235454133252927i64.neg() == (-3765235454133252927i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-6976475740811540768i64).neg() == 6976475740811540768i64
///     });
/// assert!({
///         use std::ops::Neg;
///         3358593327444037389i64.neg() == (-3358593327444037389i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         5554048477617908813i64.neg() == (-5554048477617908813i64)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 9223372036854775808i64).neg())
///     });
/// # }
/// ```
pub fn t_i64_neg() {}
/// # Properties for [`i64::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i64::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i64.overflowing_neg() == (0i64, false));
/// assert!(1i64.overflowing_neg() == ((-1i64), false));
/// assert!((-1i64).overflowing_neg() == (1i64, false));
/// assert!(9223372036854775806i64.overflowing_neg() == ((-9223372036854775806i64), false));
/// assert!(9223372036854775807i64.overflowing_neg() == ((-9223372036854775807i64), false));
/// assert!((-9223372036854775807i64).overflowing_neg() == (9223372036854775807i64, false));
/// assert!((-4225183798975475184i64).overflowing_neg() == (4225183798975475184i64, false));
/// assert!((-5400832628586981636i64).overflowing_neg() == (5400832628586981636i64, false));
/// assert!(5373388741154405930i64.overflowing_neg() == ((-5373388741154405930i64), false));
/// assert!((-2656423341156941226i64).overflowing_neg() == (2656423341156941226i64, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i64::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).overflowing_neg() == ((-9223372036854775808i64), true));
/// # }
/// ```
pub fn t_i64_overflowing_neg() {}
/// # Properties for [`i64::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x - y == i64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807i64 - 0i64 == 9223372036854775807i64);
/// assert!(1i64 - (-1i64) == 2i64);
/// assert!((-9223372036854775807i64) - (-1i64) == (-9223372036854775806i64));
/// assert!((-9223372036854775808i64) - (-9223372036854775808i64) == 0i64);
/// assert!((-1i64) - 9223372036854775807i64 == (-9223372036854775808i64));
/// assert!((-9223372036854775807i64) - (-9223372036854775807i64) == 0i64);
/// assert!(6497445510596790766i64 - 3028389739427163897i64 == 3469055771169626869i64);
/// assert!((-5206903160447305102i64) - (-181842257714328037i64) == (-5025060902732977065i64));
/// assert!(2663283334010637776i64 - 3020478674976732662i64 == (-357195340966094886i64));
/// assert!(3194578916076372799i64 - 9043748579302435514i64 == (-5849169663226062715i64));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808i64) - 1i64));
/// assert!(panics!((- 9223372036854775808i64) - 9223372036854775806i64));
/// assert!(panics!(9223372036854775807i64 - (- 9223372036854775808i64)));
/// assert!(panics!(9223372036854775806i64 - (- 9223372036854775808i64)));
/// assert!(panics!(9223372036854775807i64 - (- 9223372036854775807i64)));
/// assert!(panics!((- 9223372036854775807i64) - 9223372036854775807i64));
/// assert!(panics!((- 4551329828116398971i64) - 8620939362391595647i64));
/// assert!(panics!(6502526112719984611i64 - (- 2749517875895826519i64)));
/// assert!(panics!(7903848059316562163i64 - (- 8727728111493700818i64)));
/// assert!(panics!((- 8499003129859918606i64) - 5434061254781188687i64));
/// # }
/// ```
pub fn core_ops_sub_i64_sub() {}
/// # Properties for [`i64::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i64::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i64.checked_sub(1i64) == Some(0i64));
/// assert!((-9223372036854775808i64).checked_sub((-1i64)) == Some((-9223372036854775807i64)));
/// assert!(0i64.checked_sub((-9223372036854775807i64)) == Some(9223372036854775807i64));
/// assert!(9223372036854775806i64.checked_sub(9223372036854775806i64) == Some(0i64));
/// assert!(0i64.checked_sub((-1i64)) == Some(1i64));
/// assert!(9223372036854775806i64.checked_sub(0i64) == Some(9223372036854775806i64));
/// assert!((-675474367875466285i64).checked_sub((-8213321647790490525i64))
///         == Some(7537847279915024240i64));
/// assert!(3131397178010228959i64.checked_sub(3023851924244293686i64)
///         == Some(107545253765935273i64));
/// assert!((-5923300338445224751i64).checked_sub((-5504770060523554130i64))
///         == Some((-418530277921670621i64)));
/// assert!((-5328545996397710522i64).checked_sub((-8259734336471976319i64))
///         == Some(2931188340074265797i64));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!((-9223372036854775808i64).checked_sub(1i64) == None);
/// assert!((-9223372036854775808i64).checked_sub(9223372036854775807i64) == None);
/// assert!((-9223372036854775807i64).checked_sub(9223372036854775807i64) == None);
/// assert!(0i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!(9223372036854775806i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!(7968854053765309174i64.checked_sub((-6931210316526949623i64)) == None);
/// assert!((-8301539454848813634i64).checked_sub(4370908518200691607i64) == None);
/// assert!(9111247840847328839i64.checked_sub((-3979928739085476097i64)) == None);
/// assert!(4178176604323745817i64.checked_sub((-6122326416194897131i64)) == None);
/// # }
/// ```
pub fn core_ops_sub_i64_checked_sub() {}
/// # Properties for [`i64::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i64).wrapping_sub((-9223372036854775808i64)) == 9223372036854775807i64);
/// assert!(9223372036854775807i64.wrapping_sub(0i64) == 9223372036854775807i64);
/// assert!(1i64.wrapping_sub(9223372036854775807i64) == (-9223372036854775806i64));
/// assert!((-9223372036854775807i64).wrapping_sub((-9223372036854775807i64)) == 0i64);
/// assert!(9223372036854775806i64.wrapping_sub((-1i64)) == 9223372036854775807i64);
/// assert!((-1i64).wrapping_sub(9223372036854775807i64) == (-9223372036854775808i64));
/// assert!(4650256851600532737i64.wrapping_sub((-1548690037373197277i64))
///         == 6198946888973730014i64);
/// assert!((-5658730372590221865i64).wrapping_sub((-3107812320211324630i64))
///         == (-2550918052378897235i64));
/// assert!((-6144459618722878189i64).wrapping_sub((-7186011627347797588i64))
///         == 1041552008624919399i64);
/// assert!(1431166605943135254i64.wrapping_sub(3523538221879879649i64)
///         == (-2092371615936744395i64));
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i64::down(
///             (-i64::MIN.up() + x.up() - y.up()).rem_euclid(&(-i64::MIN.up() * 2))
///                 + i64::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).wrapping_sub(1i64) == 9223372036854775807i64);
/// assert!((-9223372036854775807i64).wrapping_sub(9223372036854775806i64) == 3i64);
/// assert!((-9223372036854775807i64).wrapping_sub(9223372036854775807i64) == 2i64);
/// assert!(9223372036854775807i64.wrapping_sub((-1i64)) == (-9223372036854775808i64));
/// assert!(9223372036854775807i64.wrapping_sub((-9223372036854775807i64)) == (-2i64));
/// assert!(9223372036854775806i64.wrapping_sub((-9223372036854775808i64)) == (-2i64));
/// assert!(7759304140136766403i64.wrapping_sub((-7930152515246132592i64))
///         == (-2757287418326652621i64));
/// assert!(8552738389447771133i64.wrapping_sub((-2193554659142067736i64))
///         == (-7700451025119712747i64));
/// assert!(8761243009880037112i64.wrapping_sub((-4526372616397489629i64))
///         == (-5159128447432024875i64));
/// assert!((-4634166286968915093i64).wrapping_sub(7592200324226771329i64)
///         == 6220377462513865194i64);
/// # }
/// ```
pub fn core_ops_sub_i64_wrapping_sub() {}
/// # Properties for [`i64::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i64::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775806i64.overflowing_sub(9223372036854775807i64) == ((-1i64), false));
/// assert!((-9223372036854775808i64).overflowing_sub(0i64) == ((-9223372036854775808i64), false));
/// assert!((-1i64).overflowing_sub(9223372036854775806i64) == ((-9223372036854775807i64), false));
/// assert!(0i64.overflowing_sub(9223372036854775806i64) == ((-9223372036854775806i64), false));
/// assert!((-9223372036854775807i64).overflowing_sub((-9223372036854775808i64)) == (1i64, false));
/// assert!((-9223372036854775808i64).overflowing_sub((-9223372036854775808i64)) == (0i64, false));
/// assert!(4885767896873992467i64.overflowing_sub((-2467685871866010984i64))
///         == (7353453768740003451i64, false));
/// assert!((-7623464496184309353i64).overflowing_sub((-7233330922901274582i64))
///         == ((-390133573283034771i64), false));
/// assert!(6145624956655326524i64.overflowing_sub(4945474956446646384i64)
///         == (1200150000208680140i64, false));
/// assert!(3884913078009884102i64.overflowing_sub(781597362745179102i64)
///         == (3103315715264705000i64, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i64::down(
///                 (-i64::MIN.up() + x.up() - y.up()).rem_euclid(&(-i64::MIN.up() * 2))
///                     + i64::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i64.overflowing_sub((-9223372036854775808i64)) == ((-9223372036854775807i64), true));
/// assert!(9223372036854775806i64.overflowing_sub((-9223372036854775808i64)) == ((-2i64), true));
/// assert!(1i64.overflowing_sub((-9223372036854775807i64)) == ((-9223372036854775808i64), true));
/// assert!(9223372036854775807i64.overflowing_sub((-9223372036854775807i64)) == ((-2i64), true));
/// assert!(0i64.overflowing_sub((-9223372036854775808i64)) == ((-9223372036854775808i64), true));
/// assert!((-9223372036854775807i64).overflowing_sub(9223372036854775806i64) == (3i64, true));
/// assert!(7496220779045867418i64.overflowing_sub((-5671854986977583365i64))
///         == ((-5278668307686100833i64), true));
/// assert!(8744409281176406064i64.overflowing_sub((-5301131656463161579i64))
///         == ((-4401203136069983973i64), true));
/// assert!((-6441517664081339448i64).overflowing_sub(4912532149531886868i64)
///         == (7092694260096325300i64, true));
/// assert!((-3216259431208045700i64).overflowing_sub(7231314152854285177i64)
///         == (7999170489647220739i64, true));
/// # }
/// ```
pub fn core_ops_sub_i64_overflowing_sub() {}
/// # Properties for [`i64::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() + y.up() <= i64::MAX.up() && x.up() + y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x + y == i64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807i64 + (-1i64) == 9223372036854775806i64);
/// assert!((-9223372036854775808i64) + 1i64 == (-9223372036854775807i64));
/// assert!(9223372036854775806i64 + (-9223372036854775807i64) == (-1i64));
/// assert!(0i64 + (-1i64) == (-1i64));
/// assert!(9223372036854775806i64 + (-9223372036854775808i64) == (-2i64));
/// assert!((-1i64) + 9223372036854775806i64 == 9223372036854775805i64);
/// assert!((-2169465385908739037i64) + 1466499893309952588i64 == (-702965492598786449i64));
/// assert!(1349323053060435338i64 + 148042970573982202i64 == 1497366023634417540i64);
/// assert!((-3369183489518631304i64) + (-5400113366271974082i64) == (-8769296855790605386i64));
/// assert!((-8970617704180755622i64) + 744921810525298287i64 == (-8225695893655457335i64));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() + y.up() > i64::MAX.up() || x.up() + y.up() < i64::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1i64 + 9223372036854775807i64));
/// assert!(panics!(9223372036854775807i64 + 9223372036854775806i64));
/// assert!(panics!(9223372036854775806i64 + 9223372036854775807i64));
/// assert!(panics!(9223372036854775807i64 + 9223372036854775807i64));
/// assert!(panics!(9223372036854775807i64 + 1i64));
/// assert!(panics!((- 9223372036854775808i64) + (- 9223372036854775807i64)));
/// assert!(panics!(3885222861042997898i64 + 9080022754530284254i64));
/// assert!(panics!(1953509541539889973i64 + 9012741507986127593i64));
/// assert!(panics!((- 6936367409590020761i64) + (- 9033068373086772228i64)));
/// assert!(panics!(7339871148793499874i64 + 7783665829142513005i64));
/// # }
/// ```
pub fn core_ops_add_i64_add() {}
/// # Properties for [`i128::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i128::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i128).checked_neg() == Some(1i128));
/// assert!(1i128.checked_neg() == Some((-1i128)));
/// assert!(170141183460469231731687303715884105726i128.checked_neg()
///         == Some((-170141183460469231731687303715884105726i128)));
/// assert!(0i128.checked_neg() == Some(0i128));
/// assert!((-170141183460469231731687303715884105727i128).checked_neg()
///         == Some(170141183460469231731687303715884105727i128));
/// assert!(170141183460469231731687303715884105727i128.checked_neg()
///         == Some((-170141183460469231731687303715884105727i128)));
/// assert!((-95691907531638855238485391376099019901i128).checked_neg()
///         == Some(95691907531638855238485391376099019901i128));
/// assert!((-152154770813539598248062471685986355254i128).checked_neg()
///         == Some(152154770813539598248062471685986355254i128));
/// assert!((-157673484561825246698680963746651689048i128).checked_neg()
///         == Some(157673484561825246698680963746651689048i128));
/// assert!(46404704159008687264559839693591623727i128.checked_neg()
///         == Some((-46404704159008687264559839693591623727i128)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).checked_neg() == None);
/// # }
/// ```
pub fn t_i128_checked_neg() {}
/// # Properties for [`i128::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i128::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         170141183460469231731687303715884105726i128.neg()
///             == (-170141183460469231731687303715884105726i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         0i128.neg() == 0i128
///     });
/// assert!({
///         use std::ops::Neg;
///         (-170141183460469231731687303715884105727i128).neg()
///             == 170141183460469231731687303715884105727i128
///     });
/// assert!({
///         use std::ops::Neg;
///         170141183460469231731687303715884105727i128.neg()
///             == (-170141183460469231731687303715884105727i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         1i128.neg() == (-1i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i128).neg() == 1i128
///     });
/// assert!({
///         use std::ops::Neg;
///         (-102695007835329317817592525002250028570i128).neg()
///             == 102695007835329317817592525002250028570i128
///     });
/// assert!({
///         use std::ops::Neg;
///         163876079837725179893900976759417435120i128.neg()
///             == (-163876079837725179893900976759417435120i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-37116044455926438107338346641554761866i128).neg()
///             == 37116044455926438107338346641554761866i128
///     });
/// assert!({
///         use std::ops::Neg;
///         (-49178828948297796057435259022778146849i128).neg()
///             == 49178828948297796057435259022778146849i128
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 170141183460469231731687303715884105728i128).neg())
///     });
/// # }
/// ```
pub fn t_i128_neg() {}
/// # Properties for [`i128::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i128::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105727i128.overflowing_neg()
///         == ((-170141183460469231731687303715884105727i128), false));
/// assert!(170141183460469231731687303715884105726i128.overflowing_neg()
///         == ((-170141183460469231731687303715884105726i128), false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_neg()
///         == (170141183460469231731687303715884105727i128, false));
/// assert!(1i128.overflowing_neg() == ((-1i128), false));
/// assert!(0i128.overflowing_neg() == (0i128, false));
/// assert!((-1i128).overflowing_neg() == (1i128, false));
/// assert!((-103938548178536339650223508405074132511i128).overflowing_neg()
///         == (103938548178536339650223508405074132511i128, false));
/// assert!(87499038819484424366699419514517538100i128.overflowing_neg()
///         == ((-87499038819484424366699419514517538100i128), false));
/// assert!(154435988416512551977935013582630987646i128.overflowing_neg()
///         == ((-154435988416512551977935013582630987646i128), false));
/// assert!(152630260867229930608098369419641102959i128.overflowing_neg()
///         == ((-152630260867229930608098369419641102959i128), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i128::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).overflowing_neg()
///         == ((-170141183460469231731687303715884105728i128), true));
/// # }
/// ```
pub fn t_i128_overflowing_neg() {}
/// # Properties for [`i128::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x - y == i128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105727i128
///         - 170141183460469231731687303715884105726i128 == 1i128);
/// assert!(170141183460469231731687303715884105726i128 - 1i128
///         == 170141183460469231731687303715884105725i128);
/// assert!((-170141183460469231731687303715884105727i128) - 0i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!(1i128 - 170141183460469231731687303715884105726i128
///         == (-170141183460469231731687303715884105725i128));
/// assert!(1i128 - (-1i128) == 2i128);
/// assert!((-1i128) - 1i128 == (-2i128));
/// assert!((-21650826256048792566864100460291287012i128)
///         - 128356083602329650645731014433393769712i128
///         == (-150006909858378443212595114893685056724i128));
/// assert!(9031863670541152078126849445699999355i128
///         - 131345265424897630197103601983014214080i128
///         == (-122313401754356478118976752537314214725i128));
/// assert!((-28023893896328724345624169264545137888i128)
///         - 121847989662082453816844541365708401874i128
///         == (-149871883558411178162468710630253539762i128));
/// assert!((-52442831776145319212826734233957583335i128)
///         - 41023123031374166675845956493061388011i128
///         == (-93465954807519485888672690727018971346i128));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0i128 - (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(1i128 - (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(
///         170141183460469231731687303715884105727i128 - (-
///         170141183460469231731687303715884105728i128)
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) -
///         170141183460469231731687303715884105726i128
///     ));
/// assert!(panics!(1i128 - (- 170141183460469231731687303715884105727i128)));
/// assert!(panics!((- 170141183460469231731687303715884105728i128) - 1i128));
/// assert!(panics!(
///         121419163227816006910921547033517090687i128 - (-
///         62301322275150284762952213880257525469i128)
///     ));
/// assert!(panics!(
///         112735923694856920014410504854618763760i128 - (-
///         118867541982651348577194103672233706669i128)
///     ));
/// assert!(panics!(
///         104390086600348097327079800989636038924i128 - (-
///         145590531562005329847214327253563656914i128)
///     ));
/// assert!(panics!(
///         85762379495869220765662983063889221620i128 - (-
///         164860523884030970246269650461174134057i128)
///     ));
/// # }
/// ```
pub fn core_ops_sub_i128_sub() {}
/// # Properties for [`i128::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i128::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105727i128
///         .checked_sub(170141183460469231731687303715884105727i128) == Some(0i128));
/// assert!((-170141183460469231731687303715884105728i128).checked_sub((-1i128))
///         == Some((-170141183460469231731687303715884105727i128)));
/// assert!(1i128.checked_sub(1i128) == Some(0i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub((-170141183460469231731687303715884105728i128)) == Some(1i128));
/// assert!(0i128.checked_sub(0i128) == Some(0i128));
/// assert!(0i128.checked_sub(1i128) == Some((-1i128)));
/// assert!(40083892414598906707896239574506890062i128
///         .checked_sub((-114447393474423676172303345617910153462i128))
///         == Some(154531285889022582880199585192417043524i128));
/// assert!((-116165890477775178595793963563160634539i128)
///         .checked_sub(35385379499132380699096308150874351550i128)
///         == Some((-151551269976907559294890271714034986089i128)));
/// assert!((-168218515842595066286435299164474475340i128)
///         .checked_sub((-100758556916939454516480034690466291109i128))
///         == Some((-67459958925655611769955264474008184231i128)));
/// assert!((-160074162829864324610600903346401564618i128)
///         .checked_sub((-13664112388435130205382948511143900916i128))
///         == Some((-146410050441429194405217954835257663702i128)));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i128.checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!((-170141183460469231731687303715884105728i128).checked_sub(1i128) == None);
/// assert!(1i128.checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!(1i128.checked_sub((-170141183460469231731687303715884105727i128)) == None);
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub(170141183460469231731687303715884105727i128) == None);
/// assert!(170141183460469231731687303715884105726i128
///         .checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!(162303195280770273463417786457500213469i128
///         .checked_sub((-144685974033130641849832814924542770611i128)) == None);
/// assert!(83292051398199712710118212332722217418i128
///         .checked_sub((-143984272439330058949420176257235358538i128)) == None);
/// assert!((-31608659230758124729196379437770870955i128)
///         .checked_sub(143763419195495013300798496587328613634i128) == None);
/// assert!(128047356692809347951430474820080441791i128
///         .checked_sub((-144629683881386173652609868891837568624i128)) == None);
/// # }
/// ```
pub fn core_ops_sub_i128_checked_sub() {}
/// # Properties for [`i128::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i128).wrapping_sub((-170141183460469231731687303715884105728i128))
///         == 170141183460469231731687303715884105727i128);
/// assert!((-1i128).wrapping_sub((-170141183460469231731687303715884105727i128))
///         == 170141183460469231731687303715884105726i128);
/// assert!(170141183460469231731687303715884105726i128.wrapping_sub(0i128)
///         == 170141183460469231731687303715884105726i128);
/// assert!(170141183460469231731687303715884105726i128
///         .wrapping_sub(170141183460469231731687303715884105726i128) == 0i128);
/// assert!(1i128.wrapping_sub((-1i128)) == 2i128);
/// assert!(1i128.wrapping_sub(170141183460469231731687303715884105726i128)
///         == (-170141183460469231731687303715884105725i128));
/// assert!(77136355809977261699074636867802433040i128
///         .wrapping_sub(49371806888733199449413887565612051395i128)
///         == 27764548921244062249660749302190381645i128);
/// assert!((-54953360832168655964648344409768109544i128)
///         .wrapping_sub(107675275980606498045267042780456393011i128)
///         == (-162628636812775154009915387190224502555i128));
/// assert!(128228348890160810579199112700847296693i128
///         .wrapping_sub(54081701298020160317925865331831709467i128)
///         == 74146647592140650261273247369015587226i128);
/// assert!(143712645059049495049787259774421251605i128
///         .wrapping_sub(6829411682473649539148403735072533921i128)
///         == 136883233376575845510638856039348717684i128);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i128::down(
///             (-i128::MIN.up() + x.up() - y.up()).rem_euclid(&(-i128::MIN.up() * 2))
///                 + i128::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105726i128
///         .wrapping_sub((-170141183460469231731687303715884105727i128)) == (-3i128));
/// assert!(1i128.wrapping_sub((-170141183460469231731687303715884105727i128))
///         == (-170141183460469231731687303715884105728i128));
/// assert!(1i128.wrapping_sub((-170141183460469231731687303715884105728i128))
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         .wrapping_sub(170141183460469231731687303715884105727i128) == 1i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub(170141183460469231731687303715884105727i128) == 2i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub(170141183460469231731687303715884105726i128) == 3i128);
/// assert!(17814744590044481359733332230596038717i128
///         .wrapping_sub((-155529549309608364966066128491700545659i128))
///         == (-166938073021285617137575146709471627080i128));
/// assert!(101390534553535730387580692900502242439i128
///         .wrapping_sub((-98780566071891863956657562388589917228i128))
///         == (-140111266295510869119136352142676051789i128));
/// assert!(80509003256298198669521581368665036065i128
///         .wrapping_sub((-108516445177213582876484535056042362876i128))
///         == (-151256918487426681917368491007060812515i128));
/// assert!(92743455348628082966061326330345695796i128
///         .wrapping_sub((-98649566796694169115891623699834944846i128))
///         == (-148889344775616211381421657401587570814i128));
/// # }
/// ```
pub fn core_ops_sub_i128_wrapping_sub() {}
/// # Properties for [`i128::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i128::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105726i128.overflowing_sub(1i128)
///         == (170141183460469231731687303715884105725i128, false));
/// assert!(0i128.overflowing_sub(1i128) == ((-1i128), false));
/// assert!(1i128.overflowing_sub((-1i128)) == (2i128, false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_sub(1i128)
///         == ((-170141183460469231731687303715884105728i128), false));
/// assert!(0i128.overflowing_sub((-1i128)) == (1i128, false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_sub(0i128)
///         == ((-170141183460469231731687303715884105727i128), false));
/// assert!(125828564064545389997748728459809835346i128
///         .overflowing_sub(36875503040791509931454496520027734964i128)
///         == (88953061023753880066294231939782100382i128, false));
/// assert!(108068719877243987509788801183286455042i128
///         .overflowing_sub(19015897762478741213037056840393859012i128)
///         == (89052822114765246296751744342892596030i128, false));
/// assert!(74612087224267480325887795704181463494i128
///         .overflowing_sub((-4705229611530188250910428094201058570i128))
///         == (79317316835797668576798223798382522064i128, false));
/// assert!(59528138715501073594247543836895901644i128
///         .overflowing_sub((-74099879679713308066556714563023638314i128))
///         == (133628018395214381660804258399919539958i128, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i128::down(
///                 (-i128::MIN.up() + x.up() - y.up()).rem_euclid(&(-i128::MIN.up() * 2))
///                     + i128::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i128.overflowing_sub((-170141183460469231731687303715884105727i128))
///         == ((-170141183460469231731687303715884105728i128), true));
/// assert!((-170141183460469231731687303715884105727i128)
///         .overflowing_sub(170141183460469231731687303715884105727i128) == (2i128, true));
/// assert!((-170141183460469231731687303715884105728i128)
///         .overflowing_sub(170141183460469231731687303715884105726i128) == (2i128, true));
/// assert!((-170141183460469231731687303715884105728i128).overflowing_sub(1i128)
///         == (170141183460469231731687303715884105727i128, true));
/// assert!(170141183460469231731687303715884105727i128
///         .overflowing_sub((-170141183460469231731687303715884105727i128))
///         == ((-2i128), true));
/// assert!(170141183460469231731687303715884105727i128
///         .overflowing_sub((-170141183460469231731687303715884105728i128))
///         == ((-1i128), true));
/// assert!((-143208219107728302060195244442391656778i128)
///         .overflowing_sub(52930383005662946835650806446065310180i128)
///         == (144143764807547214567528556543311244498i128, true));
/// assert!((-80829425589614337071051517737721238942i128)
///         .overflowing_sub(90189065338170033061254859808056308495i128)
///         == (169263875993154093331068229885990664019i128, true));
/// assert!((-126623540200825819828897065004733092038i128)
///         .overflowing_sub(153220145858432574434572544956153847870i128)
///         == (60438680861680069199904997470881271548i128, true));
/// assert!((-80439917284786024012493148783031165263i128)
///         .overflowing_sub(104999893035095897054143156765109414737i128)
///         == (154842556601056542396738301883627631456i128, true));
/// # }
/// ```
pub fn core_ops_sub_i128_overflowing_sub() {}
/// # Properties for [`i128::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() + y.up() <= i128::MAX.up() && x.up() + y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x + y == i128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i128 + (-170141183460469231731687303715884105728i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!(0i128 + (-1i128) == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128) + 1i128
///         == (-170141183460469231731687303715884105726i128));
/// assert!(0i128 + 0i128 == 0i128);
/// assert!(0i128 + 170141183460469231731687303715884105726i128
///         == 170141183460469231731687303715884105726i128);
/// assert!(170141183460469231731687303715884105727i128 + (-1i128)
///         == 170141183460469231731687303715884105726i128);
/// assert!((-52181997832256389654694617115968108509i128)
///         + (-106780841953012999729560145679617704756i128)
///         == (-158962839785269389384254762795585813265i128));
/// assert!((-19147935524697798739458439600576770430i128)
///         + (-55995203217088730394043569158699350743i128)
///         == (-75143138741786529133502008759276121173i128));
/// assert!(56595829749848347862923239201373075461i128
///         + 9364161417000525240109711593844345934i128
///         == 65959991166848873103032950795217421395i128);
/// assert!(124596170137555952592279428652217867308i128
///         + (-28157412648781052579094488388371918167i128)
///         == 96438757488774900013184940263845949141i128);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() + y.up() > i128::MAX.up() || x.up() + y.up() < i128::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 170141183460469231731687303715884105728i128) + (- 1i128)));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) + (-
///         170141183460469231731687303715884105727i128)
///     ));
/// assert!(panics!((- 1i128) + (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(1i128 + 170141183460469231731687303715884105727i128));
/// assert!(panics!(
///         170141183460469231731687303715884105726i128 +
///         170141183460469231731687303715884105727i128
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) + (-
///         170141183460469231731687303715884105728i128)
///     ));
/// assert!(panics!(
///         (- 159070151708666318704462515417394858731i128) + (-
///         134039088793779166563477490218772469396i128)
///     ));
/// assert!(panics!(
///         (- 8535415636107801328050055625043548378i128) + (-
///         162110354832652827007424206141517220391i128)
///     ));
/// assert!(panics!(
///         155571346226686409871085861706731467441i128 +
///         24384985308309877902880946694911998721i128
///     ));
/// assert!(panics!(
///         166778786978424979669639343590057790963i128 +
///         33817174348811020470376390521408982865i128
///     ));
/// # }
/// ```
pub fn core_ops_add_i128_add() {}
/// # Properties for [`isize::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(isize::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807isize.checked_neg() == Some((-9223372036854775807isize)));
/// assert!((-9223372036854775807isize).checked_neg() == Some(9223372036854775807isize));
/// assert!(9223372036854775806isize.checked_neg() == Some((-9223372036854775806isize)));
/// assert!(1isize.checked_neg() == Some((-1isize)));
/// assert!(0isize.checked_neg() == Some(0isize));
/// assert!((-1isize).checked_neg() == Some(1isize));
/// assert!(3537503154257542993isize.checked_neg() == Some((-3537503154257542993isize)));
/// assert!((-7728356602620974809isize).checked_neg() == Some(7728356602620974809isize));
/// assert!((-4308988377874137764isize).checked_neg() == Some(4308988377874137764isize));
/// assert!((-6665547116415068672isize).checked_neg() == Some(6665547116415068672isize));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).checked_neg() == None);
/// # }
/// ```
pub fn t_isize_checked_neg() {}
/// # Properties for [`isize::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == isize::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         1isize.neg() == (-1isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-9223372036854775807isize).neg() == 9223372036854775807isize
///     });
/// assert!({
///         use std::ops::Neg;
///         0isize.neg() == 0isize
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1isize).neg() == 1isize
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775806isize.neg() == (-9223372036854775806isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775807isize.neg() == (-9223372036854775807isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         7219341674998674654isize.neg() == (-7219341674998674654isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         6531535538441505164isize.neg() == (-6531535538441505164isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-2908799430378840854isize).neg() == 2908799430378840854isize
///     });
/// assert!({
///         use std::ops::Neg;
///         1317839261097513755isize.neg() == (-1317839261097513755isize)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 9223372036854775808isize).neg())
///     });
/// # }
/// ```
pub fn t_isize_neg() {}
/// # Properties for [`isize::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (isize::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0isize.overflowing_neg() == (0isize, false));
/// assert!((-1isize).overflowing_neg() == (1isize, false));
/// assert!(9223372036854775806isize.overflowing_neg() == ((-9223372036854775806isize), false));
/// assert!(1isize.overflowing_neg() == ((-1isize), false));
/// assert!(9223372036854775807isize.overflowing_neg() == ((-9223372036854775807isize), false));
/// assert!((-9223372036854775807isize).overflowing_neg() == (9223372036854775807isize, false));
/// assert!(2136591824159528080isize.overflowing_neg() == ((-2136591824159528080isize), false));
/// assert!((-5047606451370398769isize).overflowing_neg() == (5047606451370398769isize, false));
/// assert!(4094943535313926681isize.overflowing_neg() == ((-4094943535313926681isize), false));
/// assert!((-6692029559778129224isize).overflowing_neg() == (6692029559778129224isize, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (isize::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).overflowing_neg() == ((-9223372036854775808isize), true));
/// # }
/// ```
pub fn t_isize_overflowing_neg() {}
/// # Properties for [`isize::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x - y == isize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775806isize - 0isize == 9223372036854775806isize);
/// assert!((-9223372036854775807isize) - (-9223372036854775808isize) == 1isize);
/// assert!(9223372036854775806isize - (-1isize) == 9223372036854775807isize);
/// assert!((-1isize) - 0isize == (-1isize));
/// assert!((-9223372036854775808isize) - (-9223372036854775807isize) == (-1isize));
/// assert!((-1isize) - 1isize == (-2isize));
/// assert!((-7338477564517266696isize) - (-5628106797884762622isize)
///         == (-1710370766632504074isize));
/// assert!((-8138309089473986692isize) - (-8686489985010777830isize) == 548180895536791138isize);
/// assert!((-6270893275237448604isize) - (-6535369047831633476isize) == 264475772594184872isize);
/// assert!(857154350852554582isize - 8496235353108329636isize == (-7639081002255775054isize));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808isize) - 1isize));
/// assert!(panics!((- 9223372036854775808isize) - 9223372036854775807isize));
/// assert!(panics!(9223372036854775807isize - (- 9223372036854775807isize)));
/// assert!(panics!((- 9223372036854775807isize) - 9223372036854775807isize));
/// assert!(panics!((- 9223372036854775807isize) - 9223372036854775806isize));
/// assert!(panics!(9223372036854775806isize - (- 9223372036854775807isize)));
/// assert!(panics!(4503853598369427710isize - (- 7617894189483393931isize)));
/// assert!(panics!((- 8454894531472910545isize) - 3684059865734408602isize));
/// assert!(panics!(7284450373288922992isize - (- 4050578844854945209isize)));
/// assert!(panics!(3476275717907553625isize - (- 6676084141611179642isize)));
/// # }
/// ```
pub fn core_ops_sub_isize_sub() {}
/// # Properties for [`isize::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(isize::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775807isize).checked_sub((-1isize))
///         == Some((-9223372036854775806isize)));
/// assert!(9223372036854775806isize.checked_sub(9223372036854775806isize) == Some(0isize));
/// assert!(9223372036854775807isize.checked_sub(0isize) == Some(9223372036854775807isize));
/// assert!(0isize.checked_sub(9223372036854775807isize) == Some((-9223372036854775807isize)));
/// assert!(9223372036854775807isize.checked_sub(9223372036854775807isize) == Some(0isize));
/// assert!(0isize.checked_sub((-1isize)) == Some(1isize));
/// assert!((-6124743656760643910isize).checked_sub((-7907625948738860370isize))
///         == Some(1782882291978216460isize));
/// assert!(4060881704856995796isize.checked_sub((-1311929649082019865isize))
///         == Some(5372811353939015661isize));
/// assert!((-4843263809085804003isize).checked_sub(3990304723078083787isize)
///         == Some((-8833568532163887790isize)));
/// assert!(1525253900160419680isize.checked_sub(3498927651472102397isize)
///         == Some((-1973673751311682717isize)));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807isize.checked_sub((-1isize)) == None);
/// assert!((-9223372036854775807isize).checked_sub(9223372036854775807isize) == None);
/// assert!(1isize.checked_sub((-9223372036854775807isize)) == None);
/// assert!((-9223372036854775807isize).checked_sub(9223372036854775806isize) == None);
/// assert!(9223372036854775807isize.checked_sub((-9223372036854775808isize)) == None);
/// assert!((-9223372036854775808isize).checked_sub(1isize) == None);
/// assert!(3453491204601043367isize.checked_sub((-6379769121324551949isize)) == None);
/// assert!((-8922444981877071720isize).checked_sub(3320191213645445860isize) == None);
/// assert!((-6924608267607407967isize).checked_sub(5828414576109632713isize) == None);
/// assert!((-8361600093938792900isize).checked_sub(5060699018366504830isize) == None);
/// # }
/// ```
pub fn core_ops_sub_isize_checked_sub() {}
/// # Properties for [`isize::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == isize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1isize).wrapping_sub(9223372036854775807isize) == (-9223372036854775808isize));
/// assert!(1isize.wrapping_sub(9223372036854775807isize) == (-9223372036854775806isize));
/// assert!((-9223372036854775808isize).wrapping_sub((-9223372036854775807isize)) == (-1isize));
/// assert!((-9223372036854775807isize).wrapping_sub((-9223372036854775808isize)) == 1isize);
/// assert!(9223372036854775806isize.wrapping_sub((-1isize)) == 9223372036854775807isize);
/// assert!(0isize.wrapping_sub(0isize) == 0isize);
/// assert!((-2118665846411122691isize).wrapping_sub((-4093022624106893409isize))
///         == 1974356777695770718isize);
/// assert!(1765633544314613545isize.wrapping_sub((-6155539524046151672isize))
///         == 7921173068360765217isize);
/// assert!((-3878262326622918773isize).wrapping_sub(364310167721180810isize)
///         == (-4242572494344099583isize));
/// assert!((-3325162980394686616isize).wrapping_sub(4897713338364617322isize)
///         == (-8222876318759303938isize));
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == isize::down(
///             (-isize::MIN.up() + x.up() - y.up()).rem_euclid(&(-isize::MIN.up() * 2))
///                 + isize::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807isize.wrapping_sub((-9223372036854775808isize)) == (-1isize));
/// assert!(0isize.wrapping_sub((-9223372036854775808isize)) == (-9223372036854775808isize));
/// assert!(9223372036854775806isize.wrapping_sub((-9223372036854775808isize)) == (-2isize));
/// assert!((-9223372036854775808isize).wrapping_sub(9223372036854775807isize) == 1isize);
/// assert!((-9223372036854775808isize).wrapping_sub(9223372036854775806isize) == 2isize);
/// assert!(9223372036854775806isize.wrapping_sub((-9223372036854775807isize)) == (-3isize));
/// assert!((-5480730541309930839isize).wrapping_sub(4681652054233516845isize)
///         == 8284361478166103932isize);
/// assert!(5141794441144812228isize.wrapping_sub((-7919222970651713296isize))
///         == (-5385726661913026092isize));
/// assert!(4386896238067475151isize.wrapping_sub((-5828049264749490187isize))
///         == (-8231798570892586278isize));
/// assert!((-8866738695397750423isize).wrapping_sub(2576013731143679621isize)
///         == 7003991647168121572isize);
/// # }
/// ```
pub fn core_ops_sub_isize_wrapping_sub() {}
/// # Properties for [`isize::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (isize::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0isize.overflowing_sub(9223372036854775806isize)
///         == ((-9223372036854775806isize), false));
/// assert!((-1isize).overflowing_sub(0isize) == ((-1isize), false));
/// assert!(0isize.overflowing_sub(1isize) == ((-1isize), false));
/// assert!(1isize.overflowing_sub(9223372036854775807isize)
///         == ((-9223372036854775806isize), false));
/// assert!((-9223372036854775808isize).overflowing_sub((-9223372036854775807isize))
///         == ((-1isize), false));
/// assert!((-1isize).overflowing_sub(1isize) == ((-2isize), false));
/// assert!((-4100590147257551976isize).overflowing_sub((-768463483533997689isize))
///         == ((-3332126663723554287isize), false));
/// assert!(2652201538014560302isize.overflowing_sub(7062037720623157490isize)
///         == ((-4409836182608597188isize), false));
/// assert!(1526972100056696383isize.overflowing_sub((-78161275453106386isize))
///         == (1605133375509802769isize, false));
/// assert!(5320482458855998708isize.overflowing_sub(4870150821354305526isize)
///         == (450331637501693182isize, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             isize::down(
///                 (-isize::MIN.up() + x.up() - y.up()).rem_euclid(&(-isize::MIN.up() * 2))
///                     + isize::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0isize.overflowing_sub((-9223372036854775808isize))
///         == ((-9223372036854775808isize), true));
/// assert!((-9223372036854775807isize).overflowing_sub(9223372036854775807isize)
///         == (2isize, true));
/// assert!((-9223372036854775808isize).overflowing_sub(1isize)
///         == (9223372036854775807isize, true));
/// assert!(9223372036854775807isize.overflowing_sub((-9223372036854775807isize))
///         == ((-2isize), true));
/// assert!(9223372036854775806isize.overflowing_sub((-9223372036854775808isize))
///         == ((-2isize), true));
/// assert!(9223372036854775807isize.overflowing_sub((-9223372036854775808isize))
///         == ((-1isize), true));
/// assert!(9165045295486714134isize.overflowing_sub((-6435302149633211661isize))
///         == ((-2846396628589625821isize), true));
/// assert!(8485965147729987740isize.overflowing_sub((-8641414842229566103isize))
///         == ((-1319364083749997773isize), true));
/// assert!((-9219609676091240046isize).overflowing_sub(5753357497040942061isize)
///         == (3473776900577369509isize, true));
/// assert!(4605438822403994148isize.overflowing_sub((-4968905245923248448isize))
///         == ((-8872400005382309020isize), true));
/// # }
/// ```
pub fn core_ops_sub_isize_overflowing_sub() {}
/// # Properties for [`isize::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() + y.up() <= isize::MAX.up() && x.up() + y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x + y == isize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0isize + 9223372036854775806isize == 9223372036854775806isize);
/// assert!((-1isize) + 1isize == 0isize);
/// assert!((-9223372036854775807isize) + 9223372036854775806isize == (-1isize));
/// assert!(0isize + 9223372036854775807isize == 9223372036854775807isize);
/// assert!(9223372036854775806isize + (-1isize) == 9223372036854775805isize);
/// assert!(9223372036854775806isize + 0isize == 9223372036854775806isize);
/// assert!((-5577090501246700721isize) + 6123347282551532699isize == 546256781304831978isize);
/// assert!(3263784076261961438isize + 5070950801080284449isize == 8334734877342245887isize);
/// assert!((-7703283440005635866isize) + 531052765719112566isize == (-7172230674286523300isize));
/// assert!(12185998881093367isize + 207610338125592638isize == 219796337006686005isize);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() + y.up() > isize::MAX.up() || x.up() + y.up() < isize::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(9223372036854775806isize + 9223372036854775807isize));
/// assert!(panics!((- 9223372036854775808isize) + (- 9223372036854775807isize)));
/// assert!(panics!(9223372036854775807isize + 9223372036854775806isize));
/// assert!(panics!(9223372036854775806isize + 9223372036854775806isize));
/// assert!(panics!((- 9223372036854775807isize) + (- 9223372036854775808isize)));
/// assert!(panics!(9223372036854775807isize + 9223372036854775807isize));
/// assert!(panics!(6028659065494822999isize + 7477296701078633986isize));
/// assert!(panics!((- 9023495905589270361isize) + (- 3790890983243907443isize)));
/// assert!(panics!(7244071720925009634isize + 2828726473711570021isize));
/// assert!(panics!(5400842486091215893isize + 8716190792084759691isize));
/// # }
/// ```
pub fn core_ops_add_isize_add() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.cmp(&(1u8)) == core::cmp::Ordering::Less);
/// assert!(0u8.cmp(&(0u8)) == core::cmp::Ordering::Equal);
/// assert!(1u8.cmp(&(0u8)) == core::cmp::Ordering::Greater);
/// assert!(0u8.cmp(&(255u8)) == core::cmp::Ordering::Less);
/// assert!(0u8.cmp(&(254u8)) == core::cmp::Ordering::Less);
/// assert!(150u8.cmp(&(238u8)) == core::cmp::Ordering::Less);
/// assert!(186u8.cmp(&(55u8)) == core::cmp::Ordering::Greater);
/// assert!(95u8.cmp(&(168u8)) == core::cmp::Ordering::Less);
/// assert!(92u8.cmp(&(101u8)) == core::cmp::Ordering::Less);
/// assert!(211u8.cmp(&(1u8)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.lt(&(254u8)) == true);
/// assert!(255u8.lt(&(254u8)) == false);
/// assert!(254u8.lt(&(1u8)) == false);
/// assert!(1u8.lt(&(1u8)) == false);
/// assert!(0u8.lt(&(255u8)) == true);
/// assert!(0u8.lt(&(0u8)) == false);
/// assert!(218u8.lt(&(100u8)) == false);
/// assert!(22u8.lt(&(7u8)) == false);
/// assert!(120u8.lt(&(9u8)) == false);
/// assert!(8u8.lt(&(11u8)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.gt(&(0u8)) == false);
/// assert!(0u8.gt(&(255u8)) == false);
/// assert!(0u8.gt(&(1u8)) == false);
/// assert!(255u8.gt(&(0u8)) == true);
/// assert!(75u8.gt(&(243u8)) == false);
/// assert!(6u8.gt(&(67u8)) == false);
/// assert!(70u8.gt(&(200u8)) == false);
/// assert!(95u8.gt(&(56u8)) == true);
/// assert!(107u8.gt(&(50u8)) == true);
/// assert!(31u8.gt(&(32u8)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.ge(&(0u8)) == true);
/// assert!(0u8.ge(&(1u8)) == false);
/// assert!(255u8.ge(&(0u8)) == true);
/// assert!(0u8.ge(&(0u8)) == true);
/// assert!(0u8.ge(&(255u8)) == false);
/// assert!(114u8.ge(&(3u8)) == true);
/// assert!(126u8.ge(&(220u8)) == false);
/// assert!(100u8.ge(&(172u8)) == false);
/// assert!(56u8.ge(&(128u8)) == false);
/// assert!(10u8.ge(&(102u8)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.le(&(1u8)) == false);
/// assert!(254u8.le(&(254u8)) == true);
/// assert!(255u8.le(&(0u8)) == false);
/// assert!(254u8.le(&(0u8)) == false);
/// assert!(0u8.le(&(0u8)) == true);
/// assert!(255u8.le(&(254u8)) == false);
/// assert!(198u8.le(&(8u8)) == false);
/// assert!(133u8.le(&(162u8)) == true);
/// assert!(17u8.le(&(212u8)) == true);
/// assert!(97u8.le(&(163u8)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_le() {}
/// # Properties for [`core::ops::BitXor::<u8>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u8::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 ^ 0u8 == 0u8);
/// assert!(255u8 ^ 1u8 == 254u8);
/// assert!(255u8 ^ 254u8 == 1u8);
/// assert!(254u8 ^ 0u8 == 254u8);
/// assert!(149u8 ^ 65u8 == 212u8);
/// assert!(80u8 ^ 68u8 == 20u8);
/// assert!(21u8 ^ 70u8 == 83u8);
/// assert!(252u8 ^ 91u8 == 167u8);
/// assert!(31u8 ^ 188u8 == 163u8);
/// assert!(30u8 ^ 56u8 == 38u8);
/// # }
/// ```
pub fn core_ops_bit_xor_u8_xor() {}
/// # Properties for [`core::ops::BitAnd::<u8>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u8::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(255u8 & 0u8 == 0u8);
/// assert!(254u8 & 254u8 == 254u8);
/// assert!(1u8 & 255u8 == 1u8);
/// assert!(254u8 & 0u8 == 0u8);
/// assert!(0u8 & 1u8 == 0u8);
/// assert!(0u8 & 0u8 == 0u8);
/// assert!(54u8 & 64u8 == 0u8);
/// assert!(86u8 & 104u8 == 64u8);
/// assert!(7u8 & 46u8 == 6u8);
/// assert!(193u8 & 249u8 == 193u8);
/// # }
/// ```
pub fn core_ops_bit_and_u8_and() {}
/// # Properties for [`core::ops::BitAnd::<u8>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u8::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 ^ 254u8 == 254u8);
/// assert!(1u8 ^ 0u8 == 1u8);
/// assert!(0u8 ^ 0u8 == 0u8);
/// assert!(0u8 ^ 1u8 == 1u8);
/// assert!(1u8 ^ 1u8 == 1u8);
/// assert!(173u8 ^ 89u8 == 253u8);
/// assert!(234u8 ^ 125u8 == 255u8);
/// assert!(233u8 ^ 18u8 == 251u8);
/// assert!(66u8 ^ 22u8 == 86u8);
/// assert!(28u8 ^ 234u8 == 254u8);
/// # }
/// ```
pub fn core_ops_bit_and_u8_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.cmp(&(65534u16)) == core::cmp::Ordering::Less);
/// assert!(65535u16.cmp(&(65535u16)) == core::cmp::Ordering::Equal);
/// assert!(65534u16.cmp(&(0u16)) == core::cmp::Ordering::Greater);
/// assert!(65534u16.cmp(&(1u16)) == core::cmp::Ordering::Greater);
/// assert!(0u16.cmp(&(65535u16)) == core::cmp::Ordering::Less);
/// assert!(11939u16.cmp(&(64681u16)) == core::cmp::Ordering::Less);
/// assert!(22804u16.cmp(&(4773u16)) == core::cmp::Ordering::Greater);
/// assert!(20357u16.cmp(&(59508u16)) == core::cmp::Ordering::Less);
/// assert!(58367u16.cmp(&(26421u16)) == core::cmp::Ordering::Greater);
/// assert!(55574u16.cmp(&(46287u16)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.lt(&(65535u16)) == true);
/// assert!(0u16.lt(&(0u16)) == false);
/// assert!(65534u16.lt(&(1u16)) == false);
/// assert!(0u16.lt(&(1u16)) == true);
/// assert!(65535u16.lt(&(65534u16)) == false);
/// assert!(26968u16.lt(&(20109u16)) == false);
/// assert!(1311u16.lt(&(5448u16)) == true);
/// assert!(22272u16.lt(&(24695u16)) == true);
/// assert!(6338u16.lt(&(42276u16)) == true);
/// assert!(5077u16.lt(&(2890u16)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.gt(&(65534u16)) == false);
/// assert!(65534u16.gt(&(0u16)) == true);
/// assert!(65534u16.gt(&(65534u16)) == false);
/// assert!(1u16.gt(&(1u16)) == false);
/// assert!(40617u16.gt(&(16263u16)) == true);
/// assert!(62536u16.gt(&(19997u16)) == true);
/// assert!(21659u16.gt(&(27909u16)) == false);
/// assert!(51899u16.gt(&(45514u16)) == true);
/// assert!(16303u16.gt(&(11082u16)) == true);
/// assert!(36766u16.gt(&(9997u16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.ge(&(0u16)) == true);
/// assert!(1u16.ge(&(65535u16)) == false);
/// assert!(65534u16.ge(&(0u16)) == true);
/// assert!(65535u16.ge(&(65535u16)) == true);
/// assert!(65535u16.ge(&(1u16)) == true);
/// assert!(65534u16.ge(&(1u16)) == true);
/// assert!(30684u16.ge(&(26744u16)) == true);
/// assert!(58107u16.ge(&(53740u16)) == true);
/// assert!(61819u16.ge(&(2664u16)) == true);
/// assert!(59930u16.ge(&(20300u16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.le(&(0u16)) == false);
/// assert!(0u16.le(&(0u16)) == true);
/// assert!(0u16.le(&(65534u16)) == true);
/// assert!(0u16.le(&(65535u16)) == true);
/// assert!(0u16.le(&(1u16)) == true);
/// assert!(65534u16.le(&(65535u16)) == true);
/// assert!(39630u16.le(&(52301u16)) == true);
/// assert!(46239u16.le(&(28033u16)) == false);
/// assert!(42142u16.le(&(13498u16)) == false);
/// assert!(12843u16.le(&(18711u16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_le() {}
/// # Properties for [`core::ops::BitXor::<u16>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u16::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 ^ 1u16 == 1u16);
/// assert!(1u16 ^ 65534u16 == 65535u16);
/// assert!(65534u16 ^ 65534u16 == 0u16);
/// assert!(65534u16 ^ 0u16 == 65534u16);
/// assert!(65535u16 ^ 0u16 == 65535u16);
/// assert!(32419u16 ^ 52140u16 == 46351u16);
/// assert!(4768u16 ^ 48556u16 == 44812u16);
/// assert!(6110u16 ^ 28209u16 == 31215u16);
/// assert!(54632u16 ^ 56006u16 == 4014u16);
/// assert!(34111u16 ^ 15343u16 == 48848u16);
/// # }
/// ```
pub fn core_ops_bit_xor_u16_xor() {}
/// # Properties for [`core::ops::BitAnd::<u16>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u16::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16 & 65535u16 == 65534u16);
/// assert!(1u16 & 65534u16 == 0u16);
/// assert!(1u16 & 0u16 == 0u16);
/// assert!(65534u16 & 65534u16 == 65534u16);
/// assert!(0u16 & 0u16 == 0u16);
/// assert!(65535u16 & 1u16 == 1u16);
/// assert!(51744u16 & 64907u16 == 51200u16);
/// assert!(64253u16 & 7428u16 == 6148u16);
/// assert!(22582u16 & 9911u16 == 54u16);
/// assert!(54825u16 & 30247u16 == 22049u16);
/// # }
/// ```
pub fn core_ops_bit_and_u16_and() {}
/// # Properties for [`core::ops::BitAnd::<u16>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u16::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 ^ 1u16 == 1u16);
/// assert!(1u16 ^ 0u16 == 1u16);
/// assert!(0u16 ^ 0u16 == 0u16);
/// assert!(1u16 ^ 65535u16 == 65535u16);
/// assert!(0u16 ^ 65535u16 == 65535u16);
/// assert!(23763u16 ^ 6984u16 == 24539u16);
/// assert!(9097u16 ^ 47694u16 == 48079u16);
/// assert!(28153u16 ^ 9180u16 == 28669u16);
/// assert!(59916u16 ^ 46124u16 == 65068u16);
/// assert!(4849u16 ^ 38796u16 == 38909u16);
/// # }
/// ```
pub fn core_ops_bit_and_u16_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.cmp(&(4294967294u32)) == core::cmp::Ordering::Less);
/// assert!(0u32.cmp(&(0u32)) == core::cmp::Ordering::Equal);
/// assert!(4294967295u32.cmp(&(4294967294u32)) == core::cmp::Ordering::Greater);
/// assert!(1u32.cmp(&(0u32)) == core::cmp::Ordering::Greater);
/// assert!(2442341884u32.cmp(&(1645578306u32)) == core::cmp::Ordering::Greater);
/// assert!(1084989983u32.cmp(&(766142279u32)) == core::cmp::Ordering::Greater);
/// assert!(577255654u32.cmp(&(2724991446u32)) == core::cmp::Ordering::Less);
/// assert!(3433264678u32.cmp(&(756495547u32)) == core::cmp::Ordering::Greater);
/// assert!(2333914125u32.cmp(&(1828424063u32)) == core::cmp::Ordering::Greater);
/// assert!(4062343354u32.cmp(&(2710716942u32)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.lt(&(4294967294u32)) == false);
/// assert!(0u32.lt(&(4294967295u32)) == true);
/// assert!(0u32.lt(&(0u32)) == false);
/// assert!(4294967294u32.lt(&(4294967295u32)) == true);
/// assert!(0u32.lt(&(1u32)) == true);
/// assert!(4294967294u32.lt(&(1u32)) == false);
/// assert!(4186494733u32.lt(&(672218861u32)) == false);
/// assert!(4260154221u32.lt(&(2126547928u32)) == false);
/// assert!(660306068u32.lt(&(1362355201u32)) == true);
/// assert!(1260932585u32.lt(&(1843924377u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.gt(&(0u32)) == false);
/// assert!(1u32.gt(&(0u32)) == true);
/// assert!(4294967294u32.gt(&(4294967295u32)) == false);
/// assert!(4294967295u32.gt(&(0u32)) == true);
/// assert!(1u32.gt(&(1u32)) == false);
/// assert!(1019579683u32.gt(&(2212541744u32)) == false);
/// assert!(2297203529u32.gt(&(1270592818u32)) == true);
/// assert!(3588725612u32.gt(&(1994474069u32)) == true);
/// assert!(2970483077u32.gt(&(3118984377u32)) == false);
/// assert!(4192652775u32.gt(&(1903374459u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.ge(&(1u32)) == false);
/// assert!(4294967295u32.ge(&(4294967295u32)) == true);
/// assert!(4294967295u32.ge(&(0u32)) == true);
/// assert!(1u32.ge(&(0u32)) == true);
/// assert!(0u32.ge(&(4294967295u32)) == false);
/// assert!(2885658332u32.ge(&(3452996605u32)) == false);
/// assert!(1121562012u32.ge(&(761036116u32)) == true);
/// assert!(61245231u32.ge(&(3593390789u32)) == false);
/// assert!(31946347u32.ge(&(619570930u32)) == false);
/// assert!(3865012932u32.ge(&(2626588627u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.le(&(1u32)) == true);
/// assert!(0u32.le(&(0u32)) == true);
/// assert!(1u32.le(&(4294967294u32)) == true);
/// assert!(4294967295u32.le(&(4294967295u32)) == true);
/// assert!(1u32.le(&(4294967295u32)) == true);
/// assert!(3843586300u32.le(&(917664912u32)) == false);
/// assert!(3819492793u32.le(&(3232091748u32)) == false);
/// assert!(3755102746u32.le(&(2510217956u32)) == false);
/// assert!(835584527u32.le(&(1000166374u32)) == true);
/// assert!(936178043u32.le(&(1939516403u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_le() {}
/// # Properties for [`core::ops::BitXor::<u32>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u32::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967295u32 ^ 0u32 == 4294967295u32);
/// assert!(0u32 ^ 4294967294u32 == 4294967294u32);
/// assert!(0u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(0u32 ^ 0u32 == 0u32);
/// assert!(1u32 ^ 1u32 == 0u32);
/// assert!(3719428355u32 ^ 778203150u32 == 4090726669u32);
/// assert!(1878662467u32 ^ 4226111701u32 == 2485077398u32);
/// assert!(3494118586u32 ^ 1432544789u32 == 2233918127u32);
/// assert!(871940486u32 ^ 1214214866u32 == 2074587988u32);
/// assert!(2130123639u32 ^ 4008889691u32 == 2416301612u32);
/// # }
/// ```
pub fn core_ops_bit_xor_u32_xor() {}
/// # Properties for [`core::ops::BitAnd::<u32>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u32::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 & 4294967295u32 == 0u32);
/// assert!(4294967294u32 & 0u32 == 0u32);
/// assert!(4294967294u32 & 1u32 == 0u32);
/// assert!(0u32 & 1u32 == 0u32);
/// assert!(4294967294u32 & 4294967294u32 == 4294967294u32);
/// assert!(393794357u32 & 303983053u32 == 303579397u32);
/// assert!(263546110u32 & 442171647u32 == 168886526u32);
/// assert!(188824282u32 & 2331628865u32 == 172036160u32);
/// assert!(3290527797u32 & 3350842314u32 == 3290515456u32);
/// assert!(2117875958u32 & 1549192625u32 == 1544816816u32);
/// # }
/// ```
pub fn core_ops_bit_and_u32_and() {}
/// # Properties for [`core::ops::BitAnd::<u32>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u32::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32 ^ 1u32 == 4294967295u32);
/// assert!(0u32 ^ 1u32 == 1u32);
/// assert!(0u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(0u32 ^ 0u32 == 0u32);
/// assert!(4294967295u32 ^ 4294967294u32 == 4294967295u32);
/// assert!(4294967295u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(3927765223u32 ^ 1615179634u32 == 3932028919u32);
/// assert!(137193208u32 ^ 3155261017u32 == 3158144761u32);
/// assert!(3748773486u32 ^ 2179280957u32 == 3757424255u32);
/// assert!(2318011961u32 ^ 1927514390u32 == 4209741631u32);
/// # }
/// ```
pub fn core_ops_bit_and_u32_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.cmp(&(1u64)) == core::cmp::Ordering::Less);
/// assert!(18446744073709551614u64.cmp(&(18446744073709551615u64)) == core::cmp::Ordering::Less);
/// assert!(18446744073709551615u64.cmp(&(0u64)) == core::cmp::Ordering::Greater);
/// assert!(0u64.cmp(&(0u64)) == core::cmp::Ordering::Equal);
/// assert!(0u64.cmp(&(18446744073709551615u64)) == core::cmp::Ordering::Less);
/// assert!(1u64.cmp(&(18446744073709551614u64)) == core::cmp::Ordering::Less);
/// assert!(13213794433460719651u64.cmp(&(437740726794001150u64)) == core::cmp::Ordering::Greater);
/// assert!(16918565706134240658u64.cmp(&(4142892570982282199u64))
///         == core::cmp::Ordering::Greater);
/// assert!(1789306540001491632u64.cmp(&(5167049413076872212u64)) == core::cmp::Ordering::Less);
/// assert!(18221453977496644021u64.cmp(&(9815316640342419550u64))
///         == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.lt(&(18446744073709551615u64)) == true);
/// assert!(18446744073709551614u64.lt(&(18446744073709551614u64)) == false);
/// assert!(18446744073709551614u64.lt(&(0u64)) == false);
/// assert!(18446744073709551615u64.lt(&(0u64)) == false);
/// assert!(0u64.lt(&(1u64)) == true);
/// assert!(0u64.lt(&(0u64)) == false);
/// assert!(16529393551181359830u64.lt(&(10560436795421606921u64)) == false);
/// assert!(16261734463361744043u64.lt(&(3623658661856591092u64)) == false);
/// assert!(8041937897835489473u64.lt(&(8204677613264728963u64)) == true);
/// assert!(9644372205884945470u64.lt(&(234261193835538814u64)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.gt(&(18446744073709551614u64)) == false);
/// assert!(1u64.gt(&(1u64)) == false);
/// assert!(0u64.gt(&(18446744073709551615u64)) == false);
/// assert!(0u64.gt(&(0u64)) == false);
/// assert!(18446744073709551615u64.gt(&(0u64)) == true);
/// assert!(0u64.gt(&(18446744073709551614u64)) == false);
/// assert!(17295432936557638324u64.gt(&(11988615001318365476u64)) == true);
/// assert!(13366610725936001940u64.gt(&(12121811017962960719u64)) == true);
/// assert!(13485023395309231621u64.gt(&(17347952829536296490u64)) == false);
/// assert!(12096092365396812024u64.gt(&(7679456483973642085u64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.ge(&(0u64)) == true);
/// assert!(0u64.ge(&(18446744073709551615u64)) == false);
/// assert!(18446744073709551615u64.ge(&(0u64)) == true);
/// assert!(0u64.ge(&(18446744073709551614u64)) == false);
/// assert!(1u64.ge(&(18446744073709551615u64)) == false);
/// assert!(5838378661862588901u64.ge(&(5073649061566793591u64)) == true);
/// assert!(655752267200962834u64.ge(&(5755220269233793101u64)) == false);
/// assert!(13257554913055717021u64.ge(&(496724652700193207u64)) == true);
/// assert!(15489621949649744067u64.ge(&(1926042654119440727u64)) == true);
/// assert!(6585258724675476678u64.ge(&(2800731603233622589u64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.le(&(18446744073709551615u64)) == true);
/// assert!(18446744073709551615u64.le(&(18446744073709551614u64)) == false);
/// assert!(18446744073709551614u64.le(&(18446744073709551614u64)) == true);
/// assert!(1u64.le(&(18446744073709551614u64)) == true);
/// assert!(1u64.le(&(1u64)) == true);
/// assert!(18446744073709551615u64.le(&(0u64)) == false);
/// assert!(11875182648985707840u64.le(&(12080112884013250055u64)) == true);
/// assert!(14662345349485670066u64.le(&(6873013796583848173u64)) == false);
/// assert!(11009741940783154407u64.le(&(12822803268307445279u64)) == true);
/// assert!(7133606662295586269u64.le(&(6828898118031853174u64)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_le() {}
/// # Properties for [`core::ops::BitXor::<u64>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u64::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 ^ 1u64 == 1u64);
/// assert!(0u64 ^ 0u64 == 0u64);
/// assert!(0u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 18446744073709551614u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 18446744073709551615u64 == 18446744073709551614u64);
/// assert!(10569359783943770507u64 ^ 699906689060366234u64 == 11176660857731446289u64);
/// assert!(11270969108337271194u64 ^ 10320102069690709427u64 == 1392413379698103337u64);
/// assert!(5644577740280102056u64 ^ 13994097542218366393u64 == 10115247621530403089u64);
/// assert!(10354753258040340511u64 ^ 14861006022294228883u64 == 4724169375569543052u64);
/// assert!(2158328267866092919u64 ^ 2000770087314074389u64 == 448044206183849570u64);
/// # }
/// ```
pub fn core_ops_bit_xor_u64_xor() {}
/// # Properties for [`core::ops::BitAnd::<u64>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u64::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 & 0u64 == 0u64);
/// assert!(18446744073709551614u64 & 18446744073709551615u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 & 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(0u64 & 1u64 == 0u64);
/// assert!(1u64 & 18446744073709551615u64 == 1u64);
/// assert!(11119840232417178776u64 & 2154202493393872418u64 == 1747682253565109248u64);
/// assert!(1082196562632551365u64 & 12797955971551615096u64 == 72119175485327424u64);
/// assert!(13355162916232848956u64 & 8349374575297707102u64 == 3555031019067548700u64);
/// assert!(6674139916270555908u64 & 3765803897775525814u64 == 1441788085682635524u64);
/// assert!(12495065271105909016u64 & 13383029403105418927u64 == 12187305747361845256u64);
/// # }
/// ```
pub fn core_ops_bit_and_u64_and() {}
/// # Properties for [`core::ops::BitAnd::<u64>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u64::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 ^ 0u64 == 18446744073709551615u64);
/// assert!(0u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(0u64 ^ 0u64 == 0u64);
/// assert!(1u64 ^ 0u64 == 1u64);
/// assert!(18446744073709551615u64 ^ 18446744073709551614u64 == 18446744073709551615u64);
/// assert!(1643133805565809788u64 ^ 1085388174722741703u64 == 2296158510258304511u64);
/// assert!(7165776469521041699u64 ^ 314127020877158779u64 == 7457384838794811771u64);
/// assert!(5349149419772950233u64 ^ 14578199747867211752u64 == 14590536269942355961u64);
/// assert!(26170688795323855u64 ^ 17818021751783760055u64 == 17824964180947316223u64);
/// # }
/// ```
pub fn core_ops_bit_and_u64_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.cmp(&(0u128)) == core::cmp::Ordering::Equal);
/// assert!(340282366920938463463374607431768211454u128.cmp(&(0u128))
///         == core::cmp::Ordering::Greater);
/// assert!(1u128.cmp(&(340282366920938463463374607431768211454u128))
///         == core::cmp::Ordering::Less);
/// assert!(340282366920938463463374607431768211455u128
///         .cmp(&(340282366920938463463374607431768211454u128))
///         == core::cmp::Ordering::Greater);
/// assert!(340282366920938463463374607431768211454u128
///         .cmp(&(340282366920938463463374607431768211454u128))
///         == core::cmp::Ordering::Equal);
/// assert!(44975951252517382259136865103509029750u128
///         .cmp(&(257641522641119427820388980212092478833u128)) == core::cmp::Ordering::Less);
/// assert!(248434684924782269368664903704438068841u128
///         .cmp(&(312374530296163019514539641492995103423u128)) == core::cmp::Ordering::Less);
/// assert!(276195689538573184530416463225100515095u128
///         .cmp(&(160513775042236247494311639318765835762u128))
///         == core::cmp::Ordering::Greater);
/// assert!(175683973946606948385124461571982633367u128
///         .cmp(&(123785432016582443496034597215545976793u128))
///         == core::cmp::Ordering::Greater);
/// assert!(303431222115574575076079116287094063580u128
///         .cmp(&(244235506227617182388752269599350103386u128))
///         == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.lt(&(1u128)) == true);
/// assert!(1u128.lt(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(1u128.lt(&(1u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.lt(&(0u128)) == false);
/// assert!(0u128.lt(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(0u128.lt(&(0u128)) == false);
/// assert!(75011316906720783014415590577846840000u128
///         .lt(&(266474547533521045769510423900201615758u128)) == true);
/// assert!(201038579442682444526088008712651054007u128
///         .lt(&(192134843451051941324400975454732287608u128)) == false);
/// assert!(60311938750415042280017927889255909986u128
///         .lt(&(55546713626375861041064972747374848283u128)) == false);
/// assert!(179693580295672571309301752130346329015u128
///         .lt(&(45967089973199729865842876010949644173u128)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128
///         .gt(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(340282366920938463463374607431768211455u128.gt(&(0u128)) == true);
/// assert!(0u128.gt(&(1u128)) == false);
/// assert!(340282366920938463463374607431768211454u128
///         .gt(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.gt(&(0u128)) == true);
/// assert!(97999415950712288112247566704919453034u128
///         .gt(&(208998753070433299756390042979153216980u128)) == false);
/// assert!(334899404621691313609704411733688726564u128
///         .gt(&(196339828089047844343060674069606985297u128)) == true);
/// assert!(137924494976594710027400828876862275552u128
///         .gt(&(147811836854977082207430458620432092947u128)) == false);
/// assert!(234155103396747879709534580194692888023u128
///         .gt(&(240559332697003469441202067648824230668u128)) == false);
/// assert!(242617701629624030176494084981164410872u128
///         .gt(&(252969267285176317069776849254812791786u128)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128.ge(&(1u128)) == true);
/// assert!(0u128.ge(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.ge(&(0u128)) == true);
/// assert!(340282366920938463463374607431768211455u128
///         .ge(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(1u128.ge(&(0u128)) == true);
/// assert!(85350778698249401943755168099500986728u128
///         .ge(&(292989768023270401715577185850297719905u128)) == false);
/// assert!(99781833176100874890600342518182837382u128
///         .ge(&(329073392374250701387799591442585294270u128)) == false);
/// assert!(109525442522514821699432389430748993314u128
///         .ge(&(325185859195176575216648050576873446644u128)) == false);
/// assert!(209432661073022208893386794696369458418u128
///         .ge(&(67843884735641090713812193561071478408u128)) == true);
/// assert!(248582467524149512075057417092736786156u128
///         .ge(&(72510307087564574936643873496085211907u128)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.le(&(1u128)) == true);
/// assert!(340282366920938463463374607431768211455u128.le(&(0u128)) == false);
/// assert!(0u128.le(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(1u128.le(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(1u128.le(&(0u128)) == false);
/// assert!(340282366920938463463374607431768211454u128
///         .le(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(250801215464997260551538121087393625146u128
///         .le(&(153869002672428563982312923461117563158u128)) == false);
/// assert!(78003341282175475431923564063773996984u128
///         .le(&(171178867677751175301008878294487976688u128)) == true);
/// assert!(27241733892516194315162783344863098721u128
///         .le(&(132692428182727579328214936494804729172u128)) == true);
/// assert!(167440770341424486732373128370091033169u128
///         .le(&(176041857438632164844136325937955598742u128)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_le() {}
/// # Properties for [`core::ops::BitXor::<u128>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u128::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211455u128 ^ 0u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 ^ 1u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128 ^ 0u128 == 0u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128
///         ^ 340282366920938463463374607431768211455u128 == 1u128);
/// assert!(100515759497424198178004829248659080458u128
///         ^ 17028118150245040338637248546220269041u128
///         == 94800278741714510520321752997230844155u128);
/// assert!(288358532758279779567439271186483507787u128
///         ^ 194501931814576217847698088040475834518u128
///         == 99340986858347662103609737056603556573u128);
/// assert!(57382622880374650780157198074094814769u128
///         ^ 11306851733673213203679957756374939343u128
///         == 47410282820674404635847145852149166334u128);
/// assert!(277759619550344690275886542435336298432u128
///         ^ 133611315184094704890141072120924011905u128
///         == 239853386868665520510507630127115493953u128);
/// assert!(203623934697437835876315267897315370067u128
///         ^ 52751048080306502576514191901902756242u128
///         == 253383771016922966341543805800837479873u128);
/// # }
/// ```
pub fn core_ops_bit_xor_u128_xor() {}
/// # Properties for [`core::ops::BitAnd::<u128>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u128::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 & 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(1u128 & 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 & 0u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 & 1u128 == 0u128);
/// assert!(1u128 & 1u128 == 1u128);
/// assert!(213569211148110816113509090019333024313u128
///         & 64766997167711590656117367690906792293u128
///         == 43416262343584438940517772137715081249u128);
/// assert!(266015873661949288871889529590188582010u128
///         & 107258173892353790379555198495129302742u128
///         == 85236907677934868130354838221601317970u128);
/// assert!(141133148211983711975482998026898322777u128
///         & 76084245140236140005109799090257510459u128
///         == 53404078939499182926251552253615341593u128);
/// assert!(193852923623568464329391347654825815128u128
///         & 30116152647048091484200982634186104798u128
///         == 21932911284858694289216721411848473688u128);
/// assert!(61457647653246635526851678072772226335u128
///         & 92084851425794466822433558750753310850u128
///         == 5339094602163524667452007953972971522u128);
/// # }
/// ```
pub fn core_ops_bit_and_u128_and() {}
/// # Properties for [`core::ops::BitAnd::<u128>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u128::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 ^ 0u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 ^ 1u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128 ^ 1u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(285492369433559669799128476157067439727u128
///         ^ 169383910529465744645641010293640367291u128
///         == 340199119025962149046678291574123245311u128);
/// assert!(249704822134609767220238823055576019501u128
///         ^ 326769756327981943871517360675834498456u128
///         == 340116028062072600292242378908940058557u128);
/// assert!(19972987297837837733793212380068437192u128
///         ^ 236792086078273273879285746688293668429u128
///         == 254085036472701184014963742582125878989u128);
/// assert!(138564139149668003437254354273004229399u128
///         ^ 216436894795911805358155198493012730469u128
///         == 312360705621671123131155560815178494839u128);
/// assert!(13146104270789994374496685605931219780u128
///         ^ 227638032407125136102271287939617655617u128
///         == 228481790396735012667632455855797861189u128);
/// # }
/// ```
pub fn core_ops_bit_and_u128_or() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.cmp(&(0usize)) == core::cmp::Ordering::Greater);
/// assert!(0usize.cmp(&(0usize)) == core::cmp::Ordering::Equal);
/// assert!(18446744073709551615usize.cmp(&(18446744073709551614usize))
///         == core::cmp::Ordering::Greater);
/// assert!(18446744073709551614usize.cmp(&(18446744073709551615usize))
///         == core::cmp::Ordering::Less);
/// assert!(18446744073709551614usize.cmp(&(18446744073709551614usize))
///         == core::cmp::Ordering::Equal);
/// assert!(11985131576077835587usize.cmp(&(3958291493197703931usize))
///         == core::cmp::Ordering::Greater);
/// assert!(11354775757869541389usize.cmp(&(4159345084370363883usize))
///         == core::cmp::Ordering::Greater);
/// assert!(15387150155922922713usize.cmp(&(11679247064800497855usize))
///         == core::cmp::Ordering::Greater);
/// assert!(4589999633741811223usize.cmp(&(11774579480904419386usize))
///         == core::cmp::Ordering::Less);
/// assert!(16028514710437862525usize.cmp(&(3655407390760220609usize))
///         == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.lt(&(18446744073709551615usize)) == true);
/// assert!(18446744073709551614usize.lt(&(18446744073709551615usize)) == true);
/// assert!(1usize.lt(&(1usize)) == false);
/// assert!(18446744073709551614usize.lt(&(0usize)) == false);
/// assert!(0usize.lt(&(1usize)) == true);
/// assert!(0usize.lt(&(0usize)) == false);
/// assert!(15714713890293842465usize.lt(&(349710768035612000usize)) == false);
/// assert!(1720369905380735984usize.lt(&(7743801427590002070usize)) == true);
/// assert!(9428006659334079341usize.lt(&(1495095731166364445usize)) == false);
/// assert!(9255160804280748848usize.lt(&(11025678047642898287usize)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.gt(&(1usize)) == false);
/// assert!(0usize.gt(&(0usize)) == false);
/// assert!(1usize.gt(&(18446744073709551615usize)) == false);
/// assert!(18446744073709551615usize.gt(&(18446744073709551614usize)) == true);
/// assert!(5730683667489131835usize.gt(&(5639615853215457845usize)) == true);
/// assert!(5593699895427135426usize.gt(&(6163324982822489568usize)) == false);
/// assert!(5988829289079745586usize.gt(&(16374715545702080131usize)) == false);
/// assert!(5927343272697646139usize.gt(&(10621509586726952183usize)) == false);
/// assert!(14315373999655727878usize.gt(&(6352533529211929156usize)) == true);
/// assert!(3020751273598939947usize.gt(&(13343786864262190173usize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.ge(&(0usize)) == true);
/// assert!(0usize.ge(&(0usize)) == true);
/// assert!(0usize.ge(&(18446744073709551614usize)) == false);
/// assert!(1usize.ge(&(0usize)) == true);
/// assert!(1usize.ge(&(18446744073709551615usize)) == false);
/// assert!(5181005897290978282usize.ge(&(14194977609904706062usize)) == false);
/// assert!(11686138792807738610usize.ge(&(15631570959185186883usize)) == false);
/// assert!(12800151004772488048usize.ge(&(2403838441003626885usize)) == true);
/// assert!(11222644653128175521usize.ge(&(432150017781366620usize)) == true);
/// assert!(3128551739928628859usize.ge(&(10657317708656039621usize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.le(&(18446744073709551615usize)) == true);
/// assert!(18446744073709551615usize.le(&(18446744073709551615usize)) == true);
/// assert!(0usize.le(&(18446744073709551614usize)) == true);
/// assert!(0usize.le(&(0usize)) == true);
/// assert!(14305212560139004132usize.le(&(13839098958081614053usize)) == false);
/// assert!(7601692900249525794usize.le(&(6344439206118479109usize)) == false);
/// assert!(6258900699520228186usize.le(&(12404026009959922566usize)) == true);
/// assert!(7776755904459744075usize.le(&(1753002499928464433usize)) == false);
/// assert!(4118847448894079153usize.le(&(9305961511845923707usize)) == true);
/// assert!(12784784501618872669usize.le(&(9865130266555126156usize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_le() {}
/// # Properties for [`core::ops::BitXor::<usize>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == usize::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(0usize ^ 1usize == 1usize);
/// assert!(1usize ^ 0usize == 1usize);
/// assert!(1usize ^ 18446744073709551615usize == 18446744073709551614usize);
/// assert!(18446744073709551615usize ^ 18446744073709551614usize == 1usize);
/// assert!(18446744073709551615usize ^ 0usize == 18446744073709551615usize);
/// assert!(9500901950412620861usize ^ 55754319386615831usize == 9448534144609068074usize);
/// assert!(946094835113128779usize ^ 3447351400693614610usize == 2519280107293915993usize);
/// assert!(14588046241291180164usize ^ 10163617621854426481usize == 5151918570135522805usize);
/// assert!(12333616816551590420usize ^ 17682821040726607698usize == 6795878139823619398usize);
/// # }
/// ```
pub fn core_ops_bit_xor_usize_xor() {}
/// # Properties for [`core::ops::BitAnd::<usize>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == usize::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize & 18446744073709551615usize == 1usize);
/// assert!(18446744073709551614usize & 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0usize & 18446744073709551615usize == 0usize);
/// assert!(18446744073709551614usize & 0usize == 0usize);
/// assert!(0usize & 0usize == 0usize);
/// assert!(18446744073709551614usize & 1usize == 0usize);
/// assert!(14778172773690198358usize & 7884039247110388294usize == 5548603809841809478usize);
/// assert!(10133644594672480117usize & 12490878339968335652usize == 10088203909240893220usize);
/// assert!(16171634679811104222usize & 14932315837698617289usize == 13846369844523896264usize);
/// assert!(11797262059014856943usize & 1828773968770610787usize == 81073675503411299usize);
/// # }
/// ```
pub fn core_ops_bit_and_usize_and() {}
/// # Properties for [`core::ops::BitAnd::<usize>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == usize::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize ^ 1usize == 1usize);
/// assert!(18446744073709551614usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(0usize ^ 0usize == 0usize);
/// assert!(18446744073709551615usize ^ 1usize == 18446744073709551615usize);
/// assert!(1usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(2832834282097625102usize ^ 13148428058288207043usize == 13220527708695329999usize);
/// assert!(7829667660607907236usize ^ 13404414057266399565usize == 18351776856820018669usize);
/// assert!(7103657770721151807usize ^ 10650283413492189309usize == 17572319378176723839usize);
/// assert!(3138769135692281547usize ^ 2225477056868334331usize == 4607041645726138107usize);
/// assert!(17579556644086068275usize ^ 158596343760034989usize == 17579646839613799615usize);
/// # }
/// ```
pub fn core_ops_bit_and_usize_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i8.cmp(&((-1i8))) == core::cmp::Ordering::Greater);
/// assert!((-128i8).cmp(&(0i8)) == core::cmp::Ordering::Less);
/// assert!(126i8.cmp(&((-1i8))) == core::cmp::Ordering::Greater);
/// assert!(126i8.cmp(&(0i8)) == core::cmp::Ordering::Greater);
/// assert!(1i8.cmp(&((-128i8))) == core::cmp::Ordering::Greater);
/// assert!(1i8.cmp(&((-1i8))) == core::cmp::Ordering::Greater);
/// assert!(95i8.cmp(&((-58i8))) == core::cmp::Ordering::Greater);
/// assert!(68i8.cmp(&(11i8)) == core::cmp::Ordering::Greater);
/// assert!((-93i8).cmp(&(45i8)) == core::cmp::Ordering::Less);
/// assert!((-57i8).cmp(&((-72i8))) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(126i8.lt(&((-128i8))) == false);
/// assert!((-128i8).lt(&((-1i8))) == true);
/// assert!(126i8.lt(&((-1i8))) == false);
/// assert!(1i8.lt(&(126i8)) == true);
/// assert!((-1i8).lt(&(1i8)) == true);
/// assert!((-1i8).lt(&((-128i8))) == false);
/// assert!((-35i8).lt(&(83i8)) == true);
/// assert!(58i8.lt(&(40i8)) == false);
/// assert!(105i8.lt(&(29i8)) == false);
/// assert!(117i8.lt(&((-12i8))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).gt(&(126i8)) == false);
/// assert!(126i8.gt(&((-127i8))) == true);
/// assert!(0i8.gt(&(127i8)) == false);
/// assert!(0i8.gt(&((-127i8))) == true);
/// assert!((-128i8).gt(&(127i8)) == false);
/// assert!((-127i8).gt(&(1i8)) == false);
/// assert!((-46i8).gt(&(114i8)) == false);
/// assert!((-39i8).gt(&((-127i8))) == true);
/// assert!(80i8.gt(&((-12i8))) == true);
/// assert!((-47i8).gt(&(68i8)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i8.ge(&(1i8)) == false);
/// assert!((-128i8).ge(&((-127i8))) == false);
/// assert!(126i8.ge(&((-128i8))) == true);
/// assert!((-1i8).ge(&(0i8)) == false);
/// assert!(127i8.ge(&((-128i8))) == true);
/// assert!(127i8.ge(&(127i8)) == true);
/// assert!(98i8.ge(&((-59i8))) == true);
/// assert!(44i8.ge(&((-49i8))) == true);
/// assert!(111i8.ge(&((-54i8))) == true);
/// assert!(7i8.ge(&((-100i8))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i8.le(&(1i8)) == true);
/// assert!(1i8.le(&(0i8)) == false);
/// assert!(127i8.le(&(127i8)) == true);
/// assert!(0i8.le(&(126i8)) == true);
/// assert!(0i8.le(&(0i8)) == true);
/// assert!((-128i8).le(&((-128i8))) == true);
/// assert!(118i8.le(&(58i8)) == false);
/// assert!(7i8.le(&(119i8)) == true);
/// assert!((-107i8).le(&(5i8)) == true);
/// assert!((-90i8).le(&((-100i8))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_le() {}
/// # Properties for [`core::ops::BitXor::<i8>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i8::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i8 ^ 126i8 == 126i8);
/// assert!(126i8 ^ (-127i8) == (-1i8));
/// assert!((-127i8) ^ (-127i8) == 0i8);
/// assert!((-128i8) ^ 1i8 == (-127i8));
/// assert!(1i8 ^ (-127i8) == (-128i8));
/// assert!((-127i8) ^ 0i8 == (-127i8));
/// assert!((-104i8) ^ (-83i8) == 53i8);
/// assert!(101i8 ^ (-81i8) == (-54i8));
/// assert!(0i8 ^ (-112i8) == (-112i8));
/// assert!(87i8 ^ 67i8 == 20i8);
/// # }
/// ```
pub fn core_ops_bit_xor_i8_xor() {}
/// # Properties for [`core::ops::BitAnd::<i8>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i8::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i8 & (-127i8) == 1i8);
/// assert!(1i8 & 127i8 == 1i8);
/// assert!((-127i8) & 1i8 == 1i8);
/// assert!(126i8 & 126i8 == 126i8);
/// assert!(126i8 & (-1i8) == 126i8);
/// assert!(1i8 & (-1i8) == 1i8);
/// assert!((-5i8) & (-76i8) == (-80i8));
/// assert!((-74i8) & 38i8 == 38i8);
/// assert!(11i8 & 104i8 == 8i8);
/// assert!((-128i8) & 53i8 == 0i8);
/// # }
/// ```
pub fn core_ops_bit_and_i8_and() {}
/// # Properties for [`core::ops::BitAnd::<i8>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i8::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i8 ^ 126i8 == 126i8);
/// assert!((-127i8) ^ (-127i8) == (-127i8));
/// assert!((-128i8) ^ 1i8 == (-127i8));
/// assert!((-128i8) ^ (-127i8) == (-127i8));
/// assert!(1i8 ^ 127i8 == 127i8);
/// assert!(1i8 ^ (-1i8) == (-1i8));
/// assert!(90i8 ^ 7i8 == 95i8);
/// assert!((-64i8) ^ (-61i8) == (-61i8));
/// assert!(30i8 ^ 70i8 == 94i8);
/// assert!((-57i8) ^ 29i8 == (-33i8));
/// # }
/// ```
pub fn core_ops_bit_and_i8_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i16.cmp(&(32767i16)) == core::cmp::Ordering::Less);
/// assert!(32767i16.cmp(&((-32768i16))) == core::cmp::Ordering::Greater);
/// assert!(32766i16.cmp(&((-32767i16))) == core::cmp::Ordering::Greater);
/// assert!((-32768i16).cmp(&(0i16)) == core::cmp::Ordering::Less);
/// assert!((-1i16).cmp(&(32767i16)) == core::cmp::Ordering::Less);
/// assert!(32767i16.cmp(&((-1i16))) == core::cmp::Ordering::Greater);
/// assert!((-15805i16).cmp(&(21033i16)) == core::cmp::Ordering::Less);
/// assert!((-25672i16).cmp(&((-982i16))) == core::cmp::Ordering::Less);
/// assert!((-25594i16).cmp(&((-3385i16))) == core::cmp::Ordering::Less);
/// assert!(1542i16.cmp(&(24176i16)) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32766i16.lt(&(1i16)) == false);
/// assert!(1i16.lt(&(32766i16)) == true);
/// assert!((-1i16).lt(&(1i16)) == true);
/// assert!((-32768i16).lt(&((-32768i16))) == false);
/// assert!((-1i16).lt(&((-32768i16))) == false);
/// assert!((-32767i16).lt(&((-1i16))) == true);
/// assert!((-32710i16).lt(&((-20835i16))) == true);
/// assert!(17630i16.lt(&((-20346i16))) == false);
/// assert!(6123i16.lt(&((-2720i16))) == false);
/// assert!((-30809i16).lt(&((-28860i16))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16.gt(&((-32767i16))) == true);
/// assert!(32766i16.gt(&(32767i16)) == false);
/// assert!((-32768i16).gt(&(0i16)) == false);
/// assert!(0i16.gt(&(32767i16)) == false);
/// assert!(0i16.gt(&(1i16)) == false);
/// assert!(1i16.gt(&(0i16)) == true);
/// assert!(20981i16.gt(&(12750i16)) == true);
/// assert!((-6358i16).gt(&(18856i16)) == false);
/// assert!(13427i16.gt(&((-15745i16))) == true);
/// assert!((-30850i16).gt(&(12464i16)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32767i16.ge(&(32766i16)) == true);
/// assert!(0i16.ge(&(1i16)) == false);
/// assert!((-32767i16).ge(&((-32768i16))) == true);
/// assert!((-1i16).ge(&(32767i16)) == false);
/// assert!(1i16.ge(&(0i16)) == true);
/// assert!((-32768i16).ge(&((-32767i16))) == false);
/// assert!((-31774i16).ge(&(4252i16)) == false);
/// assert!((-17305i16).ge(&((-10710i16))) == false);
/// assert!(5961i16.ge(&((-10239i16))) == true);
/// assert!(24405i16.ge(&(31248i16)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(32766i16.le(&((-1i16))) == false);
/// assert!(32767i16.le(&((-1i16))) == false);
/// assert!((-32768i16).le(&(32766i16)) == true);
/// assert!((-1i16).le(&(32766i16)) == true);
/// assert!((-32767i16).le(&(0i16)) == true);
/// assert!(32767i16.le(&((-32768i16))) == false);
/// assert!((-28411i16).le(&(18227i16)) == true);
/// assert!(27421i16.le(&((-24587i16))) == false);
/// assert!((-25041i16).le(&((-27980i16))) == false);
/// assert!((-14756i16).le(&(16418i16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_le() {}
/// # Properties for [`core::ops::BitXor::<i16>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i16::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i16 ^ 0i16 == 0i16);
/// assert!(32766i16 ^ 32766i16 == 0i16);
/// assert!(0i16 ^ 1i16 == 1i16);
/// assert!((-1i16) ^ (-32767i16) == 32766i16);
/// assert!(1i16 ^ (-32767i16) == (-32768i16));
/// assert!((-32767i16) ^ (-32768i16) == 1i16);
/// assert!((-22936i16) ^ 16265i16 == (-26143i16));
/// assert!(11964i16 ^ (-1254i16) == (-10842i16));
/// assert!(30300i16 ^ 21837i16 == 8977i16);
/// assert!(2600i16 ^ (-22971i16) == (-21395i16));
/// # }
/// ```
pub fn core_ops_bit_xor_i16_xor() {}
/// # Properties for [`core::ops::BitAnd::<i16>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i16::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i16) & 1i16 == 1i16);
/// assert!(0i16 & (-32768i16) == 0i16);
/// assert!((-1i16) & 32767i16 == 32767i16);
/// assert!((-32767i16) & 32767i16 == 1i16);
/// assert!(32767i16 & 1i16 == 1i16);
/// assert!((-32767i16) & (-32767i16) == (-32767i16));
/// assert!((-18225i16) & (-5970i16) == (-22386i16));
/// assert!(7489i16 & 3236i16 == 3072i16);
/// assert!(12556i16 & (-14396i16) == 260i16);
/// assert!((-28183i16) & 6370i16 == 4320i16);
/// # }
/// ```
pub fn core_ops_bit_and_i16_and() {}
/// # Properties for [`core::ops::BitAnd::<i16>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i16::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32767i16) ^ (-32767i16) == (-32767i16));
/// assert!(0i16 ^ 0i16 == 0i16);
/// assert!((-32767i16) ^ (-32768i16) == (-32767i16));
/// assert!(0i16 ^ (-32768i16) == (-32768i16));
/// assert!((-1i16) ^ (-32768i16) == (-1i16));
/// assert!((-32768i16) ^ (-1i16) == (-1i16));
/// assert!(2107i16 ^ (-12842i16) == (-12801i16));
/// assert!(14594i16 ^ 2886i16 == 15174i16);
/// assert!(17458i16 ^ 5291i16 == 21691i16);
/// assert!(28085i16 ^ (-16045i16) == (-4617i16));
/// # }
/// ```
pub fn core_ops_bit_and_i16_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(2147483647i32.cmp(&(2147483646i32)) == core::cmp::Ordering::Greater);
/// assert!(2147483646i32.cmp(&(2147483646i32)) == core::cmp::Ordering::Equal);
/// assert!(2147483646i32.cmp(&((-1i32))) == core::cmp::Ordering::Greater);
/// assert!((-1i32).cmp(&((-1i32))) == core::cmp::Ordering::Equal);
/// assert!((-2147483647i32).cmp(&((-2147483647i32))) == core::cmp::Ordering::Equal);
/// assert!(0i32.cmp(&(2147483646i32)) == core::cmp::Ordering::Less);
/// assert!((-1841407313i32).cmp(&((-805690071i32))) == core::cmp::Ordering::Less);
/// assert!(2092158650i32.cmp(&(936158697i32)) == core::cmp::Ordering::Greater);
/// assert!((-924353999i32).cmp(&((-1189684795i32))) == core::cmp::Ordering::Greater);
/// assert!(1344352567i32.cmp(&((-685708625i32))) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).lt(&(2147483646i32)) == true);
/// assert!((-1i32).lt(&((-2147483648i32))) == false);
/// assert!((-2147483648i32).lt(&((-1i32))) == true);
/// assert!(1i32.lt(&(1i32)) == false);
/// assert!((-2147483647i32).lt(&(2147483647i32)) == true);
/// assert!((-2147483647i32).lt(&((-2147483648i32))) == false);
/// assert!(935892884i32.lt(&((-2047909803i32))) == false);
/// assert!((-996145175i32).lt(&(1311757908i32)) == true);
/// assert!((-373691915i32).lt(&(292396281i32)) == true);
/// assert!(1345382681i32.lt(&((-1388471615i32))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(2147483646i32.gt(&(1i32)) == true);
/// assert!(0i32.gt(&((-2147483648i32))) == true);
/// assert!((-1i32).gt(&((-2147483647i32))) == true);
/// assert!((-1i32).gt(&((-1i32))) == false);
/// assert!(0i32.gt(&((-1i32))) == true);
/// assert!((-2147483647i32).gt(&(0i32)) == false);
/// assert!((-954526452i32).gt(&((-1068294872i32))) == true);
/// assert!(1441040744i32.gt(&(1460033662i32)) == false);
/// assert!((-1891805140i32).gt(&((-42800554i32))) == false);
/// assert!(1715780367i32.gt(&((-1441871136i32))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i32).ge(&(2147483646i32)) == false);
/// assert!((-1i32).ge(&(1i32)) == false);
/// assert!((-2147483647i32).ge(&((-2147483647i32))) == true);
/// assert!((-2147483648i32).ge(&(2147483646i32)) == false);
/// assert!((-2147483648i32).ge(&(1i32)) == false);
/// assert!(1i32.ge(&(1i32)) == true);
/// assert!((-1319662534i32).ge(&(816540786i32)) == false);
/// assert!(704566675i32.ge(&((-1905794690i32))) == true);
/// assert!((-1419149662i32).ge(&((-1831769202i32))) == true);
/// assert!((-775628535i32).ge(&(1087856419i32)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i32).le(&((-2147483648i32))) == false);
/// assert!((-2147483648i32).le(&(2147483646i32)) == true);
/// assert!(1i32.le(&((-1i32))) == false);
/// assert!(0i32.le(&((-1i32))) == false);
/// assert!(0i32.le(&(2147483646i32)) == true);
/// assert!((-2147483647i32).le(&((-2147483648i32))) == false);
/// assert!((-809108012i32).le(&((-997135955i32))) == false);
/// assert!(783250880i32.le(&(742157490i32)) == false);
/// assert!(905457089i32.le(&((-1823697522i32))) == false);
/// assert!(1766458942i32.le(&(970312186i32)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_le() {}
/// # Properties for [`core::ops::BitXor::<i32>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i32::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i32 ^ (-2147483648i32) == (-2147483647i32));
/// assert!((-2147483647i32) ^ 1i32 == (-2147483648i32));
/// assert!(2147483647i32 ^ (-2147483648i32) == (-1i32));
/// assert!((-1i32) ^ 1i32 == (-2i32));
/// assert!(2147483646i32 ^ (-2147483648i32) == (-2i32));
/// assert!((-2147483647i32) ^ (-2147483648i32) == 1i32);
/// assert!((-1965503738i32) ^ (-376883917i32) == 1666317877i32);
/// assert!(828474412i32 ^ 472461779i32 == 759739903i32);
/// assert!((-1206195611i32) ^ (-1049673569i32) == 2037754618i32);
/// assert!(1664445039i32 ^ (-1711529506i32) == (-87471183i32));
/// # }
/// ```
pub fn core_ops_bit_xor_i32_xor() {}
/// # Properties for [`core::ops::BitAnd::<i32>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i32::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(2147483647i32 & 2147483647i32 == 2147483647i32);
/// assert!((-2147483647i32) & 2147483646i32 == 0i32);
/// assert!((-2147483648i32) & 2147483647i32 == 0i32);
/// assert!(2147483647i32 & (-2147483648i32) == 0i32);
/// assert!((-1i32) & 2147483647i32 == 2147483647i32);
/// assert!(0i32 & (-2147483648i32) == 0i32);
/// assert!((-981398523i32) & (-750124370i32) == (-1056964604i32));
/// assert!((-1676092183i32) & 1880650364i32 == 270028904i32);
/// assert!(609029961i32 & 245521214i32 == 67111688i32);
/// assert!((-1898413277i32) & (-158320684i32) == (-2037382400i32));
/// # }
/// ```
pub fn core_ops_bit_and_i32_and() {}
/// # Properties for [`core::ops::BitAnd::<i32>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i32::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i32 ^ 0i32 == 0i32);
/// assert!((-2147483648i32) ^ (-1i32) == (-1i32));
/// assert!((-2147483647i32) ^ 1i32 == (-2147483647i32));
/// assert!((-2147483648i32) ^ (-2147483648i32) == (-2147483648i32));
/// assert!(2147483646i32 ^ (-1i32) == (-1i32));
/// assert!(0i32 ^ 2147483647i32 == 2147483647i32);
/// assert!((-682800106i32) ^ 1845792065i32 == (-11678377i32));
/// assert!((-1831785395i32) ^ (-1558312987i32) == (-1277218835i32));
/// assert!(1830819895i32 ^ 541971099i32 == 1835915967i32);
/// assert!(333773748i32 ^ (-636716237i32) == (-605226057i32));
/// # }
/// ```
pub fn core_ops_bit_and_i32_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0i64.cmp(&(9223372036854775807i64)) == core::cmp::Ordering::Less);
/// assert!(1i64.cmp(&((-9223372036854775807i64))) == core::cmp::Ordering::Greater);
/// assert!((-9223372036854775808i64).cmp(&(1i64)) == core::cmp::Ordering::Less);
/// assert!(9223372036854775807i64.cmp(&(9223372036854775807i64)) == core::cmp::Ordering::Equal);
/// assert!(9223372036854775806i64.cmp(&(1i64)) == core::cmp::Ordering::Greater);
/// assert!(0i64.cmp(&(1i64)) == core::cmp::Ordering::Less);
/// assert!((-2486683957138256793i64).cmp(&(8277996399471209820i64)) == core::cmp::Ordering::Less);
/// assert!((-99199504774803798i64).cmp(&((-1617081914795357605i64)))
///         == core::cmp::Ordering::Greater);
/// assert!(9085828950505881796i64.cmp(&((-8326902558348264350i64)))
///         == core::cmp::Ordering::Greater);
/// assert!((-6705476182731419152i64).cmp(&(3094347331573725334i64)) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807i64.lt(&((-1i64))) == false);
/// assert!((-1i64).lt(&((-1i64))) == false);
/// assert!((-9223372036854775808i64).lt(&((-9223372036854775808i64))) == false);
/// assert!((-9223372036854775808i64).lt(&(9223372036854775807i64)) == true);
/// assert!((-9223372036854775808i64).lt(&(1i64)) == true);
/// assert!(0i64.lt(&(1i64)) == true);
/// assert!(5202690087385865362i64.lt(&(348130219235861940i64)) == false);
/// assert!((-3286505900327282837i64).lt(&((-6377803531022525845i64))) == false);
/// assert!((-8597586077785756264i64).lt(&(7828347004338403788i64)) == true);
/// assert!((-5034467257694868919i64).lt(&(8481378235314876814i64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i64).gt(&(9223372036854775806i64)) == false);
/// assert!(1i64.gt(&((-1i64))) == true);
/// assert!(1i64.gt(&((-9223372036854775808i64))) == true);
/// assert!(0i64.gt(&(9223372036854775807i64)) == false);
/// assert!((-9223372036854775807i64).gt(&((-9223372036854775808i64))) == true);
/// assert!((-1i64).gt(&(9223372036854775807i64)) == false);
/// assert!(3914099828243489309i64.gt(&((-7747163929945588862i64))) == true);
/// assert!((-8520296868024038220i64).gt(&(5285590624491735952i64)) == false);
/// assert!(5668592473877312042i64.gt(&((-5503331472399879898i64))) == true);
/// assert!(6309098624095935966i64.gt(&(5417266259187744285i64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).ge(&(0i64)) == false);
/// assert!(1i64.ge(&((-9223372036854775807i64))) == true);
/// assert!((-1i64).ge(&(1i64)) == false);
/// assert!((-9223372036854775807i64).ge(&((-9223372036854775807i64))) == true);
/// assert!(9223372036854775806i64.ge(&(9223372036854775806i64)) == true);
/// assert!((-9223372036854775808i64).ge(&((-9223372036854775807i64))) == false);
/// assert!(5189165648240804646i64.ge(&(8586648522440340602i64)) == false);
/// assert!(1388985995802865504i64.ge(&(5531932904654938190i64)) == false);
/// assert!(4839208036706388068i64.ge(&((-8848078371469799059i64))) == true);
/// assert!((-8811398548020492121i64).ge(&(8440581603448837258i64)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).le(&((-9223372036854775807i64))) == true);
/// assert!((-9223372036854775808i64).le(&(0i64)) == true);
/// assert!(9223372036854775807i64.le(&((-1i64))) == false);
/// assert!((-1i64).le(&(1i64)) == true);
/// assert!(9223372036854775806i64.le(&(9223372036854775806i64)) == true);
/// assert!(0i64.le(&((-9223372036854775808i64))) == false);
/// assert!((-1104538011990182423i64).le(&((-5386354854814335397i64))) == false);
/// assert!((-2330934684629811539i64).le(&((-6642474454930927820i64))) == false);
/// assert!((-5954812395640351599i64).le(&(9043451177381984129i64)) == true);
/// assert!((-9180428913451038936i64).le(&((-428424559084246027i64))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_le() {}
/// # Properties for [`core::ops::BitXor::<i64>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i64::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) ^ (-1i64) == 9223372036854775807i64);
/// assert!((-1i64) ^ (-1i64) == 0i64);
/// assert!((-1i64) ^ 9223372036854775806i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) ^ 0i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) ^ 9223372036854775806i64 == (-1i64));
/// assert!((-9223372036854775808i64) ^ (-9223372036854775808i64) == 0i64);
/// assert!(1916035531361816407i64 ^ (-5204921179365209276i64) == (-5957338813278813165i64));
/// assert!(8993774278996390019i64 ^ (-5141749892080865109i64) == (-4290633324512018392i64));
/// assert!(4837529368006336505i64 ^ 7125702587374996059i64 == 2432453884482426274i64);
/// assert!(4073746090710141580i64 ^ 4055556265588434351i64 == 54290120998137635i64);
/// # }
/// ```
pub fn core_ops_bit_xor_i64_xor() {}
/// # Properties for [`core::ops::BitAnd::<i64>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i64::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) & 9223372036854775806i64 == 0i64);
/// assert!(9223372036854775807i64 & (-1i64) == 9223372036854775807i64);
/// assert!((-1i64) & (-9223372036854775808i64) == (-9223372036854775808i64));
/// assert!(9223372036854775806i64 & (-9223372036854775807i64) == 0i64);
/// assert!((-9223372036854775807i64) & (-9223372036854775808i64) == (-9223372036854775808i64));
/// assert!(9223372036854775807i64 & 1i64 == 1i64);
/// assert!(7454527184132466446i64 & 365699410391105037i64 == 365645512573216268i64);
/// assert!((-8989329472140371142i64) & 840941056875747384i64 == 228307441465333816i64);
/// assert!((-9115373784472672009i64) & 6265913979340039470i64 == 32844132482359334i64);
/// assert!((-6297148998346677838i64) & 6462531786607217204i64 == 615867564761071664i64);
/// # }
/// ```
pub fn core_ops_bit_and_i64_and() {}
/// # Properties for [`core::ops::BitAnd::<i64>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i64::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775806i64 ^ (-1i64) == (-1i64));
/// assert!(9223372036854775807i64 ^ (-9223372036854775808i64) == (-1i64));
/// assert!(9223372036854775806i64 ^ 1i64 == 9223372036854775807i64);
/// assert!((-9223372036854775807i64) ^ (-1i64) == (-1i64));
/// assert!(9223372036854775807i64 ^ 1i64 == 9223372036854775807i64);
/// assert!(9223372036854775806i64 ^ (-9223372036854775808i64) == (-2i64));
/// assert!((-4478010683244968794i64) ^ (-8280146063443110735i64) == (-3611909961554078537i64));
/// assert!(4812329125229995749i64 ^ (-5426976829893761129i64) == (-653070497420302345i64));
/// assert!(443038175373569922i64 ^ (-4179614274202717861i64) == (-4035226366440980517i64));
/// assert!(3400034960996581163i64 ^ (-2006192277246968181i64) == (-1211508118746300501i64));
/// # }
/// ```
pub fn core_ops_bit_and_i64_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105727i128.cmp(&(0i128))
///         == core::cmp::Ordering::Greater);
/// assert!((-1i128).cmp(&(0i128)) == core::cmp::Ordering::Less);
/// assert!(170141183460469231731687303715884105726i128
///         .cmp(&((-170141183460469231731687303715884105727i128)))
///         == core::cmp::Ordering::Greater);
/// assert!(1i128.cmp(&(170141183460469231731687303715884105727i128))
///         == core::cmp::Ordering::Less);
/// assert!(170141183460469231731687303715884105726i128.cmp(&(0i128))
///         == core::cmp::Ordering::Greater);
/// assert!(1i128.cmp(&((-170141183460469231731687303715884105727i128)))
///         == core::cmp::Ordering::Greater);
/// assert!((-154005287672288482704441483385659686472i128)
///         .cmp(&(105818934551093860653441722273120519267i128)) == core::cmp::Ordering::Less);
/// assert!((-128367413642486903913878523278718529067i128)
///         .cmp(&((-64386684455420354136778675416075506643i128)))
///         == core::cmp::Ordering::Less);
/// assert!(126908221513700786904155049784630929235i128
///         .cmp(&((-9807736245756341038743965012486688995i128)))
///         == core::cmp::Ordering::Greater);
/// assert!((-10145858363607021331634454013201364899i128)
///         .cmp(&((-94541209134846576164161079650382822652i128)))
///         == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1i128.lt(&(0i128)) == false);
/// assert!((-170141183460469231731687303715884105727i128).lt(&((-1i128))) == true);
/// assert!(0i128.lt(&((-1i128))) == false);
/// assert!((-1i128).lt(&((-170141183460469231731687303715884105728i128))) == false);
/// assert!((-170141183460469231731687303715884105727i128)
///         .lt(&(170141183460469231731687303715884105726i128)) == true);
/// assert!(170141183460469231731687303715884105726i128
///         .lt(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!(43494519805289624778098993664600114576i128
///         .lt(&(37154080335617078403397938962298082139i128)) == false);
/// assert!((-153522660583401227602419573355031608777i128)
///         .lt(&(26315143328660934995840236404587824943i128)) == true);
/// assert!((-85446390431431338173136522106685719943i128)
///         .lt(&((-43594033324359124220961588176967323834i128))) == true);
/// assert!(45465377380471421298298405612041857363i128
///         .lt(&((-152973691226334666963048011591363144284i128))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105727i128)
///         .gt(&(170141183460469231731687303715884105727i128)) == false);
/// assert!(170141183460469231731687303715884105727i128.gt(&(1i128)) == true);
/// assert!(1i128.gt(&(1i128)) == false);
/// assert!(170141183460469231731687303715884105727i128.gt(&(0i128)) == true);
/// assert!((-1i128).gt(&((-170141183460469231731687303715884105728i128))) == true);
/// assert!(1i128.gt(&(170141183460469231731687303715884105726i128)) == false);
/// assert!((-168224518223778796411925806744106157139i128)
///         .gt(&(90488014374171457124259142180165163249i128)) == false);
/// assert!((-144204527530455254532306780851552032585i128)
///         .gt(&(4677638839937728979555865463169347192i128)) == false);
/// assert!((-126682904067865959319599376777694877503i128)
///         .gt(&(73563520646578614602393507597805407307i128)) == false);
/// assert!((-64436417572853670665172049096587459243i128)
///         .gt(&(130260691661457861683895436324592121286i128)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .ge(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!(1i128.ge(&(170141183460469231731687303715884105726i128)) == false);
/// assert!(170141183460469231731687303715884105726i128.ge(&(0i128)) == true);
/// assert!(1i128.ge(&(0i128)) == true);
/// assert!((-170141183460469231731687303715884105727i128).ge(&(0i128)) == false);
/// assert!(170141183460469231731687303715884105726i128.ge(&((-1i128))) == true);
/// assert!(65225598775347068974441282010745471809i128
///         .ge(&((-88598611435040776141686556543587509113i128))) == true);
/// assert!((-17255447653047990277350209539298500020i128)
///         .ge(&((-124625914073732369224663105049758509891i128))) == true);
/// assert!(36960009758739173754427666931105017954i128
///         .ge(&((-116535350064549903866387028381304492526i128))) == true);
/// assert!(158399894553718110212841247075823117625i128
///         .ge(&(36766367793339548227741183508187394298i128)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105726i128.le(&(1i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .le(&((-170141183460469231731687303715884105727i128))) == true);
/// assert!((-170141183460469231731687303715884105727i128)
///         .le(&(170141183460469231731687303715884105727i128)) == true);
/// assert!(170141183460469231731687303715884105726i128.le(&(0i128)) == false);
/// assert!(170141183460469231731687303715884105726i128
///         .le(&(170141183460469231731687303715884105727i128)) == true);
/// assert!(1i128.le(&(1i128)) == true);
/// assert!((-105983859324968750808104540289791978991i128)
///         .le(&((-108206891368088591186818757067992248961i128))) == false);
/// assert!(29151295634163322121778123342254222493i128
///         .le(&((-97452641186557571953861981070014384647i128))) == false);
/// assert!((-83803146929455115818183204681317724575i128)
///         .le(&(96547409580339960230828256369078193884i128)) == true);
/// assert!((-148899579028549076558552846115914259341i128)
///         .le(&((-110533491587967559700123238870981889461i128))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_le() {}
/// # Properties for [`core::ops::BitXor::<i128>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i128::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105727i128 ^ 0i128
///         == 170141183460469231731687303715884105727i128);
/// assert!((-170141183460469231731687303715884105727i128) ^ 1i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-1i128) ^ (-170141183460469231731687303715884105728i128)
///         == 170141183460469231731687303715884105727i128);
/// assert!((-170141183460469231731687303715884105727i128) ^ 0i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!(0i128 ^ 170141183460469231731687303715884105726i128
///         == 170141183460469231731687303715884105726i128);
/// assert!(170141183460469231731687303715884105727i128 ^ (-1i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!(77296362785470245240239181943133653598i128
///         ^ 90260007152270549564515108336526189875i128
///         == 161842859019710712841746717051533602669i128);
/// assert!((-147389228238145991813936313686024521410i128)
///         ^ (-90215486678290606517857329198194672648i128)
///         == 60131927158012058215459321311372952262i128);
/// assert!((-126854812254339674004734468123976960430i128)
///         ^ 112755464497414256173281726983229082166i128
///         == (-15601244421691506544746098691698951068i128));
/// assert!((-28229598495790211054142589152522933926i128)
///         ^ (-64693547693464455773280668096145004060i128)
///         == 49967186572710045017535297915333509310i128);
/// # }
/// ```
pub fn core_ops_bit_xor_i128_xor() {}
/// # Properties for [`core::ops::BitAnd::<i128>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i128::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1i128) & 1i128 == 1i128);
/// assert!((-1i128) & 170141183460469231731687303715884105727i128
///         == 170141183460469231731687303715884105727i128);
/// assert!(170141183460469231731687303715884105726i128 & 1i128 == 0i128);
/// assert!((-170141183460469231731687303715884105728i128) & (-1i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-1i128) & (-1i128) == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128) & 1i128 == 1i128);
/// assert!(154695490663795409770121942640604471083i128
///         & (-145582463970262384582047911060817748440i128)
///         == 21772680566135387911335799534402493992i128);
/// assert!((-88403602802597359052606455710332301378i128)
///         & 41371381080121395152679576281208977860i128
///         == 38703797840740798936709838641392830852i128);
/// assert!((-65291035525740609298238226626454047734i128)
///         & (-141018668805168117283464647281179275988i128)
///         == (-163659853247812306705952732034430401528i128));
/// assert!(76217416777828486393810534378149827217i128
///         & 93460291439715929502333534570042622556i128
///         == 367355003225598853405399709237903888i128);
/// # }
/// ```
pub fn core_ops_bit_and_i128_and() {}
/// # Properties for [`core::ops::BitAnd::<i128>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i128::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(170141183460469231731687303715884105726i128 ^ 0i128
///         == 170141183460469231731687303715884105726i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         ^ 170141183460469231731687303715884105727i128 == (-1i128));
/// assert!((-1i128) ^ (-170141183460469231731687303715884105727i128) == (-1i128));
/// assert!(0i128 ^ (-1i128) == (-1i128));
/// assert!(170141183460469231731687303715884105726i128 ^ (-1i128) == (-1i128));
/// assert!(170141183460469231731687303715884105726i128 ^ 1i128
///         == 170141183460469231731687303715884105727i128);
/// assert!(119109995725715496942246345691141015393i128
///         ^ 41767807037035115272204278512555655794i128
///         == 127605561957685810874923520655011907443i128);
/// assert!((-168223408820056722126841639627103152290i128)
///         ^ (-155661855814967702577720444169672806239i128)
///         == (-154243060143464001539824506153301706753i128));
/// assert!((-130593788546634298243011034497337031041i128)
///         ^ (-145514247264540169775499533382967230229i128)
///         == (-127901894239701688901147298676792654081i128));
/// assert!(80254761068198211476459871605497783356i128
///         ^ (-151401035702648109077697168342431681475i128)
///         == (-87097012349345547037158931770293619651i128));
/// # }
/// ```
pub fn core_ops_bit_and_i128_or() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1isize).cmp(&(0isize)) == core::cmp::Ordering::Less);
/// assert!(9223372036854775806isize.cmp(&(9223372036854775806isize))
///         == core::cmp::Ordering::Equal);
/// assert!(9223372036854775807isize.cmp(&((-1isize))) == core::cmp::Ordering::Greater);
/// assert!((-9223372036854775808isize).cmp(&(1isize)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&(9223372036854775806isize))
///         == core::cmp::Ordering::Less);
/// assert!((-1isize).cmp(&((-9223372036854775807isize))) == core::cmp::Ordering::Greater);
/// assert!((-5841171655339116679isize).cmp(&((-169589993757841491isize)))
///         == core::cmp::Ordering::Less);
/// assert!((-8024312756574099332isize).cmp(&((-5737577747031126721isize)))
///         == core::cmp::Ordering::Less);
/// assert!((-1013218334373152959isize).cmp(&((-6479110904713568964isize)))
///         == core::cmp::Ordering::Greater);
/// assert!((-9022977326710961713isize).cmp(&((-2963655115610600578isize)))
///         == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1isize).lt(&(0isize)) == true);
/// assert!(9223372036854775807isize.lt(&(0isize)) == false);
/// assert!((-9223372036854775808isize).lt(&(1isize)) == true);
/// assert!((-1isize).lt(&(1isize)) == true);
/// assert!(9223372036854775806isize.lt(&(1isize)) == false);
/// assert!((-1isize).lt(&((-9223372036854775808isize))) == false);
/// assert!((-5139041277011024099isize).lt(&((-7317843293651173999isize))) == false);
/// assert!(5077816068670392766isize.lt(&(5067344170236875232isize)) == false);
/// assert!((-8596729718638767984isize).lt(&(55412216169124994isize)) == true);
/// assert!((-5851885403838318985isize).lt(&((-6627613335458748828isize))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-1isize).gt(&((-9223372036854775807isize))) == true);
/// assert!(0isize.gt(&(9223372036854775807isize)) == false);
/// assert!(9223372036854775806isize.gt(&(9223372036854775807isize)) == false);
/// assert!((-1isize).gt(&(9223372036854775807isize)) == false);
/// assert!(9223372036854775806isize.gt(&(1isize)) == true);
/// assert!((-9223372036854775808isize).gt(&(1isize)) == false);
/// assert!((-1082288054928822402isize).gt(&((-8847817907211995231isize))) == true);
/// assert!((-2086377303770995526isize).gt(&(860152641069224317isize)) == false);
/// assert!((-2383852312148689394isize).gt(&((-5847113626420128830isize))) == true);
/// assert!((-2495679267112393867isize).gt(&(2164974843146624639isize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775806isize.ge(&((-9223372036854775807isize))) == true);
/// assert!(0isize.ge(&((-9223372036854775807isize))) == true);
/// assert!(9223372036854775806isize.ge(&((-9223372036854775808isize))) == true);
/// assert!(1isize.ge(&(0isize)) == true);
/// assert!((-1isize).ge(&(9223372036854775806isize)) == false);
/// assert!(0isize.ge(&(9223372036854775806isize)) == false);
/// assert!(6523650201985718072isize.ge(&(4892136114372110246isize)) == true);
/// assert!(2285470940906032718isize.ge(&((-2217705416837401622isize))) == true);
/// assert!((-5030959652642427281isize).ge(&(8048897448355091986isize)) == false);
/// assert!(1461826850809078831isize.ge(&(4636778294436801672isize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0isize.le(&(9223372036854775806isize)) == true);
/// assert!(0isize.le(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775807isize).le(&(1isize)) == true);
/// assert!((-9223372036854775807isize).le(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775807isize).le(&(0isize)) == true);
/// assert!((-9223372036854775808isize).le(&((-9223372036854775807isize))) == true);
/// assert!((-8576196583529848545isize).le(&((-4707748371708032498isize))) == true);
/// assert!((-7600656040235001908isize).le(&(5598566064688250776isize)) == true);
/// assert!(6393688347728473964isize.le(&((-8438421120291356802isize))) == false);
/// assert!(4276659997523259615isize.le(&(7294019476836522781isize)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_le() {}
/// # Properties for [`core::ops::BitXor::<isize>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == isize::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1isize ^ 0isize == 1isize);
/// assert!(9223372036854775807isize ^ (-9223372036854775808isize) == (-1isize));
/// assert!((-1isize) ^ 1isize == (-2isize));
/// assert!((-1isize) ^ (-9223372036854775807isize) == 9223372036854775806isize);
/// assert!((-1isize) ^ 9223372036854775806isize == (-9223372036854775807isize));
/// assert!(1isize ^ (-9223372036854775808isize) == (-9223372036854775807isize));
/// assert!((-1250290473846053667isize) ^ (-5531171348753902369isize) == 6745063510149787650isize);
/// assert!(2404659271098898348isize ^ 7313406798223135084isize == 4909311042728183488isize);
/// assert!(5859673763593276063isize ^ (-4466438813660454551isize) == (-7830146021812917258isize));
/// assert!(973436433748834533isize ^ 8590187685068976548isize == 8841741896347617601isize);
/// # }
/// ```
pub fn core_ops_bit_xor_isize_xor() {}
/// # Properties for [`core::ops::BitAnd::<isize>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == isize::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) & 9223372036854775807isize == 0isize);
/// assert!((-1isize) & (-1isize) == (-1isize));
/// assert!((-1isize) & 0isize == 0isize);
/// assert!(1isize & (-9223372036854775807isize) == 1isize);
/// assert!(0isize & 0isize == 0isize);
/// assert!(9223372036854775806isize & 9223372036854775807isize == 9223372036854775806isize);
/// assert!((-3116102647992771067isize) & 5924139994317710229isize == 5764679157732377093isize);
/// assert!((-3666504517851182454isize) & (-1778719870413368240isize)
///         == (-4246694813697245184isize));
/// assert!((-8363742715638563772isize) & (-4080858321313711476isize)
///         == (-8985806812359804924isize));
/// assert!((-5765075418812885518isize) & (-452413953302846885isize)
///         == (-6217199081986457518isize));
/// # }
/// ```
pub fn core_ops_bit_and_isize_and() {}
/// # Properties for [`core::ops::BitAnd::<isize>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == isize::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(9223372036854775807isize ^ (-9223372036854775808isize) == (-1isize));
/// assert!((-9223372036854775808isize) ^ (-9223372036854775807isize)
///         == (-9223372036854775807isize));
/// assert!(0isize ^ 0isize == 0isize);
/// assert!(9223372036854775806isize ^ 1isize == 9223372036854775807isize);
/// assert!((-1isize) ^ 9223372036854775807isize == (-1isize));
/// assert!(1isize ^ 1isize == 1isize);
/// assert!(4393958222361154664isize ^ 5149538756658639374isize == 9223090283707561582isize);
/// assert!((-4185871319422495601isize) ^ 926904095775268741isize == (-3603497079892369521isize));
/// assert!(4756165041642936274isize ^ 1131869617483972774isize == 5743637568456660982isize);
/// assert!((-1799399568822941922isize) ^ 8413847971207197461isize == (-592223734325872865isize));
/// # }
/// ```
pub fn core_ops_bit_and_isize_or() {}
